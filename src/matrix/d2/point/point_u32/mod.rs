use super::{point_u8, point_u16};

mod add;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub row: u32,
    pub col: u32,
}

impl Point {
    pub fn of(row: u32, col: u32) -> Self {
        Point { row, col }
    }

    pub fn min() -> Self {
        Point { row: 0, col: 0 }
    }

    pub fn max() -> Self {
        Point { row: u32::MAX, col: u32::MAX }
    }
}

impl From<point_u8::Point> for Point {
    fn from(p: point_u8::Point) -> Self {
        Point { row: p.row.into(), col: p.col.into() }
    }
}

impl From<point_u16::Point> for Point {
    fn from(p: point_u16::Point) -> Self {
        Point { row: p.row.into(), col: p.col.into() }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub fn delta_row(p1: &Point, p2: &Point) -> u32 {
    p2.row - p1.row
}

pub fn delta_col(p1: &Point, p2: &Point) -> u32 {
    p2.col - p1.col
}

pub fn delta(p1: &Point, p2: &Point) -> Point {
    Point { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{Point, delta, delta_col, delta_row};
    use crate::matrix::point::{point_u8, point_u16};

    #[test]
    fn point_u32() {
        assert_eq!(Point::of(0, u32::MAX), Point { row: 0, col: u32::MAX });
        assert_eq!(Point::min(), Point { row: 0, col: 0 });
        assert_eq!(Point::max(), Point { row: u32::MAX, col: u32::MAX });
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_u8::Point::min()), Point { row: u8::MIN.into(), col: u8::MIN.into() });
        assert_eq!(Point::from(point_u8::Point::max()), Point { row: u8::MAX.into(), col: u8::MAX.into() });
        assert_eq!(Point::from(point_u16::Point::min()), Point { row: u16::MIN.into(), col: u16::MIN.into() });
        assert_eq!(Point::from(point_u16::Point::max()), Point { row: u16::MAX.into(), col: u16::MAX.into() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(0, u32::MAX).to_string(), "(0, 4294967295)");
        assert_eq!(Point::min().to_string(), "(0, 0)");
        assert_eq!(Point::max().to_string(), "(4294967295, 4294967295)");
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&Point::min(), &Point::of(0, u32::MAX)), 0);
        assert_eq!(delta_row(&Point::min(), &Point::of(u32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&Point::min(), &Point::of(u32::MAX, 0)), 0);
        assert_eq!(delta_col(&Point::min(), &Point::of(0, u32::MAX)), u32::MAX);
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
        let p = Point::of(u32::MAX - 2, u32::MAX - 2);
        assert_eq!(delta(&p, &Point::of(u32::MAX - 2, u32::MAX - 2)), Point::min());
        assert_eq!(delta(&p, &Point::of(u32::MAX - 2, u32::MAX - 1)), Point::of(0, 1));
        assert_eq!(delta(&p, &Point::of(u32::MAX - 2, u32::MAX)), Point::of(0, 2));

        assert_eq!(delta(&p, &Point::of(u32::MAX - 1, u32::MAX - 2)), Point::of(1, 0));
        assert_eq!(delta(&p, &Point::of(u32::MAX - 1, u32::MAX - 1)), Point::of(1, 1));
        assert_eq!(delta(&p, &Point::of(u32::MAX - 1, u32::MAX)), Point::of(1, 2));

        assert_eq!(delta(&p, &Point::of(u32::MAX, u32::MAX - 2)), Point::of(2, 0));
        assert_eq!(delta(&p, &Point::of(u32::MAX, u32::MAX - 1)), Point::of(2, 1));
        assert_eq!(delta(&p, &Point::max()), Point::of(2, 2));
    }
}
