use algorithm::io::{Reader, Writer};
use algorithm::math::Sieve;
fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let sieve = Sieve::new(123456 * 2);
    let primes = sieve.primes();

    loop {
        let inp = r.next::<usize>();
        if inp == 0 {
            break;
        }
        let cnt = primes.partition_point(|&x| x <= inp * 2) - primes.partition_point(|&x| x <= inp);
        w.writeln(cnt);
    }
}
