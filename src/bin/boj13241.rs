use algorithm::io::{Reader, Writer};
use algorithm::math::NaturalNumber;
fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let nums = [r.next::<usize>(), r.next::<usize>()];
    let lcm = usize::lcm(nums.iter());
    w.writeln(lcm);
}
