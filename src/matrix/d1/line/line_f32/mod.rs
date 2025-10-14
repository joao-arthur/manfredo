use crate::matrix::d1::point::point_f32::Point;

#[derive(PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn of(i1: f32, i2: f32) -> Self {
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
    use crate::matrix::d1::point::point_f32::{MAX, MIN, Point};

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point { i: MIN }, max: Point { i: MAX } });
        assert_eq!(Line::min(), Line { min: Point { i: MIN }, max: Point { i: MIN } });
        assert_eq!(Line::max(), Line { min: Point { i: MAX }, max: Point { i: MAX } });
        assert_eq!(Line::of(MIN, 0.0), Line { min: Point { i: MIN }, max: Point { i: 0.0 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((-16777216), (16777215))");
        assert_eq!(Line::of(MIN, 0.0).to_string(), "((-16777216), (0))");
    }
}
