use crate::cartesian::d3::point::point_f32::Point;

#[derive(PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn new(min: (f32, f32, f32), max: (f32, f32, f32)) -> Self {
        Line { min: Point::new(min.0, min.1, min.2), max: Point::new(max.0, max.1, max.2) }
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
    use crate::cartesian::d3::point::point_f32::Point;

    #[test]
    fn line() {
        assert_eq!(Line::new((-3.0, -2.0, -1.0), (1.0, 2.0, 3.0)), Line { min: Point { x: -3.0, y: -2.0, z: -1.0 }, max: Point { x: 1.0, y: 2.0, z: 3.0 } });
        assert_eq!(Line::new((-6.0, -5.0, -4.0), (4.0, 5.0, 6.0)), Line { min: Point { x: -6.0, y: -5.0, z: -4.0 }, max: Point { x: 4.0, y: 5.0, z: 6.0 } });
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(Line::zero(), Line { min: Point::zero(), max: Point::zero() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::new((-3.0, -2.0, -1.0), (1.0, 2.0, 3.0)).to_string(), "((-3, -2, -1), (1, 2, 3))");
        assert_eq!(Line::largest().to_string(), "((-16777216, -16777216, -16777216), (16777215, 16777215, 16777215))");
        assert_eq!(Line::min().to_string(), "((-16777216, -16777216, -16777216), (-16777216, -16777216, -16777216))");
        assert_eq!(Line::max().to_string(), "((16777215, 16777215, 16777215), (16777215, 16777215, 16777215))");
        assert_eq!(Line::zero().to_string(), "((0, 0, 0), (0, 0, 0))");
    }
}
