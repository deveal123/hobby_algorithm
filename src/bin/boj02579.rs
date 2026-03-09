use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());

    let n = r.next::<usize>();
    let cost = (0..n).map(|_| r.next::<usize>()).collect::<Vec<_>>();
    let mut dp: Vec<Option<usize>> = vec![None; n * 2];

    dp[1] = Some(cost[0]);
    if n > 1 {
        dp[2] = Some(cost[1]);
        dp[3] = Some(cost[0] + cost[1]);
    }

    for i in 2..n {
        dp[2 * i] = Some(
            dp[2 * (i - 2)..2 * (i - 1)]
                .iter()
                .filter_map(|x| *x)
                .max()
                .unwrap()
                + cost[i],
        );
        match dp[2 * (i - 1)] {
            Some(val) => dp[2 * i + 1] = Some(val + cost[i]),
            None => {}
        }
    }
    w.writeln(dp[2 * n - 2..].iter().filter_map(|x| *x).max().unwrap());
}
