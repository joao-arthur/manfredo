use crate::matrix::d1::{
    line::{line_i8, line_i16},
    point::point_i32::Point,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn new(min: i32, max: i32) -> Self {
        Line { min: Point::new(min), max: Point::new(max) }
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
    use crate::matrix::d1::{
        line::{line_i8, line_i16},
        point::point_i32::Point,
    };

    #[test]
    fn line() {
        assert_eq!(Line::new(-1, 1), Line { min: Point { i: -1 }, max: Point { i: 1 } });
        assert_eq!(Line::new(-2, 2), Line { min: Point { i: -2 }, max: Point { i: 2 } });
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(Line::zero(), Line { min: Point::zero(), max: Point::zero() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::new(-1, 1).to_string(), "((-1), (1))");
        assert_eq!(Line::largest().to_string(), "((-2147483648), (2147483647))");
        assert_eq!(Line::min().to_string(), "((-2147483648), (-2147483648))");
        assert_eq!(Line::max().to_string(), "((2147483647), (2147483647))");
        assert_eq!(Line::zero().to_string(), "((0), (0))");
    }

    #[test]
    fn from() {
        assert_eq!(Line::from(line_i8::Line::largest()), Line { min: Point { i: i8::MIN.into() }, max: Point { i: i8::MAX.into() } });
        assert_eq!(Line::from(line_i16::Line::largest()), Line { min: Point { i: i16::MIN.into() }, max: Point { i: i16::MAX.into() } });
    }
}
