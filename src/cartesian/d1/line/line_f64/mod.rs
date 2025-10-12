use crate::cartesian::d1::{line::line_f32, point::point_f64::Point};

#[derive(PartialEq, Debug, Clone)]
pub struct Line {
    pub min: Point,
    pub max: Point,
}

impl Line {
    pub fn of(x1: f64, x2: f64) -> Self {
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

impl From<line_f32::Line> for Line {
    fn from(r: line_f32::Line) -> Self {
        Line { min: Point::from(r.min), max: Point::from(r.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::cartesian::d1::{
        line::line_f32,
        point::{
            point_f32,
            point_f64::{MAX, MIN, Point},
        },
    };

    #[test]
    fn line() {
        assert_eq!(Line::largest(), Line { min: Point { x: MIN }, max: Point { x: MAX } });
        assert_eq!(Line::min(), Line { min: Point { x: MIN }, max: Point { x: MIN } });
        assert_eq!(Line::max(), Line { min: Point { x: MAX }, max: Point { x: MAX } });
        assert_eq!(Line::of(MIN, 0.0), Line { min: Point { x: MIN }, max: Point { x: 0.0 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Line::largest().to_string(), "((-9007199254740992), (9007199254740991))");
        assert_eq!(Line::of(MIN, 0.0).to_string(), "((-9007199254740992), (0))");
    }

    #[test]
    fn from() {
        assert_eq!(Line::from(line_f32::Line::largest()), Line { min: Point { x: point_f32::MIN.into() }, max: Point { x: point_f32::MAX.into() } });
    }
}
