use super::{point_u8, point_u16};
use crate::matrix::d1::point::point_u32::MAX;

mod delta;

pub use self::delta::{delta, delta_col, delta_depth, delta_max, delta_min, delta_row};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub row: u32,
    pub col: u32,
    pub depth: u32,
}

impl Point {
    pub fn new(row: u32, col: u32, depth: u32) -> Self {
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

impl From<point_u8::Point> for Point {
    fn from(p: point_u8::Point) -> Self {
        Point { row: p.row.into(), col: p.col.into(), depth: p.depth.into() }
    }
}

impl From<point_u16::Point> for Point {
    fn from(p: point_u16::Point) -> Self {
        Point { row: p.row.into(), col: p.col.into(), depth: p.depth.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::matrix::{
        d1::point::point_u32::MAX,
        d3::point::{point_u8, point_u16},
    };

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
        assert_eq!(Point::max().to_string(), "(4294967295, 4294967295, 4294967295)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_u8::Point::min()), Point { row: u8::MIN.into(), col: u8::MIN.into(), depth: u8::MIN.into() });
        assert_eq!(Point::from(point_u8::Point::max()), Point { row: u8::MAX.into(), col: u8::MAX.into(), depth: u8::MAX.into() });
        assert_eq!(Point::from(point_u16::Point::min()), Point { row: u16::MIN.into(), col: u16::MIN.into(), depth: u16::MIN.into() });
        assert_eq!(Point::from(point_u16::Point::max()), Point { row: u16::MAX.into(), col: u16::MAX.into(), depth: u16::MAX.into() });
    }
}
