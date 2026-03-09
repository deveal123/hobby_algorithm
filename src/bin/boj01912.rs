use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let mut dp = (0..n).map(|_| r.next::<i32>()).collect::<Vec<_>>();
    for i in 1..n {
        dp[i] = dp[i].max(dp[i] + dp[i - 1]);
    }
    w.write(dp.iter().max().unwrap());
}
