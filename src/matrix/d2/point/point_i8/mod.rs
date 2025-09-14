use super::point_u8;

mod add;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Point {
    pub row: i8,
    pub col: i8,
}

impl Point {
    pub fn of(row: i8, col: i8) -> Self {
        Point { row, col }
    }

    pub fn min() -> Self {
        Point { row: i8::MIN, col: i8::MIN }
    }

    pub fn max() -> Self {
        Point { row: i8::MAX, col: i8::MAX }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub fn delta_row(p1: &Point, p2: &Point) -> u8 {
    (i16::from(p2.row) - i16::from(p1.row)).unsigned_abs() as u8
}

pub fn delta_col(p1: &Point, p2: &Point) -> u8 {
    (i16::from(p2.col) - i16::from(p1.col)).unsigned_abs() as u8
}

pub fn delta(p1: &Point, p2: &Point) -> point_u8::Point {
    point_u8::Point { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{Point, delta, delta_col, delta_row};
    use crate::matrix::d2::point::point_u8;

    #[test]
    fn point_i8() {
        assert_eq!(Point::of(i8::MIN, i8::MAX), Point { row: i8::MIN, col: i8::MAX });
        assert_eq!(Point::min(), Point { row: i8::MIN, col: i8::MIN });
        assert_eq!(Point::max(), Point { row: i8::MAX, col: i8::MAX });
        assert_eq!(Point::of(i8::MIN, i8::MAX).to_string(), "(-128, 127)");
        assert_eq!(Point::min().to_string(), "(-128, -128)");
        assert_eq!(Point::max().to_string(), "(127, 127)");
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&Point::of(0, i8::MIN), &Point::of(0, i8::MAX)), 0);
        assert_eq!(delta_row(&Point::of(i8::MIN, 0), &Point::of(i8::MAX, 0)), u8::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&Point::of(i8::MIN, 0), &Point::of(i8::MAX, 0)), 0);
        assert_eq!(delta_col(&Point::of(0, i8::MIN), &Point::of(0, i8::MAX)), u8::MAX);
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
