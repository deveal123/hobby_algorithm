use algorithm::io::{Reader, Writer};
use std::collections::BinaryHeap;
fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n: usize = r.next();
    let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();

    (0..n).for_each(|_| match r.next::<i32>() {
        0 => {
            if let Some((_, val)) = heap.pop() {
                w.writeln(-val);
            } else {
                w.writeln(0);
            }
        }
        val => {
            heap.push((-val.abs(), -val));
        }
    })
}
