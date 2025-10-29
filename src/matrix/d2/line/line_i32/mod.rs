use crate::matrix::d2::{
    line::{line_i8, line_i16},
    point::point_i32,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: point_i32::Point,
    pub max: point_i32::Point,
}

impl Line {
    pub fn of(row1: i32, col1: i32, row2: i32, col2: i32) -> Self {
        Line { min: point_i32::Point::of(row1, col1), max: point_i32::Point::of(row2, col2) }
    }

    pub fn largest() -> Self {
        Line { min: point_i32::Point::min(), max: point_i32::Point::max() }
    }

    pub fn min() -> Self {
        Line { min: point_i32::Point::min(), max: point_i32::Point::min() }
    }

    pub fn max() -> Self {
        Line { min: point_i32::Point::max(), max: point_i32::Point::max() }
    }

    pub fn zero() -> Self {
        Line { min: point_i32::Point::zero(), max: point_i32::Point::zero() }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

impl From<line_i8::Line> for Line {
    fn from(l: line_i8::Line) -> Self {
        Line { min: point_i32::Point::from(l.min), max: point_i32::Point::from(l.max) }
    }
}

impl From<line_i16::Line> for Line {
    fn from(l: line_i16::Line) -> Self {
        Line { min: point_i32::Point::from(l.min), max: point_i32::Point::from(l.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::matrix::d2::{
        line::{line_i8, line_i16},
        point::point_i32::Point,
    };

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
        assert_eq!(Line::largest().to_string(), "((-2147483648, -2147483648), (2147483647, 2147483647))");
        assert_eq!(Line::min().to_string(), "((-2147483648, -2147483648), (-2147483648, -2147483648))");
        assert_eq!(Line::max().to_string(), "((2147483647, 2147483647), (2147483647, 2147483647))");
        assert_eq!(Line::zero().to_string(), "((0, 0), (0, 0))");
    }

    #[test]
    fn from() {
        assert_eq!(Line::from(line_i8::Line::largest()), Line { min: Point { row: i8::MIN.into(), col: i8::MIN.into() }, max: Point { row: i8::MAX.into(), col: i8::MAX.into() } });
        assert_eq!(Line::from(line_i16::Line::largest()), Line { min: Point { row: i16::MIN.into(), col: i16::MIN.into() }, max: Point { row: i16::MAX.into(), col: i16::MAX.into() } });
    }
}
