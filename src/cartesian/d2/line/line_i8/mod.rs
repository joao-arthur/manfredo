use crate::cartesian::d2::point::point_i8::Point;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn of(x1: i8, y1: i8, x2: i8, y2: i8) -> Self {
        Line { min: Point::of(x1, y1), max: Point::of(x2, y2) }
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
    use crate::cartesian::d2::point::point_i8::Point;

    #[test]
    fn line() {
        assert_eq!(Line::of(-2, -1, 1, 2), Line { min: Point { x: -2, y: -1 }, max: Point { x: 1, y: 2 } });
        assert_eq!(Line::of(-4, -2, 2, 4), Line { min: Point { x: -4, y: -2 }, max: Point { x: 2, y: 4 } });
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
