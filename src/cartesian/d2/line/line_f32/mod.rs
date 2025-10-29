use crate::cartesian::d2::point::point_f32::Point;

#[derive(PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn of(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
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
    use crate::cartesian::d2::point::point_f32::Point;

    #[test]
    fn line() {
        assert_eq!(Line::of(-2.0, -1.0, 1.0, 2.0), Line { min: Point { x: -2.0, y: -1.0 }, max: Point { x: 1.0, y: 2.0 } });
        assert_eq!(Line::of(-4.0, -2.0, 2.0, 4.0), Line { min: Point { x: -4.0, y: -2.0 }, max: Point { x: 2.0, y: 4.0 } });
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(Line::zero(), Line { min: Point::zero(), max: Point::zero() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::of(-2.0, -1.0, 1.0, 2.0).to_string(), "((-2, -1), (1, 2))");
        assert_eq!(Line::largest().to_string(), "((-16777216, -16777216), (16777215, 16777215))");
        assert_eq!(Line::min().to_string(), "((-16777216, -16777216), (-16777216, -16777216))");
        assert_eq!(Line::max().to_string(), "((16777215, 16777215), (16777215, 16777215))");
        assert_eq!(Line::zero().to_string(), "((0, 0), (0, 0))");
    }
}
