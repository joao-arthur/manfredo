use crate::cartesian::d2::{
    line::{line_i8, line_i16, line_i32},
    point::point_i64,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: point_i64::Point,
    pub max: point_i64::Point,
}

impl Line {
    pub fn of(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
        Line { min: point_i64::Point::of(x1, y1), max: point_i64::Point::of(x2, y2) }
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
    use crate::cartesian::d2::{
        line::{line_i8, line_i16, line_i32},
        point::point_i64::Point,
    };

    const MIN: i64 = i64::MIN;
    const MAX: i64 = i64::MAX;

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point { x: MIN, y: MIN }, max: Point { x: MAX, y: MAX } });
        assert_eq!(Line::min(), Line { min: Point { x: MIN, y: MIN }, max: Point { x: MIN, y: MIN } });
        assert_eq!(Line::max(), Line { min: Point { x: MAX, y: MAX }, max: Point { x: MAX, y: MAX } });
        assert_eq!(Line::of(MIN, -1, 1, MAX), Line { min: Point { x: MIN, y: -1 }, max: Point { x: 1, y: MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((-9223372036854775808, -9223372036854775808), (9223372036854775807, 9223372036854775807))");
        assert_eq!(Line::of(MIN, -0, 0, MAX).to_string(), "((-9223372036854775808, 0), (0, 9223372036854775807))");
    }

    #[test]
    fn from() {
        assert_eq!(Line::from(line_i8::Line::largest()), Line { min: Point { x: i8::MIN.into(), y: i8::MIN.into() }, max: Point { x: i8::MAX.into(), y: i8::MAX.into() } });
        assert_eq!(Line::from(line_i16::Line::largest()), Line { min: Point { x: i16::MIN.into(), y: i16::MIN.into() }, max: Point { x: i16::MAX.into(), y: i16::MAX.into() } });
        assert_eq!(Line::from(line_i32::Line::largest()), Line { min: Point { x: i32::MIN.into(), y: i32::MIN.into() }, max: Point { x: i32::MAX.into(), y: i32::MAX.into() } });
    }
}
