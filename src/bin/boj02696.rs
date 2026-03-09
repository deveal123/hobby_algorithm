use algorithm::io::{Reader, Writer};
use std::fmt::Debug;
use std::{collections::BinaryHeap, ops::Neg};

struct DoubleSideHeap<T: Ord + Neg<Output = T> + Copy + Clone + Eq> {
    left: BinaryHeap<T>,
    right: BinaryHeap<T>,
}

impl<T: Ord + Debug + Neg<Output = T> + Copy + Clone + Eq> DoubleSideHeap<T> {
    pub fn new() -> Self {
        DoubleSideHeap {
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
        }
    }

    pub fn insert(&mut self, num: T) {
        let (ml, mr) = (
            *self.left.peek().get_or_insert(&num),
            *self.right.peek().get_or_insert(&num),
        );
        if num < *ml {
            self.left.push(num);
        } else {
            self.right.push(-num);
        }

        if self.left.len() > self.right.len() + 1 {
            self.right.push(-self.left.pop().unwrap());
        } else if self.right.len() > self.left.len() + 1 {
            self.left.push(-self.right.pop().unwrap());
        }
    }

    pub fn get_median(&self) -> Option<T> {
        if self.left.len() > self.right.len() {
            Some(*self.left.peek().unwrap())
        } else {
            match self.right.peek() {
                Some(v) => Some(-*v),
                None => None,
            }
        }
    }
}

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let tc: usize = r.next();

    (0..tc).for_each(|_| {
        let m: usize = r.next();
        w.writeln((m + 1) >> 1);
        let mut heap: DoubleSideHeap<i64> = DoubleSideHeap::new();
        let mut v = Vec::with_capacity(m);

        (0..m).for_each(|i| {
            heap.insert(r.next());
            if i & 1 == 0 {
                v.push(heap.get_median().unwrap().to_string());
            }
        });
        w.writeln(v.join(" "));
    });
}
