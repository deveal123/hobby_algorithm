use algorithm::io::{Reader, Writer};
use std::collections::HashMap;

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let (n, m) = (r.next::<usize>(), r.next::<usize>());

    let mut dict = HashMap::<usize, usize>::new();
    (0..n).for_each(|_| {
        let num = r.next::<usize>();
        *dict.entry(num).or_insert(0) += 1;
    });

    (0..m).for_each(|_| {
        let num = r.next::<usize>();
        *dict.entry(num).or_insert(0) += 1;
    });

    let count = dict.iter().filter(|(_, v)| **v == 1).count();
    w.writeln(count);
}
