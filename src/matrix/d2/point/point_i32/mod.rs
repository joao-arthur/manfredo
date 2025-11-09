use super::{point_i8, point_i16};
use crate::matrix::d1::point::point_i32::{MAX, MIN};

mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::{delta, delta_col, delta_max, delta_min, delta_row};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub row: i32,
    pub col: i32,
}

impl Point {
    pub fn new(row: i32, col: i32) -> Self {
        Point { row, col }
    }

    pub fn min() -> Self {
        Point { row: MIN, col: MIN }
    }

    pub fn max() -> Self {
        Point { row: MAX, col: MAX }
    }

    pub fn zero() -> Self {
        Point { row: 0, col: 0 }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl From<point_i8::Point> for Point {
    fn from(p: point_i8::Point) -> Self {
        Point { row: p.row.into(), col: p.col.into() }
    }
}

impl From<point_i16::Point> for Point {
    fn from(p: point_i16::Point) -> Self {
        Point { row: p.row.into(), col: p.col.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::matrix::{
        d1::point::point_i32::{MAX, MIN},
        d2::point::{point_i8, point_i16},
    };

    #[test]
    fn point() {
        assert_eq!(Point::new(-1, 1), Point { row: -1, col: 1 });
        assert_eq!(Point::new(1, -1), Point { row: 1, col: -1 });
        assert_eq!(Point::min(), Point { row: MIN, col: MIN });
        assert_eq!(Point::max(), Point { row: MAX, col: MAX });
        assert_eq!(Point::zero(), Point { row: 0, col: 0 });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::new(-1, 1).to_string(), "(-1, 1)");
        assert_eq!(Point::min().to_string(), "(-2147483648, -2147483648)");
        assert_eq!(Point::max().to_string(), "(2147483647, 2147483647)");
        assert_eq!(Point::zero().to_string(), "(0, 0)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_i8::Point::min()), Point { row: i8::MIN.into(), col: i8::MIN.into() });
        assert_eq!(Point::from(point_i8::Point::max()), Point { row: i8::MAX.into(), col: i8::MAX.into() });
        assert_eq!(Point::from(point_i16::Point::min()), Point { row: i16::MIN.into(), col: i16::MIN.into() });
        assert_eq!(Point::from(point_i16::Point::max()), Point { row: i16::MAX.into(), col: i16::MAX.into() });
    }
}
