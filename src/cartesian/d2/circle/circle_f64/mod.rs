use crate::cartesian::d2::{circle::circle_f32, point::point_f64::Point};

#[derive(PartialEq, Debug, Clone)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn of(center: Point, radius: f64) -> Self {
      Circle { center, radius }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.center, self.radius)
    }
}

impl From<circle_f32::Circle> for Circle {
    fn from(circle: circle_f32::Circle) -> Self {
        Circle { center: Point::from(circle.center), radius: f64::from(circle.radius) }
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;
    use crate::cartesian::d2::{
        circle::circle_f32,
        point::{
            point_f32,
            point_f64::{MAX, MIN, Point},
        },
    };

    #[test]
    fn circle() {
        assert_eq!(Circle::of(Point::of(MIN, MIN), MAX), Circle { center: Point { x: MIN, y: MIN }, radius: MAX });
        assert_eq!(Circle::of(Point::of(MIN, MAX), MAX), Circle { center: Point { x: MIN, y: MAX }, radius: MAX });
        assert_eq!(Circle::of(Point::of(MAX, MIN), MAX), Circle { center: Point { x: MAX, y: MIN }, radius: MAX });
        assert_eq!(Circle::of(Point::of(MAX, MAX), MAX), Circle { center: Point { x: MAX, y: MAX }, radius: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::of(Point::of(MIN, MIN), MAX).to_string(), "((-9007199254740992, -9007199254740992), 9007199254740991)");
        assert_eq!(Circle::of(Point::of(MIN, MAX), MAX).to_string(), "((-9007199254740992, 9007199254740991), 9007199254740991)");
        assert_eq!(Circle::of(Point::of(MAX, MIN), MAX).to_string(), "((9007199254740991, -9007199254740992), 9007199254740991)");
        assert_eq!(Circle::of(Point::of(MAX, MAX), MAX).to_string(), "((9007199254740991, 9007199254740991), 9007199254740991)");
    }

    #[test]
    fn from() {
        assert_eq!(
            Circle::from(circle_f32::Circle::of(point_f32::Point::of(point_f32::MIN, point_f32::MAX), point_f32::MAX)),
            Circle { center: Point { x: point_f32::MIN.into(), y: point_f32::MAX.into() }, radius: point_f32::MAX.into() }
        );
    }
}
