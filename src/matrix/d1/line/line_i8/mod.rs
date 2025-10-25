use crate::matrix::d1::point::point_i8::Point;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn of(i1: i8, i2: i8) -> Self {
        Line { min: Point::of(i1), max: Point::of(i2) }
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

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::matrix::d1::point::point_i8::Point;

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(Line::of(-1, 1), Line { min: Point { i: -1 }, max: Point { i: 1 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((-128), (127))");
        assert_eq!(Line::min().to_string(), "((-128), (-128))");
        assert_eq!(Line::max().to_string(), "((127), (127))");
        assert_eq!(Line::of(-1, 1).to_string(), "((-1), (1))");
    }
}
