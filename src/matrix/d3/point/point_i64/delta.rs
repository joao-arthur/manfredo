use super::Point;
use crate::matrix::d3::point::point_u64;

pub fn delta_row(p1: &Point, p2: &Point) -> u64 {
    (i128::from(p2.row) - i128::from(p1.row)).unsigned_abs() as u64
}

pub fn delta_col(p1: &Point, p2: &Point) -> u64 {
    (i128::from(p2.col) - i128::from(p1.col)).unsigned_abs() as u64
}

pub fn delta_depth(p1: &Point, p2: &Point) -> u64 {
    (i128::from(p2.depth) - i128::from(p1.depth)).unsigned_abs() as u64
}

pub fn delta_min(p1: &Point, p2: &Point) -> u64 {
    delta_row(p1, p2).min(delta_col(p1, p2)).min(delta_depth(p1, p2))
}

pub fn delta_max(p1: &Point, p2: &Point) -> u64 {
    delta_row(p1, p2).max(delta_col(p1, p2)).max(delta_depth(p1, p2))
}

pub fn delta(p1: &Point, p2: &Point) -> point_u64::Point {
    point_u64::Point { row: delta_row(p1, p2), col: delta_col(p1, p2), depth: delta_depth(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{delta, delta_col, delta_depth, delta_max, delta_min, delta_row};
    use crate::matrix::{
        d1::point::point_i64::{MAX, MIN},
        d3::point::{point_i64::Point, point_u64},
    };

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&Point::of(MIN, 0, 0), &Point::of(MAX, 0, 0)), u64::MAX);
        assert_eq!(delta_row(&Point::of(0, MIN, 0), &Point::of(0, MAX, 0)), 0);
        assert_eq!(delta_row(&Point::of(0, 0, MIN), &Point::of(0, 0, MAX)), 0);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&Point::of(MIN, 0, 0), &Point::of(MAX, 0, 0)), 0);
        assert_eq!(delta_col(&Point::of(0, MIN, 0), &Point::of(0, MAX, 0)), u64::MAX);
        assert_eq!(delta_col(&Point::of(0, 0, MIN), &Point::of(0, 0, MAX)), 0);
    }

    #[test]
    fn test_delta_depth() {
        assert_eq!(delta_depth(&Point::of(MIN, 0, 0), &Point::of(MAX, 0, 0)), 0);
        assert_eq!(delta_depth(&Point::of(0, MIN, 0), &Point::of(0, MAX, 0)), 0);
        assert_eq!(delta_depth(&Point::of(0, 0, MIN), &Point::of(0, 0, MAX)), u64::MAX);
    }

    #[test]
    fn test_delta_max() {
        assert_eq!(delta_max(&Point::of(0, 1, 2), &Point::of(10, 10, 10)), 10);
        assert_eq!(delta_max(&Point::of(1, 2, 0), &Point::of(9, 9, 9)), 9);
        assert_eq!(delta_max(&Point::of(2, 0, 1), &Point::of(8, 8, 8)), 8);
    }

    #[test]
    fn test_delta_min() {
        assert_eq!(delta_min(&Point::of(0, 1, 2), &Point::of(10, 10, 10)), 8);
        assert_eq!(delta_min(&Point::of(1, 2, 0), &Point::of(9, 9, 9)), 7);
        assert_eq!(delta_min(&Point::of(2, 0, 1), &Point::of(8, 8, 8)), 6);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::zero(), &Point::zero()), point_u64::Point::min());
        assert_eq!(delta(&Point::min(), &Point::max()), point_u64::Point::max());
    }
}
