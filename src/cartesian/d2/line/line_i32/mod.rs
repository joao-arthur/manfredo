use crate::cartesian::d2::{
    line::{line_i8, line_i16},
    point::point_i32::Point,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn of(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Line { min: Point::of(x1, y1), max: Point::of(x2, y2) }
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
    use crate::cartesian::d2::{
        line::{line_i8, line_i16},
        point::point_i32::Point,
    };

    #[test]
    fn line() {
        assert_eq!(Line::of(-2, -1, 1, 2), Line { min: Point { x: -2, y: -1 }, max: Point { x: 1, y: 2 } });
        assert_eq!(Line::of(-4, -2, 2, 4), Line { min: Point { x: -4, y: -2 }, max: Point { x: 2, y: 4 } });
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
        assert_eq!(Line::from(line_i8::Line::largest()), Line { min: Point { x: i8::MIN.into(), y: i8::MIN.into() }, max: Point { x: i8::MAX.into(), y: i8::MAX.into() } });
        assert_eq!(Line::from(line_i16::Line::largest()), Line { min: Point { x: i16::MIN.into(), y: i16::MIN.into() }, max: Point { x: i16::MAX.into(), y: i16::MAX.into() } });
    }
}
