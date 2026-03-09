use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let mut vals = [1; 21];
    for i in 2..=20 {
        vals[i] = vals[i - 1] * i;
    }

    let n = r.next::<usize>();
    w.writeln(vals[n]);
}
