use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let (n, m) = (r.next::<usize>(), r.next::<usize>());

    let mut numbers = (0..n * n).map(|_| r.next::<usize>()).collect::<Vec<_>>();
    (0..n * n).for_each(|idx| {
        if idx >= n {
            numbers[idx] += numbers[idx - n];
        }
    });

    (0..n * n).for_each(|idx| {
        if idx % n > 0 {
            numbers[idx] += numbers[idx - 1];
        }
    });

    (0..m).for_each(|_| {
        let (x1, y1, x2, y2) = (
            r.next::<usize>() - 1,
            r.next::<usize>() - 1,
            r.next::<usize>() - 1,
            r.next::<usize>() - 1,
        );
        let s1 = if x1 == 0 || y1 == 0 {
            0
        } else {
            numbers[(x1 - 1) * n + (y1 - 1)]
        };
        let s2 = if x1 == 0 {
            0
        } else {
            numbers[(x1 - 1) * n + y2]
        };
        let s3 = if y1 == 0 {
            0
        } else {
            numbers[x2 * n + (y1 - 1)]
        };
        let s4 = numbers[x2 * n + y2];
        w.writeln(s4 + s1 - s2 - s3);
    })
}
