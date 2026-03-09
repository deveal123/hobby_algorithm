use algorithm::io::{Reader, Writer};
use std::collections::{LinkedList, VecDeque};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let k = r.next::<usize>();
    let mut q = VecDeque::from_iter(1..(n + 1));
    let mut seq = Vec::with_capacity(n);
    while q.len() > 0 {
        (0..(k - 1)).for_each(|_| {
            let x = q.pop_front().unwrap();
            q.push_back(x);
        });
        let y = q.pop_front().unwrap();
        seq.push(y.to_string());
    }
    w.write(format!("<{}>", seq.join(", ")));
}
