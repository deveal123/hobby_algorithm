use algorithm::io::{Reader, Writer};
use std::collections::HashMap;

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let mut counts = HashMap::<i32, usize>::with_capacity(n);
    (0..n).for_each(|_| {
        let num = r.next::<i32>();
        *counts.entry(num).or_insert(0) += 1;
    });

    let m = r.next::<usize>();
    (0..m).for_each(|_| {
        let num = r.next::<i32>();
        w.write(counts.get(&num).unwrap_or(&0));
    });
}
