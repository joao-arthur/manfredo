use crate::cartesian::d2::point::point_u8::Point;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Circle {
    pub center: Point,
    pub radius: u8,
}

impl Circle {
    pub fn of(center: Point, radius: u8) -> Self {
      Circle { center, radius }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.center, self.radius)
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;
    use crate::cartesian::d2::point::point_u8::Point;

    const MAX: u8 = u8::MAX;

    #[test]
    fn circle() {
        assert_eq!(Circle::of(Point::of(0, 0), u8::MAX), Circle { center: Point { x: 0, y: 0 }, radius: u8::MAX });
        assert_eq!(Circle::of(Point::of(0, MAX), u8::MAX), Circle { center: Point { x: 0, y: MAX }, radius: u8::MAX });
        assert_eq!(Circle::of(Point::of(MAX, 0), u8::MAX), Circle { center: Point { x: MAX, y: 0 }, radius: u8::MAX });
        assert_eq!(Circle::of(Point::of(MAX, MAX), u8::MAX), Circle { center: Point { x: MAX, y: MAX }, radius: u8::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::of(Point::of(0, 0), u8::MAX).to_string(), "((0, 0), 255)");
        assert_eq!(Circle::of(Point::of(0, MAX), u8::MAX).to_string(), "((0, 255), 255)");
        assert_eq!(Circle::of(Point::of(MAX, 0), u8::MAX).to_string(), "((255, 0), 255)");
        assert_eq!(Circle::of(Point::of(MAX, MAX), u8::MAX).to_string(), "((255, 255), 255)");
    }
}
