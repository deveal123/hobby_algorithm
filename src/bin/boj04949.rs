use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    loop {
        let s = r.next_line();
        if s == "." {
            break;
        }
        let mut stack = Vec::new();
        let mut valid = true;
        for c in s.chars() {
            match c {
                '(' => stack.push(c),
                '[' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        valid = false;
                        break;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        valid = false;
                        break;
                    }
                }
                _ => {}
            }
        }
        if stack.is_empty() && valid {
            w.writeln("yes");
        } else {
            w.writeln("no");
        }
    }
}
