use algorithm::io::{Reader, Writer};
use std::collections::HashSet;

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let (n, m) = (r.next::<usize>(), r.next::<usize>());

    let dict = (0..n).map(|_| r.next::<String>()).collect::<HashSet<_>>();

    let mut cnt = 0;

    (0..m).for_each(|_| {
        let word = r.next::<String>();
        if dict.contains(&word) {
            cnt += 1;
        }
    });

    w.write(cnt);
}
