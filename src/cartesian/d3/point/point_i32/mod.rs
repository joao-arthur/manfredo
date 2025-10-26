use super::{point_i8, point_i16};
use crate::cartesian::d1::point::point_i32::{MAX, MIN};

mod delta;

pub use self::delta::{delta, delta_max, delta_min, delta_x, delta_y, delta_z};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point {
    pub fn of(x: i32, y: i32, z: i32) -> Self {
        Point { x, y, z }
    }

    pub fn min() -> Self {
        Point { x: MIN, y: MIN, z: MIN }
    }

    pub fn max() -> Self {
        Point { x: MAX, y: MAX, z: MAX }
    }

    pub fn zero() -> Self {
        Point { x: 0, y: 0, z: 0 }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<point_i8::Point> for Point {
    fn from(p: point_i8::Point) -> Self {
        Point { x: p.x.into(), y: p.y.into(), z: p.z.into() }
    }
}

impl From<point_i16::Point> for Point {
    fn from(p: point_i16::Point) -> Self {
        Point { x: p.x.into(), y: p.y.into(), z: p.z.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::cartesian::{
        d1::point::point_i32::{MAX, MIN},
        d3::point::{point_i8, point_i16},
    };

    #[test]
    fn point() {
        assert_eq!(Point::of(-1, 0, 1), Point { x: -1, y: 0, z: 1 });
        assert_eq!(Point::of(0, 1, -1), Point { x: 0, y: 1, z: -1 });
        assert_eq!(Point::min(), Point { x: MIN, y: MIN, z: MIN });
        assert_eq!(Point::max(), Point { x: MAX, y: MAX, z: MAX });
        assert_eq!(Point::zero(), Point { x: 0, y: 0, z: 0 });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(-1, 0, 1).to_string(), "(-1, 0, 1)");
        assert_eq!(Point::min().to_string(), "(-2147483648, -2147483648, -2147483648)");
        assert_eq!(Point::max().to_string(), "(2147483647, 2147483647, 2147483647)");
        assert_eq!(Point::zero().to_string(), "(0, 0, 0)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_i8::Point::min()), Point { x: i8::MIN.into(), y: i8::MIN.into(), z: i8::MIN.into() });
        assert_eq!(Point::from(point_i8::Point::max()), Point { x: i8::MAX.into(), y: i8::MAX.into(), z: i8::MAX.into() });
        assert_eq!(Point::from(point_i16::Point::min()), Point { x: i16::MIN.into(), y: i16::MIN.into(), z: i16::MIN.into() });
        assert_eq!(Point::from(point_i16::Point::max()), Point { x: i16::MAX.into(), y: i16::MAX.into(), z: i16::MAX.into() });
    }
}
