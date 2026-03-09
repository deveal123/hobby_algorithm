mod algorithm {
    pub mod io {
        mod reader {
            pub struct Reader {
                pub context: Vec<u8>,
                pub index: usize,
            }

            impl Reader {
                pub fn new() -> Self {
                    use std::io::Read;

                    let mut context = Vec::new();
                    #[cfg(feature = "local")]
                    std::fs::File::open("input.txt")
                        .unwrap()
                        .read_to_end(&mut context)
                        .expect("Cannot read input");

                    #[cfg(not(feature = "local"))]
                    std::io::stdin()
                        .read_to_end(&mut context)
                        .expect("Cannot read input");
                    Reader { context, index: 0 }
                }

                pub fn try_next<T: std::str::FromStr>(&mut self) -> Result<T, String>
                where
                    <T as std::str::FromStr>::Err: std::fmt::Debug,
                {
                    while self.index < self.context.len()
                        && self.context[self.index].is_ascii_whitespace()
                    {
                        self.index += 1;
                    }

                    if self.index >= self.context.len() {
                        return Err(format!("Not enough data to read: {}", self.index));
                    }

                    let start_index = self.index;

                    while self.index < self.context.len()
                        && !self.context[self.index].is_ascii_whitespace()
                    {
                        self.index += 1;
                    }

                    let end_index = self.index;
                    let slice = &self.context[start_index..end_index];
                    T::from_str(std::str::from_utf8(slice).unwrap()).map_err(|_| {
                        format!("Cannot parse {}", std::str::from_utf8(slice).unwrap())
                    })
                }

                pub fn next<T: std::str::FromStr>(&mut self) -> T
                where
                    <T as std::str::FromStr>::Err: std::fmt::Debug,
                {
                    self.try_next().unwrap()
                }
            }
        }
        mod writer {

            pub struct Writer {
                buffer: Vec<u8>,
            }

            impl Writer {
                pub fn new() -> Self {
                    Self {
                        buffer: Vec::with_capacity(1024),
                    }
                }
                pub fn write<T: std::fmt::Display>(&mut self, val: T) {
                    use std::io::Write;

                    write!(&mut self.buffer, "{} ", val).unwrap();
                }

                pub fn writeln<T: std::fmt::Display>(&mut self, val: T) {
                    use std::io::Write;

                    writeln!(&mut self.buffer, "{} ", val).unwrap();
                }
            }

            impl Drop for Writer {
                fn drop(&mut self) {
                    use std::io::Write;

                    let stdout = std::io::stdout();
                    let mut handle = stdout.lock();
                    handle.write_all(&self.buffer).unwrap();
                }
            }
        }

        pub use reader::Reader;
        pub use writer::Writer;
    }
    pub mod search {
        pub mod binary_search {
            pub fn lower_bound_fn<T: Ord + PartialOrd>(
                func: impl Fn(i64) -> T,
                val: T,
                lo: i64,
                hi: i64,
            ) -> Result<i64, String> {
                let (mut low, mut high) = (lo, hi);
                if func(low) >= val || func(high) < val {
                    return Err("Value out of bounds".to_string());
                }
                while low < high {
                    let mid = (low + high) / 2;
                    if func(mid) <= val {
                        low = mid;
                    } else {
                        high = mid;
                    }
                }
                Ok(low)
            }

            pub fn upper_bound_fn<T: Ord + PartialOrd>(
                func: impl Fn(i64) -> T,
                val: T,
                lo: i64,
                hi: i64,
            ) -> Result<i64, String> {
                let (mut low, mut high) = (lo, hi);
                if func(low) > val || func(high) <= val {
                    return Err("Value out of bounds".to_string());
                }
                while low + 1 < high {
                    let mid = (low + high) / 2;
                    if func(mid) >= val {
                        high = mid;
                    } else {
                        low = mid;
                    }
                }
                Ok(high)
            }
        }
    }
}

// Write code here.

use algorithm::io::{Reader, Writer};
use algorithm::search::binary_search::upper_bound_fn;

fn main() {
    let mut r = Reader::new();
    let mut w = Writer::new();

    let (a, b, v) = (r.next::<i64>(), r.next::<i64>(), r.next::<i64>());
    let f = |n: i64| -> i64 { a * n - b * (n - 1) };

    let ans = upper_bound_fn(f, v, 0, v).unwrap();
    w.writeln(ans);
}
