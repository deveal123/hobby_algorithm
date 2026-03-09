use algorithm::io::{Reader, Writer};
use algorithm::math::NaturalNumber;

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    loop {
        let (a, b) = (r.next::<usize>(), r.next::<usize>());
        if a == 0 && b == 0 {
            break;
        }
        if a.divided_by(b).unwrap() {
            w.writeln("multiple");
        } else if b.divided_by(a).unwrap() {
            w.writeln("factor");
        } else {
            w.writeln("neither");
        }
    }
}
