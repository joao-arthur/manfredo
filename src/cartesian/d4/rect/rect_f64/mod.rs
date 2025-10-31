use crate::cartesian::d4::{point::point_f64::Point, rect::rect_f32};

#[derive(PartialEq, Debug, Clone)]
pub struct Rect {
    pub min: Point,
    pub max: Point,
}

impl Rect {
    pub fn of(min: (f64, f64, f64, f64), max: (f64, f64, f64, f64)) -> Self {
        Rect { min: Point { x: min.0, y: min.1, z: min.2, w: min.3 }, max: Point { x: max.0, y: max.1, z: max.2, w: max.3 } }
    }

    pub fn largest() -> Self {
        Rect { min: Point::min(), max: Point::max() }
    }

    pub fn min() -> Self {
        Rect { min: Point::min(), max: Point::min() }
    }

    pub fn max() -> Self {
        Rect { min: Point::max(), max: Point::max() }
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

impl From<rect_f32::Rect> for Rect {
    fn from(l: rect_f32::Rect) -> Self {
        Rect { min: Point::from(l.min), max: Point::from(l.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Rect;
    use crate::cartesian::{
        d1::point::point_f32,
        d4::{point::point_f64::Point, rect::rect_f32},
    };

    #[test]
    fn rect() {
        assert_eq!(Rect::largest(), Rect { min: Point::min(), max: Point::max() });
        assert_eq!(Rect::min(), Rect { min: Point::min(), max: Point::min() });
        assert_eq!(Rect::max(), Rect { min: Point::max(), max: Point::max() });
        assert_eq!(
            Rect::of((-4.0, -3.0, -2.0, -1.0), (1.0, 2.0, 3.0, 4.0)),
            Rect { min: Point { x: -4.0, y: -3.0, z: -2.0, w: -1.0 }, max: Point { x: 1.0, y: 2.0, z: 3.0, w: 4.0 } }
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(
            Rect::largest().to_string(),
            "((-9007199254740992, -9007199254740992, -9007199254740992, -9007199254740992), (9007199254740991, 9007199254740991, 9007199254740991, 9007199254740991))"
        );
        assert_eq!(
            Rect::min().to_string(),
            "((-9007199254740992, -9007199254740992, -9007199254740992, -9007199254740992), (-9007199254740992, -9007199254740992, -9007199254740992, -9007199254740992))"
        );
        assert_eq!(Rect::max().to_string(), "((9007199254740991, 9007199254740991, 9007199254740991, 9007199254740991), (9007199254740991, 9007199254740991, 9007199254740991, 9007199254740991))");
        assert_eq!(Rect::of((-4.0, -3.0, -2.0, -1.0), (1.0, 2.0, 3.0, 4.0)).to_string(), "((-4, -3, -2, -1), (1, 2, 3, 4))");
    }

    #[test]
    fn from() {
        assert_eq!(
            Rect::from(rect_f32::Rect::largest()),
            Rect {
                min: Point { x: point_f32::MIN.into(), y: point_f32::MIN.into(), z: point_f32::MIN.into(), w: point_f32::MIN.into() },
                max: Point { x: point_f32::MAX.into(), y: point_f32::MAX.into(), z: point_f32::MAX.into(), w: point_f32::MAX.into() }
            }
        );
    }
}
