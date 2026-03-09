use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let k = r.next::<usize>();
    w.write(
        (1..=n).product::<usize>() / ((1..=k).product::<usize>() * (1..=n - k).product::<usize>()),
    );
}
