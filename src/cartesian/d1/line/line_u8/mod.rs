use crate::cartesian::d1::point::point_u8::Point;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn new(min: u8, max: u8) -> Self {
        Line { min: Point::new(min), max: Point::new(max) }
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
    use crate::cartesian::d1::point::point_u8::Point;

    #[test]
    fn line() {
        assert_eq!(Line::new(0, 1), Line { min: Point { x: 0 }, max: Point { x: 1 } });
        assert_eq!(Line::new(2, 3), Line { min: Point { x: 2 }, max: Point { x: 3 } });
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::new(0, 1).to_string(), "((0), (1))");
        assert_eq!(Line::largest().to_string(), "((0), (255))");
        assert_eq!(Line::min().to_string(), "((0), (0))");
        assert_eq!(Line::max().to_string(), "((255), (255))");
    }
}
