use algorithm::io::{Reader, Writer};
use algorithm::search::binary_search::upper_bound_fn;

fn main() {
    let mut r = Reader::new();
    let mut w = Writer::new();

    let f = |n: i64| -> i64 { n * (n + 1) >> 1 };

    let x = r.next::<i64>();
    let t = upper_bound_fn(f, x, 0, 10000).unwrap();
    let res = f(t) - x;
    if t & 1 == 1 {
        w.write(format!("{}/{}", 1 + res, t - res));
    } else {
        w.write(format!("{}/{}", t - res, 1 + res));
    }
}
