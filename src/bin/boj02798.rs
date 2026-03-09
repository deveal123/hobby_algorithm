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
}

use std::collections::BTreeSet;

use algorithm::io::{Reader, Writer};

fn flatten_vec<T: Clone>(vec: &Vec<Vec<T>>) -> Vec<T> {
    let total_len: usize = vec.iter().map(|v| v.len()).sum();
    let mut flat_vec: Vec<T> = Vec::with_capacity(total_len);
    for v in vec.iter() {
        flat_vec.extend(v.iter().cloned());
    }
    flat_vec
}

fn main() {
    let (mut reader, mut writer) = (Reader::new(), Writer::new());
    let (n, m): (usize, usize) = (reader.next(), reader.next());

    let arr: Vec<usize> = (0..n).map(|_| reader.next()).collect();

    let mut three_sum = (0..n)
        .map(|i| {
            let mut two_sum = (i + 1..n)
                .map(|j| {
                    (j + 1..n)
                        .map(|k| arr[i] + arr[j] + arr[k])
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            flatten_vec(&mut two_sum)
        })
        .collect::<Vec<_>>();
    let three_sum = flatten_vec(&mut three_sum);
    let result = three_sum
        .iter()
        .filter_map(|s| if *s > m { None } else { Some(*s) })
        .max()
        .unwrap();
    writer.writeln(result);
}
