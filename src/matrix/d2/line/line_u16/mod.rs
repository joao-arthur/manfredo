use crate::matrix::d2::{line::line_u8, point::point_u16};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: point_u16::Point,
    pub max: point_u16::Point,
}

impl Line {
    pub fn of(row1: u16, col1: u16, row2: u16, col2: u16) -> Self {
        Line { min: point_u16::Point::of(row1, col1), max: point_u16::Point::of(row2, col2) }
    }

    pub fn largest() -> Self {
        Line { min: point_u16::Point::min(), max: point_u16::Point::max() }
    }

    pub fn min() -> Self {
        Line { min: point_u16::Point::min(), max: point_u16::Point::min() }
    }

    pub fn max() -> Self {
        Line { min: point_u16::Point::max(), max: point_u16::Point::max() }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

impl From<line_u8::Line> for Line {
    fn from(l: line_u8::Line) -> Self {
        Line { min: point_u16::Point::from(l.min), max: point_u16::Point::from(l.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::matrix::d2::{line::line_u8, point::point_u16::Point};

    const MAX: u16 = u16::MAX;

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point { row: 0, col: 0 }, max: Point { row: MAX, col: MAX } });
        assert_eq!(Line::min(), Line { min: Point { row: 0, col: 0 }, max: Point { row: 0, col: 0 } });
        assert_eq!(Line::max(), Line { min: Point { row: MAX, col: MAX }, max: Point { row: MAX, col: MAX } });
        assert_eq!(Line::of(16, 32, 16, 32), Line { min: Point { row: 16, col: 32 }, max: Point { row: 16, col: 32 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((0, 0), (65535, 65535))");
        assert_eq!(Line::of(16, 32, 16, 32).to_string(), "((16, 32), (16, 32))");
    }

    #[test]
    fn from() {
        assert_eq!(Line::from(line_u8::Line::largest()), Line { min: Point { row: 0, col: 0 }, max: Point { row: u8::MAX.into(), col: u8::MAX.into() } });
    }
}
