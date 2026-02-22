mod algorithm{
    pub mod io{
        mod reader{
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

    pub mod string{
        pub struct StringIter<'a>{
            byte_arr: &'a [u8],
            idx: usize,
            end_idx: usize,
        }

        impl StringIter<'_>{
            fn char_at(&self, idx: usize) -> char{
                self.byte_arr[idx] as char
            }

            fn first(&self) -> char{
                self.byte_arr[0] as char
            }

            fn end(&self) -> char{
                self.byte_arr[self.end_idx] as char
            }
        }

        impl<'a> Iterator for StringIter<'a>{
            type Item = char;

            fn next(&mut self) -> Option<char>{
                if self.idx >= self.end_idx{
                    None
                } else{
                    let c = self.byte_arr[self.idx] as char;
                    self.idx += 1;
                    Some(c)
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>){
                let len = self.end_idx - self.idx;
                (len, Some(len))
            }
        }

        impl ExactSizeIterator for StringIter<'_>{}

        impl DoubleEndedIterator for StringIter<'_>{
            fn next_back(&mut self) -> Option<char>{
                if self.end_idx > 0{
                    self.end_idx -= 1;
                    Some(self.byte_arr[self.end_idx] as char)
                } else{
                    None
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
                let byte_arr = self.as_bytes();
                StringIter{
                    byte_arr,
                    idx: 0,
                    end_idx: byte_arr.len(),
                }
            }
        }

        impl StringIndexTrait for &str{
            fn iter(&self) -> StringIter{
                let byte_arr = self.as_bytes();
                StringIter{
                    byte_arr,
                    idx: 0,
                    end_idx: byte_arr.len(),
                }
            }
        }
    }
}

// Write code here.

use algorithm::string::StringIndexTrait;
use algorithm::io::{Reader, Writer};


fn main() {
    let mut r = Reader::new();
    let mut w = Writer::new();
    let number = r.next::<String>();

    let ch_to_number = |c: char| -> usize{
        if '0' <= c && c <= '9'{
            c as usize - '0' as usize
        } else{
            (10usize + c as usize - 'A' as usize)
        }
    };

    let base = r.next::<usize>();

    let mut res = 0usize;

    for ch in number.iter(){
        res = res * base + ch_to_number(ch);
    }
    w.write(res);
}