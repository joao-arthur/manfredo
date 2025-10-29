use crate::matrix::d3::point::point_i8::Point;

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

    pub fn zero() -> Self {
        Line { min: Point::zero(), max: Point::zero() }
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
    use crate::matrix::d3::point::point_i8::Point;

    #[test]
    fn line() {
        assert_eq!(Line::of(Point::of(-3, -2, -1), Point::of(1, 2, 3)), Line { min: Point { row: -3, col: -2, depth: -1 }, max: Point { row: 1, col: 2, depth: 3 } });
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(Line::zero(), Line { min: Point::zero(), max: Point::zero() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::of(Point::of(-3, -2, -1), Point::of(1, 2, 3)).to_string(), "((-3, -2, -1), (1, 2, 3))");
        assert_eq!(Line::largest().to_string(), "((-128, -128, -128), (127, 127, 127))");
        assert_eq!(Line::min().to_string(), "((-128, -128, -128), (-128, -128, -128))");
        assert_eq!(Line::max().to_string(), "((127, 127, 127), (127, 127, 127))");
        assert_eq!(Line::zero().to_string(), "((0, 0, 0), (0, 0, 0))");
    }
}
