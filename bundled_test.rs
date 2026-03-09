pub mod algorithm {

    pub mod io {

        pub use reader::Reader;
        pub use writer::Writer;
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

                pub fn try_next_line(&mut self) -> Result<String, String> {
                    if self.cursor > 0 && self.cursor < self.buf.len() {
                        let s = &self.buf[self.cursor..];
                        if s.chars().all(|c| c == '\r' || c == '\n') {
                            self.buf.clear();
                            self.cursor = 0;
                        }
                    }

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

                    let res = self.buf[self.cursor..]
                        .trim_end_matches(|c| c == '\n' || c == '\r')
                        .to_string();
                    self.cursor = self.buf.len();
                    Ok(res)
                }

                pub fn next_line(&mut self) -> String {
                    self.try_next_line().unwrap()
                }
            }
        }
    }
}

// Original algorithm code is in https://github.com/deveal123/hobby_algorithm
use algorithm::io::{Reader, Writer};
use std::collections::BinaryHeap;

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let (n, k): (usize, usize) = (r.next(), r.next());
    let mut cost = (0..n)
        .map(|_| (r.next(), r.next::<i64>()))
        .collect::<Vec<(usize, i64)>>();
    cost.sort();

    let mut capacity = (0..k).map(|_| r.next()).collect::<Vec<usize>>();
    capacity.sort();
    capacity.reverse();

    let mut cur = 0usize;
    let mut heap = BinaryHeap::<i64>::new();

    for (m, v) in cost.iter().rev() {
        while cur < capacity.len() && *m <= capacity[cur] {
            cur += 1;
        }

        heap.push(-*v);
        if heap.len() > cur {
            heap.pop();
        }
    }

    w.write(heap.iter().map(|i| -*i).sum::<i64>());
}
