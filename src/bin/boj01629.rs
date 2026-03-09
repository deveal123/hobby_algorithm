use algorithm::io::{Reader, Writer};
use algorithm::math::NaturalNumber;

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let (a, b, c) = (r.next::<u64>(), r.next::<u64>(), r.next::<u64>());
    w.write(a.powmod(b, c));
}
