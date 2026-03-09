use std::cmp::{Eq, PartialEq};
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

#[derive(Clone, Hash)]
pub struct Bitset {
    state: Vec<u64>,
    capacity: usize,
}

impl Bitset {
    #[inline]
    pub fn new(capacity: usize) -> Self {
        let blocks = (capacity + 63) >> 6;
        Bitset {
            state: vec![0; blocks.max(1)],
            capacity: blocks,
        }
    }

    #[inline]
    pub fn insert(&mut self, position: usize) {
        let idx = position >> 6;
        if idx < self.state.len() {
            let offset = position & 63;
            self.state[idx] |= 1_u64 << offset;
        }
    }

    #[inline]
    pub fn pop(&mut self, position: usize) {
        let idx = position >> 6;
        if idx < self.state.len() {
            let offset = position & 63;
            self.state[idx] &= !(1_u64 << offset);
        }
    }

    #[inline]
    pub fn has(&self, position: usize) -> bool {
        let idx = position >> 6;
        if idx < self.state.len() {
            let offset = position & 63;
            (self.state[idx] >> offset) & 1 == 1
        } else {
            false
        }
    }

    #[inline]
    pub fn items(&self) -> Vec<usize> {
        let mut v = Vec::new();
        for (idx, &block) in self.state.iter().enumerate() {
            let mut b = block;
            while b != 0 {
                let j = b.trailing_zeros();
                v.push((idx << 6) + j as usize);
                b &= b - 1;
            }
        }
        v
    }
}

impl ShlAssign<usize> for Bitset {
    #[inline]
    fn shl_assign(&mut self, rhs: usize) {
        if rhs == 0 {
            return;
        }
        let rotate_offset = rhs >> 6;
        let actual_shift = rhs & 63;
        let len = self.state.len();

        if rotate_offset >= len {
            self.state.fill(0);
            return;
        }

        if rotate_offset > 0 {
            self.state
                .copy_within(0..(len - rotate_offset), rotate_offset);
            self.state[0..rotate_offset].fill(0);
        }

        if actual_shift > 0 {
            let inv_shift = 64 - actual_shift;
            for i in (rotate_offset..len).rev() {
                let current = self.state[i];
                let next = if i > rotate_offset {
                    self.state[i - 1]
                } else {
                    0
                };
                self.state[i] = (current << actual_shift) | (next >> inv_shift);
            }
        }
    }
}

impl Shl<usize> for Bitset {
    type Output = Self;

    #[inline]
    fn shl(mut self, rhs: usize) -> Self {
        self.shl_assign(rhs);
        self
    }
}

impl Shl<usize> for &Bitset {
    type Output = Bitset;

    #[inline]
    fn shl(self, rhs: usize) -> Self::Output {
        let mut ret = self.clone();
        ret.shl_assign(rhs);
        ret
    }
}

impl ShrAssign<usize> for Bitset {
    #[inline]
    fn shr_assign(&mut self, rhs: usize) {
        if rhs == 0 {
            return;
        }
        let rotate_offset = rhs >> 6;
        let actual_shift = rhs & 63;
        let len = self.state.len();

        if rotate_offset >= len {
            self.state.fill(0);
            return;
        }

        if rotate_offset > 0 {
            self.state.copy_within(rotate_offset..len, 0);
            self.state[(len - rotate_offset)..len].fill(0);
        }

        if actual_shift > 0 {
            let inv_shift = 64 - actual_shift;
            let end = len - rotate_offset;
            for i in 0..end {
                let current = self.state[i];
                let next = if i + 1 < end { self.state[i + 1] } else { 0 };
                self.state[i] = (current >> actual_shift) | (next << inv_shift);
            }
        }
    }
}

impl Shr<usize> for Bitset {
    type Output = Self;

    #[inline]
    fn shr(mut self, rhs: usize) -> Self {
        self.shr_assign(rhs);
        self
    }
}

impl Shr<usize> for &Bitset {
    type Output = Bitset;

    #[inline]
    fn shr(self, rhs: usize) -> Self::Output {
        let mut ret = self.clone();
        ret.shr_assign(rhs);
        ret
    }
}

impl PartialEq<Self> for Bitset {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let min_len = self.state.len().min(other.state.len());
        if self.state[..min_len] != other.state[..min_len] {
            return false;
        }
        if self.state.len() > min_len {
            self.state[min_len..].iter().all(|&x| x == 0)
        } else {
            other.state[min_len..].iter().all(|&x| x == 0)
        }
    }
}

impl Eq for Bitset {}

impl Not for Bitset {
    type Output = Self;

