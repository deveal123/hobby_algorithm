use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let mut vals = [1; 21];
    vals[0] = 0;

    for i in (2..=20) {
        vals[i] = vals[i - 1] + vals[i - 2];
    }

    w.writeln(vals[n]);
}
