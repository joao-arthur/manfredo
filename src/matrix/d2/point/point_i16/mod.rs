use super::point_i8;
use crate::matrix::d1::point::point_i16::{MAX, MIN};

mod add;
mod delta;
mod distance;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::{delta, delta_col, delta_max, delta_min, delta_row};
pub use self::distance::distance;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub row: i16,
    pub col: i16,
}

impl Point {
    pub fn of(row: i16, col: i16) -> Self {
        Point { row, col }
    }

    pub fn min() -> Self {
        Point { row: MIN, col: MIN }
    }

    pub fn max() -> Self {
        Point { row: MAX, col: MAX }
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

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::matrix::{
        d1::point::point_i16::{MAX, MIN},
        d2::point::point_i8,
    };

    #[test]
    fn point() {
        assert_eq!(Point::of(MIN, MAX), Point { row: MIN, col: MAX });
        assert_eq!(Point::min(), Point { row: MIN, col: MIN });
        assert_eq!(Point::max(), Point { row: MAX, col: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(MIN, MAX).to_string(), "(-32768, 32767)");
        assert_eq!(Point::min().to_string(), "(-32768, -32768)");
        assert_eq!(Point::max().to_string(), "(32767, 32767)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_i8::Point::min()), Point { row: i8::MIN.into(), col: i8::MIN.into() });
        assert_eq!(Point::from(point_i8::Point::max()), Point { row: i8::MAX.into(), col: i8::MAX.into() });
    }
}
