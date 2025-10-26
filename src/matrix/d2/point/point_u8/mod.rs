use crate::matrix::d1::point::point_u8::MAX;

mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::{delta, delta_col, delta_max, delta_min, delta_row};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub row: u8,
    pub col: u8,
}

impl Point {
    pub fn of(row: u8, col: u8) -> Self {
        Point { row, col }
    }

    pub fn min() -> Self {
        Point { row: 0, col: 0 }
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

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::matrix::d1::point::point_u8::MAX;

    #[test]
    fn point() {
        assert_eq!(Point::of(1, 2), Point { row: 1, col: 2 });
        assert_eq!(Point::of(2, 1), Point { row: 2, col: 1 });
        assert_eq!(Point::min(), Point { row: 0, col: 0 });
        assert_eq!(Point::max(), Point { row: MAX, col: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(1, 2).to_string(), "(1, 2)");
        assert_eq!(Point::min().to_string(), "(0, 0)");
        assert_eq!(Point::max().to_string(), "(255, 255)");
    }
}
