use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());

    let mut dp = [1usize; 101];
    (dp[3], dp[4]) = (2, 2);
    for i in 5..=100 {
        dp[i] = dp[i - 1] + dp[i - 5];
    }
    for _ in (0..r.next::<usize>()) {
        w.writeln(dp[r.next::<usize>() - 1]);
    }
}
