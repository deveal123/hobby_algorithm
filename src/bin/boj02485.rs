use algorithm::io::{Reader, Writer};
use algorithm::math::NaturalNumber;
fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let loc = (0..n).map(|_| r.next::<usize>()).collect::<Vec<_>>();
    let total_leng = loc[n - 1] - loc[0];
    let diff = loc
        .clone()
        .into_iter()
        .zip(loc.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();

    let g = usize::gcd(diff.iter());

    let needs = (total_leng / g) + 1;

    w.writeln(needs - n);
}
