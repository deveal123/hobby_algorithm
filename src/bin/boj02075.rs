use algorithm::io::{Reader, Writer};
use std::collections::BTreeSet;

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n: usize = r.next();

    let mut set: BTreeSet<i32> = BTreeSet::new();

    (0..n * n).for_each(|_| {
        let num: i32 = r.next();
        set.insert(-num);

        if set.len() > n {
            set.pop_last();
        }
    });

    w.write(-set.pop_last().unwrap());
}
