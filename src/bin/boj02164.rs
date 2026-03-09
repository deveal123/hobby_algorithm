use algorithm::io::{Reader, Writer};
use std::collections::VecDeque;

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let mut q = VecDeque::from_iter(1..(n + 1));
    while q.len() > 1 {
        let _ = q.pop_front().unwrap();
        let y = q.pop_front().unwrap();
        q.push_back(y);
    }
    w.writeln(q.pop_front().unwrap());
}
