use super::{point_u8, point_u16, point_u32};

mod add;
mod delta;
mod distance;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::{delta, delta_max, delta_min, delta_x, delta_y};
pub use self::distance::distance;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: u64,
    pub y: u64,
}

impl Point {
    pub fn of(x: u64, y: u64) -> Self {
        Point { x, y }
    }

    pub fn min() -> Self {
        Point { x: 0, y: 0 }
    }

    pub fn max() -> Self {
        Point { x: u64::MAX, y: u64::MAX }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<point_u8::Point> for Point {
    fn from(p: point_u8::Point) -> Self {
        Point { x: p.x.into(), y: p.y.into() }
    }
}

impl From<point_u16::Point> for Point {
    fn from(p: point_u16::Point) -> Self {
        Point { x: p.x.into(), y: p.y.into() }
    }
}

impl From<point_u32::Point> for Point {
    fn from(p: point_u32::Point) -> Self {
        Point { x: p.x.into(), y: p.y.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::cartesian::d2::point::{point_u8, point_u16, point_u32};

    #[test]
    fn point() {
        assert_eq!(Point::of(0, u64::MAX), Point { x: 0, y: u64::MAX });
        assert_eq!(Point::min(), Point { x: 0, y: 0 });
        assert_eq!(Point::max(), Point { x: u64::MAX, y: u64::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(0, u64::MAX).to_string(), "(0, 18446744073709551615)");
        assert_eq!(Point::min().to_string(), "(0, 0)");
        assert_eq!(Point::max().to_string(), "(18446744073709551615, 18446744073709551615)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_u8::Point::min()), Point { x: u8::MIN.into(), y: u8::MIN.into() });
        assert_eq!(Point::from(point_u8::Point::max()), Point { x: u8::MAX.into(), y: u8::MAX.into() });
        assert_eq!(Point::from(point_u16::Point::min()), Point { x: u16::MIN.into(), y: u16::MIN.into() });
        assert_eq!(Point::from(point_u16::Point::max()), Point { x: u16::MAX.into(), y: u16::MAX.into() });
        assert_eq!(Point::from(point_u32::Point::min()), Point { x: u32::MIN.into(), y: u32::MIN.into() });
        assert_eq!(Point::from(point_u32::Point::max()), Point { x: u32::MAX.into(), y: u32::MAX.into() });
    }
}
