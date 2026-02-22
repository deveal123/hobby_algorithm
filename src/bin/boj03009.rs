mod algorithm{
    pub mod io{
        mod reader {
            pub struct Reader{
                pub context: Vec<u8>,
                pub index: usize,
            }

            impl Reader {
                pub fn new() -> Self {
                    use std::io::Read;

                    let mut context = Vec::new();
                    #[cfg(feature = "local")]
                    std::fs::File::open("input.txt").unwrap().read_to_end(&mut context).expect("Cannot read input");

                    #[cfg(not(feature = "local"))]
                    std::io::stdin().read_to_end(&mut context).expect("Cannot read input");
                    Reader {
                        context,
                        index: 0,
                    }
                }

                pub fn try_next<T: std::str::FromStr>(&mut self) -> Result<T, String>
                where
                    <T as std::str::FromStr>::Err: std::fmt::Debug,
                {
                    while self.index < self.context.len() && self.context[self.index].is_ascii_whitespace() {
                        self.index += 1;
                    }

                    if self.index >= self.context.len() {
                        return Err(format!("Not enough data to read: {}", self.index));
                    }

                    let start_index = self.index;

                    while self.index < self.context.len() && !self.context[self.index].is_ascii_whitespace() {
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
        mod writer{

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

#[derive(Copy, Clone)]
pub struct Point{
    x: i64,
    y: i64,
}

impl Point{
    pub fn new(x: i64, y: i64) -> Self{
        Self{
            x,
            y,
        }
    }

    pub fn dist2(&self, p: &Point) -> i64{
        (self.x - p.x) * (self.x - p.x) + (self.y - p.y) * (self.y - p.y)
    }
}

use algorithm::io::{Reader, Writer};
fn main() {
    let (mut reader, mut writer) = (Reader::new(), Writer::new());

    let p1 = Point::new(reader.next::<i64>(), reader.next::<i64>());
    let p2 = Point::new(reader.next::<i64>(), reader.next::<i64>());
    let p3 = Point::new(reader.next::<i64>(), reader.next::<i64>());

    let d12 = p1.dist2(&p2);
    let d13 = p1.dist2(&p3);
    let d23 = p2.dist2(&p3);

    if d23 == d12 + d13{
        writer.writeln(format!("{} {}",p2.x + p3.x - p1.x, p2.y + p3.y - p1.y));
    } else if d12 == d13 + d23{
        writer.writeln(format!("{} {}",p1.x + p2.x - p3.x, p1.y + p2.y - p3.y));
    } else if d13 == d12 + d23{
        writer.writeln(format!("{} {}",p1.x + p3.x - p2.x, p1.y + p3.y - p2.y));
    } else {
        panic!("Not a triangle");
    }
}