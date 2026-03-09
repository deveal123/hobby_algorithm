use crate::math::Arithmetic;

mod point;
pub mod polygon;
mod segment;

pub trait DistanceTrait<T: Arithmetic + std::ops::Div<Output = T>, OtherType> {
    fn distance_sq(&self, other: &OtherType) -> T;
}
