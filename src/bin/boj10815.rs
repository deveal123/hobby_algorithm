use std::collections::HashSet;

use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();

    let cards = (0..n).map(|_| r.next::<i32>()).collect::<HashSet<_>>();

    let m = r.next::<usize>();

    (0..m).for_each(|_| {
        let num = r.next::<i32>();
        if cards.contains(&num) {
            w.write(1);
        } else {
            w.write(0);
        }
    });
}
