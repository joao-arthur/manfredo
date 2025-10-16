use super::{point_i8, point_i16, point_i32};

mod add;
mod delta;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::delta;

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: i64,
}

impl Point {
    pub fn of(x: i64) -> Self {
        Point { x }
    }

    pub fn min() -> Self {
        Point { x: MIN }
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

impl From<point_i8::Point> for Point {
    fn from(p: point_i8::Point) -> Self {
        Point { x: p.x.into() }
    }
}

impl From<point_i16::Point> for Point {
    fn from(p: point_i16::Point) -> Self {
        Point { x: p.x.into() }
    }
}

impl From<point_i32::Point> for Point {
    fn from(p: point_i32::Point) -> Self {
        Point { x: p.x.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX, MIN, Point};
    use crate::cartesian::d1::point::{point_i8, point_i16, point_i32};

    #[test]
    fn point() {
        assert_eq!(Point::of(-10), Point { x: -10 });
        assert_eq!(Point::of(10), Point { x: 10 });
        assert_eq!(Point::min(), Point { x: MIN });
        assert_eq!(Point::max(), Point { x: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(-10).to_string(), "(-10)");
        assert_eq!(Point::of(10).to_string(), "(10)");
        assert_eq!(Point::min().to_string(), "(-9223372036854775808)");
        assert_eq!(Point::max().to_string(), "(9223372036854775807)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_i8::Point::min()), Point { x: i8::MIN.into() });
        assert_eq!(Point::from(point_i8::Point::max()), Point { x: i8::MAX.into() });
        assert_eq!(Point::from(point_i16::Point::min()), Point { x: i16::MIN.into() });
        assert_eq!(Point::from(point_i16::Point::max()), Point { x: i16::MAX.into() });
        assert_eq!(Point::from(point_i32::Point::min()), Point { x: i32::MIN.into() });
        assert_eq!(Point::from(point_i32::Point::max()), Point { x: i32::MAX.into() });
    }
}
