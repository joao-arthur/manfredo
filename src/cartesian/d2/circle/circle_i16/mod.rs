use crate::cartesian::d2::{circle::circle_i8, point::point_i16::Point};

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

impl From<circle_i8::Circle> for Circle {
    fn from(circle: circle_i8::Circle) -> Self {
        Circle { center: Point::from(circle.center), radius: u16::from(circle.radius) }
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;
    use crate::cartesian::d2::{circle::circle_i8, point::{point_i8, point_i16::Point}};

    const MIN: i16 = i16::MIN;
    const MAX: i16 = i16::MAX;

    #[test]
    fn circle() {
        assert_eq!(Circle::of(Point::of(MIN, MIN), u16::MAX), Circle { center: Point { x: MIN, y: MIN }, radius: u16::MAX });
        assert_eq!(Circle::of(Point::of(MIN, MAX), u16::MAX), Circle { center: Point { x: MIN, y: MAX }, radius: u16::MAX });
        assert_eq!(Circle::of(Point::of(MAX, MIN), u16::MAX), Circle { center: Point { x: MAX, y: MIN }, radius: u16::MAX });
        assert_eq!(Circle::of(Point::of(MAX, MAX), u16::MAX), Circle { center: Point { x: MAX, y: MAX }, radius: u16::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::of(Point::of(MIN, MIN), u16::MAX).to_string(), "((-32768, -32768), 65535)");
        assert_eq!(Circle::of(Point::of(MIN, MAX), u16::MAX).to_string(), "((-32768, 32767), 65535)");
        assert_eq!(Circle::of(Point::of(MAX, MIN), u16::MAX).to_string(), "((32767, -32768), 65535)");
        assert_eq!(Circle::of(Point::of(MAX, MAX), u16::MAX).to_string(), "((32767, 32767), 65535)");
    }

    #[test]
    fn from() {
        assert_eq!(
            Circle::from(circle_i8::Circle::of(point_i8::Point::of(i8::MIN, i8::MAX), u8::MAX)),
            Circle { center: Point { x: i8::MIN.into(), y: i8::MAX.into() }, radius: u8::MAX.into() }
        );
    }
}
