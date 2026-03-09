pub mod geometry;
pub mod linalg;
mod number_theory;

pub use number_theory::factorization::Sieve;

pub use crate::z_modulo_k_macro as z_modulo_k;
use std::fmt::Formatter;

pub trait Arithmetic:
    std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::iter::Sum
    + Default
    + Copy
    + Sized
{
    fn zero() -> Self;
    fn one() -> Self;
}

macro_rules! impl_arithmetic{
    ($($t:ty)*) =>{
        $(impl Arithmetic for $t{
            fn zero() -> Self{
                0 as $t
            }

            fn one() -> Self{
                1 as $t
            }
        })*
    }
}

impl_arithmetic! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64}

#[derive(Copy, Clone)]
pub struct ZModuloK {
    pub modulo: Option<usize>,
    pub val: usize,
}

impl ZModuloK {
    pub fn new(modulo: usize, val: usize) -> Self {
        if modulo == 0 {
            panic!("ZModuloK new : modulo cannot be zero");
        }
        let val = (modulo + (val % modulo)) % modulo;
        Self {
            modulo: Some(modulo),
            val,
        }
    }
}
#[macro_export]
#[allow(unused_macros)]
macro_rules! z_modulo_k_macro {
    ($modulo: expr, $val: expr) => {
        ZModuloK::new($modulo as usize, $val as usize)
    };
}

impl Default for ZModuloK {
    fn default() -> Self {
        ZModuloK {
            modulo: None,
            val: 0,
        }
    }
}

impl std::ops::Add for ZModuloK {
    type Output = ZModuloK;
    fn add(self, rhs: Self) -> Self::Output {
        let modulo = if self.modulo.is_none() && rhs.modulo.is_none() {
            panic!("ZModulo add : One of left modulo or right modulo should not be none.")
        } else if self.modulo.is_none() {
            rhs.modulo
        } else if rhs.modulo.is_none() {
            self.modulo
        } else {
            if self.modulo != rhs.modulo {
                panic!("ZModuloK add : left modulo and right modulo are not same");
            }
            self.modulo
        };

        Self {
            modulo,
            val: (self.val + rhs.val) % modulo.unwrap(),
        }
    }
}

impl std::ops::Sub for ZModuloK {
    type Output = ZModuloK;
    fn sub(self, rhs: Self) -> Self::Output {
        let modulo = if self.modulo.is_none() && rhs.modulo.is_none() {
            panic!("ZModulo sub : One of left modulo or right modulo should not be none.")
        } else if self.modulo.is_none() {
            rhs.modulo
        } else if rhs.modulo.is_none() {
            self.modulo
        } else {
            if self.modulo != rhs.modulo {
                panic!("ZModuloK sub : left modulo and right modulo are not same");
            }
            self.modulo
        };

        Self {
            modulo,
            val: (self.val - rhs.val + modulo.unwrap()) % modulo.unwrap(),
        }
    }
}

impl std::ops::Mul for ZModuloK {
    type Output = ZModuloK;
    fn mul(self, rhs: Self) -> Self::Output {
        let modulo = if self.modulo.is_none() && rhs.modulo.is_none() {
            panic!("ZModulo mul : One of left modulo or right modulo should not be none.")
        } else if self.modulo.is_none() {
            rhs.modulo
        } else if rhs.modulo.is_none() {
            self.modulo
        } else {
            if self.modulo != rhs.modulo {
                panic!("ZModuloK mul : left modulo and right modulo are not same");
            }
            self.modulo
        };

        Self {
            modulo,
            val: (self.val * rhs.val) % modulo.unwrap(),
        }
    }
}

impl std::iter::Sum for ZModuloK {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = Self::default();
        for item in iter {
            res = res + item;
        }
        res
    }
}

impl Arithmetic for ZModuloK {
    fn zero() -> Self {
        Self {
            modulo: None,
            val: 0,
        }
    }

    fn one() -> Self {
        Self {
            modulo: None,
            val: 1,
        }
    }
}

impl std::fmt::Debug for ZModuloK {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.val)
    }
}

pub struct FactorIterator<T: NaturalNumber> {
    pub n: T,
    factors: Vec<T>,
    sqrt: Option<T>,
    index: usize,
}

impl<T: NaturalNumber> FactorIterator<T> {
    fn new(n: T) -> Self {
        let mut curr = T::one();
        let mut factors = Vec::new();
        let mut sqrt = None;
        while curr * curr <= n {
            if n.divided_by(curr).unwrap() {
                factors.push(curr);
            }
            curr = curr + T::one();
            if curr * curr == n {
                sqrt = Some(curr);
            }
        }
        FactorIterator {
            n,
            factors,
            sqrt,
            index: 0,
        }
    }
}

