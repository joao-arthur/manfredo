use crate::cartesian::d2::{circle::circle_i8, point::point_i16::Point};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Circle {
    pub p: Point,
    pub r: u16,
}

mod area;

pub use self::area::area;

impl Circle {
    pub fn of(p: Point, r: u16) -> Self {
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
        Circle { p: Point::from(c.p), r: u16::from(c.r) }
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;
    use crate::cartesian::{
        d1::point::point_i16::{MAX, MIN},
        d2::{
            circle::circle_i8,
            point::{point_i8, point_i16::Point},
        },
    };

    #[test]
    fn circle() {
        assert_eq!(Circle::of(Point::min(), u16::MAX), Circle { p: Point { x: MIN, y: MIN }, r: u16::MAX });
        assert_eq!(Circle::of(Point::of(MIN, MAX), u16::MAX), Circle { p: Point { x: MIN, y: MAX }, r: u16::MAX });
        assert_eq!(Circle::of(Point::of(MAX, MIN), u16::MAX), Circle { p: Point { x: MAX, y: MIN }, r: u16::MAX });
        assert_eq!(Circle::of(Point::max(), u16::MAX), Circle { p: Point { x: MAX, y: MAX }, r: u16::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::of(Point::min(), u16::MAX).to_string(), "((-32768, -32768), 65535)");
        assert_eq!(Circle::of(Point::of(MIN, MAX), u16::MAX).to_string(), "((-32768, 32767), 65535)");
        assert_eq!(Circle::of(Point::of(MAX, MIN), u16::MAX).to_string(), "((32767, -32768), 65535)");
        assert_eq!(Circle::of(Point::max(), u16::MAX).to_string(), "((32767, 32767), 65535)");
    }

    #[test]
    fn from() {
        assert_eq!(Circle::from(circle_i8::Circle::of(point_i8::Point::of(i8::MIN, i8::MAX), u8::MAX)), Circle { p: Point { x: i8::MIN.into(), y: i8::MAX.into() }, r: u8::MAX.into() });
    }
}
