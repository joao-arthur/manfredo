use crate::matrix::d2::{
    line::{line_u8, line_u16, line_u32},
    point::point_u64,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: point_u64::Point,
    pub max: point_u64::Point,
}

impl Line {
    pub fn of(row1: u64, col1: u64, row2: u64, col2: u64) -> Self {
        Line { min: point_u64::Point::of(row1, col1), max: point_u64::Point::of(row2, col2) }
    }

    pub fn largest() -> Self {
        Line { min: point_u64::Point::min(), max: point_u64::Point::max() }
    }

    pub fn min() -> Self {
        Line { min: point_u64::Point::min(), max: point_u64::Point::min() }
    }

    pub fn max() -> Self {
        Line { min: point_u64::Point::max(), max: point_u64::Point::max() }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

impl From<line_u8::Line> for Line {
    fn from(l: line_u8::Line) -> Self {
        Line { min: point_u64::Point::from(l.min), max: point_u64::Point::from(l.max) }
    }
}

impl From<line_u16::Line> for Line {
    fn from(l: line_u16::Line) -> Self {
        Line { min: point_u64::Point::from(l.min), max: point_u64::Point::from(l.max) }
    }
}

impl From<line_u32::Line> for Line {
    fn from(l: line_u32::Line) -> Self {
        Line { min: point_u64::Point::from(l.min), max: point_u64::Point::from(l.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::matrix::{
        d1::point::point_u64::MAX,
        d2::{
            line::{line_u8, line_u16, line_u32},
            point::point_u64::Point,
        },
    };

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point { row: 0, col: 0 }, max: Point { row: MAX, col: MAX } });
        assert_eq!(Line::min(), Line { min: Point { row: 0, col: 0 }, max: Point { row: 0, col: 0 } });
        assert_eq!(Line::max(), Line { min: Point { row: MAX, col: MAX }, max: Point { row: MAX, col: MAX } });
        assert_eq!(Line::of(4096, 8192, 16384, 32768), Line { min: Point { row: 4096, col: 8192 }, max: Point { row: 16384, col: 32768 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((0, 0), (18446744073709551615, 18446744073709551615))");
        assert_eq!(Line::of(4096, 8192, 16384, 32768).to_string(), "((4096, 8192), (16384, 32768))");
    }

    #[test]
    fn from() {
        assert_eq!(Line::from(line_u8::Line::largest()), Line { min: Point { row: 0, col: 0 }, max: Point { row: u8::MAX.into(), col: u8::MAX.into() } });
        assert_eq!(Line::from(line_u16::Line::largest()), Line { min: Point { row: 0, col: 0 }, max: Point { row: u16::MAX.into(), col: u16::MAX.into() } });
        assert_eq!(Line::from(line_u32::Line::largest()), Line { min: Point { row: 0, col: 0 }, max: Point { row: u32::MAX.into(), col: u32::MAX.into() } });
    }
}
