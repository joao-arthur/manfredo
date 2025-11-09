use super::point_i8;

mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::delta;

pub const MIN: i16 = i16::MIN;
pub const MAX: i16 = i16::MAX;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub i: i16,
}

impl Point {
    pub fn new(i: i16) -> Self {
        Point { i }
    }

    pub fn min() -> Self {
        Point { i: MIN }
    }

    pub fn max() -> Self {
        Point { i: MAX }
    }

    pub fn zero() -> Self {
        Point { i: 0 }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.i)
    }
}

impl From<point_i8::Point> for Point {
    fn from(p: point_i8::Point) -> Self {
        Point { i: p.i.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX, MIN, Point};
    use crate::matrix::d1::point::point_i8;

    #[test]
    fn point() {
        assert_eq!(Point::new(-1), Point { i: -1 });
        assert_eq!(Point::new(1), Point { i: 1 });
        assert_eq!(Point::min(), Point { i: MIN });
        assert_eq!(Point::max(), Point { i: MAX });
        assert_eq!(Point::zero(), Point { i: 0 });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::new(-1).to_string(), "(-1)");
        assert_eq!(Point::min().to_string(), "(-32768)");
        assert_eq!(Point::max().to_string(), "(32767)");
        assert_eq!(Point::zero().to_string(), "(0)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_i8::Point::min()), Point { i: i8::MIN.into() });
        assert_eq!(Point::from(point_i8::Point::max()), Point { i: i8::MAX.into() });
    }
}
