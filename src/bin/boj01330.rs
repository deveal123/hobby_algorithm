use algorithm::io::{Reader, Writer};
fn main() {
    let mut r = Reader::new();
    let mut w = Writer::new();

    let a = r.next::<i32>();
    let b = r.next::<i32>();

    if a > b {
        w.write(">");
    } else if a < b {
        w.write("<");
    } else {
        w.write("==");
    }
}
