use std::io::{self, BufWriter, Stdout, Write};

pub struct Writer {
    writer: BufWriter<Stdout>,
}

impl Writer {
    pub fn new() -> Self {
        Self {
            writer: BufWriter::new(io::stdout()),
        }
    }
    pub fn write<T: std::fmt::Display>(&mut self, val: T) {
        write!(self.writer, "{} ", val).unwrap();
    }

    pub fn writeln<T: std::fmt::Display>(&mut self, val: T) {
        writeln!(self.writer, "{} ", val).unwrap();
    }
}
