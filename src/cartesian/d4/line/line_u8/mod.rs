use crate::cartesian::d4::point::point_u8::Point;

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
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::cartesian::d4::point::point_u8::Point;

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(Line::of(Point::of(0, 1, 2, 3), Point::of(4, 5, 6, 7)), Line { min: Point { x: 0, y: 1, z: 2, w: 3 }, max: Point { x: 4, y: 5, z: 6, w: 7 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((0, 0, 0, 0), (255, 255, 255, 255))");
        assert_eq!(Line::min().to_string(), "((0, 0, 0, 0), (0, 0, 0, 0))");
        assert_eq!(Line::max().to_string(), "((255, 255, 255, 255), (255, 255, 255, 255))");
        assert_eq!(Line::of(Point::of(0, 1, 2, 3), Point::of(4, 5, 6, 7)).to_string(), "((0, 1, 2, 3), (4, 5, 6, 7))");
    }
}
