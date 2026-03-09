use algorithm::io::{Reader, Writer};
fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let mut stack = Vec::new();
    for _ in 0..n {
        let op = r.next::<usize>();
        match op {
            1 => {
                let x = r.next::<i32>();
                stack.push(x);
            }
            2 => match stack.pop() {
                Some(x) => w.writeln(x),
                None => w.writeln(-1),
            },
            3 => {
                w.writeln(stack.len());
            }
            4 => match stack.is_empty() {
                true => w.writeln(1),
                false => w.writeln(0),
            },
            5 => match stack.last() {
                Some(x) => w.writeln(*x),
                None => w.writeln(-1),
            },
            _ => {}
        }
    }
}
