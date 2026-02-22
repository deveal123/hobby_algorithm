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
