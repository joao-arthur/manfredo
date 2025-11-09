use super::{point_u8, point_u16};
use crate::matrix::d1::point::point_u32::MAX;

mod delta;

pub use self::delta::{delta, delta_col, delta_depth, delta_max, delta_min, delta_row};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub row: u32,
    pub col: u32,
    pub depth: u32,
    pub channel: u32,
}

impl Point {
    pub fn new(row: u32, col: u32, depth: u32, channel: u32) -> Self {
        Point { row, col, depth, channel }
    }

    pub fn min() -> Self {
        Point { row: 0, col: 0, depth: 0, channel: 0 }
    }

    pub fn max() -> Self {
        Point { row: MAX, col: MAX, depth: MAX, channel: MAX }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.row, self.col, self.depth, self.channel)
    }
}

impl From<point_u8::Point> for Point {
    fn from(p: point_u8::Point) -> Self {
        Point { row: p.row.into(), col: p.col.into(), depth: p.depth.into(), channel: p.channel.into() }
    }
}

impl From<point_u16::Point> for Point {
    fn from(p: point_u16::Point) -> Self {
        Point { row: p.row.into(), col: p.col.into(), depth: p.depth.into(), channel: p.channel.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::matrix::{
        d1::point::point_u32::MAX,
        d4::point::{point_u8, point_u16},
    };

    #[test]
    fn point() {
        assert_eq!(Point::new(1, 2, 3, 4), Point { row: 1, col: 2, depth: 3, channel: 4 });
        assert_eq!(Point::new(4, 3, 2, 1), Point { row: 4, col: 3, depth: 2, channel: 1 });
        assert_eq!(Point::min(), Point { row: 0, col: 0, depth: 0, channel: 0 });
        assert_eq!(Point::max(), Point { row: MAX, col: MAX, depth: MAX, channel: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::new(1, 2, 3, 4).to_string(), "(1, 2, 3, 4)");
        assert_eq!(Point::min().to_string(), "(0, 0, 0, 0)");
        assert_eq!(Point::max().to_string(), "(4294967295, 4294967295, 4294967295, 4294967295)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_u8::Point::min()), Point { row: u8::MIN.into(), col: u8::MIN.into(), depth: u8::MIN.into(), channel: u8::MIN.into() });
        assert_eq!(Point::from(point_u8::Point::max()), Point { row: u8::MAX.into(), col: u8::MAX.into(), depth: u8::MAX.into(), channel: u8::MAX.into() });
        assert_eq!(Point::from(point_u16::Point::min()), Point { row: u16::MIN.into(), col: u16::MIN.into(), depth: u16::MIN.into(), channel: u16::MIN.into() });
        assert_eq!(Point::from(point_u16::Point::max()), Point { row: u16::MAX.into(), col: u16::MAX.into(), depth: u16::MAX.into(), channel: u16::MAX.into() });
    }
}
