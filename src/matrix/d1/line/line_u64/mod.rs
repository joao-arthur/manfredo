use crate::matrix::d1::{
    line::{line_u8, line_u16, line_u32},
    point::point_u64::Point,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn of(i1: u64, i2: u64) -> Self {
        Line { min: Point::of(i1), max: Point::of(i2) }
    }

    pub fn largest() -> Self {
        Line { min: Point::min(), max: Point::max() }
    }

    pub fn min() -> Self {
        Line { min: Point::min(), max: Point::min() }
    }

    pub fn max() -> Self {
        Line { min: Point::max(), max: Point::max() }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

impl From<line_u8::Line> for Line {
    fn from(l: line_u8::Line) -> Self {
        Line { min: Point::from(l.min), max: Point::from(l.max) }
    }
}

impl From<line_u16::Line> for Line {
    fn from(l: line_u16::Line) -> Self {
        Line { min: Point::from(l.min), max: Point::from(l.max) }
    }
}

impl From<line_u32::Line> for Line {
    fn from(l: line_u32::Line) -> Self {
        Line { min: Point::from(l.min), max: Point::from(l.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::matrix::d1::{
        line::{line_u8, line_u16, line_u32},
        point::point_u64::{MAX, Point},
    };

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point { i: 0 }, max: Point { i: MAX } });
        assert_eq!(Line::min(), Line { min: Point { i: 0 }, max: Point { i: 0 } });
        assert_eq!(Line::max(), Line { min: Point { i: MAX }, max: Point { i: MAX } });
        assert_eq!(Line::of(4096, 16384), Line { min: Point { i: 4096 }, max: Point { i: 16384 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((0), (18446744073709551615))");
        assert_eq!(Line::of(4096, 16384).to_string(), "((4096), (16384))");
    }

    #[test]
    fn from() {
        assert_eq!(Line::from(line_u8::Line::largest()), Line { min: Point { i: 0 }, max: Point { i: u8::MAX.into() } });
        assert_eq!(Line::from(line_u16::Line::largest()), Line { min: Point { i: 0 }, max: Point { i: u16::MAX.into() } });
        assert_eq!(Line::from(line_u32::Line::largest()), Line { min: Point { i: 0 }, max: Point { i: u32::MAX.into() } });
    }
}
