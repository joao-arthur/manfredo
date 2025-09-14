use super::{point_i8, point_i16, point_u32};

mod add;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub row: i32,
    pub col: i32,
}

impl Point {
    pub fn of(row: i32, col: i32) -> Self {
        Point { row, col }
    }

    pub fn min() -> Self {
        Point { row: i32::MIN, col: i32::MIN }
    }

    pub fn max() -> Self {
        Point { row: i32::MAX, col: i32::MAX }
    }
}

impl From<point_i8::Point> for Point {
    fn from(p: point_i8::Point) -> Self {
        Point { row: p.row.into(), col: p.col.into() }
    }
}

impl From<point_i16::Point> for Point {
    fn from(p: point_i16::Point) -> Self {
        Point { row: p.row.into(), col: p.col.into() }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub fn delta_row(p1: &Point, p2: &Point) -> u32 {
    (i64::from(p2.row) - i64::from(p1.row)).unsigned_abs() as u32
}

pub fn delta_col(p1: &Point, p2: &Point) -> u32 {
    (i64::from(p2.col) - i64::from(p1.col)).unsigned_abs() as u32
}

pub fn delta(p1: &Point, p2: &Point) -> point_u32::Point {
    point_u32::Point { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{Point, delta, delta_col, delta_row};
    use crate::matrix::d2::point::{point_i8, point_i16, point_u32};

    #[test]
    fn point_i32() {
        assert_eq!(Point::of(i32::MIN, i32::MAX), Point { row: i32::MIN, col: i32::MAX });
        assert_eq!(Point::min(), Point { row: i32::MIN, col: i32::MIN });
        assert_eq!(Point::max(), Point { row: i32::MAX, col: i32::MAX });
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_i8::Point::min()), Point { row: i8::MIN.into(), col: i8::MIN.into() });
        assert_eq!(Point::from(point_i8::Point::max()), Point { row: i8::MAX.into(), col: i8::MAX.into() });
        assert_eq!(Point::from(point_i16::Point::min()), Point { row: i16::MIN.into(), col: i16::MIN.into() });
        assert_eq!(Point::from(point_i16::Point::max()), Point { row: i16::MAX.into(), col: i16::MAX.into() });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(i32::MIN, i32::MAX).to_string(), "(-2147483648, 2147483647)");
        assert_eq!(Point::min().to_string(), "(-2147483648, -2147483648)");
        assert_eq!(Point::max().to_string(), "(2147483647, 2147483647)");
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&Point::of(0, i32::MIN), &Point::of(0, i32::MAX)), 0);
        assert_eq!(delta_row(&Point::of(i32::MIN, 0), &Point::of(i32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&Point::of(i32::MIN, 0), &Point::of(i32::MAX, 0)), 0);
        assert_eq!(delta_col(&Point::of(0, i32::MIN), &Point::of(0, i32::MAX)), u32::MAX);
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
