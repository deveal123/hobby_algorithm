use super::super::Arithmetic;

pub trait CoorShapeTrait<T: Arithmetic + std::ops::Div<Output = Self>> {}
pub trait ShapeTrait<T: Arithmetic + std::ops::Div<Output = Self>> {
    fn area() -> T;
    fn perimeter() -> T;
}
