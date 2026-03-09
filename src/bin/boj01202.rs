use algorithm::io::{Reader, Writer};
use std::collections::BinaryHeap;

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let (n, k): (usize, usize) = (r.next(), r.next());
    let mut cost = (0..n)
        .map(|_| (r.next(), r.next::<i64>()))
        .collect::<Vec<(usize, i64)>>();
    cost.sort();

    let mut capacity = (0..k).map(|_| r.next()).collect::<Vec<usize>>();
    capacity.sort();
    capacity.reverse();

    let mut cur = 0usize;
    let mut heap = BinaryHeap::<i64>::new();

    for (m, v) in cost.iter().rev() {
        while cur < capacity.len() && *m <= capacity[cur] {
            cur += 1;
        }

        heap.push(-*v);
        if heap.len() > cur {
            heap.pop();
        }
    }

    w.write(heap.iter().map(|i| -*i).sum::<i64>());
}
