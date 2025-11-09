use crate::cartesian::d2::{
    circle::{circle_i8, circle_i16},
    point::point_i32::Point,
};

mod area;

pub use self::area::area;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Circle {
    pub p: Point,
    pub r: u32,
}

impl Circle {
    pub fn new(p: Point, r: u32) -> Self {
        Circle { p, r }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.p, self.r)
    }
}

impl From<circle_i8::Circle> for Circle {
    fn from(c: circle_i8::Circle) -> Self {
        Circle { p: Point::from(c.p), r: u32::from(c.r) }
    }
}

impl From<circle_i16::Circle> for Circle {
    fn from(c: circle_i16::Circle) -> Self {
        Circle { p: Point::from(c.p), r: u32::from(c.r) }
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;
    use crate::cartesian::{
        d1::point::point_i32::{MAX, MIN},
        d2::{
            circle::{circle_i8, circle_i16},
            point::{point_i8, point_i16, point_i32::Point},
        },
    };

    #[test]
    fn circle() {
        assert_eq!(Circle::new(Point::min(), u32::MAX), Circle { p: Point { x: MIN, y: MIN }, r: u32::MAX });
        assert_eq!(Circle::new(Point::new(MIN, MAX), u32::MAX), Circle { p: Point { x: MIN, y: MAX }, r: u32::MAX });
        assert_eq!(Circle::new(Point::new(MAX, MIN), u32::MAX), Circle { p: Point { x: MAX, y: MIN }, r: u32::MAX });
        assert_eq!(Circle::new(Point::max(), u32::MAX), Circle { p: Point { x: MAX, y: MAX }, r: u32::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::new(Point::min(), u32::MAX).to_string(), "((-2147483648, -2147483648), 4294967295)");
        assert_eq!(Circle::new(Point::new(MIN, MAX), u32::MAX).to_string(), "((-2147483648, 2147483647), 4294967295)");
        assert_eq!(Circle::new(Point::new(MAX, MIN), u32::MAX).to_string(), "((2147483647, -2147483648), 4294967295)");
        assert_eq!(Circle::new(Point::max(), u32::MAX).to_string(), "((2147483647, 2147483647), 4294967295)");
    }

    #[test]
    fn from() {
        assert_eq!(Circle::from(circle_i8::Circle::new(point_i8::Point::new(i8::MIN, i8::MAX), u8::MAX)), Circle { p: Point { x: i8::MIN.into(), y: i8::MAX.into() }, r: u8::MAX.into() });
        assert_eq!(Circle::from(circle_i16::Circle::new(point_i16::Point::new(i16::MIN, i16::MAX), u16::MAX)), Circle { p: Point { x: i16::MIN.into(), y: i16::MAX.into() }, r: u16::MAX.into() });
    }
}