    #[inline]
    fn not(mut self) -> Self {
        self.state.iter_mut().for_each(|x| *x = !*x);
        self
    }
}

impl Not for &Bitset {
    type Output = Bitset;

    #[inline]
    fn not(self) -> Self::Output {
        let mut ret = self.clone();
        ret.state.iter_mut().for_each(|x| *x = !*x);
        ret
    }
}

impl BitAndAssign for Bitset {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.state
            .iter_mut()
            .zip(rhs.state.iter())
            .for_each(|(l, r)| *l &= *r);
    }
}

impl BitAnd for Bitset {
    type Output = Self;

    #[inline]
    fn bitand(mut self, rhs: Self) -> Self {
        self.bitand_assign(rhs);
        self
    }
}

impl BitAnd for &Bitset {
    type Output = Bitset;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        let mut ret = self.clone();
        ret.bitand_assign(rhs.clone());
        ret
    }
}

impl BitOrAssign for Bitset {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.state
            .iter_mut()
            .zip(rhs.state.iter())
            .for_each(|(l, r)| *l |= *r);
    }
}

impl BitOr for Bitset {
    type Output = Self;

    #[inline]
    fn bitor(mut self, rhs: Self) -> Self {
        self.bitor_assign(rhs);
        self
    }
}

impl BitOr for &Bitset {
    type Output = Bitset;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut ret = self.clone();
        ret.bitor_assign(rhs.clone());
        ret
    }
}

impl BitXorAssign for Bitset {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.state
            .iter_mut()
            .zip(rhs.state.iter())
            .for_each(|(l, r)| *l ^= *r);
    }
}

impl BitXor for Bitset {
    type Output = Self;

    #[inline]
    fn bitxor(mut self, rhs: Self) -> Self {
        self.bitxor_assign(rhs);
        self
    }
}

impl BitXor for &Bitset {
    type Output = Bitset;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut ret = self.clone();
        ret.bitxor_assign(rhs.clone());
        ret
    }
}

impl std::fmt::Debug for Bitset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        self.state
            .iter()
            .rev()
            .for_each(|x| s.push_str(&format!("{:064b}", x)));
        f.debug_struct("Bitset").field("state", &s).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitset_basic() {
        let mut bs = Bitset::new(100);
        assert_eq!(bs.has(10), false);
        bs.insert(10);
        assert_eq!(bs.has(10), true);
        bs.pop(10);
        assert_eq!(bs.has(10), false);
    }

    #[test]
    fn test_bitset_items1() {
        let mut bs = Bitset::new(100);
        bs.insert(5);
        bs.insert(65);
        let items = bs.items();
        assert_eq!(items, vec![5, 65]);
    }

    #[test]
    fn test_bitset_items2() {
        let mut bs = Bitset::new(100);
        (0..100).for_each(|i| bs.insert(i));
        let items = bs.items();
        assert_eq!(items.len(), 100);
        assert_eq!(items, (0..100).collect::<Vec<_>>());
    }

    #[test]
    fn test_bitset_shl() {
        let mut bs = Bitset::new(100);
        bs.insert(5);
        bs <<= 10;
        assert_eq!(bs.has(15), true);
        assert_eq!(bs.has(5), false);

        bs <<= 60;
        assert!(bs.has(75));

        let mut bs2 = Bitset::new(100);
        bs2.insert(63);
        bs2 <<= 1;
        assert!(bs2.has(64));
    }

    #[test]
    fn test_bitset_shr() {
        let mut bs = Bitset::new(100);
        bs.insert(10);
        bs >>= 80;
        assert_eq!(bs.items().len(), 0);

        let mut bs2 = Bitset::new(100);
        bs2.insert(64);
        bs2 >>= 1;
        assert!(bs2.has(63));
    }

    #[test]
    fn test_bitwise_ops() {
        let mut bs1 = Bitset::new(100);
        let mut bs2 = Bitset::new(100);
        bs1.insert(5);
        bs1.insert(10);
        bs2.insert(10);
        bs2.insert(15);

        let mut and_bs = bs1.clone();
        and_bs &= bs2.clone();
        assert_eq!(and_bs.items(), vec![10]);

        let mut or_bs = bs1.clone();
        or_bs |= bs2.clone();
        assert_eq!(or_bs.items(), vec![5, 10, 15]);

        let mut xor_bs = bs1.clone();
        xor_bs ^= bs2.clone();
        assert_eq!(xor_bs.items(), vec![5, 15]);

        let not_bs = !bs1.clone();
        assert_eq!(not_bs.has(5), false);
        assert_eq!(not_bs.has(10), false);
        assert_eq!(not_bs.has(0), true);
        assert_eq!(not_bs.has(1), true);
    }
}
