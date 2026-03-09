use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let mut cost = (0..(3 * n)).map(|_| r.next::<usize>()).collect::<Vec<_>>();
    for i in 1..n {
        cost[3 * i] = cost[3 * i - 1].min(cost[3 * i - 2]) + cost[3 * i];
        cost[3 * i + 1] = cost[3 * i - 3].min(cost[3 * i - 1]) + cost[3 * i + 1];
        cost[3 * i + 2] = cost[3 * i - 2].min(cost[3 * i - 3]) + cost[3 * i + 2];
    }
    w.write(cost[3 * n - 3..].iter().min().unwrap());
}
