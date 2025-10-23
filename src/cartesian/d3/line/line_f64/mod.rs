use crate::cartesian::d3::{line::line_f32, point::point_f64::Point};

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

impl From<line_f32::Line> for Line {
    fn from(l: line_f32::Line) -> Self {
        Line { min: Point::from(l.min), max: Point::from(l.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::cartesian::{
        d1::point::{
            point_f32,
            point_f64::{MAX, MIN},
        },
        d3::{line::line_f32, point::point_f64::Point},
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
        assert_eq!(Line::largest().to_string(), "((-9007199254740992, -9007199254740992, -9007199254740992), (9007199254740991, 9007199254740991, 9007199254740991))");
        assert_eq!(Line::min().to_string(), "((-9007199254740992, -9007199254740992, -9007199254740992), (-9007199254740992, -9007199254740992, -9007199254740992))");
        assert_eq!(Line::max().to_string(), "((9007199254740991, 9007199254740991, 9007199254740991), (9007199254740991, 9007199254740991, 9007199254740991))");
        assert_eq!(
            Line::of(Point::of(MIN, MIN + 1.0, MIN + 2.0), Point::of(MAX - 2.0, MAX - 1.0, MAX)).to_string(),
            "((-9007199254740992, -9007199254740991, -9007199254740990), (9007199254740989, 9007199254740990, 9007199254740991))"
        );
    }

    #[test]
    fn from() {
        assert_eq!(
            Line::from(line_f32::Line::largest()),
            Line {
                min: Point { x: point_f32::MIN.into(), y: point_f32::MIN.into(), z: point_f32::MIN.into() },
                max: Point { x: point_f32::MAX.into(), y: point_f32::MAX.into(), z: point_f32::MAX.into() }
            }
        );
    }
}
