use super::{point_i8, point_i16};

mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::{delta, delta_x, delta_y};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn of(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn min() -> Self {
        Point { x: i32::MIN, y: i32::MIN }
    }

    pub fn max() -> Self {
        Point { x: i32::MAX, y: i32::MAX }
    }
}

impl From<point_i8::Point> for Point {
    fn from(p: point_i8::Point) -> Self {
        Point { x: p.x.into(), y: p.y.into() }
    }
}

impl From<point_i16::Point> for Point {
    fn from(p: point_i16::Point) -> Self {
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
    use crate::cartesian::d2::point::{point_i8, point_i16};

    #[test]
    fn point_i32() {
        assert_eq!(Point::of(i32::MIN, i32::MAX), Point { x: i32::MIN, y: i32::MAX });
        assert_eq!(Point::min(), Point { x: i32::MIN, y: i32::MIN });
        assert_eq!(Point::max(), Point { x: i32::MAX, y: i32::MAX });
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_i8::Point::min()), Point { x: i8::MIN.into(), y: i8::MIN.into() });
        assert_eq!(Point::from(point_i8::Point::max()), Point { x: i8::MAX.into(), y: i8::MAX.into() });
        assert_eq!(Point::from(point_i16::Point::min()), Point { x: i16::MIN.into(), y: i16::MIN.into() });
        assert_eq!(Point::from(point_i16::Point::max()), Point { x: i16::MAX.into(), y: i16::MAX.into() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(i32::MIN, i32::MAX).to_string(), "(-2147483648, 2147483647)");
        assert_eq!(Point::min().to_string(), "(-2147483648, -2147483648)");
        assert_eq!(Point::max().to_string(), "(2147483647, 2147483647)");
    }
}
