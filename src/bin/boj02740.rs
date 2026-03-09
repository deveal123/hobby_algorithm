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
        pub mod linalg {
            use super::Arithmetic;

            pub struct Matrix2<T> {
                pub _row: usize,
                pub _col: usize,
                pub _arr: Vec<T>,
            }

            pub struct StepIterator<'a, T> {
                start: usize,
                end: usize,
                step: usize,
                arr: &'a Vec<T>,
            }

            impl<T> Iterator for StepIterator<'_, T>
            where
                T: Copy,
            {
                type Item = T;

                fn next(&mut self) -> Option<Self::Item> {
                    if self.start >= self.end {
                        return None;
                    }
                    let val = self.arr[self.start];
                    self.start += self.step;
                    Some(val)
                }

                fn count(self) -> usize
                where
                    Self: Sized,
                {
                    let _count = self.end - 1 - self.start;
                    _count / self.step
                }

                fn nth(&mut self, n: usize) -> Option<Self::Item> {
                    let idx = self.start + self.step * n;
                    if idx >= self.end {
                        None
                    } else {
                        Some(self.arr[idx])
                    }
                }
            }

            impl<T> Matrix2<T>
            where
                T: Copy + Default,
            {
                pub fn new(row: usize, col: usize, arr: Vec<T>) -> Result<Self, String> {
                    if row * col != arr.len() {
                        Err(
                            "Martix init fail : shape does not match with length of array."
                                .to_string(),
                        )
                    } else {
                        Ok(Self {
                            _row: row,
                            _col: col,
                            _arr: arr,
                        })
                    }
                }

                pub fn shape(&self) -> (usize, usize) {
                    (self._row, self._col)
                }

                pub fn rows(&self, idx: usize) -> Result<StepIterator<T>, String> {
                    if idx >= self._row {
                        Err("Matrix rows : Out of bounds".to_string())
                    } else {
                        Ok(StepIterator {
                            start: idx * self._col,
                            end: (idx + 1) * self._col,
                            step: 1,
                            arr: &self._arr,
                        })
                    }
                }

                pub fn cols(&self, idx: usize) -> Result<StepIterator<T>, String> {
                    if idx >= self._col {
                        Err("Matrix cols : Out of bounds".to_string())
                    } else {
                        Ok(StepIterator {
                            start: idx,
                            end: self._row * self._col,
                            step: self._col,
                            arr: &self._arr,
                        })
                    }
                }

                pub fn transpose(&self) -> Self {
                    let mut new_arr = Vec::new();
                    new_arr.resize_with(self._row * self._col, T::default);

                    self._arr.iter().enumerate().for_each(|(ij, val)| {
                        let (i, j) = (ij / self._col, ij % self._col);
                        new_arr[j * self._row + i] = *val;
                    });
                    Self {
                        _row: self._col,
                        _col: self._row,
                        _arr: new_arr,
                    }
                }
            }

            impl<T> std::fmt::Debug for Matrix2<T>
            where
                T: std::fmt::Debug + Copy + Default,
            {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "[")?;
                    for i in (0..self._row) {
                        write!(f, "[")?;
                        let mut row_iter = self.rows(i).unwrap();
                        for (i, val) in row_iter.enumerate() {
                            if i == 0 {
                                write!(f, "{:?}", val)?;
                            } else {
                                write!(f, " {:?}", val)?;
                            }
                        }
                        write!(f, "]")?;
                    }
                    write!(f, "]")
                }
            }

            impl<T> std::ops::Add for Matrix2<T>
            where
                T: Arithmetic,
            {
                type Output = Result<Matrix2<T>, String>;

                fn add(self, rhs: Self) -> Self::Output {
                    if self._row != rhs._row || self._col != rhs._col {
                        Err("Matrix add : Shape Mismatch.".to_string())
                    } else {
                        let _arr = self
                            ._arr
                            .iter()
                            .zip(rhs._arr.iter())
                            .map(|(l, r)| *l + *r)
                            .collect::<Vec<_>>();
                        Ok(Self {
                            _row: self._row,
                            _col: self._col,
                            _arr,
                        })
                    }
                }
            }

            impl<T> std::ops::Sub for Matrix2<T>
            where
                T: Arithmetic,
            {
                type Output = Result<Matrix2<T>, String>;

                fn sub(self, rhs: Self) -> Self::Output {
                    if self._row != rhs._row || self._col != rhs._col {
                        Err("Matrix sub : Shape Mismatch.".to_string())
                    } else {
                        let _arr = self
                            ._arr
                            .iter()
                            .zip(rhs._arr.iter())
                            .map(|(l, r)| *l - *r)
                            .collect::<Vec<_>>();
                        Ok(Self {
                            _row: self._row,
                            _col: self._col,
                            _arr,
                        })
                    }
                }
            }

            impl<T> std::ops::Mul for Matrix2<T>
            where
                T: Arithmetic + Default,
            {
                type Output = Result<Matrix2<T>, String>;

                fn mul(self, rhs: Self) -> Self::Output {
                    if self._col != rhs._row {
                        Err("Matrix mul : Shape Mismatch.".to_string())
                    } else {
                        let rhs_transposed = rhs.transpose();
                        let mut _arr = Vec::with_capacity(self._row * rhs._col);
                        for i in (0..self._row) {
                            for j in (0..rhs._col) {
                                _arr.push(
                                    self.rows(i)
                                        .unwrap()
                                        .zip(rhs_transposed.rows(j).unwrap())
                                        .map(|(l, r)| l * r)
                                        .sum(),
                                );
                            }
                        }

                        Ok(Self {
                            _row: self._row,
                            _col: rhs._col,
                            _arr,
                        })
                    }
                }
            }
        }

        pub use crate::z_modulo_k_macro as z_modulo_k;
        use std::iter::Sum;
        use std::ops::{Add, Mul, Sub};

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

        #[derive(Copy, Clone)]
        pub struct ZModuloK {
            pub modulo: Option<usize>,
            val: usize,
        }

        impl ZModuloK {
            pub fn new(modulo: usize, val: usize) -> Self {
                if modulo == 0 {
                    panic!("ZModuloK new : modulo cannot be zero");
                }
                let val = (val + (val % modulo)) % modulo;
                Self {
                    modulo: Some(modulo),
                    val,
                }
            }
        }
        #[macro_export]
        #[allow(unused_macros)]
        macro_rules! z_modulo_k_macro {
            ($modulo: expr, $val: expr) => {
                ZModuloK::new($modulo as usize, $val as usize)
            };
        }

        impl Default for ZModuloK {
            fn default() -> Self {
                ZModuloK {
                    modulo: None,
                    val: 0,
                }
            }
        }

        impl std::ops::Add for ZModuloK {
            type Output = ZModuloK;
            fn add(self, rhs: Self) -> Self::Output {
                if self.modulo != rhs.modulo {
                    panic!("ZModuloK add : left modulo and right modulo are not same");
                }
                let modulo = if self.modulo.is_none() && rhs.modulo.is_none() {
                    panic!("ZModulo add : One of left modulo or right modulo should not be none.")
                } else if self.modulo.is_none() {
                    rhs.modulo
                } else {
                    self.modulo
                };

                Self {
                    modulo,
                    val: (self.val + rhs.val) % modulo.unwrap(),
                }
            }
        }

        impl std::ops::Sub for ZModuloK {
            type Output = ZModuloK;
            fn sub(self, rhs: Self) -> Self::Output {
                if self.modulo != rhs.modulo {
                    panic!("ZModuloK sub : left modulo and right modulo are not same");
                }
                let modulo = if self.modulo.is_none() && rhs.modulo.is_none() {
                    panic!("ZModulo sub : One of left modulo or right modulo should not be none.")
                } else if self.modulo.is_none() {
                    rhs.modulo
                } else {
                    self.modulo
                };

                Self {
                    modulo,
                    val: (self.val - rhs.val + modulo.unwrap()) % modulo.unwrap(),
                }
            }
        }

        impl std::ops::Mul for ZModuloK {
            type Output = ZModuloK;
            fn mul(self, rhs: Self) -> Self::Output {
                if self.modulo != rhs.modulo {
                    panic!("ZModuloK mul : left modulo and right modulo are not same");
                }
                let modulo = if self.modulo.is_none() && rhs.modulo.is_none() {
                    panic!("ZModulo mul : One of left modulo or right modulo should not be none.")
                } else if self.modulo.is_none() {
                    rhs.modulo
                } else {
                    self.modulo
                };

                Self {
                    modulo,
                    val: (self.val * rhs.val) % modulo.unwrap(),
                }
            }
        }

        impl Sum for ZModuloK {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                let mut res = Self::default();
                for item in iter {
                    res = res + item;
                }
                res
            }
        }

        impl Arithmetic for ZModuloK {
            fn zero() -> Self {
                Self {
                    modulo: None,
                    val: 0,
                }
            }

            fn one() -> Self {
                Self {
                    modulo: None,
                    val: 1,
                }
            }
        }
    }
}

// Write code here.

use algorithm::io::{Reader, Writer};
use algorithm::math::linalg::*;

fn main() {
    let mut r = Reader::new();
    let mut w = Writer::new();

    let n = r.next::<usize>();
    let m = r.next::<usize>();

    let _arr1 = (0..(n * m)).map(|_| r.next::<i64>()).collect::<Vec<_>>();
    let arr1 = Matrix2::new(n, m, _arr1).unwrap();

    let _ = r.next::<usize>();
    let k = r.next::<usize>();
    let _arr2 = (0..(m * k)).map(|_| r.next::<i64>()).collect::<Vec<_>>();
    let arr2 = Matrix2::new(m, k, _arr2).unwrap();

    let arr = (arr1 * arr2).unwrap();
    for i in (0..arr.shape().0) {
        let mut s = Vec::with_capacity(arr.shape().1);
        arr.rows(i).unwrap().for_each(|val| s.push(val.to_string()));
        w.writeln(s.join(" "));
    }
}
