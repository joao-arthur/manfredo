use crate::cartesian::d4::{line::line_f32, point::point_f64::Point};

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
        d1::point::point_f32,
        d4::{line::line_f32, point::point_f64::Point},
    };

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point::min(), max: Point::max() });
        assert_eq!(Line::min(), Line { min: Point::min(), max: Point::min() });
        assert_eq!(Line::max(), Line { min: Point::max(), max: Point::max() });
        assert_eq!(
            Line::of(Point::of(-4.0, -3.0, -2.0, -1.0), Point::of(1.0, 2.0, 3.0, 4.0)),
            Line { min: Point { x: -4.0, y: -3.0, z: -2.0, w: -1.0 }, max: Point { x: 1.0, y: 2.0, z: 3.0, w: 4.0 } }
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(
            Line::largest().to_string(),
            "((-9007199254740992, -9007199254740992, -9007199254740992, -9007199254740992), (9007199254740991, 9007199254740991, 9007199254740991, 9007199254740991))"
        );
        assert_eq!(
            Line::min().to_string(),
            "((-9007199254740992, -9007199254740992, -9007199254740992, -9007199254740992), (-9007199254740992, -9007199254740992, -9007199254740992, -9007199254740992))"
        );
        assert_eq!(Line::max().to_string(), "((9007199254740991, 9007199254740991, 9007199254740991, 9007199254740991), (9007199254740991, 9007199254740991, 9007199254740991, 9007199254740991))");
        assert_eq!(Line::of(Point::of(-4.0, -3.0, -2.0, -1.0), Point::of(1.0, 2.0, 3.0, 4.0)).to_string(), "((-4, -3, -2, -1), (1, 2, 3, 4))");
    }

    #[test]
    fn from() {
        assert_eq!(
            Line::from(line_f32::Line::largest()),
            Line {
                min: Point { x: point_f32::MIN.into(), y: point_f32::MIN.into(), z: point_f32::MIN.into(), w: point_f32::MIN.into() },
                max: Point { x: point_f32::MAX.into(), y: point_f32::MAX.into(), z: point_f32::MAX.into(), w: point_f32::MAX.into() }
            }
        );
    }
}
