use super::super::Arithmetic;
use super::DistanceTrait;

pub struct Point<T: Arithmetic + std::ops::Div<Output = T>>{
    x: T,
    y: T,
}
impl<T: Arithmetic + std::ops::Div<Output = T>> DistanceTrait<T, Point<T>> for Point<T>{
    #[inline]
    fn distance_sq(&self, other: &Self) -> T{
        (self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)
    }
}
