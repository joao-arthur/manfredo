use super::point_f32;

mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::delta;

pub const MIN: f64 = -9_007_199_254_740_992.0;
pub const MAX: f64 = 9_007_199_254_740_991.0;

#[derive(PartialEq, Debug, Clone)]
pub struct Point {
    pub x: f64,
}

impl Point {
    pub fn of(x: f64) -> Self {
        Point { x }
    }

    pub fn min() -> Self {
        Point { x: MIN }
    }

    pub fn max() -> Self {
        Point { x: MAX }
    }

    pub fn zero() -> Self {
        Point { x: 0.0 }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.x)
    }
}

impl From<point_f32::Point> for Point {
    fn from(p: point_f32::Point) -> Self {
        Point { x: p.x.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX, MIN, Point};
    use crate::cartesian::d1::point::point_f32;

    #[test]
    fn point() {
        assert_eq!(Point::of(-10.0), Point { x: -10.0 });
        assert_eq!(Point::of(10.0), Point { x: 10.0 });
        assert_eq!(Point::min(), Point { x: MIN });
        assert_eq!(Point::max(), Point { x: MAX });
        assert_eq!(Point::zero(), Point { x: 0.0 });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(-10.0).to_string(), "(-10)");
        assert_eq!(Point::of(10.0).to_string(), "(10)");
        assert_eq!(Point::min().to_string(), "(-9007199254740992)");
        assert_eq!(Point::max().to_string(), "(9007199254740991)");
        assert_eq!(Point::zero().to_string(), "(0)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_f32::Point::min()), Point { x: point_f32::MIN.into() });
        assert_eq!(Point::from(point_f32::Point::max()), Point { x: point_f32::MAX.into() });
    }
}
