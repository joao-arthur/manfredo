use crate::matrix::d2::point::point_i8;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: point_i8::Point,
    pub max: point_i8::Point,
}

impl Line {
    pub fn of(row1: i8, col1: i8, row2: i8, col2: i8) -> Self {
        Line { min: point_i8::Point::of(row1, col1), max: point_i8::Point::of(row2, col2) }
    }

    pub fn largest() -> Self {
        Line { min: point_i8::Point::min(), max: point_i8::Point::max() }
    }

    pub fn min() -> Self {
        Line { min: point_i8::Point::min(), max: point_i8::Point::min() }
    }

    pub fn max() -> Self {
        Line { min: point_i8::Point::max(), max: point_i8::Point::max() }
    }

    pub fn zero() -> Self {
        Line { min: point_i8::Point::zero(), max: point_i8::Point::zero() }
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
    use crate::matrix::d2::point::point_i8::Point;

    #[test]
    fn line() {
        assert_eq!(Line::of(-2, -1, 1, 2), Line { min: Point { row: -2, col: -1 }, max: Point { row: 1, col: 2 } });
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(Line::zero(), Line { min: Point::zero(), max: Point::zero() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::of(-2, -1, 1, 2).to_string(), "((-2, -1), (1, 2))");
        assert_eq!(Line::largest().to_string(), "((-128, -128), (127, 127))");
        assert_eq!(Line::min().to_string(), "((-128, -128), (-128, -128))");
        assert_eq!(Line::max().to_string(), "((127, 127), (127, 127))");
        assert_eq!(Line::zero().to_string(), "((0, 0), (0, 0))");
    }
}
