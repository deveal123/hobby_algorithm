use algorithm::io::{Reader, Writer};

fn main() {
    let mut r = Reader::new();
    let mut w = Writer::new();

    let arr = (0..81).map(|_| r.next::<usize>()).collect::<Vec<_>>();

    let (mut loc, mut max_val) = (0, 0);
    arr.iter().enumerate().for_each(
        |(idx, val)|{
            if *val > max_val{
                (loc, max_val) = (idx, *val);
            }
        }
    );
    w.writeln(max_val);
    w.write(format!("{} {}", (loc / 9) + 1, (loc % 9) + 1));
}