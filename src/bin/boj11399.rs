use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let mut tlist = (0..n).map(|_| r.next::<usize>()).collect::<Vec<_>>();
    tlist.sort();
    let mut cost = 0;

    for i in (0..n) {
        cost += (n - i) * tlist[i];
    }
    w.write(cost);
}
