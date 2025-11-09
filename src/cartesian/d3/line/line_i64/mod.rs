use crate::cartesian::d3::{
    line::{line_i8, line_i16, line_i32},
    point::point_i64::Point,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn new(min: (i64, i64, i64), max: (i64, i64, i64)) -> Self {
        Line { min: Point::new(min.0, min.1, min.2), max: Point::new(max.0, max.1, max.2) }
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

impl From<line_i32::Line> for Line {
    fn from(l: line_i32::Line) -> Self {
        Line { min: Point::from(l.min), max: Point::from(l.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::cartesian::d3::{
        line::{line_i8, line_i16, line_i32},
        point::point_i64::Point,
    };

    #[test]
    fn line() {
        assert_eq!(Line::new((-3, -2, -1), (1, 2, 3)), Line { min: Point { x: -3, y: -2, z: -1 }, max: Point { x: 1, y: 2, z: 3 } });
        assert_eq!(Line::new((-6, -5, -4), (4, 5, 6)), Line { min: Point { x: -6, y: -5, z: -4 }, max: Point { x: 4, y: 5, z: 6 } });
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(Line::zero(), Line { min: Point::zero(), max: Point::zero() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::new((-3, -2, -1), (1, 2, 3)).to_string(), "((-3, -2, -1), (1, 2, 3))");
        assert_eq!(Line::largest().to_string(), "((-9223372036854775808, -9223372036854775808, -9223372036854775808), (9223372036854775807, 9223372036854775807, 9223372036854775807))");
        assert_eq!(Line::min().to_string(), "((-9223372036854775808, -9223372036854775808, -9223372036854775808), (-9223372036854775808, -9223372036854775808, -9223372036854775808))");
        assert_eq!(Line::max().to_string(), "((9223372036854775807, 9223372036854775807, 9223372036854775807), (9223372036854775807, 9223372036854775807, 9223372036854775807))");
        assert_eq!(Line::zero().to_string(), "((0, 0, 0), (0, 0, 0))");
    }

    #[test]
    fn from() {
        assert_eq!(
            Line::from(line_i8::Line::largest()),
            Line { min: Point { x: i8::MIN.into(), y: i8::MIN.into(), z: i8::MIN.into() }, max: Point { x: i8::MAX.into(), y: i8::MAX.into(), z: i8::MAX.into() } }
        );
        assert_eq!(
            Line::from(line_i16::Line::largest()),
            Line { min: Point { x: i16::MIN.into(), y: i16::MIN.into(), z: i16::MIN.into() }, max: Point { x: i16::MAX.into(), y: i16::MAX.into(), z: i16::MAX.into() } }
        );
        assert_eq!(
            Line::from(line_i32::Line::largest()),
            Line { min: Point { x: i32::MIN.into(), y: i32::MIN.into(), z: i32::MIN.into() }, max: Point { x: i32::MAX.into(), y: i32::MAX.into(), z: i32::MAX.into() } }
        );
    }
}
