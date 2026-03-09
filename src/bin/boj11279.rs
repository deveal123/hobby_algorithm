use algorithm::io::{Reader, Writer};
use std::collections::BinaryHeap;

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();

    let mut heap = BinaryHeap::with_capacity(n);

    (0..n).for_each(|_| {
        let num = r.next::<usize>();
        if num == 0 {
            match heap.pop() {
                Some(val) => w.writeln(val),
                None => w.writeln(0),
            }
        } else {
            heap.push(num);
        }
    });
}
