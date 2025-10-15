use crate::cartesian::d2::point::point_f32::Point;

#[derive(PartialEq, Debug, Clone)]
pub struct Circle {
    pub center: Point,
    pub radius: f32,
}

impl Circle {
    pub fn of(center: Point, radius: f32) -> Self {
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
    use crate::cartesian::d2::point::point_f32::{MAX, MIN, Point};

    #[test]
    fn circle() {
        assert_eq!(Circle::of(Point::of(MIN, MIN), MAX), Circle { center: Point { x: MIN, y: MIN }, radius: MAX });
        assert_eq!(Circle::of(Point::of(MIN, MAX), MAX), Circle { center: Point { x: MIN, y: MAX }, radius: MAX });
        assert_eq!(Circle::of(Point::of(MAX, MIN), MAX), Circle { center: Point { x: MAX, y: MIN }, radius: MAX });
        assert_eq!(Circle::of(Point::of(MAX, MAX), MAX), Circle { center: Point { x: MAX, y: MAX }, radius: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::of(Point::of(MIN, MIN), MAX).to_string(), "((-16777216, -16777216), 16777215)");
        assert_eq!(Circle::of(Point::of(MIN, MAX), MAX).to_string(), "((-16777216, 16777215), 16777215)");
        assert_eq!(Circle::of(Point::of(MAX, MIN), MAX).to_string(), "((16777215, -16777216), 16777215)");
        assert_eq!(Circle::of(Point::of(MAX, MAX), MAX).to_string(), "((16777215, 16777215), 16777215)");
    }
}
