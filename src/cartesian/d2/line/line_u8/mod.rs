use crate::cartesian::d2::point::point_u8::Point;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn of(x1: u8, y1: u8, x2: u8, y2: u8) -> Self {
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
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::cartesian::d2::point::point_u8::Point;

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(Line::of(0, 2, 4, 8), Line { min: Point { x: 0, y: 2 }, max: Point { x: 4, y: 8 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((0, 0), (255, 255))");
        assert_eq!(Line::min().to_string(), "((0, 0), (0, 0))");
        assert_eq!(Line::max().to_string(), "((255, 255), (255, 255))");
        assert_eq!(Line::of(0, 2, 4, 8).to_string(), "((0, 2), (4, 8))");
    }
}
