use super::super::Arithmetic;
use super::point::Point;

#[allow(unused)]
pub struct Segment<T: Arithmetic + std::ops::Div<Output = T>> {
    points: [Point<T>; 2],
}
