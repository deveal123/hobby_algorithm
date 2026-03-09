use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());

    let n = r.next::<usize>();
    let a = (0..n).map(|_| r.next::<usize>()).collect::<Vec<_>>();
    let a_rev = a.iter().rev().cloned().collect::<Vec<_>>();

    let mut dp1 = vec![0usize; 1001 * n];
    let mut dp2 = vec![0usize; 1001 * n];

    for i in 0..n {
        if i > 0 {
            (1001 * i..1001 * (i + 1)).for_each(|j| {
                dp1[j] = dp1[j - 1001];
            });
            dp1[1001 * i + a[i]] = (0..a[i]).map(|j| dp1[1001 * (i - 1) + j]).max().unwrap() + 1;
        } else {
            dp1[a[i]] = 1;
        }

        if i > 0 {
            // n - 1 - i --> n - i
            (1001 * (n - 1 - i)..1001 * (n - i)).for_each(|j| {
                dp2[j] = dp2[j + 1001];
            });
            dp2[1001 * (n - 1 - i) + a_rev[i]] = (0..a_rev[i])
                .map(|j| dp2[1001 * (n - i) + j])
                .max()
                .unwrap()
                + 1;
        } else {
            dp2[(n - 1) * 1001 + a_rev[i]] = 1;
        }
    }

    let res = dp1
        .iter()
        .zip(dp2.iter())
        .filter_map(|(x, y)| if x + y > 0 { Some(x + y - 1) } else { None })
        .max()
        .unwrap();
    w.write(res);
}
