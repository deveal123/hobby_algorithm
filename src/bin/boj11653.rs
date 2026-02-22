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
    pub mod math{
        mod number_theory{
            pub struct Sieve{
                num_vec: Vec<bool>,
            }

            impl Sieve{
                pub fn new(n: usize) -> Self{
                    let mut num_vec = Vec::new();
                    num_vec.resize_with(((n >> 1) + 1), || true);
                    num_vec[0] = false;

                    for i in 1..((n >> 1) + 1){
                        if !num_vec[i]{continue;}
                        let mut j = 2 * i * (i + 1);
                        if j >= ((n >> 1) + 1){break;}
                        while j < ((n >> 1) + 1){
                            num_vec[j] = false;
                            j += ((i << 1) | 1);
                        }
                    }
                    Self{
                        num_vec,
                    }
                }

                pub fn is_prime(&self, n: usize) -> Result<bool, String>{
                    if (n > (self.num_vec.len() << 1) | 1){
                        return Err(format!("Sieve : Sieve size is too small. n = {}, capacity = {}", n, self.num_vec.len() << 1 | 1))
                    }

                    if (n == 2) {
                        Ok(true)
                    } else if (n & 1 == 0) {
                        Ok(false)
                    } else {
                        Ok(self.num_vec[n >> 1])
                    }
                }

                pub fn sieve(&self) -> Vec<bool>{
                    self.num_vec.clone()
                }
            }
        }
        pub use number_theory::Sieve;
    }
}

use algorithm::io::{Reader, Writer};
use algorithm::math::Sieve;

fn main(){
    let (mut r, mut w) = (Reader::new(), Writer::new());

    let mut n = r.next::<usize>();
    let sieve = Sieve::new(4000);

    while n & 1 == 0{
        w.writeln(2);
        n >>= 1;
    }

    let mut j = 3;
    while n != 1 && j * j <= n{
        while n % j == 0{
            w.writeln(j);
            n /= j;
        }
        j += 2;
        while !sieve.is_prime(j).unwrap(){
            j += 2;
        }
    }

    if n != 1{
        w.writeln(n);
    }
}