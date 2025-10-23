use crate::cartesian::d3::point::point_i8::Point;

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
    use crate::cartesian::{
        d1::point::point_i8::{MAX, MIN},
        d3::point::point_i8::Point,
    };

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(Line::of(Point::of(MIN, MIN + 1, MIN + 2), Point::of(MAX - 2, MAX - 1, MAX)), Line { min: Point { x: MIN, y: MIN + 1, z: MIN + 2 }, max: Point { x: MAX - 2, y: MAX - 1, z: MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((-128, -128, -128), (127, 127, 127))");
        assert_eq!(Line::min().to_string(), "((-128, -128, -128), (-128, -128, -128))");
        assert_eq!(Line::max().to_string(), "((127, 127, 127), (127, 127, 127))");
        assert_eq!(Line::of(Point::of(MIN, MIN + 1, MIN + 2), Point::of(MAX - 2, MAX - 1, MAX)).to_string(), "((-128, -127, -126), (125, 126, 127))");
    }
}
