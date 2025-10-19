use crate::matrix::d2::point::point_u8;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: point_u8::Point,
    pub max: point_u8::Point,
}

impl Line {
    pub fn of(row1: u8, col1: u8, row2: u8, col2: u8) -> Self {
        Line { min: point_u8::Point::of(row1, col1), max: point_u8::Point::of(row2, col2) }
    }

    pub fn largest() -> Self {
        Line { min: point_u8::Point::min(), max: point_u8::Point::max() }
    }

    pub fn min() -> Self {
        Line { min: point_u8::Point::min(), max: point_u8::Point::min() }
    }

    pub fn max() -> Self {
        Line { min: point_u8::Point::max(), max: point_u8::Point::max() }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::matrix::{d1::point::point_u8::MAX, d2::point::point_u8::Point};

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point { row: 0, col: 0 }, max: Point { row: MAX, col: MAX } });
        assert_eq!(Line::min(), Line { min: Point { row: 0, col: 0 }, max: Point { row: 0, col: 0 } });
        assert_eq!(Line::max(), Line { min: Point { row: MAX, col: MAX }, max: Point { row: MAX, col: MAX } });
        assert_eq!(Line::of(0, 2, 4, 8), Line { min: Point { row: 0, col: 2 }, max: Point { row: 4, col: 8 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((0, 0), (255, 255))");
        assert_eq!(Line::of(0, 2, 4, 8).to_string(), "((0, 2), (4, 8))");
    }
}
