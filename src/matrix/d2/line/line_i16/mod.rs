use crate::matrix::d2::{line::line_i8, point::point_i16};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: point_i16::Point,
    pub max: point_i16::Point,
}

impl Line {
    pub fn of(row1: i16, col1: i16, row2: i16, col2: i16) -> Self {
        Line { min: point_i16::Point::of(row1, col1), max: point_i16::Point::of(row2, col2) }
    }

    pub fn largest() -> Self {
        Line { min: point_i16::Point::min(), max: point_i16::Point::max() }
    }

    pub fn min() -> Self {
        Line { min: point_i16::Point::min(), max: point_i16::Point::min() }
    }

    pub fn max() -> Self {
        Line { min: point_i16::Point::max(), max: point_i16::Point::max() }
    }

    pub fn zero() -> Self {
        Line { min: point_i16::Point::zero(), max: point_i16::Point::zero() }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

impl From<line_i8::Line> for Line {
    fn from(l: line_i8::Line) -> Self {
        Line { min: point_i16::Point::from(l.min), max: point_i16::Point::from(l.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::matrix::d2::{line::line_i8, point::point_i16::Point};

    #[test]
    fn line() {
        assert_eq!(Line::of(-2, -1, 1, 2), Line { min: Point { row: -2, col: -1 }, max: Point { row: 1, col: 2 } });
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(Line::zero(), Line { min: Point::zero(), max: Point::zero() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::of(-2, -1, 1, 2).to_string(), "((-2, -1), (1, 2))");
        assert_eq!(Line::largest().to_string(), "((-32768, -32768), (32767, 32767))");
        assert_eq!(Line::min().to_string(), "((-32768, -32768), (-32768, -32768))");
        assert_eq!(Line::max().to_string(), "((32767, 32767), (32767, 32767))");
        assert_eq!(Line::zero().to_string(), "((0, 0), (0, 0))");
    }

    #[test]
    fn from() {
        assert_eq!(Line::from(line_i8::Line::largest()), Line { min: Point { row: i8::MIN.into(), col: i8::MIN.into() }, max: Point { row: i8::MAX.into(), col: i8::MAX.into() } });
    }
}
