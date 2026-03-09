use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();

    let mut curr = 1;
    let mut stk = Vec::new();

    (0..n).for_each(|_| {
        while stk.last() == Some(&curr) {
            stk.pop();
            curr += 1;
        }

        let num = r.next::<usize>();
        if num == curr {
            curr += 1;
        } else {
            stk.push(num);
        }
    });

    while stk.last() == Some(&curr) {
        stk.pop();
        curr += 1;
    }

    if stk.is_empty() {
        w.writeln("Nice");
    } else {
        w.writeln("Sad");
    }
}
