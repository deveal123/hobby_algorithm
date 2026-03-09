use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let (n, mut k) = (r.next::<usize>(), r.next::<usize>());

    let mut coins = (0..n).map(|_| r.next::<usize>()).collect::<Vec<_>>();
    let mut count = 0;

    coins.iter().rev().for_each(|coin| {
        count += k / coin;
        k %= coin;
    });

    w.write(count);
}
