use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let k = r.next::<usize>();

    let cost = (0..n).map(|_| r.next::<i32>()).collect::<Vec<_>>();

    let max_cost: i32 = cost[..k].iter().sum();

    let sum = (k..n)
        .scan(max_cost, |acc, idx| {
            *acc += (cost[idx] - cost[idx - k]);
            Some(*acc)
        })
        .max();
    match sum {
        Some(v) => w.write(v.max(max_cost)),
        _ => w.write(max_cost),
    }
}
