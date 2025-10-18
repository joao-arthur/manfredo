use crate::cartesian::d2::point::point_f32::Point;

mod area;

pub use self::area::area;

#[derive(PartialEq, Debug, Clone)]
pub struct Circle {
    pub p: Point,
    pub r: f32,
}

impl Circle {
    pub fn of(p: Point, r: f32) -> Self {
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
    use crate::cartesian::d2::point::point_f32::{MAX, MIN, Point};

    #[test]
    fn circle() {
        assert_eq!(Circle::of(Point::min(), MAX), Circle { p: Point { x: MIN, y: MIN }, r: MAX });
        assert_eq!(Circle::of(Point::of(MIN, MAX), MAX), Circle { p: Point { x: MIN, y: MAX }, r: MAX });
        assert_eq!(Circle::of(Point::of(MAX, MIN), MAX), Circle { p: Point { x: MAX, y: MIN }, r: MAX });
        assert_eq!(Circle::of(Point::max(), MAX), Circle { p: Point { x: MAX, y: MAX }, r: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::of(Point::min(), MAX).to_string(), "((-16777216, -16777216), 16777215)");
        assert_eq!(Circle::of(Point::of(MIN, MAX), MAX).to_string(), "((-16777216, 16777215), 16777215)");
        assert_eq!(Circle::of(Point::of(MAX, MIN), MAX).to_string(), "((16777215, -16777216), 16777215)");
        assert_eq!(Circle::of(Point::max(), MAX).to_string(), "((16777215, 16777215), 16777215)");
    }
}
