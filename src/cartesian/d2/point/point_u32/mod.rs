use crate::cartesian::d1::point::point_u32::MAX;

use super::{point_u8, point_u16};

mod add;
mod delta;
mod distance;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::{delta, delta_max, delta_min, delta_x, delta_y};
pub use self::distance::distance;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn of(x: u32, y: u32) -> Self {
        Point { x, y }
    }

    pub fn min() -> Self {
        Point { x: 0, y: 0 }
    }

    pub fn max() -> Self {
        Point { x: MAX, y: MAX }
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

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::cartesian::d1::point::point_u32::MAX;
    use crate::cartesian::d2::point::{point_u8, point_u16};

    #[test]
    fn point() {
        assert_eq!(Point::of(0, MAX), Point { x: 0, y: MAX });
        assert_eq!(Point::of(MAX, 0), Point { x: MAX, y: 0 });
        assert_eq!(Point::min(), Point { x: 0, y: 0 });
        assert_eq!(Point::max(), Point { x: MAX, y: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(0, MAX).to_string(), "(0, 4294967295)");
        assert_eq!(Point::of(MAX, 0).to_string(), "(4294967295, 0)");
        assert_eq!(Point::min().to_string(), "(0, 0)");
        assert_eq!(Point::max().to_string(), "(4294967295, 4294967295)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_u8::Point::min()), Point { x: u8::MIN.into(), y: u8::MIN.into() });
        assert_eq!(Point::from(point_u8::Point::max()), Point { x: u8::MAX.into(), y: u8::MAX.into() });
        assert_eq!(Point::from(point_u16::Point::min()), Point { x: u16::MIN.into(), y: u16::MIN.into() });
        assert_eq!(Point::from(point_u16::Point::max()), Point { x: u16::MAX.into(), y: u16::MAX.into() });
    }
}
