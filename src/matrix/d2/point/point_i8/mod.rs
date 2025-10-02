mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::{delta, delta_col, delta_row};

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
        Point { row: i8::MIN, col: i8::MIN }
    }

    pub fn max() -> Self {
        Point { row: i8::MAX, col: i8::MAX }
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

    #[test]
    fn point_i8() {
        assert_eq!(Point::of(i8::MIN, i8::MAX), Point { row: i8::MIN, col: i8::MAX });
        assert_eq!(Point::min(), Point { row: i8::MIN, col: i8::MIN });
        assert_eq!(Point::max(), Point { row: i8::MAX, col: i8::MAX });
        assert_eq!(Point::of(i8::MIN, i8::MAX).to_string(), "(-128, 127)");
        assert_eq!(Point::min().to_string(), "(-128, -128)");
        assert_eq!(Point::max().to_string(), "(127, 127)");
    }
}
