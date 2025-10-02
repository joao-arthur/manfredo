use crate::matrix::d2::point::{point_i8::Point, point_u8};

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
    use super::{delta, delta_col, delta_row};
    use crate::matrix::d2::point::{point_i8::Point, point_u8};

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
