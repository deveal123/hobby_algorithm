use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let mut factors = (0..n).map(|_| r.next::<usize>()).collect::<Vec<usize>>();
    factors.sort();
    w.write(factors[0] * factors[n - 1]);
}
