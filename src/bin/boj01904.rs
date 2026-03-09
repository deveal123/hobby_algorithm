use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let mut dp = vec![0u16; 1_000_001];
    dp[1] = 1;
    dp[2] = 2;
    for i in 3..=1_000_000 {
        dp[i] = (dp[i - 1] + dp[i - 2]) % 15746;
    }
    let n = r.next::<usize>();
    w.writeln(dp[n]);
}