impl<T: NaturalNumber> Iterator for FactorIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let cnt = self.len();
        if cnt == 0 {
            return None;
        }
        let res = if cnt - 1 < self.factors.len() {
            Some(self.n / self.factors[cnt - 1])
        } else {
            Some(self.factors[self.index])
        };
        self.index += 1;
        res
    }
}

impl<T: NaturalNumber> ExactSizeIterator for FactorIterator<T> {
    fn len(&self) -> usize
    where
        Self: Sized,
    {
        let mut ind = 2 * self.factors.len();
        if self.sqrt.is_some() {
            ind -= 1;
        }
        ind - self.index
    }
}

fn _gcd<T: NaturalNumber>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

pub trait NaturalNumber:
    Sized
    + Copy
    + Clone
    + Arithmetic
    + std::ops::Div<Output = Self>
    + std::ops::Rem<Output = Self>
    + PartialOrd
    + Ord
{
    #[inline]
    fn divided_by(&self, rhs: Self) -> Result<bool, String> {
        if rhs == Self::zero() {
            return Err("NaturalNumber : Cannot divide by zero".to_string());
        }
        let divisor = *self / rhs;
        if *self - (rhs * divisor) == Self::zero() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[inline]
    fn remainder(&self, rhs: Self) -> Result<Self, String> {
        if rhs == Self::zero() {
            return Err("NaturalNumber : Cannot divide by zero".to_string());
        }
        let divisor = *self / rhs;
        Ok(*self - (rhs * divisor))
    }

    fn check_overflow_mul(&self, rhs: Self) -> Option<Self>;

    fn factors(&self) -> FactorIterator<Self> {
        FactorIterator::new(*self)
    }

    fn gcd<I>(mut nums: impl Iterator<Item = I>) -> Self
    where
        I: std::borrow::Borrow<Self>,
    {
        let first = match nums.next() {
            Some(val) => *val.borrow(),
            None => return Self::zero(),
        };
        nums.fold(first, |acc, x| _gcd(acc, *x.borrow()))
    }

    fn lcm<I>(mut nums: impl Iterator<Item = I>) -> Self
    where
        I: std::borrow::Borrow<Self>,
    {
        let first = match nums.next() {
            Some(val) => *val.borrow(),
            None => return Self::zero(),
        };
        nums.fold(first, |acc, x| {
            let val = *x.borrow();
            let g = _gcd(acc, val);
            if g == Self::zero() {
                Self::zero()
            } else {
                (acc / g)
                    .check_overflow_mul(val)
                    .expect("Overflow occurred while calculating lcm")
            }
        })
    }

    fn is_prime(&self) -> bool {
        let it = FactorIterator::new(*self);
        if it.len() == 1 { true } else { false }
    }

    fn powmod(&self, pow: Self, p: Self) -> Self {
        let mut ret = Self::one();
        let mut base = *self;
        let mut exp = pow;
        let two = Self::one() + Self::one();
        while exp > Self::zero() {
            if exp % two == Self::one() {
                ret = (ret * base) % p;
            }
            base = (base * base) % p;
            exp = exp / two;
        }
        ret
    }

    fn inv(&self, p: Self) -> Self {
        if !p.is_prime() {
            panic!("Number is not prime");
        }
        self.inv_unchecked(p)
    }

    fn inv_unchecked(&self, p: Self) -> Self {
        let p_minus_two = p - Self::one() - Self::one();
        self.powmod(p_minus_two, p)
    }
}

impl NaturalNumber for usize {
    fn check_overflow_mul(&self, rhs: Self) -> Option<Self> {
        self.checked_mul(rhs)
    }
}

impl NaturalNumber for u8 {
    fn check_overflow_mul(&self, rhs: Self) -> Option<Self> {
        self.checked_mul(rhs)
    }
}
impl NaturalNumber for u16 {
    fn check_overflow_mul(&self, rhs: Self) -> Option<Self> {
        self.checked_mul(rhs)
    }
}
impl NaturalNumber for u32 {
    fn check_overflow_mul(&self, rhs: Self) -> Option<Self> {
        self.checked_mul(rhs)
    }
}
impl NaturalNumber for u64 {
    fn check_overflow_mul(&self, rhs: Self) -> Option<Self> {
        self.checked_mul(rhs)
    }
}
