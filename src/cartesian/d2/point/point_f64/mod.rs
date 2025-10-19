use super::point_f32;
use crate::cartesian::d1::point::point_f64::{MAX, MIN};

mod add;
mod delta;
mod distance;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::{delta, delta_max, delta_min, delta_x, delta_y};
pub use self::distance::distance;

#[derive(PartialEq, Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn of(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn min() -> Self {
        Point { x: MIN, y: MIN }
    }

    pub fn max() -> Self {
        Point { x: MAX, y: MAX }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<point_f32::Point> for Point {
    fn from(p: point_f32::Point) -> Self {
        Point { x: p.x.into(), y: p.y.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX, MIN, Point};
    use crate::cartesian::{d1, d2};

    #[test]
    fn point() {
        assert_eq!(Point::of(MIN, MAX), Point { x: MIN, y: MAX });
        assert_eq!(Point::of(MAX, MIN), Point { x: MAX, y: MIN });
        assert_eq!(Point::min(), Point { x: MIN, y: MIN });
        assert_eq!(Point::max(), Point { x: MAX, y: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(MIN, MAX).to_string(), "(-9007199254740992, 9007199254740991)");
        assert_eq!(Point::of(MAX, MIN).to_string(), "(9007199254740991, -9007199254740992)");
        assert_eq!(Point::min().to_string(), "(-9007199254740992, -9007199254740992)");
        assert_eq!(Point::max().to_string(), "(9007199254740991, 9007199254740991)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(d2::point::point_f32::Point::min()), Point { x: d1::point::point_f32::MIN.into(), y: d1::point::point_f32::MIN.into() });
        assert_eq!(Point::from(d2::point::point_f32::Point::max()), Point { x: d1::point::point_f32::MAX.into(), y: d1::point::point_f32::MAX.into() });
    }
}
