use crate::cartesian::d4::{line::line_i8, point::point_i16::Point};

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

impl From<line_i8::Line> for Line {
    fn from(l: line_i8::Line) -> Self {
        Line { min: Point::from(l.min), max: Point::from(l.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::cartesian::d4::{line::line_i8, point::point_i16::Point};

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(Line::of(Point::of(-4, -3, -2, -1), Point::of(1, 2, 3, 4)), Line { min: Point { x: -4, y: -3, z: -2, w: -1 }, max: Point { x: 1, y: 2, z: 3, w: 4 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((-32768, -32768, -32768, -32768), (32767, 32767, 32767, 32767))");
        assert_eq!(Line::min().to_string(), "((-32768, -32768, -32768, -32768), (-32768, -32768, -32768, -32768))");
        assert_eq!(Line::max().to_string(), "((32767, 32767, 32767, 32767), (32767, 32767, 32767, 32767))");
        assert_eq!(Line::of(Point::of(-4, -3, -2, -1), Point::of(1, 2, 3, 4)).to_string(), "((-4, -3, -2, -1), (1, 2, 3, 4))");
    }

    #[test]
    fn from() {
        assert_eq!(
            Line::from(line_i8::Line::largest()),
            Line { min: Point { x: i8::MIN.into(), y: i8::MIN.into(), z: i8::MIN.into(), w: i8::MIN.into() }, max: Point { x: i8::MAX.into(), y: i8::MAX.into(), z: i8::MAX.into(), w: i8::MAX.into() } }
        );
    }
}
