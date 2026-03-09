use algorithm::io::{Reader, Writer};
use algorithm::math::Sieve;

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let sieve = Sieve::new(1000000).sieve();
    let tc = r.next::<usize>();

    for _ in 0..tc {
        let inp = (r.next::<usize>() >> 1) - 1;
        if inp == 1 {
            w.writeln(1);
            continue;
        }
        let cnt = (0..(inp >> 1) + 1)
            .filter(|&x| sieve[x] && sieve[inp - x])
            .count();
        w.writeln(cnt);
    }
}
