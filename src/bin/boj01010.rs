use algorithm::io::{Reader, Writer};
use algorithm::math::NaturalNumber;

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let p = 1000000007;
    let t = r.next::<usize>();

    let mut fact = vec![1usize; 31];
    let mut inv_fact = vec![1usize; 31];
    for i in 2..31 {
        fact[i] = (fact[i - 1] * i) % p;
        inv_fact[i] = (inv_fact[i - 1] * (i as usize).inv_unchecked(p)) % p;
    }

    for _ in 0..t {
        let n = r.next::<usize>();
        let m = r.next::<usize>();

        let res = (fact[m] * inv_fact[n]) % p;
        let res = (res * inv_fact[m - n]) % p;
        w.writeln(res);
    }
}
