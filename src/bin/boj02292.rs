use algorithm::io::{Reader, Writer};
use algorithm::search::binary_search::upper_bound_fn;

fn main() {
    let mut r = Reader::new();
    let mut w = Writer::new();

    let f = |n: i64| -> i64{
        3 * n * (n - 1) + 1
    };

    let inp = r.next::<i64>();
    if inp <= 1{
        w.write(1);
    } else{
        let ans = upper_bound_fn::<i64>(f, inp, 1, 20000);
        match ans {
            Ok(v) => w.write(v),
            Err(e) => w.write(e),
        }
    }
}