use crate::cartesian::d3::point::point_f32::Point;

#[derive(PartialEq, Debug, Clone)]
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
    use crate::cartesian::{
        d1::point::point_f32::{MAX, MIN},
        d3::point::point_f32::Point,
    };

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(
            Line::of(Point::of(MIN, MIN + 1.0, MIN + 2.0), Point::of(MAX - 2.0, MAX - 1.0, MAX)),
            Line { min: Point { x: MIN, y: MIN + 1.0, z: MIN + 2.0 }, max: Point { x: MAX - 2.0, y: MAX - 1.0, z: MAX } }
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((-16777216, -16777216, -16777216), (16777215, 16777215, 16777215))");
        assert_eq!(Line::min().to_string(), "((-16777216, -16777216, -16777216), (-16777216, -16777216, -16777216))");
        assert_eq!(Line::max().to_string(), "((16777215, 16777215, 16777215), (16777215, 16777215, 16777215))");
        assert_eq!(Line::of(Point::of(MIN, MIN + 1.0, MIN + 2.0), Point::of(MAX - 2.0, MAX - 1.0, MAX)).to_string(), "((-16777216, -16777215, -16777214), (16777213, 16777214, 16777215))");
    }
}
