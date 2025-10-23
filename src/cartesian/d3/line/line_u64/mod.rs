use crate::cartesian::d3::{
    line::{line_u8, line_u16, line_u32},
    point::point_u64::Point,
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

impl From<line_u32::Line> for Line {
    fn from(l: line_u32::Line) -> Self {
        Line { min: Point::from(l.min), max: Point::from(l.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::cartesian::{
        d1::point::point_u64::MAX,
        d3::{
            line::{line_u8, line_u16, line_u32},
            point::point_u64::Point,
        },
    };

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(Line::of(Point::of(0, 1, 2), Point::of(MAX - 2, MAX - 1, MAX)), Line { min: Point { x: 0, y: 1, z: 2 }, max: Point { x: MAX - 2, y: MAX - 1, z: MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((0, 0, 0), (18446744073709551615, 18446744073709551615, 18446744073709551615))");
        assert_eq!(Line::min().to_string(), "((0, 0, 0), (0, 0, 0))");
        assert_eq!(Line::max().to_string(), "((18446744073709551615, 18446744073709551615, 18446744073709551615), (18446744073709551615, 18446744073709551615, 18446744073709551615))");
        assert_eq!(Line::of(Point::of(0, 1, 2), Point::of(MAX - 2, MAX - 1, MAX)).to_string(), "((0, 1, 2), (18446744073709551613, 18446744073709551614, 18446744073709551615))");
    }

    #[test]
    fn from() {
        assert_eq!(Line::from(line_u8::Line::largest()), Line { min: Point::min(), max: Point { x: u8::MAX.into(), y: u8::MAX.into(), z: u8::MAX.into() } });
        assert_eq!(Line::from(line_u16::Line::largest()), Line { min: Point::min(), max: Point { x: u16::MAX.into(), y: u16::MAX.into(), z: u16::MAX.into() } });
        assert_eq!(Line::from(line_u32::Line::largest()), Line { min: Point::min(), max: Point { x: u32::MAX.into(), y: u32::MAX.into(), z: u32::MAX.into() } });
    }
}
