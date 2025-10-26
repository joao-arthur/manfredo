use super::{point_u8, point_u16, point_u32};
use crate::cartesian::d1::point::point_u64::MAX;

mod delta;

pub use self::delta::{delta, delta_max, delta_min, delta_w, delta_x, delta_y, delta_z};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: u64,
    pub y: u64,
    pub z: u64,
    pub w: u64,
}

impl Point {
    pub fn of(x: u64, y: u64, z: u64, w: u64) -> Self {
        Point { x, y, z, w }
    }

    pub fn min() -> Self {
        Point { x: 0, y: 0, z: 0, w: 0 }
    }

    pub fn max() -> Self {
        Point { x: MAX, y: MAX, z: MAX, w: MAX }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl From<point_u8::Point> for Point {
    fn from(p: point_u8::Point) -> Self {
        Point { x: p.x.into(), y: p.y.into(), z: p.z.into(), w: p.w.into() }
    }
}

impl From<point_u16::Point> for Point {
    fn from(p: point_u16::Point) -> Self {
        Point { x: p.x.into(), y: p.y.into(), z: p.z.into(), w: p.w.into() }
    }
}

impl From<point_u32::Point> for Point {
    fn from(p: point_u32::Point) -> Self {
        Point { x: p.x.into(), y: p.y.into(), z: p.z.into(), w: p.w.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::cartesian::{
        d1::point::point_u64::MAX,
        d4::point::{point_u8, point_u16, point_u32},
    };

    #[test]
    fn point() {
        assert_eq!(Point::of(1, 2, 3, 4), Point { x: 1, y: 2, z: 3, w: 4 });
        assert_eq!(Point::of(4, 3, 2, 1), Point { x: 4, y: 3, z: 2, w: 1 });
        assert_eq!(Point::min(), Point { x: 0, y: 0, z: 0, w: 0 });
        assert_eq!(Point::max(), Point { x: MAX, y: MAX, z: MAX, w: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(1, 2, 3, 4).to_string(), "(1, 2, 3, 4)");
        assert_eq!(Point::min().to_string(), "(0, 0, 0, 0)");
        assert_eq!(Point::max().to_string(), "(18446744073709551615, 18446744073709551615, 18446744073709551615, 18446744073709551615)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_u8::Point::min()), Point { x: u8::MIN.into(), y: u8::MIN.into(), z: u8::MIN.into(), w: u8::MIN.into() });
        assert_eq!(Point::from(point_u8::Point::max()), Point { x: u8::MAX.into(), y: u8::MAX.into(), z: u8::MAX.into(), w: u8::MAX.into() });
        assert_eq!(Point::from(point_u16::Point::min()), Point { x: u16::MIN.into(), y: u16::MIN.into(), z: u16::MIN.into(), w: u16::MIN.into() });
        assert_eq!(Point::from(point_u16::Point::max()), Point { x: u16::MAX.into(), y: u16::MAX.into(), z: u16::MAX.into(), w: u16::MAX.into() });
        assert_eq!(Point::from(point_u32::Point::min()), Point { x: u32::MIN.into(), y: u32::MIN.into(), z: u32::MIN.into(), w: u32::MIN.into() });
        assert_eq!(Point::from(point_u32::Point::max()), Point { x: u32::MAX.into(), y: u32::MAX.into(), z: u32::MAX.into(), w: u32::MAX.into() });
    }
}
