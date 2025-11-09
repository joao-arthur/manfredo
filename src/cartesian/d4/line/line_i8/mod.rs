use crate::cartesian::d4::point::point_i8::Point;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn new(min: (i8, i8, i8, i8), max: (i8, i8, i8, i8)) -> Self {
        Line { min: Point::new(min.0, min.1, min.2, min.3), max: Point::new(max.0, max.1, max.2, max.3) }
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
    use crate::cartesian::d4::point::point_i8::Point;

    #[test]
    fn line() {
        assert_eq!(Line::new((-4, -3, -2, -1), (1, 2, 3, 4)), Line { min: Point { x: -4, y: -3, z: -2, w: -1 }, max: Point { x: 1, y: 2, z: 3, w: 4 } });
        assert_eq!(Line::new((-8, -7, -6, -5), (5, 6, 7, 8)), Line { min: Point { x: -8, y: -7, z: -6, w: -5 }, max: Point { x: 5, y: 6, z: 7, w: 8 } });
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(Line::zero(), Line { min: Point::zero(), max: Point::zero() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::new((-4, -3, -2, -1), (1, 2, 3, 4)).to_string(), "((-4, -3, -2, -1), (1, 2, 3, 4))");
        assert_eq!(Line::largest().to_string(), "((-128, -128, -128, -128), (127, 127, 127, 127))");
        assert_eq!(Line::min().to_string(), "((-128, -128, -128, -128), (-128, -128, -128, -128))");
        assert_eq!(Line::max().to_string(), "((127, 127, 127, 127), (127, 127, 127, 127))");
        assert_eq!(Line::zero().to_string(), "((0, 0, 0, 0), (0, 0, 0, 0))");
    }
}
