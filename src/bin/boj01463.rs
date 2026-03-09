use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let mut dp = vec![0usize; n + 1];
    for i in 2..=n {
        dp[i] = dp[i - 1] + 1;
        if i & 1 == 0 {
            dp[i] = dp[i].min(dp[i >> 1] + 1);
        }
        if i % 3 == 0 {
            dp[i] = dp[i].min(dp[i / 3] + 1);
        }
    }

    w.write(dp[n]);
}
