use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let mut dp = vec![0usize; 10 * n];
    let MOD = 1_000_000_000usize;
    for i in 1..10 {
        dp[i] = 1;
    }
    for i in 1..n {
        (10 * i..10 * i + 9).for_each(|ind| {
            dp[ind] = (dp[ind - 9] + dp[ind]) % MOD;
        });
        (10 * i + 1..10 * (i + 1)).for_each(|ind| {
            dp[ind] = (dp[ind] + dp[ind - 11]) % MOD;
        })
    }

    w.write(dp[10 * (n - 1)..10 * n].iter().sum::<usize>() % MOD);
}
