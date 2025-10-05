use super::{point_u8, point_u16, point_u32};

mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::{delta, delta_col, delta_row};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub row: u64,
    pub col: u64,
}

impl Point {
    pub fn of(row: u64, col: u64) -> Self {
        Point { row, col }
    }

    pub fn min() -> Self {
        Point { row: 0, col: 0 }
    }

    pub fn max() -> Self {
        Point { row: u64::MAX, col: u64::MAX }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl From<point_u8::Point> for Point {
    fn from(p: point_u8::Point) -> Self {
        Point { row: p.row.into(), col: p.col.into() }
    }
}

impl From<point_u16::Point> for Point {
    fn from(p: point_u16::Point) -> Self {
        Point { row: p.row.into(), col: p.col.into() }
    }
}

impl From<point_u32::Point> for Point {
    fn from(p: point_u32::Point) -> Self {
        Point { row: p.row.into(), col: p.col.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::matrix::d2::point::{point_u8, point_u16, point_u32};

    #[test]
    fn point() {
        assert_eq!(Point::of(0, u64::MAX), Point { row: 0, col: u64::MAX });
        assert_eq!(Point::min(), Point { row: 0, col: 0 });
        assert_eq!(Point::max(), Point { row: u64::MAX, col: u64::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(0, u64::MAX).to_string(), "(0, 18446744073709551615)");
        assert_eq!(Point::min().to_string(), "(0, 0)");
        assert_eq!(Point::max().to_string(), "(18446744073709551615, 18446744073709551615)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_u8::Point::min()), Point { row: u8::MIN.into(), col: u8::MIN.into() });
        assert_eq!(Point::from(point_u8::Point::max()), Point { row: u8::MAX.into(), col: u8::MAX.into() });
        assert_eq!(Point::from(point_u16::Point::min()), Point { row: u16::MIN.into(), col: u16::MIN.into() });
        assert_eq!(Point::from(point_u16::Point::max()), Point { row: u16::MAX.into(), col: u16::MAX.into() });
        assert_eq!(Point::from(point_u32::Point::min()), Point { row: u32::MIN.into(), col: u32::MIN.into() });
        assert_eq!(Point::from(point_u32::Point::max()), Point { row: u32::MAX.into(), col: u32::MAX.into() });
    }
}
