use crate::cartesian::d4::{
    line::{line_u8, line_u16},
    point::point_u32::Point,
};

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

impl From<line_u16::Line> for Line {
    fn from(l: line_u16::Line) -> Self {
        Line { min: Point::from(l.min), max: Point::from(l.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::cartesian::d4::{
        line::{line_u8, line_u16},
        point::point_u32::Point,
    };

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(Line::of(Point::of(0, 1, 2, 3), Point::of(4, 5, 6, 7)), Line { min: Point { x: 0, y: 1, z: 2, w: 3 }, max: Point { x: 4, y: 5, z: 6, w: 7 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((0, 0, 0, 0), (4294967295, 4294967295, 4294967295, 4294967295))");
        assert_eq!(Line::min().to_string(), "((0, 0, 0, 0), (0, 0, 0, 0))");
        assert_eq!(Line::max().to_string(), "((4294967295, 4294967295, 4294967295, 4294967295), (4294967295, 4294967295, 4294967295, 4294967295))");
        assert_eq!(Line::of(Point::of(0, 1, 2, 3), Point::of(4, 5, 6, 7)).to_string(), "((0, 1, 2, 3), (4, 5, 6, 7))");
    }

    #[test]
    fn from() {
        assert_eq!(Line::from(line_u8::Line::largest()), Line { min: Point::min(), max: Point { x: u8::MAX.into(), y: u8::MAX.into(), z: u8::MAX.into(), w: u8::MAX.into() } });
        assert_eq!(Line::from(line_u16::Line::largest()), Line { min: Point::min(), max: Point { x: u16::MAX.into(), y: u16::MAX.into(), z: u16::MAX.into(), w: u16::MAX.into() } });
    }
}
