use crate::cartesian::d4::{point::point_f64::Point, rect::rect_f32};
use crate::iter::iter_f64;

#[derive(PartialEq, Debug, Clone)]
pub struct Rect {
    pub min: Point,
    pub max: Point,
}

impl Rect {
    pub fn new(min: (f64, f64, f64, f64), max: (f64, f64, f64, f64)) -> Self {
        Rect { min: Point { x: min.0, y: min.1, z: min.2, w: min.3 }, max: Point { x: max.0, y: max.1, z: max.2, w: max.3 } }
    }

    pub fn largest() -> Self {
        Rect { min: Point::min(), max: Point::max() }
    }

    pub fn largest_min() -> Self {
        Rect { min: Point::min(), max: Point::new(-1.0, -1.0, -1.0, -1.0) }
    }

    pub fn largest_max() -> Self {
        Rect { min: Point::zero(), max: Point::max() }
    }

    pub fn min() -> Self {
        Rect { min: Point::min(), max: Point::min() }
    }

    pub fn max() -> Self {
        Rect { min: Point::max(), max: Point::max() }
    }

    pub fn zero() -> Self {
        Rect { min: Point::zero(), max: Point::zero() }
    }

    pub fn iter_x(&self) -> iter_f64::Iter {
        iter_f64::Iter::new(self.min.x, self.max.x)
    }

    pub fn iter_y(&self) -> iter_f64::Iter {
        iter_f64::Iter::new(self.min.y, self.max.y)
    }

    pub fn iter_z(&self) -> iter_f64::Iter {
        iter_f64::Iter::new(self.min.z, self.max.z)
    }

    pub fn iter_w(&self) -> iter_f64::Iter {
        iter_f64::Iter::new(self.min.w, self.max.w)
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
        assert_eq!(Rect::new((-4.0, -3.0, -2.0, -1.0), (1.0, 2.0, 3.0, 4.0)), Rect { min: Point { x: -4.0, y: -3.0, z: -2.0, w: -1.0 }, max: Point { x: 1.0, y: 2.0, z: 3.0, w: 4.0 } });
        assert_eq!(Rect::largest(), Rect { min: Point::min(), max: Point::max() });
        assert_eq!(Rect::min(), Rect { min: Point::min(), max: Point::min() });
        assert_eq!(Rect::max(), Rect { min: Point::max(), max: Point::max() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::new((-4.0, -3.0, -2.0, -1.0), (1.0, 2.0, 3.0, 4.0)).to_string(), "((-4, -3, -2, -1), (1, 2, 3, 4))");
        assert_eq!(
            Rect::largest().to_string(),
            "((-9007199254740992, -9007199254740992, -9007199254740992, -9007199254740992), (9007199254740991, 9007199254740991, 9007199254740991, 9007199254740991))"
        );
        assert_eq!(
            Rect::min().to_string(),
            "((-9007199254740992, -9007199254740992, -9007199254740992, -9007199254740992), (-9007199254740992, -9007199254740992, -9007199254740992, -9007199254740992))"
        );
        assert_eq!(Rect::max().to_string(), "((9007199254740991, 9007199254740991, 9007199254740991, 9007199254740991), (9007199254740991, 9007199254740991, 9007199254740991, 9007199254740991))");
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

    #[test]
    fn iter_x() {
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-9.0, -6.0, -5.0, -4.0)).iter_x().collect::<Vec<f64>>(), []);
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-8.0, -6.0, -5.0, -4.0)).iter_x().collect::<Vec<f64>>(), [-8.0]);
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-7.0, -6.0, -5.0, -4.0)).iter_x().collect::<Vec<f64>>(), [-8.0, -7.0]);
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-7.0, -6.0, -5.0, -4.0)).iter_x().rev().collect::<Vec<f64>>(), [-7.0, -8.0]);
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-8.0, -6.0, -5.0, -4.0)).iter_x().rev().collect::<Vec<f64>>(), [-8.0]);
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-9.0, -6.0, -5.0, -4.0)).iter_x().rev().collect::<Vec<f64>>(), []);
    }

    #[test]
    fn iter_y() {
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-7.0, -8.0, -5.0, -4.0)).iter_y().collect::<Vec<f64>>(), []);
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-7.0, -7.0, -5.0, -4.0)).iter_y().collect::<Vec<f64>>(), [-7.0]);
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-7.0, -6.0, -5.0, -4.0)).iter_y().collect::<Vec<f64>>(), [-7.0, -6.0]);
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-7.0, -6.0, -5.0, -4.0)).iter_y().rev().collect::<Vec<f64>>(), [-6.0, -7.0]);
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-7.0, -7.0, -5.0, -4.0)).iter_y().rev().collect::<Vec<f64>>(), [-7.0]);
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-7.0, -8.0, -5.0, -4.0)).iter_y().rev().collect::<Vec<f64>>(), []);
    }

    #[test]
    fn iter_z() {
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-7.0, -6.0, -7.0, -4.0)).iter_z().collect::<Vec<f64>>(), []);
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-7.0, -6.0, -6.0, -4.0)).iter_z().collect::<Vec<f64>>(), [-6.0]);
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-7.0, -6.0, -5.0, -4.0)).iter_z().collect::<Vec<f64>>(), [-6.0, -5.0]);
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-7.0, -6.0, -5.0, -4.0)).iter_z().rev().collect::<Vec<f64>>(), [-5.0, -6.0]);
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-7.0, -6.0, -6.0, -4.0)).iter_z().rev().collect::<Vec<f64>>(), [-6.0]);
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-7.0, -6.0, -7.0, -4.0)).iter_z().rev().collect::<Vec<f64>>(), []);
    }

    #[test]
    fn iter_w() {
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-7.0, -6.0, -5.0, -6.0)).iter_w().collect::<Vec<f64>>(), []);
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-7.0, -6.0, -5.0, -5.0)).iter_w().collect::<Vec<f64>>(), [-5.0]);
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-7.0, -6.0, -5.0, -4.0)).iter_w().collect::<Vec<f64>>(), [-5.0, -4.0]);
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-7.0, -6.0, -5.0, -4.0)).iter_w().rev().collect::<Vec<f64>>(), [-4.0, -5.0]);
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-7.0, -6.0, -5.0, -5.0)).iter_w().rev().collect::<Vec<f64>>(), [-5.0]);
        assert_eq!(Rect::new((-8.0, -7.0, -6.0, -5.0), (-7.0, -6.0, -5.0, -6.0)).iter_w().rev().collect::<Vec<f64>>(), []);
    }
}
