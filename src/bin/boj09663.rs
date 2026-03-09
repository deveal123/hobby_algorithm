use std::collections::HashSet;

use algorithm::io::{Reader, Writer};

fn f(n: usize, m: usize, m1: &mut usize, m2: &mut usize, m3: &mut usize, cnt: &mut usize) {
    if m == n {
        *cnt += 1;
        return;
    }

    for i in 0..n {
        if *m1 & (1 << i) != 0 || *m2 & (1 << (i + m)) != 0 || *m3 & (1 << (i + n - m)) != 0 {
            continue;
        }
        *m1 |= 1 << i;
        *m2 |= 1 << (i + m);
        *m3 |= 1 << (i + n - m);
        f(n, m + 1, m1, m2, m3, cnt);
        *m1 &= !(1 << i);
        *m2 &= !(1 << (i + m));
        *m3 &= !(1 << (i + n - m));
    }
}

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let mut cnt = 0;

    let mut m1 = 0;
    let mut m2 = 0;
    let mut m3 = 0;

    f(n, 0, &mut m1, &mut m2, &mut m3, &mut cnt);
    w.writeln(cnt);
}
