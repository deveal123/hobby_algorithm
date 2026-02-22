use algorithm::io::{Reader, Writer};
use algorithm::math::Sieve;

fn main(){
    let (mut r, mut w) = (Reader::new(), Writer::new());

    let n = r.next::<usize>();
    let numbers = (0..n).map(|_| r.next::<usize>()).collect::<Vec<_>>();
    let sieve = Sieve::new(1000);
    let v = sieve.sieve();
    let cnt = numbers
        .into_iter()
        .filter(|&x| sieve.is_prime(x).unwrap())
        .count();
    w.writeln(cnt);
}