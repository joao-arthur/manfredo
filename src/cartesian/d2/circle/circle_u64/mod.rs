use crate::cartesian::d2::{
    circle::{circle_u8, circle_u16, circle_u32},
    point::point_u64::Point,
};

mod area;

pub use self::area::area;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Circle {
    pub p: Point,
    pub r: u64,
}

impl Circle {
    pub fn of(p: Point, r: u64) -> Self {
        Circle { p, r }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.p, self.r)
    }
}

impl From<circle_u8::Circle> for Circle {
    fn from(c: circle_u8::Circle) -> Self {
        Circle { p: Point::from(c.p), r: u64::from(c.r) }
    }
}

impl From<circle_u16::Circle> for Circle {
    fn from(c: circle_u16::Circle) -> Self {
        Circle { p: Point::from(c.p), r: u64::from(c.r) }
    }
}

impl From<circle_u32::Circle> for Circle {
    fn from(c: circle_u32::Circle) -> Self {
        Circle { p: Point::from(c.p), r: u64::from(c.r) }
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;
    use crate::cartesian::d2::{
        circle::{circle_u8, circle_u16, circle_u32},
        point::{point_u8, point_u16, point_u32, point_u64::Point},
    };

    const MAX: u64 = u64::MAX;

    #[test]
    fn circle() {
        assert_eq!(Circle::of(Point::min(), u64::MAX), Circle { p: Point { x: 0, y: 0 }, r: u64::MAX });
        assert_eq!(Circle::of(Point::of(0, MAX), u64::MAX), Circle { p: Point { x: 0, y: MAX }, r: u64::MAX });
        assert_eq!(Circle::of(Point::of(MAX, 0), u64::MAX), Circle { p: Point { x: MAX, y: 0 }, r: u64::MAX });
        assert_eq!(Circle::of(Point::max(), u64::MAX), Circle { p: Point { x: MAX, y: MAX }, r: u64::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::of(Point::min(), u64::MAX).to_string(), "((0, 0), 18446744073709551615)");
        assert_eq!(Circle::of(Point::of(0, MAX), u64::MAX).to_string(), "((0, 18446744073709551615), 18446744073709551615)");
        assert_eq!(Circle::of(Point::of(MAX, 0), u64::MAX).to_string(), "((18446744073709551615, 0), 18446744073709551615)");
        assert_eq!(Circle::of(Point::max(), u64::MAX).to_string(), "((18446744073709551615, 18446744073709551615), 18446744073709551615)");
    }

    #[test]
    fn from() {
        assert_eq!(Circle::from(circle_u8::Circle::of(point_u8::Point::of(0, u8::MAX), u8::MAX)), Circle { p: Point { x: 0, y: u8::MAX.into() }, r: u8::MAX.into() });
        assert_eq!(Circle::from(circle_u16::Circle::of(point_u16::Point::of(0, u16::MAX), u16::MAX)), Circle { p: Point { x: 0, y: u16::MAX.into() }, r: u16::MAX.into() });
        assert_eq!(Circle::from(circle_u32::Circle::of(point_u32::Point::of(0, u32::MAX), u32::MAX)), Circle { p: Point { x: 0, y: u32::MAX.into() }, r: u32::MAX.into() });
    }
}
