use crate::matrix::d1::point::point_i8::{MAX, MIN};

mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::{delta, delta_col, delta_max, delta_min, delta_row};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub row: i8,
    pub col: i8,
}

impl Point {
    pub fn of(row: i8, col: i8) -> Self {
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

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::matrix::d1::point::point_i8::{MAX, MIN};

    #[test]
    fn point() {
        assert_eq!(Point::of(-1, 1), Point { row: -1, col: 1 });
        assert_eq!(Point::of(1, -1), Point { row: 1, col: -1 });
        assert_eq!(Point::min(), Point { row: MIN, col: MIN });
        assert_eq!(Point::max(), Point { row: MAX, col: MAX });
        assert_eq!(Point::zero(), Point { row: 0, col: 0 });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(-1, 1).to_string(), "(-1, 1)");
        assert_eq!(Point::min().to_string(), "(-128, -128)");
        assert_eq!(Point::max().to_string(), "(127, 127)");
        assert_eq!(Point::zero().to_string(), "(0, 0)");
    }
}
