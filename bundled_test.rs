pub mod algorithm {

pub mod io {

pub use reader::Reader;
pub use writer::Writer;
pub mod reader {
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

pub struct Reader {
    reader: BufReader<Box<dyn Read>>,
    buf: String,
    cursor: usize,
}

impl Reader {
    pub fn new() -> Self {
        let reader: Box<dyn Read> = if cfg!(feature = "local") {
            Box::new(File::open("input.txt").expect("Cannot open input.txt"))
        } else {
            Box::new(io::stdin())
        };

        Self {
            reader: BufReader::new(reader),
            buf: String::new(),
            cursor: 0,
        }
    }

    pub fn try_next<T: std::str::FromStr>(&mut self) -> Result<T, String>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        loop {
            if self.cursor >= self.buf.len() {
                self.buf.clear();
                self.cursor = 0;
                let bytes_read = self
                    .reader
                    .read_line(&mut self.buf)
                    .map_err(|e| e.to_string())?;
                if bytes_read == 0 {
                    return Err("EndOfStream".to_string());
                }
            }

            // Skip whitespace
            while self.cursor < self.buf.len() {
                let s = &self.buf[self.cursor..];
                let ch = s.chars().next().unwrap();
                if !ch.is_whitespace() {
                    break;
                }
                self.cursor += ch.len_utf8();
            }

            if self.cursor >= self.buf.len() {
                continue;
            }

            let start = self.cursor;
            while self.cursor < self.buf.len() {
                let s = &self.buf[self.cursor..];
                let ch = s.chars().next().unwrap();
                if ch.is_whitespace() {
                    break;
                }
                self.cursor += ch.len_utf8();
            }

            let token = &self.buf[start..self.cursor];
            return T::from_str(token).map_err(|_| format!("Cannot parse {}", token));
        }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.try_next().unwrap()
    }
}
}
pub mod writer {
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
}
}
}

use algorithm::io::{Reader, Writer};
use std::collections::HashSet;

fn main(){
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let s = r.next::<String>();

    // This could be solved using trie data structure.
    // But in this problem, input size is small enough to be solved using brute force.
    
    let mut set = HashSet::new();
    for i in 0..s.len(){
        for j in i..s.len(){
            set.insert(&s[i..j+1]);
        }
    }
    w.write(set.len());
}