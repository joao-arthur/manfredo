use crate::cartesian::d2::{circle::circle_u8, point::point_u16::Point};

mod area;

pub use self::area::area;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Circle {
    pub p: Point,
    pub r: u16,
}

impl Circle {
    pub fn new(p: Point, r: u16) -> Self {
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
        Circle { p: Point::from(c.p), r: u16::from(c.r) }
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;
    use crate::cartesian::{
        d1::point::point_u16::MAX,
        d2::{
            circle::circle_u8,
            point::{point_u8, point_u16::Point},
        },
    };

    #[test]
    fn circle() {
        assert_eq!(Circle::new(Point::min(), MAX), Circle { p: Point { x: 0, y: 0 }, r: MAX });
        assert_eq!(Circle::new(Point::new(0, MAX), MAX), Circle { p: Point { x: 0, y: MAX }, r: MAX });
        assert_eq!(Circle::new(Point::new(MAX, 0), MAX), Circle { p: Point { x: MAX, y: 0 }, r: MAX });
        assert_eq!(Circle::new(Point::max(), MAX), Circle { p: Point { x: MAX, y: MAX }, r: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::new(Point::min(), MAX).to_string(), "((0, 0), 65535)");
        assert_eq!(Circle::new(Point::new(0, MAX), MAX).to_string(), "((0, 65535), 65535)");
        assert_eq!(Circle::new(Point::new(MAX, 0), MAX).to_string(), "((65535, 0), 65535)");
        assert_eq!(Circle::new(Point::max(), MAX).to_string(), "((65535, 65535), 65535)");
    }

    #[test]
    fn from() {
        assert_eq!(Circle::from(circle_u8::Circle::new(point_u8::Point::new(0, u8::MAX), u8::MAX)), Circle { p: Point { x: 0, y: u8::MAX.into() }, r: u8::MAX.into() });
    }
}
