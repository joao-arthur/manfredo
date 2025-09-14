use super::point_u8;

mod add;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}

impl Point {
    pub fn of(x: i8, y: i8) -> Self {
        Point { x, y }
    }

    pub fn min() -> Self {
        Point { x: i8::MIN, y: i8::MIN }
    }

    pub fn max() -> Self {
        Point { x: i8::MAX, y: i8::MAX }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &Point, p2: &Point) -> u8 {
    (i16::from(p2.x) - i16::from(p1.x)).unsigned_abs() as u8
}

pub fn delta_y(p1: &Point, p2: &Point) -> u8 {
    (i16::from(p2.y) - i16::from(p1.y)).unsigned_abs() as u8
}

pub fn delta(p1: &Point, p2: &Point) -> point_u8::Point {
    point_u8::Point { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{Point, delta, delta_x, delta_y};
    use crate::cartesian::d2::point::point_u8;

    #[test]
    fn point_i8() {
        assert_eq!(Point::of(i8::MIN, i8::MAX), Point { x: i8::MIN, y: i8::MAX });
        assert_eq!(Point::min(), Point { x: i8::MIN, y: i8::MIN });
        assert_eq!(Point::max(), Point { x: i8::MAX, y: i8::MAX });
        assert_eq!(Point::of(i8::MIN, i8::MAX).to_string(), "(-128, 127)");
        assert_eq!(Point::min().to_string(), "(-128, -128)");
        assert_eq!(Point::max().to_string(), "(127, 127)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Point::of(0, i8::MIN), &Point::of(0, i8::MAX)), 0);
        assert_eq!(delta_x(&Point::of(i8::MIN, 0), &Point::of(i8::MAX, 0)), u8::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Point::of(i8::MIN, 0), &Point::of(i8::MAX, 0)), 0);
        assert_eq!(delta_y(&Point::of(0, i8::MIN), &Point::of(0, i8::MAX)), u8::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::of(0, 0), &Point::of(0, 0)), point_u8::Point::min());
        assert_eq!(delta(&Point::min(), &Point::max()), point_u8::Point::max());
    }

    #[test]
    fn delta_min() {
        let p = Point::min();
        assert_eq!(delta(&p, &Point::min()), point_u8::Point::min());
        assert_eq!(delta(&p, &Point::of(i8::MIN, i8::MIN + 1)), point_u8::Point::of(0, 1));
        assert_eq!(delta(&p, &Point::of(i8::MIN, i8::MIN + 2)), point_u8::Point::of(0, 2));

        assert_eq!(delta(&p, &Point::of(i8::MIN + 1, i8::MIN)), point_u8::Point::of(1, 0));
        assert_eq!(delta(&p, &Point::of(i8::MIN + 1, i8::MIN + 1)), point_u8::Point::of(1, 1));
        assert_eq!(delta(&p, &Point::of(i8::MIN + 1, i8::MIN + 2)), point_u8::Point::of(1, 2));

        assert_eq!(delta(&p, &Point::of(i8::MIN + 2, i8::MIN)), point_u8::Point::of(2, 0));
        assert_eq!(delta(&p, &Point::of(i8::MIN + 2, i8::MIN + 1)), point_u8::Point::of(2, 1));
        assert_eq!(delta(&p, &Point::of(i8::MIN + 2, i8::MIN + 2)), point_u8::Point::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = Point::of(i8::MAX - 2, i8::MAX - 2);
        assert_eq!(delta(&p, &Point::of(i8::MAX - 2, i8::MAX - 2)), point_u8::Point::min());
        assert_eq!(delta(&p, &Point::of(i8::MAX - 2, i8::MAX - 1)), point_u8::Point::of(0, 1));
        assert_eq!(delta(&p, &Point::of(i8::MAX - 2, i8::MAX)), point_u8::Point::of(0, 2));

        assert_eq!(delta(&p, &Point::of(i8::MAX - 1, i8::MAX - 2)), point_u8::Point::of(1, 0));
        assert_eq!(delta(&p, &Point::of(i8::MAX - 1, i8::MAX - 1)), point_u8::Point::of(1, 1));
        assert_eq!(delta(&p, &Point::of(i8::MAX - 1, i8::MAX)), point_u8::Point::of(1, 2));

        assert_eq!(delta(&p, &Point::of(i8::MAX, i8::MAX - 2)), point_u8::Point::of(2, 0));
        assert_eq!(delta(&p, &Point::of(i8::MAX, i8::MAX - 1)), point_u8::Point::of(2, 1));
        assert_eq!(delta(&p, &Point::max()), point_u8::Point::of(2, 2));
    }
}
