use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let (n, k) = (r.next::<usize>(), r.next::<usize>() - 1);
    if k == 0 {
        w.write(1);
        return;
    }

    let (mut l, mut r) = (1, n * n + 1);

    while l + 1 < r {
        let mid = (l + r) / 2;
        if (1..=n).map(|i| (mid / i).min(n)).sum::<usize>() > k {
            r = mid;
        } else {
            l = mid;
        }
    }

    w.write(r);
}
