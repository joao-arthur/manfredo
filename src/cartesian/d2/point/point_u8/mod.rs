mod add;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

impl Point {
    pub fn of(x: u8, y: u8) -> Self {
        Point { x, y }
    }

    pub fn min() -> Self {
        Point { x: 0, y: 0 }
    }

    pub fn max() -> Self {
        Point { x: u8::MAX, y: u8::MAX }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &Point, p2: &Point) -> u8 {
    p2.x - p1.x
}

pub fn delta_y(p1: &Point, p2: &Point) -> u8 {
    p2.y - p1.y
}

pub fn delta(p1: &Point, p2: &Point) -> Point {
    Point { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{Point, delta, delta_x, delta_y};

    #[test]
    fn point_u8() {
        assert_eq!(Point::of(0, u8::MAX), Point { x: 0, y: u8::MAX });
        assert_eq!(Point::min(), Point { x: 0, y: 0 });
        assert_eq!(Point::max(), Point { x: u8::MAX, y: u8::MAX });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(0, u8::MAX).to_string(), "(0, 255)");
        assert_eq!(Point::min().to_string(), "(0, 0)");
        assert_eq!(Point::max().to_string(), "(255, 255)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Point::min(), &Point::of(0, u8::MAX)), 0);
        assert_eq!(delta_x(&Point::min(), &Point::of(u8::MAX, 0)), u8::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Point::min(), &Point::of(u8::MAX, 0)), 0);
        assert_eq!(delta_y(&Point::min(), &Point::of(0, u8::MAX)), u8::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::min(), &Point::min()), Point::min());
        assert_eq!(delta(&Point::min(), &Point::max()), Point::max());
    }

    #[test]
    fn delta_min() {
        let p = Point::min();
        assert_eq!(delta(&p, &Point::min()), Point::min());
        assert_eq!(delta(&p, &Point::of(0, 1)), Point::of(0, 1));
        assert_eq!(delta(&p, &Point::of(0, 2)), Point::of(0, 2));

        assert_eq!(delta(&p, &Point::of(1, 0)), Point::of(1, 0));
        assert_eq!(delta(&p, &Point::of(1, 1)), Point::of(1, 1));
        assert_eq!(delta(&p, &Point::of(1, 2)), Point::of(1, 2));

        assert_eq!(delta(&p, &Point::of(2, 0)), Point::of(2, 0));
        assert_eq!(delta(&p, &Point::of(2, 1)), Point::of(2, 1));
        assert_eq!(delta(&p, &Point::of(2, 2)), Point::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = Point::of(u8::MAX - 2, u8::MAX - 2);
        assert_eq!(delta(&p, &Point::of(u8::MAX - 2, u8::MAX - 2)), Point::min());
        assert_eq!(delta(&p, &Point::of(u8::MAX - 2, u8::MAX - 1)), Point::of(0, 1));
        assert_eq!(delta(&p, &Point::of(u8::MAX - 2, u8::MAX)), Point::of(0, 2));

        assert_eq!(delta(&p, &Point::of(u8::MAX - 1, u8::MAX - 2)), Point::of(1, 0));
        assert_eq!(delta(&p, &Point::of(u8::MAX - 1, u8::MAX - 1)), Point::of(1, 1));
        assert_eq!(delta(&p, &Point::of(u8::MAX - 1, u8::MAX)), Point::of(1, 2));

        assert_eq!(delta(&p, &Point::of(u8::MAX, u8::MAX - 2)), Point::of(2, 0));
        assert_eq!(delta(&p, &Point::of(u8::MAX, u8::MAX - 1)), Point::of(2, 1));
        assert_eq!(delta(&p, &Point::max()), Point::of(2, 2));
    }
}
