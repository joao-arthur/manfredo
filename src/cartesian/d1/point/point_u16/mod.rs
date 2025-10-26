use super::point_u8;

mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::delta;

pub const MAX: u16 = u16::MAX;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: u16,
}

impl Point {
    pub fn of(x: u16) -> Self {
        Point { x }
    }

    pub fn min() -> Self {
        Point { x: 0 }
    }

    pub fn max() -> Self {
        Point { x: MAX }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.x)
    }
}

impl From<point_u8::Point> for Point {
    fn from(p: point_u8::Point) -> Self {
        Point { x: p.x.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX, Point};
    use crate::cartesian::d1::point::point_u8;

    #[test]
    fn point() {
        assert_eq!(Point::of(1), Point { x: 1 });
        assert_eq!(Point::of(2), Point { x: 2 });
        assert_eq!(Point::min(), Point { x: 0 });
        assert_eq!(Point::max(), Point { x: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(1).to_string(), "(1)");
        assert_eq!(Point::min().to_string(), "(0)");
        assert_eq!(Point::max().to_string(), "(65535)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_u8::Point::min()), Point { x: u8::MIN.into() });
        assert_eq!(Point::from(point_u8::Point::max()), Point { x: u8::MAX.into() });
    }
}
