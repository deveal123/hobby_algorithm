use algorithm::io::{Reader, Writer};
fn main() {
    let mut r = Reader::new();
    let mut w = Writer::new();

    let a = r.next::<i32>();
    let b = r.next::<i32>();
    let c = r.next::<i32>();

    if a == b && a == c {
        w.write(10000 + a * 1000);
    } else if a == b || a == c {
        w.write(1000 + a * 100);
    } else if b == c {
        w.write(1000 + b * 100);
    } else {
        w.write(a.max(b).max(c) * 100);
    }
}
