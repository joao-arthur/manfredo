use super::point_i8;
use crate::matrix::d1::point::point_i16::{MAX, MIN};

mod delta;

pub use self::delta::{delta, delta_col, delta_depth, delta_max, delta_min, delta_row};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub row: i16,
    pub col: i16,
    pub depth: i16,
}

impl Point {
    pub fn of(row: i16, col: i16, depth: i16) -> Self {
        Point { row, col, depth }
    }

    pub fn min() -> Self {
        Point { row: MIN, col: MIN, depth: MIN }
    }

    pub fn max() -> Self {
        Point { row: MAX, col: MAX, depth: MAX }
    }

    pub fn zero() -> Self {
        Point { row: 0, col: 0, depth: 0 }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.row, self.col, self.depth)
    }
}

impl From<point_i8::Point> for Point {
    fn from(p: point_i8::Point) -> Self {
        Point { row: p.row.into(), col: p.col.into(), depth: p.depth.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::matrix::{
        d1::point::point_i16::{MAX, MIN},
        d3::point::point_i8,
    };

    #[test]
    fn point() {
        assert_eq!(Point::of(-1, 0, 1), Point { row: -1, col: 0, depth: 1 });
        assert_eq!(Point::of(0, 1, -1), Point { row: 0, col: 1, depth: -1 });
        assert_eq!(Point::min(), Point { row: MIN, col: MIN, depth: MIN });
        assert_eq!(Point::max(), Point { row: MAX, col: MAX, depth: MAX });
        assert_eq!(Point::zero(), Point { row: 0, col: 0, depth: 0 });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(-1, 0, 1).to_string(), "(-1, 0, 1)");
        assert_eq!(Point::min().to_string(), "(-32768, -32768, -32768)");
        assert_eq!(Point::max().to_string(), "(32767, 32767, 32767)");
        assert_eq!(Point::zero().to_string(), "(0, 0, 0)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_i8::Point::min()), Point { row: i8::MIN.into(), col: i8::MIN.into(), depth: i8::MIN.into() });
        assert_eq!(Point::from(point_i8::Point::max()), Point { row: i8::MAX.into(), col: i8::MAX.into(), depth: i8::MAX.into() });
    }
}
