use super::{point_i8, point_i16, point_i32};

mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::{delta, delta_max, delta_min, delta_x, delta_y};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn of(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    pub fn min() -> Self {
        Point { x: i64::MIN, y: i64::MIN }
    }

    pub fn max() -> Self {
        Point { x: i64::MAX, y: i64::MAX }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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

impl From<point_i32::Point> for Point {
    fn from(p: point_i32::Point) -> Self {
        Point { x: p.x.into(), y: p.y.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::cartesian::d2::point::{point_i8, point_i16, point_i32};

    #[test]
    fn point() {
        assert_eq!(Point::of(i64::MIN, i64::MAX), Point { x: i64::MIN, y: i64::MAX });
        assert_eq!(Point::min(), Point { x: i64::MIN, y: i64::MIN });
        assert_eq!(Point::max(), Point { x: i64::MAX, y: i64::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(i64::MIN, i64::MAX).to_string(), "(-9223372036854775808, 9223372036854775807)");
        assert_eq!(Point::min().to_string(), "(-9223372036854775808, -9223372036854775808)");
        assert_eq!(Point::max().to_string(), "(9223372036854775807, 9223372036854775807)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_i8::Point::min()), Point { x: i8::MIN.into(), y: i8::MIN.into() });
        assert_eq!(Point::from(point_i8::Point::max()), Point { x: i8::MAX.into(), y: i8::MAX.into() });
        assert_eq!(Point::from(point_i16::Point::min()), Point { x: i16::MIN.into(), y: i16::MIN.into() });
        assert_eq!(Point::from(point_i16::Point::max()), Point { x: i16::MAX.into(), y: i16::MAX.into() });
        assert_eq!(Point::from(point_i32::Point::min()), Point { x: i32::MIN.into(), y: i32::MIN.into() });
        assert_eq!(Point::from(point_i32::Point::max()), Point { x: i32::MAX.into(), y: i32::MAX.into() });
    }
}
