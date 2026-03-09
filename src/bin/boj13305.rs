use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();

    let dist = (0..n - 1).map(|_| r.next::<usize>()).collect::<Vec<_>>();
    let mut cost = (0..n).map(|_| r.next::<usize>()).collect::<Vec<_>>();

    for i in (1..n) {
        cost[i] = cost[i - 1].min(cost[i]);
    }

    let mut res = 0;
    for i in (0..n - 1) {
        res += dist[i] * cost[i];
    }
    w.write(res);
}
