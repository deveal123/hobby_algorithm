use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let k = r.next::<usize>();

    let mut dp = vec![0usize; (k + 1)];

    (0..n).for_each(|i| {
        let w = r.next::<usize>();
        let v = r.next::<usize>();
        (w..=k).rev().for_each(|j| {
            dp[j] = dp[j].max(dp[j - w] + v);
        });
    });

    w.write(dp[k]);
}
