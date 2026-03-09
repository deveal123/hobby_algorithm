use algorithm::io::{Reader, Writer};
use std::collections::VecDeque;

fn main() {
    let mut reader = Reader::new();
    let mut writer = Writer::new();
    let n = reader.next::<usize>();
    let mut deque = VecDeque::with_capacity(n);
    for _ in 0..n {
        let command = reader.next::<usize>();
        match command {
            1 => {
                deque.push_front(reader.next::<i32>());
            }
            2 => {
                deque.push_back(reader.next::<i32>());
            }
            3 => {
                writer.writeln(deque.pop_front().unwrap_or(-1));
            }
            4 => {
                writer.writeln(deque.pop_back().unwrap_or(-1));
            }
            5 => {
                writer.writeln(deque.len());
            }
            6 => {
                writer.writeln(deque.is_empty() as i32);
            }
            7 => {
                writer.writeln(deque.front().unwrap_or(&-1));
            }
            8 => {
                writer.writeln(deque.back().unwrap_or(&-1));
            }
            _ => unreachable!(),
        }
    }
}
