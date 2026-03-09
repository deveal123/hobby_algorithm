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
    pub mod math {
        pub mod number_theory {
            pub struct Sieve {
                num_vec: Vec<bool>,
            }

            impl Sieve {
                pub fn new(n: usize) -> Self {
                    let mut num_vec = Vec::new();
                    num_vec.resize_with((n >> 1 + 1), || true);
                    num_vec[0] = false;

                    for i in 1..(n >> 1 + 1) {
                        if !num_vec[i] {
                            continue;
                        }
                        let mut j = 2 * i * (i + 1);
                        while j < (n >> 1 + 1) {
                            num_vec[j] = false;
                            j += ((i << 1) | 1);
                        }
                    }
                    Self { num_vec }
                }

                pub fn is_prime(&self, n: usize) -> Result<bool, String> {
                    if (n > (self.num_vec.len() << 1) | 1) {
                        return Err(format!(
                            "Sieve : Sieve size is too small. n = {}, capacity = {}",
                            n,
                            self.num_vec.len() << 1 | 1
                        ));
                    }

                    if (n == 2) {
                        Ok(true)
                    } else if (n & 1 == 0) {
                        Ok(false)
                    } else {
                        Ok(self.num_vec[n >> 1])
                    }
                }
            }
        }
        use std::fmt::Formatter;

        pub trait Arithmetic:
            std::ops::Add<Output = Self>
            + std::ops::Sub<Output = Self>
            + std::ops::Mul<Output = Self>
            + std::iter::Sum
            + Default
            + Copy
            + Sized
        {
            fn zero() -> Self;
            fn one() -> Self;
        }

        macro_rules! impl_arithmetic{
            ($($t:ty)*) =>{
                $(impl Arithmetic for $t{
                    fn zero() -> Self{
                        0 as $t
                    }

                    fn one() -> Self{
                        1 as $t
                    }
                })*
            }
        }

        impl_arithmetic! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64}
        pub struct FactorIterator<T: NaturalNumber> {
            pub n: T,
            factors: Vec<T>,
            sqrt: Option<T>,
            index: usize,
        }

        impl<T: NaturalNumber> FactorIterator<T> {
            fn new(n: T) -> Self {
                let mut curr = T::one();
                let mut factors = Vec::new();
                let mut sqrt = None;
                while curr * curr <= n {
                    if n.divided_by(curr).unwrap() {
                        factors.push(curr);
                    }
                    curr = curr + T::one();
                    if curr * curr == n {
                        sqrt = Some(curr);
                    }
                }
                FactorIterator {
                    n,
                    factors,
                    sqrt,
                    index: 0,
                }
            }
        }

        impl<T: NaturalNumber> Iterator for FactorIterator<T> {
            type Item = T;

            fn next(&mut self) -> Option<Self::Item> {
                let cnt = self.len();
                if cnt == 0 {
                    return None;
                }
                let mut res = None;
                if cnt - 1 < self.factors.len() {
                    res = Some(self.n / self.factors[cnt - 1]);
                } else {
                    res = Some(self.factors[self.index]);
                }
                self.index += 1;
                res
            }
        }

        impl<T: NaturalNumber> ExactSizeIterator for FactorIterator<T> {
            fn len(&self) -> usize
            where
                Self: Sized,
            {
                let mut ind = 2 * self.factors.len();
                if self.sqrt.is_some() {
                    ind -= 1;
                }
                ind - self.index
            }
        }
        pub trait NaturalNumber:
            Sized + Copy + Clone + Arithmetic + std::ops::Div<Output = Self> + PartialOrd
        {
            fn divided_by(&self, rhs: Self) -> Result<bool, String> {
                if rhs == Self::zero() {
                    return Err("NaturalNumber : Cannot divide by zero".to_string());
                }
                let divisor = *self / rhs;
                if *self - (rhs * divisor) == Self::zero() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }

            fn factors(&self) -> FactorIterator<Self> {
                FactorIterator::new(*self)
            }
        }

        impl NaturalNumber for usize {}
        impl NaturalNumber for u8 {}
        impl NaturalNumber for u16 {}
        impl NaturalNumber for u32 {}
        impl NaturalNumber for u64 {}
    }
}

use algorithm::io::{Reader, Writer};
use algorithm::math::NaturalNumber;

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());

    loop {
        let n = r.try_next::<usize>();
        if n.is_err() {
            break;
        }

        let n = n.unwrap();
        let mut factors = n.factors().collect::<Vec<usize>>();
        factors.pop();
        if factors.iter().sum::<usize>() == n {
            let factor_string = factors
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<String>>()
                .join(" + ");
            w.writeln(format_args!("{} = {}", n, factor_string));
        } else {
            w.writeln(format_args!("{} is NOT perfect.", n));
        }
    }
}
