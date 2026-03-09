use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let mut stack = Vec::new();
    for _ in 0..n {
        let num = r.next::<usize>();
        match num {
            0 => {
                stack.pop();
            }
            num => {
                stack.push(num);
            }
        };
    }
    w.writeln(stack.iter().sum::<usize>());
}
