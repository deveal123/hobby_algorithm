use algorithm::io::{Reader, Writer};
use std::collections::VecDeque;

struct Node {
    idx: usize,
    val: isize,
}

impl Node {
    fn new(idx: usize, val: isize) -> Self {
        Self { idx, val }
    }
}

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let mut q = VecDeque::from_iter((0..n).map(|i| Node::new(i + 1, r.next::<isize>())));
    loop {
        let n = q.pop_front().unwrap();
        w.write(n.idx);
        let val = n.val;
        if q.is_empty() {
            break;
        }
        if val > 0 {
            q.rotate_left((val - 1) as usize % q.len());
        } else {
            q.rotate_right((-val) as usize % q.len());
        }
    }
}
