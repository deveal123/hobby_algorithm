use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let (n, m, k) = (r.next::<usize>(), r.next::<usize>(), r.next::<usize>());

    let mut status = vec![0usize; m * n];
    (0..n).for_each(|i| {
        r.next::<String>().chars().enumerate().for_each(|(j, c)| {
            if (i + j) % 2 == 0 && c == 'B' || (i + j) % 2 == 1 && c == 'W' {
                status[i * m + j] = 1;
            } else {
                status[i * m + j] = 0;
            }
        });
    });

    let tmp = (0..n)
        .map(|i| {
            let mut s = status[i * m..i * m + k].iter().sum::<usize>();
            let mut v = Vec::with_capacity(m - k + 1);
            v.push(s);
            for j in k..m {
                s = s + status[i * m + j] - status[i * m + j - k];
                v.push(s);
            }
            v
        })
        .collect::<Vec<_>>();

    let mut s = vec![0usize; (n - k + 1) * (m - k + 1)];
    (0..m - k + 1).for_each(|j| {
        s[j] = (0..k).map(|i| tmp[i][j]).sum::<usize>();
    });
    (1..n - k + 1).for_each(|i| {
        (0..m - k + 1).for_each(|j| {
            s[i * (m - k + 1) + j] =
                s[(i - 1) * (m - k + 1) + j] + tmp[k - 1 + i][j] - tmp[i - 1][j];
        });
    });

    w.write(s.iter().map(|n| *n.min(&(k * k - n))).min().unwrap());
}
