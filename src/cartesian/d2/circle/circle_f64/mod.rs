use crate::cartesian::d2::{circle::circle_f32, point::point_f64::Point};

mod area;

pub use self::area::area;

#[derive(PartialEq, Debug, Clone)]
pub struct Circle {
    pub p: Point,
    pub r: f64,
}

impl Circle {
    pub fn of(p: Point, r: f64) -> Self {
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
        d1::{
            self,
            point::point_f64::{MAX, MIN},
        },
        d2::{
            circle::circle_f32,
            point::{point_f32, point_f64::Point},
        },
    };

    #[test]
    fn circle() {
        assert_eq!(Circle::of(Point::min(), MAX), Circle { p: Point { x: MIN, y: MIN }, r: MAX });
        assert_eq!(Circle::of(Point::of(MIN, MAX), MAX), Circle { p: Point { x: MIN, y: MAX }, r: MAX });
        assert_eq!(Circle::of(Point::of(MAX, MIN), MAX), Circle { p: Point { x: MAX, y: MIN }, r: MAX });
        assert_eq!(Circle::of(Point::max(), MAX), Circle { p: Point { x: MAX, y: MAX }, r: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Circle::of(Point::min(), MAX).to_string(), "((-9007199254740992, -9007199254740992), 9007199254740991)");
        assert_eq!(Circle::of(Point::of(MIN, MAX), MAX).to_string(), "((-9007199254740992, 9007199254740991), 9007199254740991)");
        assert_eq!(Circle::of(Point::of(MAX, MIN), MAX).to_string(), "((9007199254740991, -9007199254740992), 9007199254740991)");
        assert_eq!(Circle::of(Point::max(), MAX).to_string(), "((9007199254740991, 9007199254740991), 9007199254740991)");
    }

    #[test]
    fn from() {
        assert_eq!(
            Circle::from(circle_f32::Circle::of(point_f32::Point::of(d1::point::point_f32::MIN, d1::point::point_f32::MAX), d1::point::point_f32::MAX)),
            Circle { p: Point { x: d1::point::point_f32::MIN.into(), y: d1::point::point_f32::MAX.into() }, r: d1::point::point_f32::MAX.into() }
        );
    }
}
