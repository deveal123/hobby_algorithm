use algorithm::io::{Reader, Writer};
use algorithm::math::NaturalNumber;

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let p = 1_000_000_007;
    let (n, k) = (r.next::<u64>(), r.next::<u64>());
    let fact = (0..=n)
        .scan(1, |acc, num| {
            if num != 0 {
                *acc = *acc * num % p;
            }
            Some(*acc)
        })
        .collect::<Vec<_>>();
    let res = fact[n as usize] * fact[k as usize].inv_unchecked(p) % p
        * fact[(n - k) as usize].inv_unchecked(p)
        % p;
    w.write(res);
}
