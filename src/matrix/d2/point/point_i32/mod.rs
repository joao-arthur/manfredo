use super::{point_i8, point_i16, point_u32};

mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::{delta, delta_col, delta_row};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub row: i32,
    pub col: i32,
}

impl Point {
    pub fn of(row: i32, col: i32) -> Self {
        Point { row, col }
    }

    pub fn min() -> Self {
        Point { row: i32::MIN, col: i32::MIN }
    }

    pub fn max() -> Self {
        Point { row: i32::MAX, col: i32::MAX }
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

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::matrix::d2::point::{point_i8, point_i16, point_u32};

    #[test]
    fn point_i32() {
        assert_eq!(Point::of(i32::MIN, i32::MAX), Point { row: i32::MIN, col: i32::MAX });
        assert_eq!(Point::min(), Point { row: i32::MIN, col: i32::MIN });
        assert_eq!(Point::max(), Point { row: i32::MAX, col: i32::MAX });
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_i8::Point::min()), Point { row: i8::MIN.into(), col: i8::MIN.into() });
        assert_eq!(Point::from(point_i8::Point::max()), Point { row: i8::MAX.into(), col: i8::MAX.into() });
        assert_eq!(Point::from(point_i16::Point::min()), Point { row: i16::MIN.into(), col: i16::MIN.into() });
        assert_eq!(Point::from(point_i16::Point::max()), Point { row: i16::MAX.into(), col: i16::MAX.into() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(i32::MIN, i32::MAX).to_string(), "(-2147483648, 2147483647)");
        assert_eq!(Point::min().to_string(), "(-2147483648, -2147483648)");
        assert_eq!(Point::max().to_string(), "(2147483647, 2147483647)");
    }
}
