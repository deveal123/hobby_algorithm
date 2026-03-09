use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());

    let n = r.next::<usize>();
    let cost = (0..n).map(|_| r.next::<usize>()).collect::<Vec<_>>();
    let mut dp = vec![0usize; n];

    dp[0] = cost[0];
    if n > 1 {
        dp[1] = cost[0] + cost[1];
    }

    if n > 2 {
        dp[2] = dp[1].max(cost[0].max(cost[1]) + cost[2]);
    }

    for i in 3..n {
        dp[i] = dp[i - 1]
            .max(dp[i - 2] + cost[i])
            .max(dp[i - 3] + cost[i - 1] + cost[i]);
    }
    w.writeln(dp[n - 1]);
}
