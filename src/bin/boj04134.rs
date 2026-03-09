use std::collections::BTreeSet;

use algorithm::io::{Reader, Writer};
use algorithm::math::Sieve;
fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let tc = r.next::<usize>();
    let sieve = Sieve::new(90000); // (9e4)^2 > 4e9 * 2 (Bertrand's postulate)
    let primes = sieve.primes();

    for _ in 0..tc {
        let inp = r.next::<usize>();
        if inp <= 2 {
            w.writeln(2);
            continue;
        }

        let mut st = inp | 1;
        loop {
            let mut is_prime = true;
            for &p in &primes {
                if p * p > st {
                    break;
                }
                if st % p == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                w.writeln(st);
                break;
            }
            st += 2;
        }
    }
}
