use crate::cartesian::d2::point::point_i8::Point;

mod area;

pub use self::area::area;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Circle {
    pub p: Point,
    pub r: u8,
}

impl Circle {
    pub fn of(p: Point, r: u8) -> Self {
        Circle { p, r }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.p, self.r)
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;
    use crate::cartesian::{
        d1::point::point_i8::{MAX, MIN},
        d2::point::point_i8::Point,
    };

    #[test]
    fn circle() {
        assert_eq!(Circle::of(Point::min(), u8::MAX), Circle { p: Point { x: MIN, y: MIN }, r: u8::MAX });
        assert_eq!(Circle::of(Point::of(MIN, MAX), u8::MAX), Circle { p: Point { x: MIN, y: MAX }, r: u8::MAX });
        assert_eq!(Circle::of(Point::of(MAX, MIN), u8::MAX), Circle { p: Point { x: MAX, y: MIN }, r: u8::MAX });
        assert_eq!(Circle::of(Point::max(), u8::MAX), Circle { p: Point { x: MAX, y: MAX }, r: u8::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::of(Point::min(), u8::MAX).to_string(), "((-128, -128), 255)");
        assert_eq!(Circle::of(Point::of(MIN, MAX), u8::MAX).to_string(), "((-128, 127), 255)");
        assert_eq!(Circle::of(Point::of(MAX, MIN), u8::MAX).to_string(), "((127, -128), 255)");
        assert_eq!(Circle::of(Point::max(), u8::MAX).to_string(), "((127, 127), 255)");
    }
}
