use super::Point;
use crate::matrix::d2::point::point_u8;

pub fn delta_row(p1: &Point, p2: &Point) -> u8 {
    (i16::from(p2.row) - i16::from(p1.row)).unsigned_abs() as u8
}

pub fn delta_col(p1: &Point, p2: &Point) -> u8 {
    (i16::from(p2.col) - i16::from(p1.col)).unsigned_abs() as u8
}

pub fn delta_min(p1: &Point, p2: &Point) -> u8 {
    delta_row(p1, p2).min(delta_col(p1, p2))
}

pub fn delta_max(p1: &Point, p2: &Point) -> u8 {
    delta_row(p1, p2).max(delta_col(p1, p2))
}

pub fn delta(p1: &Point, p2: &Point) -> point_u8::Point {
    point_u8::Point { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{delta, delta_col, delta_max, delta_min, delta_row};
    use crate::matrix::{
        d1::point::point_i8::{MAX, MIN},
        d2::point::{point_i8::Point, point_u8},
    };

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&Point::of(MIN, 0), &Point::of(MAX, 0)), u8::MAX);
        assert_eq!(delta_row(&Point::of(0, MIN), &Point::of(0, MAX)), 0);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&Point::of(MIN, 0), &Point::of(MAX, 0)), 0);
        assert_eq!(delta_col(&Point::of(0, MIN), &Point::of(0, MAX)), u8::MAX);
    }

    #[test]
    fn test_delta_max() {
        assert_eq!(delta_max(&Point::of(0, 1), &Point::of(10, 10)), 10);
        assert_eq!(delta_max(&Point::of(1, 0), &Point::of(9, 9)), 9);
    }

    #[test]
    fn test_delta_min() {
        assert_eq!(delta_min(&Point::of(0, 1), &Point::of(10, 10)), 9);
        assert_eq!(delta_min(&Point::of(1, 0), &Point::of(9, 9)), 8);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::of(0, 0), &Point::of(0, 0)), point_u8::Point::min());
        assert_eq!(delta(&Point::min(), &Point::max()), point_u8::Point::max());
    }
}
