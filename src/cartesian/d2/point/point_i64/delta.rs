use super::Point;
use crate::cartesian::d2::point::point_u64;

pub fn delta_x(p1: &Point, p2: &Point) -> u64 {
    (i128::from(p2.x) - i128::from(p1.x)).unsigned_abs() as u64
}

pub fn delta_y(p1: &Point, p2: &Point) -> u64 {
    (i128::from(p2.y) - i128::from(p1.y)).unsigned_abs() as u64
}

pub fn delta_min(p1: &Point, p2: &Point) -> u64 {
    delta_x(p1, p2).min(delta_y(p1, p2))
}

pub fn delta_max(p1: &Point, p2: &Point) -> u64 {
    delta_x(p1, p2).max(delta_y(p1, p2))
}

pub fn delta(p1: &Point, p2: &Point) -> point_u64::Point {
    point_u64::Point { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{delta, delta_max, delta_min, delta_x, delta_y};
    use crate::cartesian::{
        d1::point::point_i64::{MAX, MIN},
        d2::point::{point_i64::Point, point_u64},
    };

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Point::of(MIN, 0), &Point::of(MAX, 0)), u64::MAX);
        assert_eq!(delta_x(&Point::of(0, MIN), &Point::of(0, MAX)), 0);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Point::of(MIN, 0), &Point::of(MAX, 0)), 0);
        assert_eq!(delta_y(&Point::of(0, MIN), &Point::of(0, MAX)), u64::MAX);
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
        assert_eq!(delta(&Point::of(0, 0), &Point::of(0, 0)), point_u64::Point::min());
        assert_eq!(delta(&Point::min(), &Point::max()), point_u64::Point::max());
    }
}
