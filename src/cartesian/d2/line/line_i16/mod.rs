use crate::cartesian::d2::{line::line_i8, point::point_i16};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: point_i16::Point,
    pub max: point_i16::Point,
}

impl Line {
    pub fn of(x1: i16, y1: i16, x2: i16, y2: i16) -> Self {
        Line { min: point_i16::Point::of(x1, y1), max: point_i16::Point::of(x2, y2) }
    }

    pub fn largest() -> Self {
        Line { min: point_i16::Point::min(), max: point_i16::Point::max() }
    }

    pub fn min() -> Self {
        Line { min: point_i16::Point::min(), max: point_i16::Point::min() }
    }

    pub fn max() -> Self {
        Line { min: point_i16::Point::max(), max: point_i16::Point::max() }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

impl From<line_i8::Line> for Line {
    fn from(r: line_i8::Line) -> Self {
        Line { min: point_i16::Point::from(r.min), max: point_i16::Point::from(r.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::cartesian::d2::{line::line_i8, point::point_i16::Point};

    const MIN: i16 = i16::MIN;
    const MAX: i16 = i16::MAX;

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point { x: MIN, y: MIN }, max: Point { x: MAX, y: MAX } });
        assert_eq!(Line::min(), Line { min: Point { x: MIN, y: MIN }, max: Point { x: MIN, y: MIN } });
        assert_eq!(Line::max(), Line { min: Point { x: MAX, y: MAX }, max: Point { x: MAX, y: MAX } });
        assert_eq!(Line::of(MIN, -1, 1, MAX), Line { min: Point { x: MIN, y: -1 }, max: Point { x: 1, y: MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((-32768, -32768), (32767, 32767))");
        assert_eq!(Line::of(MIN, -0, 0, MAX).to_string(), "((-32768, 0), (0, 32767))");
    }

    #[test]
    fn from() {
        assert_eq!(Line::from(line_i8::Line::largest()), Line { min: Point { x: i8::MIN.into(), y: i8::MIN.into() }, max: Point { x: i8::MAX.into(), y: i8::MAX.into() } });
    }
}
