use crate::math::Arithmetic;

pub mod polygon;
mod point;
mod segment;

pub trait DistanceTrait<T:Arithmetic + std::ops::Div<Output = T>, OtherType>{
    fn distance_sq(&self, other: &OtherType) -> T;
}