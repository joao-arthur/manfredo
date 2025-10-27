use crate::matrix::d1::point::point_i8::{MAX, MIN};

mod delta;

pub use self::delta::{delta, delta_col, delta_depth, delta_max, delta_min, delta_row};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub row: i8,
    pub col: i8,
    pub depth: i8,
    pub channel: i8,
}

impl Point {
    pub fn of(row: i8, col: i8, depth: i8, channel: i8) -> Self {
        Point { row, col, depth, channel }
    }

    pub fn min() -> Self {
        Point { row: MIN, col: MIN, depth: MIN, channel: MIN }
    }

    pub fn max() -> Self {
        Point { row: MAX, col: MAX, depth: MAX, channel: MAX }
    }

    pub fn zero() -> Self {
        Point { row: 0, col: 0, depth: 0, channel: 0 }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.row, self.col, self.depth, self.channel)
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::matrix::d1::point::point_i8::{MAX, MIN};

    #[test]
    fn point() {
        assert_eq!(Point::of(-2, -1, 1, 2), Point { row: -2, col: -1, depth: 1, channel: 2 });
        assert_eq!(Point::of(2, 1, -1, -2), Point { row: 2, col: 1, depth: -1, channel: -2 });
        assert_eq!(Point::min(), Point { row: MIN, col: MIN, depth: MIN, channel: MIN });
        assert_eq!(Point::max(), Point { row: MAX, col: MAX, depth: MAX, channel: MAX });
        assert_eq!(Point::zero(), Point { row: 0, col: 0, depth: 0, channel: 0 });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(-2, -1, 1, 2).to_string(), "(-2, -1, 1, 2)");
        assert_eq!(Point::min().to_string(), "(-128, -128, -128, -128)");
        assert_eq!(Point::max().to_string(), "(127, 127, 127, 127)");
        assert_eq!(Point::zero().to_string(), "(0, 0, 0, 0)");
    }
}
