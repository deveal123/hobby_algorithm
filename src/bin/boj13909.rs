use algorithm::io::{Reader, Writer};
use algorithm::search::binary_search::lower_bound_fn;

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();

    let ans = lower_bound_fn(|x| x * x, n as i64, 0, n as i64 + 1).unwrap();
    w.writeln(ans);
}
