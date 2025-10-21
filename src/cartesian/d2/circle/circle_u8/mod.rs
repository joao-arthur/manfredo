use crate::cartesian::d2::point::point_u8::Point;

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
    use crate::cartesian::{d1::point::point_u8::MAX, d2::point::point_u8::Point};

    #[test]
    fn circle() {
        assert_eq!(Circle::of(Point::min(), MAX), Circle { p: Point { x: 0, y: 0 }, r: MAX });
        assert_eq!(Circle::of(Point::of(0, MAX), MAX), Circle { p: Point { x: 0, y: MAX }, r: MAX });
        assert_eq!(Circle::of(Point::of(MAX, 0), MAX), Circle { p: Point { x: MAX, y: 0 }, r: MAX });
        assert_eq!(Circle::of(Point::max(), MAX), Circle { p: Point { x: MAX, y: MAX }, r: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::of(Point::min(), MAX).to_string(), "((0, 0), 255)");
        assert_eq!(Circle::of(Point::of(0, MAX), MAX).to_string(), "((0, 255), 255)");
        assert_eq!(Circle::of(Point::of(MAX, 0), MAX).to_string(), "((255, 0), 255)");
        assert_eq!(Circle::of(Point::max(), MAX).to_string(), "((255, 255), 255)");
    }
}
