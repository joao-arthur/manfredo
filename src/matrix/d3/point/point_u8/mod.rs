use crate::matrix::d1::point::point_u8::MAX;

mod delta;

pub use self::delta::{delta, delta_col, delta_depth, delta_max, delta_min, delta_row};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub row: u8,
    pub col: u8,
    pub depth: u8,
}

impl Point {
    pub fn new(row: u8, col: u8, depth: u8) -> Self {
        Point { row, col, depth }
    }

    pub fn min() -> Self {
        Point { row: 0, col: 0, depth: 0 }
    }

    pub fn max() -> Self {
        Point { row: MAX, col: MAX, depth: MAX }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.row, self.col, self.depth)
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::matrix::d1::point::point_u8::MAX;

    #[test]
    fn point() {
        assert_eq!(Point::new(1, 2, 3), Point { row: 1, col: 2, depth: 3 });
        assert_eq!(Point::new(2, 3, 1), Point { row: 2, col: 3, depth: 1 });
        assert_eq!(Point::min(), Point { row: 0, col: 0, depth: 0 });
        assert_eq!(Point::max(), Point { row: MAX, col: MAX, depth: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::new(1, 2, 3).to_string(), "(1, 2, 3)");
        assert_eq!(Point::min().to_string(), "(0, 0, 0)");
        assert_eq!(Point::max().to_string(), "(255, 255, 255)");
    }
}
