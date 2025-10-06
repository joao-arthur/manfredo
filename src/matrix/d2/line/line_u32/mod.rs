use crate::matrix::d2::{
    line::{line_u8, line_u16},
    point::point_u32,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: point_u32::Point,
    pub max: point_u32::Point,
}

impl Line {
    pub fn of(row1: u32, col1: u32, row2: u32, col2: u32) -> Self {
        Line { min: point_u32::Point::of(row1, col1), max: point_u32::Point::of(row2, col2) }
    }

    pub fn largest() -> Self {
        Line { min: point_u32::Point::min(), max: point_u32::Point::max() }
    }

    pub fn min() -> Self {
        Line { min: point_u32::Point::min(), max: point_u32::Point::min() }
    }

    pub fn max() -> Self {
        Line { min: point_u32::Point::max(), max: point_u32::Point::max() }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

impl From<line_u8::Line> for Line {
    fn from(r: line_u8::Line) -> Self {
        Line { min: point_u32::Point::from(r.min), max: point_u32::Point::from(r.max) }
    }
}

impl From<line_u16::Line> for Line {
    fn from(r: line_u16::Line) -> Self {
        Line { min: point_u32::Point::from(r.min), max: point_u32::Point::from(r.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::matrix::d2::{
        line::{line_u8, line_u16},
        point::point_u32::Point,
    };

    const MAX: u32 = u32::MAX;

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point { row: 0, col: 0 }, max: Point { row: MAX, col: MAX } });
        assert_eq!(Line::min(), Line { min: Point { row: 0, col: 0 }, max: Point { row: 0, col: 0 } });
        assert_eq!(Line::max(), Line { min: Point { row: MAX, col: MAX }, max: Point { row: MAX, col: MAX } });
        assert_eq!(Line::of(256, 512, 1024, 2048), Line { min: Point { row: 256, col: 512 }, max: Point { row: 1024, col: 2048 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((0, 0), (4294967295, 4294967295))");
        assert_eq!(Line::of(256, 512, 1024, 2048).to_string(), "((256, 512), (1024, 2048))");
    }

    #[test]
    fn from() {
        assert_eq!(Line::from(line_u8::Line::largest()), Line { min: Point { row: 0, col: 0 }, max: Point { row: u8::MAX.into(), col: u8::MAX.into() } });
        assert_eq!(Line::from(line_u16::Line::largest()), Line { min: Point { row: 0, col: 0 }, max: Point { row: u16::MAX.into(), col: u16::MAX.into() } });
    }
}
