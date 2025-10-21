use super::point_f32;
use crate::cartesian::d1::point::point_f64::{MAX, MIN};

#[derive(PartialEq, Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Point {
    pub fn of(x: f64, y: f64, z: f64, w: f64) -> Self {
        Point { x, y, z, w }
    }

    pub fn min() -> Self {
        Point { x: MIN, y: MIN, z: MIN, w: MIN }
    }

    pub fn max() -> Self {
        Point { x: MAX, y: MAX, z: MAX, w: MAX }
    }

    pub fn zero() -> Self {
        Point { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl From<point_f32::Point> for Point {
    fn from(p: point_f32::Point) -> Self {
        Point { x: p.x.into(), y: p.y.into(), z: p.z.into(), w: p.w.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::cartesian::{
        d1::point::{
            point_f32::{MAX as MAX_F32, MIN as MIN_F32},
            point_f64::{MAX, MIN},
        },
        d4::point::point_f32::Point as PointF32,
    };

    #[test]
    fn point() {
        assert_eq!(Point::of(-20.0, -10.0, 10.0, 20.0), Point { x: -20.0, y: -10.0, z: 10.0, w: 20.0 });
        assert_eq!(Point::min(), Point { x: MIN, y: MIN, z: MIN, w: MIN });
        assert_eq!(Point::max(), Point { x: MAX, y: MAX, z: MAX, w: MAX });
        assert_eq!(Point::zero(), Point { x: 0.0, y: 0.0, z: 0.0, w: 0.0 });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(-20.0, -10.0, 10.0, 20.0).to_string(), "(-20, -10, 10, 20)");
        assert_eq!(Point::min().to_string(), "(-9007199254740992, -9007199254740992, -9007199254740992, -9007199254740992)");
        assert_eq!(Point::max().to_string(), "(9007199254740991, 9007199254740991, 9007199254740991, 9007199254740991)");
        assert_eq!(Point::zero().to_string(), "(0, 0, 0, 0)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(PointF32::min()), Point { x: MIN_F32.into(), y: MIN_F32.into(), z: MIN_F32.into(), w: MIN_F32.into() });
        assert_eq!(Point::from(PointF32::max()), Point { x: MAX_F32.into(), y: MAX_F32.into(), z: MAX_F32.into(), w: MAX_F32.into() });
    }
}
