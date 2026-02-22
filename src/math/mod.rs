pub mod linalg;
mod number_theory;
pub mod geometry;

pub use number_theory::Sieve;

use std::fmt::Formatter;
pub use crate::z_modulo_k_macro as z_modulo_k;

pub trait Arithmetic:
std::ops::Add<Output = Self> +
std::ops::Sub<Output = Self> +
std::ops::Mul<Output = Self> +
std::iter::Sum + Default + Copy + Sized{
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

impl_arithmetic!{ usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64}

#[derive(Copy, Clone)]
pub struct ZModuloK {
    pub modulo: Option<usize>,
    pub val: usize,
}

impl ZModuloK{
    pub fn new(modulo: usize, val: usize) -> Self{
        if modulo == 0{
            panic!("ZModuloK new : modulo cannot be zero");
        }
        let val = (modulo + (val % modulo)) % modulo;
        Self{
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

impl Default for ZModuloK{
    fn default() -> Self {
        ZModuloK{
            modulo: None,
            val: 0,
        }
    }
}

impl std::ops::Add for ZModuloK{
    type Output = ZModuloK;
    fn add(self, rhs: Self) -> Self::Output {
        let modulo = if self.modulo.is_none() && rhs.modulo.is_none(){
            panic!("ZModulo add : One of left modulo or right modulo should not be none.")
        } else if self.modulo.is_none(){
            rhs.modulo
        } else if rhs.modulo.is_none(){
            self.modulo
        } else{
            if self.modulo != rhs.modulo{
                panic!("ZModuloK add : left modulo and right modulo are not same");
            }
            self.modulo
        };

        Self{
            modulo,
            val: (self.val + rhs.val) % modulo.unwrap()
        }
    }
}

impl std::ops::Sub for ZModuloK{
    type Output = ZModuloK;
    fn sub(self, rhs: Self) -> Self::Output {
        let modulo = if self.modulo.is_none() && rhs.modulo.is_none(){
            panic!("ZModulo sub : One of left modulo or right modulo should not be none.")
        } else if self.modulo.is_none(){
            rhs.modulo
        } else if rhs.modulo.is_none(){
            self.modulo
        } else{
            if self.modulo != rhs.modulo{
                panic!("ZModuloK sub : left modulo and right modulo are not same");
            }
            self.modulo
        };

        Self{
            modulo,
            val: (self.val - rhs.val + modulo.unwrap()) % modulo.unwrap()
        }
    }
}

impl std::ops::Mul for ZModuloK {
    type Output = ZModuloK;
    fn mul(self, rhs: Self) -> Self::Output {
        let modulo = if self.modulo.is_none() && rhs.modulo.is_none(){
            panic!("ZModulo mul : One of left modulo or right modulo should not be none.")
        } else if self.modulo.is_none(){
            rhs.modulo
        } else if rhs.modulo.is_none(){
            self.modulo
        } else{
            if self.modulo != rhs.modulo{
                panic!("ZModuloK mul : left modulo and right modulo are not same");
            }
            self.modulo
        };

        Self{
            modulo,
            val: (self.val * rhs.val) % modulo.unwrap()
        }
    }
}

impl std::iter::Sum for ZModuloK {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        let mut res = Self::default();
        for item in iter{
            res = res + item;
        }
        res
    }
}

impl Arithmetic for ZModuloK{
    fn zero() -> Self {
        Self{
            modulo: None,
            val: 0,
        }
    }

    fn one() -> Self {
        Self{
            modulo: None,
            val: 1,
        }
    }
}

impl std::fmt::Debug for ZModuloK{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.val)
    }
}

pub struct FactorIterator<T: NaturalNumber>{
    pub n: T,
    factors: Vec<T>,
    sqrt: Option<T>,
    index: usize,
}

impl<T: NaturalNumber> FactorIterator<T>{
    fn new(n: T) -> Self{
        let mut curr = T::one();
        let mut factors = Vec::new();
        let mut sqrt = None;
        while curr * curr <= n{
            if n.divided_by(curr).unwrap(){
                factors.push(curr);
            }
            curr = curr + T::one();
            if curr * curr == n{
                sqrt = Some(curr);
            }
        }
        FactorIterator{
            n,
            factors,
            sqrt,
            index: 0
        }
    }
}

impl<T: NaturalNumber> Iterator for FactorIterator<T>{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let cnt = self.len();
        if cnt == 0{
            return None;
        }
        let mut res = None;
        if cnt - 1 < self.factors.len(){
            res = Some(self.n / self.factors[cnt - 1]);
        } else{
            res = Some(self.factors[self.index]);
        }
        self.index += 1;
        res
    }
}

impl<T: NaturalNumber> ExactSizeIterator for FactorIterator<T>{
    fn len(&self) -> usize
    where
        Self: Sized,
    {
        let mut ind = 2 * self.factors.len();
        if self.sqrt.is_some(){
            ind -= 1;
        }
        ind - self.index
    }
}
pub trait NaturalNumber: Sized + Copy + Clone + Arithmetic + std::ops::Div<Output=Self> + PartialOrd {
    fn divided_by(&self, rhs: Self) -> Result<bool, String>{
        if rhs == Self::zero(){
            return Err("NaturalNumber : Cannot divide by zero".to_string());
        }
        let divisor = *self / rhs;
        if *self - (rhs * divisor) == Self::zero(){
            Ok(true)
        } else{
            Ok(false)
        }
    }

    fn factors(&self) -> FactorIterator<Self>{
        FactorIterator::new(*self)
    }
}


impl NaturalNumber for usize{}
impl NaturalNumber for u8{}
impl NaturalNumber for u16{}
impl NaturalNumber for u32{}
impl NaturalNumber for u64{}