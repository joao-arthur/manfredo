use crate::cartesian::d1::point::point_u16::MAX;

use super::point_u8;

mod delta;

pub use self::delta::{delta, delta_max, delta_min, delta_x, delta_y, delta_z};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

impl Point {
    pub fn of(x: u16, y: u16, z: u16) -> Self {
        Point { x, y, z }
    }

    pub fn min() -> Self {
        Point { x: 0, y: 0, z: 0 }
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

impl From<point_u8::Point> for Point {
    fn from(p: point_u8::Point) -> Self {
        Point { x: p.x.into(), y: p.y.into(), z: p.z.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::cartesian::d1::point::point_u16::MAX;
    use crate::cartesian::d3::point::point_u8;

    #[test]
    fn point() {
        assert_eq!(Point::of(0, MAX, 0), Point { x: 0, y: MAX, z: 0 });
        assert_eq!(Point::of(MAX, 0, MAX), Point { x: MAX, y: 0, z: MAX });
        assert_eq!(Point::min(), Point { x: 0, y: 0, z: 0 });
        assert_eq!(Point::max(), Point { x: MAX, y: MAX, z: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(0, MAX, 0).to_string(), "(0, 65535, 0)");
        assert_eq!(Point::of(MAX, 0, MAX).to_string(), "(65535, 0, 65535)");
        assert_eq!(Point::min().to_string(), "(0, 0, 0)");
        assert_eq!(Point::max().to_string(), "(65535, 65535, 65535)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_u8::Point::min()), Point { x: u8::MIN.into(), y: u8::MIN.into(), z: u8::MIN.into() });
        assert_eq!(Point::from(point_u8::Point::max()), Point { x: u8::MAX.into(), y: u8::MAX.into(), z: u8::MAX.into() });
    }
}
