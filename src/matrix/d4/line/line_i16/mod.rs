use crate::matrix::d4::{line::line_i8, point::point_i16::Point};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn of(min: (i16, i16, i16, i16), max: (i16, i16, i16, i16)) -> Self {
        Line { min: Point::of(min.0, min.1, min.2, min.3), max: Point::of(max.0, max.1, max.2, max.3) }
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
    use crate::matrix::d4::{line::line_i8, point::point_i16::Point};

    #[test]
    fn line() {
        assert_eq!(Line::of((-4, -3, -2, -1), (1, 2, 3, 4)), Line { min: Point { row: -4, col: -3, depth: -2, channel: -1 }, max: Point { row: 1, col: 2, depth: 3, channel: 4 } });
        assert_eq!(Line::of((-8, -7, -6, -5), (5, 6, 7, 8)), Line { min: Point { row: -8, col: -7, depth: -6, channel: -5 }, max: Point { row: 5, col: 6, depth: 7, channel: 8 } });
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(Line::zero(), Line { min: Point::zero(), max: Point::zero() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::of((-4, -3, -2, -1), (1, 2, 3, 4)).to_string(), "((-4, -3, -2, -1), (1, 2, 3, 4))");
        assert_eq!(Line::largest().to_string(), "((-32768, -32768, -32768, -32768), (32767, 32767, 32767, 32767))");
        assert_eq!(Line::min().to_string(), "((-32768, -32768, -32768, -32768), (-32768, -32768, -32768, -32768))");
        assert_eq!(Line::max().to_string(), "((32767, 32767, 32767, 32767), (32767, 32767, 32767, 32767))");
        assert_eq!(Line::zero().to_string(), "((0, 0, 0, 0), (0, 0, 0, 0))");
    }

    #[test]
    fn from() {
        assert_eq!(
            Line::from(line_i8::Line::largest()),
            Line {
                min: Point { row: i8::MIN.into(), col: i8::MIN.into(), depth: i8::MIN.into(), channel: i8::MIN.into() },
                max: Point { row: i8::MAX.into(), col: i8::MAX.into(), depth: i8::MAX.into(), channel: i8::MAX.into() }
            }
        );
    }
}
