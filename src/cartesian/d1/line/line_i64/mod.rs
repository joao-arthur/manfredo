use crate::cartesian::d1::{
    line::{line_i8, line_i16, line_i32},
    point::point_i64::Point,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn of(x1: i64, x2: i64) -> Self {
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

impl From<line_i32::Line> for Line {
    fn from(l: line_i32::Line) -> Self {
        Line { min: Point::from(l.min), max: Point::from(l.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::cartesian::d1::{
        line::{line_i8, line_i16, line_i32},
        point::point_i64::Point,
    };

    const MIN: i64 = i64::MIN;
    const MAX: i64 = i64::MAX;

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point { x: MIN }, max: Point { x: MAX } });
        assert_eq!(Line::min(), Line { min: Point { x: MIN }, max: Point { x: MIN } });
        assert_eq!(Line::max(), Line { min: Point { x: MAX }, max: Point { x: MAX } });
        assert_eq!(Line::of(MIN, 1), Line { min: Point { x: MIN }, max: Point { x: 1 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((-9223372036854775808), (9223372036854775807))");
        assert_eq!(Line::of(MIN, 0).to_string(), "((-9223372036854775808), (0))");
    }

    #[test]
    fn from() {
        assert_eq!(Line::from(line_i8::Line::largest()), Line { min: Point { x: i8::MIN.into() }, max: Point { x: i8::MAX.into() } });
        assert_eq!(Line::from(line_i16::Line::largest()), Line { min: Point { x: i16::MIN.into() }, max: Point { x: i16::MAX.into() } });
        assert_eq!(Line::from(line_i32::Line::largest()), Line { min: Point { x: i32::MIN.into() }, max: Point { x: i32::MAX.into() } });
    }
}
