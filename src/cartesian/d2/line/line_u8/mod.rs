use crate::cartesian::d2::point::point_u8;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: point_u8::Point,
    pub max: point_u8::Point,
}

impl Line {
    pub fn of(x1: u8, y1: u8, x2: u8, y2: u8) -> Self {
        Line { min: point_u8::Point::of(x1, y1), max: point_u8::Point::of(x2, y2) }
    }

    pub fn largest() -> Self {
        Line { min: point_u8::Point::min(), max: point_u8::Point::max() }
    }

    pub fn min() -> Self {
        Line { min: point_u8::Point::min(), max: point_u8::Point::min() }
    }

    pub fn max() -> Self {
        Line { min: point_u8::Point::max(), max: point_u8::Point::max() }
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

    const MAX: u8 = u8::MAX;

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point { x: 0, y: 0 }, max: Point { x: MAX, y: MAX } });
        assert_eq!(Line::min(), Line { min: Point { x: 0, y: 0 }, max: Point { x: 0, y: 0 } });
        assert_eq!(Line::max(), Line { min: Point { x: MAX, y: MAX }, max: Point { x: MAX, y: MAX } });
        assert_eq!(Line::of(0, 2, 4, 8), Line { min: Point { x: 0, y: 2 }, max: Point { x: 4, y: 8 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((0, 0), (255, 255))");
        assert_eq!(Line::of(0, 2, 4, 8).to_string(), "((0, 2), (4, 8))");
    }
}
