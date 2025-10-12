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

    const MIN: i8 = i8::MIN;
    const MAX: i8 = i8::MAX;

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point { x: MIN, y: MIN }, max: Point { x: MAX, y: MAX } });
        assert_eq!(Line::min(), Line { min: Point { x: MIN, y: MIN }, max: Point { x: MIN, y: MIN } });
        assert_eq!(Line::max(), Line { min: Point { x: MAX, y: MAX }, max: Point { x: MAX, y: MAX } });
        assert_eq!(Line::of(MIN, -1, 1, MAX), Line { min: Point { x: MIN, y: -1 }, max: Point { x: 1, y: MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((-128, -128), (127, 127))");
        assert_eq!(Line::of(MIN, -0, 0, MAX).to_string(), "((-128, 0), (0, 127))");
    }
}
