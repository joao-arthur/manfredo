use super::point_f32;

mod delta;

pub use self::delta::{delta, delta_max, delta_min, delta_x, delta_y, delta_z};

pub const MIN: f64 = -9_007_199_254_740_992.0;
pub const MAX: f64 = 9_007_199_254_740_991.0;

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
    use super::{MAX, MIN, Point};
    use crate::cartesian::d3::point::point_f32;

    #[test]
    fn point() {
        assert_eq!(Point::of(MIN, MAX, MIN), Point { x: MIN, y: MAX, z: MIN });
        assert_eq!(Point::of(MAX, MIN, MAX), Point { x: MAX, y: MIN, z: MAX });
        assert_eq!(Point::min(), Point { x: MIN, y: MIN, z: MIN });
        assert_eq!(Point::max(), Point { x: MAX, y: MAX, z: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(MIN, MAX, MIN).to_string(), "(-9007199254740992, 9007199254740991, -9007199254740992)");
        assert_eq!(Point::of(MAX, MIN, MAX).to_string(), "(9007199254740991, -9007199254740992, 9007199254740991)");
        assert_eq!(Point::min().to_string(), "(-9007199254740992, -9007199254740992, -9007199254740992)");
        assert_eq!(Point::max().to_string(), "(9007199254740991, 9007199254740991, 9007199254740991)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_f32::Point::min()), Point { x: point_f32::MIN.into(), y: point_f32::MIN.into(), z: point_f32::MIN.into() });
        assert_eq!(Point::from(point_f32::Point::max()), Point { x: point_f32::MAX.into(), y: point_f32::MAX.into(), z: point_f32::MAX.into() });
    }
}
