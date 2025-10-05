use crate::cartesian::d2::point::point_f32;

#[derive(PartialEq, Debug, Clone)]
pub struct Line {
    pub min: point_f32::Point,
    pub max: point_f32::Point,
}

impl Line {
    pub fn of(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Line { min: point_f32::Point { x: x1, y: y1 }, max: point_f32::Point { x: x2, y: y2 } }
    }

    pub fn largest() -> Self {
        Line { min: point_f32::Point::min(), max: point_f32::Point::max() }
    }

    pub fn min() -> Self {
        Line { min: point_f32::Point::min(), max: point_f32::Point::min() }
    }

    pub fn max() -> Self {
        Line { min: point_f32::Point::max(), max: point_f32::Point::max() }
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
    use crate::cartesian::d2::point::point_f32::{MAX, MIN, Point};

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point { x: MIN, y: MIN }, max: Point { x: MAX, y: MAX } });
        assert_eq!(Line::min(), Line { min: Point { x: MIN, y: MIN }, max: Point { x: MIN, y: MIN } });
        assert_eq!(Line::max(), Line { min: Point { x: MAX, y: MAX }, max: Point { x: MAX, y: MAX } });
        assert_eq!(Line::of(MIN, -0.0, 0.0, MAX), Line { min: Point { x: MIN, y: -0.0 }, max: Point { x: 0.0, y: MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((-16777216, -16777216), (16777215, 16777215))");
        assert_eq!(Line::of(MIN, -0.0, 0.0, MAX).to_string(), "((-16777216, -0), (0, 16777215))");
    }
}
