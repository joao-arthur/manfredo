use super::{point_i8, point_i16, point_i32};
use crate::matrix::d1::point::point_i64::{MAX, MIN};

mod delta;

pub use self::delta::{delta, delta_col, delta_depth, delta_max, delta_min, delta_row};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub row: i64,
    pub col: i64,
    pub depth: i64,
    pub channel: i64,
}

impl Point {
    pub fn new(row: i64, col: i64, depth: i64, channel: i64) -> Self {
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

impl From<point_i8::Point> for Point {
    fn from(p: point_i8::Point) -> Self {
        Point { row: p.row.into(), col: p.col.into(), depth: p.depth.into(), channel: p.channel.into() }
    }
}

impl From<point_i16::Point> for Point {
    fn from(p: point_i16::Point) -> Self {
        Point { row: p.row.into(), col: p.col.into(), depth: p.depth.into(), channel: p.channel.into() }
    }
}

impl From<point_i32::Point> for Point {
    fn from(p: point_i32::Point) -> Self {
        Point { row: p.row.into(), col: p.col.into(), depth: p.depth.into(), channel: p.channel.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::matrix::{
        d1::point::point_i64::{MAX, MIN},
        d4::point::{point_i8, point_i16, point_i32},
    };

    #[test]
    fn point() {
        assert_eq!(Point::new(-2, -1, 1, 2), Point { row: -2, col: -1, depth: 1, channel: 2 });
        assert_eq!(Point::new(2, 1, -1, -2), Point { row: 2, col: 1, depth: -1, channel: -2 });
        assert_eq!(Point::min(), Point { row: MIN, col: MIN, depth: MIN, channel: MIN });
        assert_eq!(Point::max(), Point { row: MAX, col: MAX, depth: MAX, channel: MAX });
        assert_eq!(Point::zero(), Point { row: 0, col: 0, depth: 0, channel: 0 });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::new(-2, -1, 1, 2).to_string(), "(-2, -1, 1, 2)");
        assert_eq!(Point::min().to_string(), "(-9223372036854775808, -9223372036854775808, -9223372036854775808, -9223372036854775808)");
        assert_eq!(Point::max().to_string(), "(9223372036854775807, 9223372036854775807, 9223372036854775807, 9223372036854775807)");
        assert_eq!(Point::zero().to_string(), "(0, 0, 0, 0)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_i8::Point::min()), Point { row: i8::MIN.into(), col: i8::MIN.into(), depth: i8::MIN.into(), channel: i8::MIN.into() });
        assert_eq!(Point::from(point_i8::Point::max()), Point { row: i8::MAX.into(), col: i8::MAX.into(), depth: i8::MAX.into(), channel: i8::MAX.into() });
        assert_eq!(Point::from(point_i16::Point::min()), Point { row: i16::MIN.into(), col: i16::MIN.into(), depth: i16::MIN.into(), channel: i16::MIN.into() });
        assert_eq!(Point::from(point_i16::Point::max()), Point { row: i16::MAX.into(), col: i16::MAX.into(), depth: i16::MAX.into(), channel: i16::MAX.into() });
        assert_eq!(Point::from(point_i32::Point::min()), Point { row: i32::MIN.into(), col: i32::MIN.into(), depth: i32::MIN.into(), channel: i32::MIN.into() });
        assert_eq!(Point::from(point_i32::Point::max()), Point { row: i32::MAX.into(), col: i32::MAX.into(), depth: i32::MAX.into(), channel: i32::MAX.into() });
    }
}
