use super::point_f32;
use crate::cartesian::d1::point::point_f64::{MAX, MIN};

mod delta;

pub use self::delta::{delta, delta_max, delta_min, delta_x, delta_y, delta_z};

#[derive(PartialEq, Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn of(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    pub fn min() -> Self {
        Point { x: MIN, y: MIN, z: MIN }
    }

    pub fn max() -> Self {
        Point { x: MAX, y: MAX, z: MAX }
    }

    pub fn zero() -> Self {
        Point { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<point_f32::Point> for Point {
    fn from(p: point_f32::Point) -> Self {
        Point { x: p.x.into(), y: p.y.into(), z: p.z.into() }
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
        d3::point::point_f32::Point as PointF32,
    };

    #[test]
    fn point() {
        assert_eq!(Point::of(-1.0, 0.0, 1.0), Point { x: -1.0, y: 0.0, z: 1.0 });
        assert_eq!(Point::of(0.0, 1.0, -1.0), Point { x: 0.0, y: 1.0, z: -1.0 });
        assert_eq!(Point::min(), Point { x: MIN, y: MIN, z: MIN });
        assert_eq!(Point::max(), Point { x: MAX, y: MAX, z: MAX });
        assert_eq!(Point::zero(), Point { x: 0.0, y: 0.0, z: 0.0 });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(-1.0, 0.0, 1.0).to_string(), "(-1, 0, 1)");
        assert_eq!(Point::min().to_string(), "(-9007199254740992, -9007199254740992, -9007199254740992)");
        assert_eq!(Point::max().to_string(), "(9007199254740991, 9007199254740991, 9007199254740991)");
        assert_eq!(Point::zero().to_string(), "(0, 0, 0)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(PointF32::min()), Point { x: MIN_F32.into(), y: MIN_F32.into(), z: MIN_F32.into() });
        assert_eq!(Point::from(PointF32::max()), Point { x: MAX_F32.into(), y: MAX_F32.into(), z: MAX_F32.into() });
    }
}
