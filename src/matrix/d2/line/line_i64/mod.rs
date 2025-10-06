use crate::matrix::d2::{
    line::{line_i8, line_i16, line_i32},
    point::point_i64,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: point_i64::Point,
    pub max: point_i64::Point,
}

impl Line {
    pub fn of(row1: i64, col1: i64, row2: i64, col2: i64) -> Self {
        Line { min: point_i64::Point::of(row1, col1), max: point_i64::Point::of(row2, col2) }
    }

    pub fn largest() -> Self {
        Line { min: point_i64::Point::min(), max: point_i64::Point::max() }
    }

    pub fn min() -> Self {
        Line { min: point_i64::Point::min(), max: point_i64::Point::min() }
    }

    pub fn max() -> Self {
        Line { min: point_i64::Point::max(), max: point_i64::Point::max() }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

impl From<line_i8::Line> for Line {
    fn from(r: line_i8::Line) -> Self {
        Line { min: point_i64::Point::from(r.min), max: point_i64::Point::from(r.max) }
    }
}

impl From<line_i16::Line> for Line {
    fn from(r: line_i16::Line) -> Self {
        Line { min: point_i64::Point::from(r.min), max: point_i64::Point::from(r.max) }
    }
}

impl From<line_i32::Line> for Line {
    fn from(r: line_i32::Line) -> Self {
        Line { min: point_i64::Point::from(r.min), max: point_i64::Point::from(r.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::matrix::d2::{
        line::{line_i8, line_i16, line_i32},
        point::point_i64::Point,
    };

    const MIN: i64 = i64::MIN;
    const MAX: i64 = i64::MAX;

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point { row: MIN, col: MIN }, max: Point { row: MAX, col: MAX } });
        assert_eq!(Line::min(), Line { min: Point { row: MIN, col: MIN }, max: Point { row: MIN, col: MIN } });
        assert_eq!(Line::max(), Line { min: Point { row: MAX, col: MAX }, max: Point { row: MAX, col: MAX } });
        assert_eq!(Line::of(MIN, -1, 1, MAX), Line { min: Point { row: MIN, col: -1 }, max: Point { row: 1, col: MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((-9223372036854775808, -9223372036854775808), (9223372036854775807, 9223372036854775807))");
        assert_eq!(Line::of(MIN, -0, 0, MAX).to_string(), "((-9223372036854775808, 0), (0, 9223372036854775807))");
    }

    #[test]
    fn from() {
        assert_eq!(Line::from(line_i8::Line::largest()), Line { min: Point { row: i8::MIN.into(), col: i8::MIN.into() }, max: Point { row: i8::MAX.into(), col: i8::MAX.into() } });
        assert_eq!(Line::from(line_i16::Line::largest()), Line { min: Point { row: i16::MIN.into(), col: i16::MIN.into() }, max: Point { row: i16::MAX.into(), col: i16::MAX.into() } });
        assert_eq!(Line::from(line_i32::Line::largest()), Line { min: Point { row: i32::MIN.into(), col: i32::MIN.into() }, max: Point { row: i32::MAX.into(), col: i32::MAX.into() } });
    }
}
