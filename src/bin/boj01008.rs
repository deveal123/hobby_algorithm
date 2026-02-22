use algorithm::io::{Reader, Writer};

fn main() {
    let mut reader = Reader::new();
    let mut writer = Writer::new();

    let a = reader.try_next::<i8>().unwrap();
    let b = reader.try_next::<i8>().unwrap();

    let c = (a as f64) / (b as f64);
    writer.write(format!("{:.11}", c));
}