use algorithm::io::{Reader, Writer};
use algorithm::math::NaturalNumber;
fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let (a1, b1) = (r.next::<usize>(), r.next::<usize>());
    let (a2, b2) = (r.next::<usize>(), r.next::<usize>());

    let (mut numer, mut denom) = (a1 * b2 + a2 * b1, b1 * b2);
    let g = usize::gcd([numer, denom].iter());
    numer /= g;
    denom /= g;
    w.writeln(format!("{} {}", numer, denom));
}
