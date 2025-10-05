use super::{point_u8, point_u16};

mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::{delta, delta_col, delta_row};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub row: u32,
    pub col: u32,
}

impl Point {
    pub fn of(row: u32, col: u32) -> Self {
        Point { row, col }
    }

    pub fn min() -> Self {
        Point { row: 0, col: 0 }
    }

    pub fn max() -> Self {
        Point { row: u32::MAX, col: u32::MAX }
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

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::matrix::d2::point::{point_u8, point_u16};

    #[test]
    fn point() {
        assert_eq!(Point::of(0, u32::MAX), Point { row: 0, col: u32::MAX });
        assert_eq!(Point::min(), Point { row: 0, col: 0 });
        assert_eq!(Point::max(), Point { row: u32::MAX, col: u32::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(0, u32::MAX).to_string(), "(0, 4294967295)");
        assert_eq!(Point::min().to_string(), "(0, 0)");
        assert_eq!(Point::max().to_string(), "(4294967295, 4294967295)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_u8::Point::min()), Point { row: u8::MIN.into(), col: u8::MIN.into() });
        assert_eq!(Point::from(point_u8::Point::max()), Point { row: u8::MAX.into(), col: u8::MAX.into() });
        assert_eq!(Point::from(point_u16::Point::min()), Point { row: u16::MIN.into(), col: u16::MIN.into() });
        assert_eq!(Point::from(point_u16::Point::max()), Point { row: u16::MAX.into(), col: u16::MAX.into() });
    }
}
