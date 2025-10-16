use super::{point_i8, point_i16, point_i32};

pub const MIN: i64 = i64::MIN;
pub const MAX: i64 = i64::MAX;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point {
    pub fn of(x: i64, y: i64, z: i64) -> Self {
        Point { x, y, z }
    }

    pub fn min() -> Self {
        Point { x: MIN, y: MIN, z: MIN }
    }

    pub fn max() -> Self {
        Point { x: MAX, y: MAX, z: MAX }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<point_i8::Point> for Point {
    fn from(p: point_i8::Point) -> Self {
        Point { x: p.x.into(), y: p.y.into(), z: p.z.into() }
    }
}

impl From<point_i16::Point> for Point {
    fn from(p: point_i16::Point) -> Self {
        Point { x: p.x.into(), y: p.y.into(), z: p.z.into() }
    }
}

impl From<point_i32::Point> for Point {
    fn from(p: point_i32::Point) -> Self {
        Point { x: p.x.into(), y: p.y.into(), z: p.z.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX, MIN, Point};
    use crate::cartesian::d3::point::{point_i8, point_i16, point_i32};

    #[test]
    fn point() {
        assert_eq!(Point::of(MIN, MAX, MIN), Point { x: MIN, y: MAX, z: MIN });
        assert_eq!(Point::of(MAX, MIN, MAX), Point { x: MAX, y: MIN, z: MAX });
        assert_eq!(Point::min(), Point { x: MIN, y: MIN, z: MIN });
        assert_eq!(Point::max(), Point { x: MAX, y: MAX, z: MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(MIN, MAX, MIN).to_string(), "(-9223372036854775808, 9223372036854775807, -9223372036854775808)");
        assert_eq!(Point::of(MAX, MIN, MAX).to_string(), "(9223372036854775807, -9223372036854775808, 9223372036854775807)");
        assert_eq!(Point::min().to_string(), "(-9223372036854775808, -9223372036854775808, -9223372036854775808)");
        assert_eq!(Point::max().to_string(), "(9223372036854775807, 9223372036854775807, 9223372036854775807)");
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_i8::Point::min()), Point { x: i8::MIN.into(), y: i8::MIN.into(), z: i8::MIN.into() });
        assert_eq!(Point::from(point_i8::Point::max()), Point { x: i8::MAX.into(), y: i8::MAX.into(), z: i8::MAX.into() });
        assert_eq!(Point::from(point_i16::Point::min()), Point { x: i16::MIN.into(), y: i16::MIN.into(), z: i16::MIN.into() });
        assert_eq!(Point::from(point_i16::Point::max()), Point { x: i16::MAX.into(), y: i16::MAX.into(), z: i16::MAX.into() });
        assert_eq!(Point::from(point_i32::Point::min()), Point { x: i32::MIN.into(), y: i32::MIN.into(), z: i32::MIN.into() });
        assert_eq!(Point::from(point_i32::Point::max()), Point { x: i32::MAX.into(), y: i32::MAX.into(), z: i32::MAX.into() });
    }
}
