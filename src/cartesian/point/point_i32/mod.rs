use super::{point_i8, point_i16, point_u32};

mod add;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};

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

pub fn delta_x(p1: &Point, p2: &Point) -> u32 {
    (i64::from(p2.x) - i64::from(p1.x)).unsigned_abs() as u32
}

pub fn delta_y(p1: &Point, p2: &Point) -> u32 {
    (i64::from(p2.y) - i64::from(p1.y)).unsigned_abs() as u32
}

pub fn delta(p1: &Point, p2: &Point) -> point_u32::Point {
    point_u32::Point { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{Point, delta, delta_x, delta_y};
    use crate::cartesian::point::{point_i8, point_i16, point_u32};

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

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Point::of(0, i32::MIN), &Point::of(0, i32::MAX)), 0);
        assert_eq!(delta_x(&Point::of(i32::MIN, 0), &Point::of(i32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Point::of(i32::MIN, 0), &Point::of(i32::MAX, 0)), 0);
        assert_eq!(delta_y(&Point::of(0, i32::MIN), &Point::of(0, i32::MAX)), u32::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::of(0, 0), &Point::of(0, 0)), point_u32::Point::min());
        assert_eq!(delta(&Point::min(), &Point::max()), point_u32::Point::max());
    }

    #[test]
    fn delta_min() {
        let p = Point::min();
        assert_eq!(delta(&p, &Point::min()), point_u32::Point::min());
        assert_eq!(delta(&p, &Point::of(i32::MIN, i32::MIN + 1)), point_u32::Point::of(0, 1));
        assert_eq!(delta(&p, &Point::of(i32::MIN, i32::MIN + 2)), point_u32::Point::of(0, 2));

        assert_eq!(delta(&p, &Point::of(i32::MIN + 1, i32::MIN)), point_u32::Point::of(1, 0));
        assert_eq!(delta(&p, &Point::of(i32::MIN + 1, i32::MIN + 1)), point_u32::Point::of(1, 1));
        assert_eq!(delta(&p, &Point::of(i32::MIN + 1, i32::MIN + 2)), point_u32::Point::of(1, 2));

        assert_eq!(delta(&p, &Point::of(i32::MIN + 2, i32::MIN)), point_u32::Point::of(2, 0));
        assert_eq!(delta(&p, &Point::of(i32::MIN + 2, i32::MIN + 1)), point_u32::Point::of(2, 1));
        assert_eq!(delta(&p, &Point::of(i32::MIN + 2, i32::MIN + 2)), point_u32::Point::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = Point::of(i32::MAX - 2, i32::MAX - 2);
        assert_eq!(delta(&p, &Point::of(i32::MAX - 2, i32::MAX - 2)), point_u32::Point::min());
        assert_eq!(delta(&p, &Point::of(i32::MAX - 2, i32::MAX - 1)), point_u32::Point::of(0, 1));
        assert_eq!(delta(&p, &Point::of(i32::MAX - 2, i32::MAX)), point_u32::Point::of(0, 2));

        assert_eq!(delta(&p, &Point::of(i32::MAX - 1, i32::MAX - 2)), point_u32::Point::of(1, 0));
        assert_eq!(delta(&p, &Point::of(i32::MAX - 1, i32::MAX - 1)), point_u32::Point::of(1, 1));
        assert_eq!(delta(&p, &Point::of(i32::MAX - 1, i32::MAX)), point_u32::Point::of(1, 2));

        assert_eq!(delta(&p, &Point::of(i32::MAX, i32::MAX - 2)), point_u32::Point::of(2, 0));
        assert_eq!(delta(&p, &Point::of(i32::MAX, i32::MAX - 1)), point_u32::Point::of(2, 1));
        assert_eq!(delta(&p, &Point::max()), point_u32::Point::of(2, 2));
    }
}
