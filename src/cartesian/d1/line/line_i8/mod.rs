use crate::cartesian::d1::point::point_i8::Point;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn of(x1: i8, x2: i8) -> Self {
        Line { min: Point::of(x1), max: Point::of(x2) }
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
    use crate::cartesian::d1::point::point_i8::{MAX, MIN, Point};

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point { x: MIN }, max: Point { x: MAX } });
        assert_eq!(Line::min(), Line { min: Point { x: MIN }, max: Point { x: MIN } });
        assert_eq!(Line::max(), Line { min: Point { x: MAX }, max: Point { x: MAX } });
        assert_eq!(Line::of(MIN, 1), Line { min: Point { x: MIN }, max: Point { x: 1 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((-128), (127))");
        assert_eq!(Line::of(MIN, 0).to_string(), "((-128), (0))");
    }
}
