use super::{point_i8, point_i16, point_i32, point_u64};

mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::{delta, delta_col, delta_row};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub row: i64,
    pub col: i64,
}

impl Point {
    pub fn of(row: i64, col: i64) -> Self {
        Point { row, col }
    }

    pub fn min() -> Self {
        Point { row: i64::MIN, col: i64::MIN }
    }

    pub fn max() -> Self {
        Point { row: i64::MAX, col: i64::MAX }
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

impl From<point_i32::Point> for Point {
    fn from(p: point_i32::Point) -> Self {
        Point { row: p.row.into(), col: p.col.into() }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::matrix::d2::point::{point_i8, point_i16, point_i32, point_u64};

    #[test]
    fn point_i64() {
        assert_eq!(Point::of(i64::MIN, i64::MAX), Point { row: i64::MIN, col: i64::MAX });
        assert_eq!(Point::min(), Point { row: i64::MIN, col: i64::MIN });
        assert_eq!(Point::max(), Point { row: i64::MAX, col: i64::MAX });
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_i8::Point::min()), Point { row: i8::MIN.into(), col: i8::MIN.into() });
        assert_eq!(Point::from(point_i8::Point::max()), Point { row: i8::MAX.into(), col: i8::MAX.into() });
        assert_eq!(Point::from(point_i16::Point::min()), Point { row: i16::MIN.into(), col: i16::MIN.into() });
        assert_eq!(Point::from(point_i16::Point::max()), Point { row: i16::MAX.into(), col: i16::MAX.into() });
        assert_eq!(Point::from(point_i32::Point::min()), Point { row: i32::MIN.into(), col: i32::MIN.into() });
        assert_eq!(Point::from(point_i32::Point::max()), Point { row: i32::MAX.into(), col: i32::MAX.into() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(i64::MIN, i64::MAX).to_string(), "(-9223372036854775808, 9223372036854775807)");
        assert_eq!(Point::min().to_string(), "(-9223372036854775808, -9223372036854775808)");
        assert_eq!(Point::max().to_string(), "(9223372036854775807, 9223372036854775807)");
    }
}
