use algorithm::io::{Reader, Writer};

fn main() {
    let mut r = Reader::new();
    let mut w = Writer::new();

    let a: i8 = r.try_next().unwrap();
    let b: i8 = r.try_next().unwrap();

    w.write(format_args!("{}", a - b));
}
