use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

fn main() {
    let file_path = Path::new("input.txt");
    let mut file = File::create(file_path).unwrap();

    let mut w = BufWriter::new(file);

    writeln!(&mut w, "64");
    for i in (0..64) {
        for j in (0..64) {
            write!(&mut w, "{} ", (i + j) & 1);
        }
        writeln!(&mut w, "");
    }
}
