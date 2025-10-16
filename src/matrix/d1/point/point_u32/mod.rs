use super::{point_u8, point_u16};

mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::delta;

const MAX: u32 = u32::MAX;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub i: u32,
}

impl Point {
    pub fn of(i: u32) -> Self {
        Point { i }
    }

    pub fn min() -> Self {
        Point { i: 0 }
    }

    pub fn max() -> Self {
        Point { i: MAX }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.i)
    }
}

impl From<point_u8::Point> for Point {
    fn from(p: point_u8::Point) -> Self {
        Point { i: p.i.into() }
    }
}

impl From<point_u16::Point> for Point {
    fn from(p: point_u16::Point) -> Self {
        Point { i: p.i.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX, Point};
    use crate::matrix::d1::point::{point_u8, point_u16};

    #[test]
    fn point() {
        assert_eq!(Point::of(10), Point { i: 10 });
        assert_eq!(Point::min(), Point { i: 0 });
        assert_eq!(Point::max(), Point { i: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(10).to_string(), "(10)");
        assert_eq!(Point::min().to_string(), "(0)");
        assert_eq!(Point::max().to_string(), "(4294967295)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_u8::Point::min()), Point { i: u8::MIN.into() });
        assert_eq!(Point::from(point_u8::Point::max()), Point { i: u8::MAX.into() });
        assert_eq!(Point::from(point_u16::Point::min()), Point { i: u16::MIN.into() });
        assert_eq!(Point::from(point_u16::Point::max()), Point { i: u16::MAX.into() });
    }
}
