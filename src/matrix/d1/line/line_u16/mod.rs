use crate::matrix::d1::{line::line_u8, point::point_u16::Point};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn of(i1: u16, i2: u16) -> Self {
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

impl From<line_u8::Line> for Line {
    fn from(l: line_u8::Line) -> Self {
        Line { min: Point::from(l.min), max: Point::from(l.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::matrix::d1::{line::line_u8, point::point_u16::Point};

    #[test]
    fn line() {
        assert_eq!(Line::of(0, 1), Line { min: Point { i: 0 }, max: Point { i: 1 } });
        assert_eq!(Line::of(2, 3), Line { min: Point { i: 2 }, max: Point { i: 3 } });
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::of(0, 1).to_string(), "((0), (1))");
        assert_eq!(Line::largest().to_string(), "((0), (65535))");
        assert_eq!(Line::min().to_string(), "((0), (0))");
        assert_eq!(Line::max().to_string(), "((65535), (65535))");
    }

    #[test]
    fn from() {
        assert_eq!(Line::from(line_u8::Line::largest()), Line { min: Point::min(), max: Point { i: u8::MAX.into() } });
    }
}
