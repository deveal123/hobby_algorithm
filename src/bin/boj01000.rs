use algorithm::io::{Reader, Writer};

fn main() {
    let mut r = Reader::new();
    let mut w = Writer::new();

    let a = r.try_next::<u8>().expect("a is not a number");
    let b = r.try_next::<u8>().expect("b is not a number");

    w.write(format_args!("{}", a + b));
}
