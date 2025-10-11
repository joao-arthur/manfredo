use super::point_i8;

mod add;
mod delta;
mod distance;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::delta::{delta, delta_max, delta_min, delta_x, delta_y};
pub use self::distance::distance;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub fn of(x: i16, y: i16) -> Self {
        Point { x, y }
    }

    pub fn min() -> Self {
        Point { x: i16::MIN, y: i16::MIN }
    }

    pub fn max() -> Self {
        Point { x: i16::MAX, y: i16::MAX }
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

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::cartesian::d2::point::point_i8;

    #[test]
    fn point() {
        assert_eq!(Point::of(i16::MIN, i16::MAX), Point { x: i16::MIN, y: i16::MAX });
        assert_eq!(Point::min(), Point { x: i16::MIN, y: i16::MIN });
        assert_eq!(Point::max(), Point { x: i16::MAX, y: i16::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(i16::MIN, i16::MAX).to_string(), "(-32768, 32767)");
        assert_eq!(Point::min().to_string(), "(-32768, -32768)");
        assert_eq!(Point::max().to_string(), "(32767, 32767)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_i8::Point::min()), Point { x: i8::MIN.into(), y: i8::MIN.into() });
        assert_eq!(Point::from(point_i8::Point::max()), Point { x: i8::MAX.into(), y: i8::MAX.into() });
    }
}
