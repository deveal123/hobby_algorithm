use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let mut dp = [0usize; 41];
    dp[1] = 1;
    for i in 2..41 {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    w.write(dp[n]);
    w.write(n - 2);
}
