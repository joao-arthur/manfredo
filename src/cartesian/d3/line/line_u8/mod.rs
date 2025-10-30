use crate::cartesian::d3::point::point_u8::Point;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn of(min: (u8, u8, u8), max: (u8, u8, u8)) -> Self {
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
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::cartesian::d3::point::point_u8::Point;

    #[test]
    fn line() {
        assert_eq!(Line::of((0, 1, 2), (3, 4, 5)), Line { min: Point { x: 0, y: 1, z: 2 }, max: Point { x: 3, y: 4, z: 5 } });
        assert_eq!(Line::of((6, 7, 8), (9, 10, 11)), Line { min: Point { x: 6, y: 7, z: 8 }, max: Point { x: 9, y: 10, z: 11 } });
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::of((0, 1, 2), (3, 4, 5)).to_string(), "((0, 1, 2), (3, 4, 5))");
        assert_eq!(Line::largest().to_string(), "((0, 0, 0), (255, 255, 255))");
        assert_eq!(Line::min().to_string(), "((0, 0, 0), (0, 0, 0))");
        assert_eq!(Line::max().to_string(), "((255, 255, 255), (255, 255, 255))");
    }
}
