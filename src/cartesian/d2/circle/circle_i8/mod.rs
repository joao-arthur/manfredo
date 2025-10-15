use crate::cartesian::d2::point::point_i8::Point;

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
    use crate::cartesian::d2::point::point_i8::Point;

    const MIN: i8 = i8::MIN;
    const MAX: i8 = i8::MAX;

    #[test]
    fn circle() {
        assert_eq!(Circle::of(Point::of(MIN, MIN), u8::MAX), Circle { center: Point { x: MIN, y: MIN }, radius: u8::MAX });
        assert_eq!(Circle::of(Point::of(MIN, MAX), u8::MAX), Circle { center: Point { x: MIN, y: MAX }, radius: u8::MAX });
        assert_eq!(Circle::of(Point::of(MAX, MIN), u8::MAX), Circle { center: Point { x: MAX, y: MIN }, radius: u8::MAX });
        assert_eq!(Circle::of(Point::of(MAX, MAX), u8::MAX), Circle { center: Point { x: MAX, y: MAX }, radius: u8::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::of(Point::of(MIN, MIN), u8::MAX).to_string(), "((-128, -128), 255)");
        assert_eq!(Circle::of(Point::of(MIN, MAX), u8::MAX).to_string(), "((-128, 127), 255)");
        assert_eq!(Circle::of(Point::of(MAX, MIN), u8::MAX).to_string(), "((127, -128), 255)");
        assert_eq!(Circle::of(Point::of(MAX, MAX), u8::MAX).to_string(), "((127, 127), 255)");
    }
}
