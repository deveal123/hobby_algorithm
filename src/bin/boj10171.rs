mod algorithm {
    pub mod io {
        pub mod writer {

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
        pub use writer::Writer;
    }
}

// Write code here.

use algorithm::io::Writer;
fn main() {
    let mut w = Writer::new();
    w.writeln(
        r#"|\_/|
|q p|   /}
( 0 )"""\
|"^"`    |
||_/=\\__|"#,
    );
}
