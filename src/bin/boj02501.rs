use algorithm::io::{Reader, Writer};
use algorithm::math::NaturalNumber;

fn main(){
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let (n, k) = (r.next::<usize>(), r.next::<usize>());
    let factors = n.factors().collect::<Vec<_>>();

    if factors.len() < k{
        w.write(0);
    } else{
        w.write(factors[k - 1]);
    }
}