use crate::matrix::d3::point::point_i8::Point;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn of(min: (i8, i8, i8), max: (i8, i8, i8)) -> Self {
        Line { min: Point::of(min.0, min.1, min.2), max: Point::of(max.0, max.1, max.2) }
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
        assert_eq!(Line::of((-3, -2, -1), (1, 2, 3)), Line { min: Point { row: -3, col: -2, depth: -1 }, max: Point { row: 1, col: 2, depth: 3 } });
        assert_eq!(Line::of((-6, -5, -4), (4, 5, 6)), Line { min: Point { row: -6, col: -5, depth: -4 }, max: Point { row: 4, col: 5, depth: 6 } });
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(Line::zero(), Line { min: Point::zero(), max: Point::zero() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::of((-3, -2, -1), (1, 2, 3)).to_string(), "((-3, -2, -1), (1, 2, 3))");
        assert_eq!(Line::largest().to_string(), "((-128, -128, -128), (127, 127, 127))");
        assert_eq!(Line::min().to_string(), "((-128, -128, -128), (-128, -128, -128))");
        assert_eq!(Line::max().to_string(), "((127, 127, 127), (127, 127, 127))");
        assert_eq!(Line::zero().to_string(), "((0, 0, 0), (0, 0, 0))");
    }
}
