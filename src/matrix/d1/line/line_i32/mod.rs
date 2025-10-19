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
    pub fn of(i1: i32, i2: i32) -> Self {
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
        point::point_i32::{MAX, MIN, Point},
    };

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point { i: MIN }, max: Point { i: MAX } });
        assert_eq!(Line::min(), Line { min: Point { i: MIN }, max: Point { i: MIN } });
        assert_eq!(Line::max(), Line { min: Point { i: MAX }, max: Point { i: MAX } });
        assert_eq!(Line::of(MIN, 1), Line { min: Point { i: MIN }, max: Point { i: 1 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((-2147483648), (2147483647))");
        assert_eq!(Line::of(MIN, 0).to_string(), "((-2147483648), (0))");
    }

    #[test]
    fn from() {
        assert_eq!(Line::from(line_i8::Line::largest()), Line { min: Point { i: i8::MIN.into() }, max: Point { i: i8::MAX.into() } });
        assert_eq!(Line::from(line_i16::Line::largest()), Line { min: Point { i: i16::MIN.into() }, max: Point { i: i16::MAX.into() } });
    }
}
