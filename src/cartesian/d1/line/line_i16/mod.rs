use crate::cartesian::d1::{line::line_i8, point::point_i16::Point};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn of(min: i16, max: i16) -> Self {
        Line { min: Point::of(min), max: Point::of(max) }
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

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::cartesian::d1::{line::line_i8, point::point_i16::Point};

    #[test]
    fn line() {
        assert_eq!(Line::of(-1, 1), Line { min: Point { x: -1 }, max: Point { x: 1 } });
        assert_eq!(Line::of(-2, 2), Line { min: Point { x: -2 }, max: Point { x: 2 } });
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(Line::zero(), Line { min: Point::zero(), max: Point::zero() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::of(-1, 1).to_string(), "((-1), (1))");
        assert_eq!(Line::largest().to_string(), "((-32768), (32767))");
        assert_eq!(Line::min().to_string(), "((-32768), (-32768))");
        assert_eq!(Line::max().to_string(), "((32767), (32767))");
        assert_eq!(Line::zero().to_string(), "((0), (0))");
    }

    #[test]
    fn from() {
        assert_eq!(Line::from(line_i8::Line::largest()), Line { min: Point { x: i8::MIN.into() }, max: Point { x: i8::MAX.into() } });
    }
}
