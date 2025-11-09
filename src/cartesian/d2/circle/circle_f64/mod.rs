use crate::cartesian::d2::{circle::circle_f32, point::point_f64::Point};

mod area;

pub use self::area::area;

#[derive(PartialEq, Debug, Clone)]
pub struct Circle {
    pub p: Point,
    pub r: f64,
}

impl Circle {
    pub fn new(p: Point, r: f64) -> Self {
        Circle { p, r }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.p, self.r)
    }
}

impl From<circle_f32::Circle> for Circle {
    fn from(c: circle_f32::Circle) -> Self {
        Circle { p: Point::from(c.p), r: f64::from(c.r) }
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;
    use crate::cartesian::{
        d1::point::{
            point_f32::{MAX as MAX_F32, MIN as MIN_F32},
            point_f64::{MAX, MIN},
        },
        d2::{
            circle::circle_f32::Circle as CircleF32,
            point::{point_f32::Point as PointF32, point_f64::Point},
        },
    };

    #[test]
    fn circle() {
        assert_eq!(Circle::new(Point::min(), MAX), Circle { p: Point { x: MIN, y: MIN }, r: MAX });
        assert_eq!(Circle::new(Point::new(MIN, MAX), MAX), Circle { p: Point { x: MIN, y: MAX }, r: MAX });
        assert_eq!(Circle::new(Point::new(MAX, MIN), MAX), Circle { p: Point { x: MAX, y: MIN }, r: MAX });
        assert_eq!(Circle::new(Point::max(), MAX), Circle { p: Point { x: MAX, y: MAX }, r: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::new(Point::min(), MAX).to_string(), "((-9007199254740992, -9007199254740992), 9007199254740991)");
        assert_eq!(Circle::new(Point::new(MIN, MAX), MAX).to_string(), "((-9007199254740992, 9007199254740991), 9007199254740991)");
        assert_eq!(Circle::new(Point::new(MAX, MIN), MAX).to_string(), "((9007199254740991, -9007199254740992), 9007199254740991)");
        assert_eq!(Circle::new(Point::max(), MAX).to_string(), "((9007199254740991, 9007199254740991), 9007199254740991)");
    }

    #[test]
    fn from() {
        assert_eq!(Circle::from(CircleF32::new(PointF32::new(MIN_F32, MAX_F32), MAX_F32)), Circle { p: Point { x: MIN_F32.into(), y: MAX_F32.into() }, r: MAX_F32.into() });
    }
}
