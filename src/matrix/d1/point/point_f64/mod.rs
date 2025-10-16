use super::point_f32;

mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::delta;

pub const MIN: f64 = -9_007_199_254_740_992.0;
pub const MAX: f64 = 9_007_199_254_740_991.0;

#[derive(PartialEq, Debug, Clone)]
pub struct Point {
    pub i: f64,
}

impl Point {
    pub fn of(i: f64) -> Self {
        Point { i }
    }

    pub fn min() -> Self {
        Point { i: MIN }
    }

    pub fn max() -> Self {
        Point { i: MAX }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.i)
    }
}

impl From<point_f32::Point> for Point {
    fn from(p: point_f32::Point) -> Self {
        Point { i: p.i.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX, MIN, Point};
    use crate::matrix::d1::point::point_f32;

    #[test]
    fn point() {
        assert_eq!(Point::of(-10.0), Point { i: -10.0 });
        assert_eq!(Point::of(10.0), Point { i: 10.0 });
        assert_eq!(Point::min(), Point { i: MIN });
        assert_eq!(Point::max(), Point { i: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(-10.0).to_string(), "(-10)");
        assert_eq!(Point::of(10.0).to_string(), "(10)");
        assert_eq!(Point::min().to_string(), "(-9007199254740992)");
        assert_eq!(Point::max().to_string(), "(9007199254740991)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_f32::Point::min()), Point { i: point_f32::MIN.into() });
        assert_eq!(Point::from(point_f32::Point::max()), Point { i: point_f32::MAX.into() });
    }
}
