mod algorithm{
    pub mod io{
        pub mod reader{
            pub struct Reader{
                pub context: Vec<u8>,
                pub index: usize,
            }

            impl Reader{
                pub fn new() -> Self{
                    use std::io::Read;

                    let mut context = Vec::new();
                    #[cfg(feature = "local")]
                    std::fs::File::open("input.txt").unwrap().read_to_end(&mut context).expect("Cannot read input");

                    #[cfg(not(feature = "local"))]
                    std::io::stdin().read_to_end(&mut context).expect("Cannot read input");
                    Reader{
                        context,
                        index: 0,
                    }
                }

                pub fn try_next<T: std::str::FromStr>(&mut self) -> Result<T, String>
                where <T as std::str::FromStr>::Err: std::fmt::Debug,
                {
                    while self.index < self.context.len() && self.context[self.index].is_ascii_whitespace() {
                        self.index += 1;
                    }

                    let start_index = self.index;

                    while self.index < self.context.len() && !self.context[self.index].is_ascii_whitespace() {
                        self.index += 1;
                    }

                    let end_index = self.index;
                    let slice = &self.context[start_index..end_index];
                    T::from_str(std::str::from_utf8(slice).unwrap()).map_err(|_|{
                        format!("Cannot parse {}", std::str::from_utf8(slice).unwrap())
                    })
                }

                pub fn next<T: std::str::FromStr>(&mut self) -> T
                where <T as std::str::FromStr>::Err: std::fmt::Debug,
                {
                    self.try_next().unwrap()
                }
            }
        }
        pub mod writer{

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
    pub mod string{
        pub struct StringIter<'a>{
            pub byte_arr: &'a [u8],
            pub idx: usize,
        }

        impl StringIter<'_>{
            fn char_at(&self, idx: usize) -> char{
                self.byte_arr[idx] as char
            }

            fn first(&self) -> char{
                self.byte_arr[0] as char
            }

            fn end(&self) -> char{
                self.byte_arr[self.byte_arr.len() - 1] as char
            }
        }

        impl<'a> Iterator for StringIter<'a>{
            type Item = char;

            fn next(&mut self) -> Option<char>{
                if self.idx == self.byte_arr.len(){
                    None
                } else{
                    let c = self.byte_arr[self.idx] as char;
                    self.idx += 1;
                    Some(c)
                }
            }
        }



        pub trait StringIndexTrait{
            fn iter(&self) -> StringIter;

            fn char_at(&self, idx: usize) -> char{
                self.iter().char_at(idx)
            }

            fn first(&self) -> char{
                self.iter().first()
            }

            fn end(&self) -> char{
                self.iter().end()
            }
        }

        impl StringIndexTrait for String{
            fn iter(&self) -> StringIter{
                StringIter{
                    byte_arr: self.as_bytes(),
                    idx: 0,
                }
            }
        }

        impl StringIndexTrait for &str{
            fn iter(&self) -> StringIter{
                StringIter{
                    byte_arr: self.as_bytes(),
                    idx: 0,
                }
            }
        }
    }
}

// Write code here.

use algorithm::io::{Reader, Writer};
use algorithm::string::*;
fn main(){
    let mut r = Reader::new();
    let mut w = Writer::new();

    let _ = r.next::<usize>();
    let word = r.next::<String>();

    let mut res = 0usize;
    let zero = '0' as u8;

    for ch in word.iter(){
        res += ((ch as u8) - zero) as usize;
    }
    w.write(res);
}