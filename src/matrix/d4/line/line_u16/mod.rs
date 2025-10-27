use crate::matrix::d4::{line::line_u8, point::point_u16::Point};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn of(min: Point, max: Point) -> Self {
        Line { min, max }
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
    use crate::matrix::d4::{line::line_u8, point::point_u16::Point};

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(Line::of(Point::of(0, 1, 2, 3), Point::of(4, 5, 6, 7)), Line { min: Point { row: 0, col: 1, depth: 2, channel: 3 }, max: Point { row: 4, col: 5, depth: 6, channel: 7 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((0, 0, 0, 0), (65535, 65535, 65535, 65535))");
        assert_eq!(Line::min().to_string(), "((0, 0, 0, 0), (0, 0, 0, 0))");
        assert_eq!(Line::max().to_string(), "((65535, 65535, 65535, 65535), (65535, 65535, 65535, 65535))");
        assert_eq!(Line::of(Point::of(0, 1, 2, 3), Point::of(4, 5, 6, 7)).to_string(), "((0, 1, 2, 3), (4, 5, 6, 7))");
    }

    #[test]
    fn from() {
        assert_eq!(Line::from(line_u8::Line::largest()), Line { min: Point::min(), max: Point { row: u8::MAX.into(), col: u8::MAX.into(), depth: u8::MAX.into(), channel: u8::MAX.into() } });
    }
}
