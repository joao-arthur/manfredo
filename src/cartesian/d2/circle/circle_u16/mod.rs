use crate::cartesian::d2::{circle::circle_u8, point::point_u16::Point};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Circle {
    pub center: Point,
    pub radius: u16,
}

impl Circle {
    pub fn of(center: Point, radius: u16) -> Self {
      Circle { center, radius }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.center, self.radius)
    }
}

impl From<circle_u8::Circle> for Circle {
    fn from(circle: circle_u8::Circle) -> Self {
        Circle { center: Point::from(circle.center), radius: u16::from(circle.radius) }
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;
    use crate::cartesian::d2::{circle::circle_u8, point::{point_u16::Point, point_u8}};

    const MAX: u16 = u16::MAX;

    #[test]
    fn circle() {
        assert_eq!(Circle::of(Point::of(0, 0), u16::MAX), Circle { center: Point { x: 0, y: 0 }, radius: u16::MAX });
        assert_eq!(Circle::of(Point::of(0, MAX), u16::MAX), Circle { center: Point { x: 0, y: MAX }, radius: u16::MAX });
        assert_eq!(Circle::of(Point::of(MAX, 0), u16::MAX), Circle { center: Point { x: MAX, y: 0 }, radius: u16::MAX });
        assert_eq!(Circle::of(Point::of(MAX, MAX), u16::MAX), Circle { center: Point { x: MAX, y: MAX }, radius: u16::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::of(Point::of(0, 0), u16::MAX).to_string(), "((0, 0), 65535)");
        assert_eq!(Circle::of(Point::of(0, MAX), u16::MAX).to_string(), "((0, 65535), 65535)");
        assert_eq!(Circle::of(Point::of(MAX, 0), u16::MAX).to_string(), "((65535, 0), 65535)");
        assert_eq!(Circle::of(Point::of(MAX, MAX), u16::MAX).to_string(), "((65535, 65535), 65535)");
    }

    #[test]
    fn from() {
        assert_eq!(
            Circle::from(circle_u8::Circle::of(point_u8::Point::of(0, u8::MAX), u8::MAX)),
            Circle { center: Point { x: 0, y: u8::MAX.into() }, radius: u8::MAX.into() }
        );
    }
}
