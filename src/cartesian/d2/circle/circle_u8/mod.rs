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
    use crate::cartesian::d2::point::point_u8::Point;

    const MAX: u8 = u8::MAX;

    #[test]
    fn circle() {
        assert_eq!(Circle::of(Point::of(0, 0), u8::MAX), Circle { p: Point { x: 0, y: 0 }, r: u8::MAX });
        assert_eq!(Circle::of(Point::of(0, MAX), u8::MAX), Circle { p: Point { x: 0, y: MAX }, r: u8::MAX });
        assert_eq!(Circle::of(Point::of(MAX, 0), u8::MAX), Circle { p: Point { x: MAX, y: 0 }, r: u8::MAX });
        assert_eq!(Circle::of(Point::of(MAX, MAX), u8::MAX), Circle { p: Point { x: MAX, y: MAX }, r: u8::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::of(Point::of(0, 0), u8::MAX).to_string(), "((0, 0), 255)");
        assert_eq!(Circle::of(Point::of(0, MAX), u8::MAX).to_string(), "((0, 255), 255)");
        assert_eq!(Circle::of(Point::of(MAX, 0), u8::MAX).to_string(), "((255, 0), 255)");
        assert_eq!(Circle::of(Point::of(MAX, MAX), u8::MAX).to_string(), "((255, 255), 255)");
    }
}
