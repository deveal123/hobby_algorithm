use algorithm::io::{Reader, Writer};
use std::collections::BinaryHeap;

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());

    let n: usize = r.next();
    let mut heap: BinaryHeap<i32> = BinaryHeap::with_capacity(n);

    (0..n).for_each(|_| match r.next() {
        0 => match heap.pop() {
            Some(val) => w.writeln(-val),
            None => w.writeln(0),
        },
        val => heap.push(-val),
    })
}
