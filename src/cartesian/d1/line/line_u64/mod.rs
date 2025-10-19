use crate::cartesian::d1::{
    line::{line_u8, line_u16, line_u32},
    point::point_u64::Point,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn of(x1: u64, x2: u64) -> Self {
        Line { min: Point::of(x1), max: Point::of(x2) }
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
    use crate::cartesian::d1::{
        line::{line_u8, line_u16, line_u32},
        point::point_u64::{MAX, Point},
    };

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point { x: 0 }, max: Point { x: MAX } });
        assert_eq!(Line::min(), Line { min: Point { x: 0 }, max: Point { x: 0 } });
        assert_eq!(Line::max(), Line { min: Point { x: MAX }, max: Point { x: MAX } });
        assert_eq!(Line::of(4096, 16384), Line { min: Point { x: 4096 }, max: Point { x: 16384 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((0), (18446744073709551615))");
        assert_eq!(Line::of(4096, 16384).to_string(), "((4096), (16384))");
    }

    #[test]
    fn from() {
        assert_eq!(Line::from(line_u8::Line::largest()), Line { min: Point { x: 0 }, max: Point { x: u8::MAX.into() } });
        assert_eq!(Line::from(line_u16::Line::largest()), Line { min: Point { x: 0 }, max: Point { x: u16::MAX.into() } });
        assert_eq!(Line::from(line_u32::Line::largest()), Line { min: Point { x: 0 }, max: Point { x: u32::MAX.into() } });
    }
}
