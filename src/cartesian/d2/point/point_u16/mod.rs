use super::point_u8;

mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::{delta, delta_x, delta_y};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn of(x: u16, y: u16) -> Self {
        Point { x, y }
    }

    pub fn min() -> Self {
        Point { x: 0, y: 0 }
    }

    pub fn max() -> Self {
        Point { x: u16::MAX, y: u16::MAX }
    }
}

impl From<point_u8::Point> for Point {
    fn from(p: point_u8::Point) -> Self {
        Point { x: p.x.into(), y: p.y.into() }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::cartesian::d2::point::point_u8;

    #[test]
    fn point() {
        assert_eq!(Point::of(0, u16::MAX), Point { x: 0, y: u16::MAX });
        assert_eq!(Point::min(), Point { x: 0, y: 0 });
        assert_eq!(Point::max(), Point { x: u16::MAX, y: u16::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(0, u16::MAX).to_string(), "(0, 65535)");
        assert_eq!(Point::min().to_string(), "(0, 0)");
        assert_eq!(Point::max().to_string(), "(65535, 65535)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_u8::Point::min()), Point { x: u8::MIN.into(), y: u8::MIN.into() });
        assert_eq!(Point::from(point_u8::Point::max()), Point { x: u8::MAX.into(), y: u8::MAX.into() });
    }
}
