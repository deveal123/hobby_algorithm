use algorithm::io::{Reader, Writer};
use std::collections::VecDeque;

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let mut q = VecDeque::new();
    for _ in 0..n {
        match r.next::<String>().as_str() {
            "push" => q.push_back(r.next::<isize>()),
            "pop" => w.writeln(q.pop_front().unwrap_or(-1)),
            "front" => w.writeln(q.front().unwrap_or(&-1)),
            "back" => w.writeln(q.back().unwrap_or(&-1)),
            "size" => w.writeln(q.len()),
            "empty" => w.writeln(if q.is_empty() { 1 } else { 0 }),
            _ => unreachable!(),
        }
    }
}
