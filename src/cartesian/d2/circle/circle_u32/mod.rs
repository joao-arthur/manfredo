use crate::cartesian::d2::{
    circle::{circle_u8, circle_u16},
    point::point_u32::Point,
};

mod area;

pub use self::area::area;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Circle {
    pub p: Point,
    pub r: u32,
}

impl Circle {
    pub fn of(p: Point, r: u32) -> Self {
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
        Circle { p: Point::from(c.p), r: u32::from(c.r) }
    }
}

impl From<circle_u16::Circle> for Circle {
    fn from(c: circle_u16::Circle) -> Self {
        Circle { p: Point::from(c.p), r: u32::from(c.r) }
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;
    use crate::cartesian::{
        d1::point::point_u32::MAX,
        d2::{
            circle::{circle_u8, circle_u16},
            point::{point_u8, point_u16, point_u32::Point},
        },
    };

    #[test]
    fn circle() {
        assert_eq!(Circle::of(Point::min(), u32::MAX), Circle { p: Point { x: 0, y: 0 }, r: u32::MAX });
        assert_eq!(Circle::of(Point::of(0, MAX), u32::MAX), Circle { p: Point { x: 0, y: MAX }, r: u32::MAX });
        assert_eq!(Circle::of(Point::of(MAX, 0), u32::MAX), Circle { p: Point { x: MAX, y: 0 }, r: u32::MAX });
        assert_eq!(Circle::of(Point::max(), u32::MAX), Circle { p: Point { x: MAX, y: MAX }, r: u32::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::of(Point::min(), u32::MAX).to_string(), "((0, 0), 4294967295)");
        assert_eq!(Circle::of(Point::of(0, MAX), u32::MAX).to_string(), "((0, 4294967295), 4294967295)");
        assert_eq!(Circle::of(Point::of(MAX, 0), u32::MAX).to_string(), "((4294967295, 0), 4294967295)");
        assert_eq!(Circle::of(Point::max(), u32::MAX).to_string(), "((4294967295, 4294967295), 4294967295)");
    }

    #[test]
    fn from() {
        assert_eq!(Circle::from(circle_u8::Circle::of(point_u8::Point::of(0, u8::MAX), u8::MAX)), Circle { p: Point { x: 0, y: u8::MAX.into() }, r: u8::MAX.into() });
        assert_eq!(Circle::from(circle_u16::Circle::of(point_u16::Point::of(0, u16::MAX), u16::MAX)), Circle { p: Point { x: 0, y: u16::MAX.into() }, r: u16::MAX.into() });
    }
}
