use crate::matrix::d2::{
    line::{line_i8, line_i16},
    point::point_i32::Point,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn of(min: (i32, i32), max: (i32, i32)) -> Self {
        Line { min: Point::of(min.0, min.1), max: Point::of(max.0, max.1) }
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

    pub fn zero() -> Self {
        Line { min: Point::zero(), max: Point::zero() }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

impl From<line_i8::Line> for Line {
    fn from(l: line_i8::Line) -> Self {
        Line { min: Point::from(l.min), max: Point::from(l.max) }
    }
}

impl From<line_i16::Line> for Line {
    fn from(l: line_i16::Line) -> Self {
        Line { min: Point::from(l.min), max: Point::from(l.max) }
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
        assert_eq!(Line::of((-2, -1), (1, 2)), Line { min: Point { row: -2, col: -1 }, max: Point { row: 1, col: 2 } });
        assert_eq!(Line::of((-4, -3), (3, 4)), Line { min: Point { row: -4, col: -3 }, max: Point { row: 3, col: 4 } });
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(Line::zero(), Line { min: Point::zero(), max: Point::zero() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::of((-2, -1), (1, 2)).to_string(), "((-2, -1), (1, 2))");
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
