use crate::cartesian::d2::{
    circle::{circle_i8, circle_i16, circle_i32},
    point::point_i64::Point,
};

mod area;

pub use self::area::area;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Circle {
    pub p: Point,
    pub r: u64,
}

impl Circle {
    pub fn new(p: Point, r: u64) -> Self {
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
        Circle { p: Point::from(c.p), r: u64::from(c.r) }
    }
}

impl From<circle_i16::Circle> for Circle {
    fn from(c: circle_i16::Circle) -> Self {
        Circle { p: Point::from(c.p), r: u64::from(c.r) }
    }
}

impl From<circle_i32::Circle> for Circle {
    fn from(c: circle_i32::Circle) -> Self {
        Circle { p: Point::from(c.p), r: u64::from(c.r) }
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;
    use crate::cartesian::{
        d1::point::point_i64::{MAX, MIN},
        d2::{
            circle::{circle_i8, circle_i16, circle_i32},
            point::{point_i8, point_i16, point_i32, point_i64::Point},
        },
    };

    #[test]
    fn circle() {
        assert_eq!(Circle::new(Point::min(), u64::MAX), Circle { p: Point { x: MIN, y: MIN }, r: u64::MAX });
        assert_eq!(Circle::new(Point::new(MIN, MAX), u64::MAX), Circle { p: Point { x: MIN, y: MAX }, r: u64::MAX });
        assert_eq!(Circle::new(Point::new(MAX, MIN), u64::MAX), Circle { p: Point { x: MAX, y: MIN }, r: u64::MAX });
        assert_eq!(Circle::new(Point::max(), u64::MAX), Circle { p: Point { x: MAX, y: MAX }, r: u64::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::new(Point::min(), u64::MAX).to_string(), "((-9223372036854775808, -9223372036854775808), 18446744073709551615)");
        assert_eq!(Circle::new(Point::new(MIN, MAX), u64::MAX).to_string(), "((-9223372036854775808, 9223372036854775807), 18446744073709551615)");
        assert_eq!(Circle::new(Point::new(MAX, MIN), u64::MAX).to_string(), "((9223372036854775807, -9223372036854775808), 18446744073709551615)");
        assert_eq!(Circle::new(Point::max(), u64::MAX).to_string(), "((9223372036854775807, 9223372036854775807), 18446744073709551615)");
    }

    #[test]
    fn from() {
        assert_eq!(Circle::from(circle_i8::Circle::new(point_i8::Point::new(i8::MIN, i8::MAX), u8::MAX)), Circle { p: Point { x: i8::MIN.into(), y: i8::MAX.into() }, r: u8::MAX.into() });
        assert_eq!(Circle::from(circle_i16::Circle::new(point_i16::Point::new(i16::MIN, i16::MAX), u16::MAX)), Circle { p: Point { x: i16::MIN.into(), y: i16::MAX.into() }, r: u16::MAX.into() });
        assert_eq!(Circle::from(circle_i32::Circle::new(point_i32::Point::new(i32::MIN, i32::MAX), u32::MAX)), Circle { p: Point { x: i32::MIN.into(), y: i32::MAX.into() }, r: u32::MAX.into() });
    }
}
