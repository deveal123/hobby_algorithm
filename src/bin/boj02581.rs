use algorithm::io::{Reader, Writer};
use algorithm::math::Sieve;

fn main(){
    let (mut r, mut w) = (Reader::new(), Writer::new());

    let (m, n) = (r.next::<usize>(), r.next::<usize>());
    let sieve = Sieve::new(10000);
    let primes = (m..=n).filter(|&x| sieve.is_prime(x).unwrap()).collect::<Vec<_>>();
    if !primes.is_empty(){
        w.writeln(primes.iter().sum::<usize>());
        w.writeln(primes[0]);
    } else{
        w.writeln(-1);
    }
}