use super::Point;
use crate::cartesian::d4::point::point_u8;

pub fn delta_x(p1: &Point, p2: &Point) -> u8 {
    (i16::from(p2.x) - i16::from(p1.x)).unsigned_abs() as u8
}

pub fn delta_y(p1: &Point, p2: &Point) -> u8 {
    (i16::from(p2.y) - i16::from(p1.y)).unsigned_abs() as u8
}

pub fn delta_z(p1: &Point, p2: &Point) -> u8 {
    (i16::from(p2.z) - i16::from(p1.z)).unsigned_abs() as u8
}

pub fn delta_w(p1: &Point, p2: &Point) -> u8 {
    (i16::from(p2.w) - i16::from(p1.w)).unsigned_abs() as u8
}

pub fn delta_min(p1: &Point, p2: &Point) -> u8 {
    delta_x(p1, p2).min(delta_y(p1, p2)).min(delta_z(p1, p2)).min(delta_w(p1, p2))
}

pub fn delta_max(p1: &Point, p2: &Point) -> u8 {
    delta_x(p1, p2).max(delta_y(p1, p2)).max(delta_z(p1, p2)).max(delta_w(p1, p2))
}

pub fn delta(p1: &Point, p2: &Point) -> point_u8::Point {
    point_u8::Point { x: delta_x(p1, p2), y: delta_y(p1, p2), z: delta_z(p1, p2), w: delta_w(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{delta, delta_max, delta_min, delta_w, delta_x, delta_y, delta_z};
    use crate::cartesian::{
        d1::point::point_i8::{MAX, MIN},
        d4::point::{point_i8::Point, point_u8},
    };

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Point::of(MIN, 0, 0, 0), &Point::of(MAX, 0, 0, 0)), u8::MAX);
        assert_eq!(delta_x(&Point::of(0, MIN, 0, 0), &Point::of(0, MAX, 0, 0)), 0);
        assert_eq!(delta_x(&Point::of(0, 0, MIN, 0), &Point::of(0, 0, MAX, 0)), 0);
        assert_eq!(delta_x(&Point::of(0, 0, 0, MIN), &Point::of(0, 0, 0, MAX)), 0);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Point::of(MIN, 0, 0, 0), &Point::of(MAX, 0, 0, 0)), 0);
        assert_eq!(delta_y(&Point::of(0, MIN, 0, 0), &Point::of(0, MAX, 0, 0)), u8::MAX);
        assert_eq!(delta_y(&Point::of(0, 0, MIN, 0), &Point::of(0, 0, MAX, 0)), 0);
        assert_eq!(delta_y(&Point::of(0, 0, 0, MIN), &Point::of(0, 0, 0, MAX)), 0);
    }

    #[test]
    fn test_delta_z() {
        assert_eq!(delta_z(&Point::of(MIN, 0, 0, 0), &Point::of(MAX, 0, 0, 0)), 0);
        assert_eq!(delta_z(&Point::of(0, MIN, 0, 0), &Point::of(0, MAX, 0, 0)), 0);
        assert_eq!(delta_z(&Point::of(0, 0, MIN, 0), &Point::of(0, 0, MAX, 0)), u8::MAX);
        assert_eq!(delta_z(&Point::of(0, 0, 0, MIN), &Point::of(0, 0, 0, MAX)), 0);
    }

    #[test]
    fn test_delta_w() {
        assert_eq!(delta_w(&Point::of(MIN, 0, 0, 0), &Point::of(MAX, 0, 0, 0)), 0);
        assert_eq!(delta_w(&Point::of(0, MIN, 0, 0), &Point::of(0, MAX, 0, 0)), 0);
        assert_eq!(delta_w(&Point::of(0, 0, MIN, 0), &Point::of(0, 0, MAX, 0)), 0);
        assert_eq!(delta_w(&Point::of(0, 0, 0, MIN), &Point::of(0, 0, 0, MAX)), u8::MAX);
    }

    #[test]
    fn test_delta_max() {
        assert_eq!(delta_max(&Point::of(0, 1, 2, 3), &Point::of(10, 10, 10, 10)), 10);
        assert_eq!(delta_max(&Point::of(1, 2, 3, 0), &Point::of(9, 9, 9, 9)), 9);
        assert_eq!(delta_max(&Point::of(2, 3, 0, 1), &Point::of(8, 8, 8, 8)), 8);
        assert_eq!(delta_max(&Point::of(3, 0, 1, 2), &Point::of(7, 7, 7, 7)), 7);
    }

    #[test]
    fn test_delta_min() {
        assert_eq!(delta_min(&Point::of(0, 1, 2, 3), &Point::of(10, 10, 10, 10)), 7);
        assert_eq!(delta_min(&Point::of(1, 2, 3, 0), &Point::of(9, 9, 9, 9)), 6);
        assert_eq!(delta_min(&Point::of(2, 3, 0, 1), &Point::of(8, 8, 8, 8)), 5);
        assert_eq!(delta_min(&Point::of(3, 0, 1, 1), &Point::of(7, 7, 7, 7)), 4);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::zero(), &Point::zero()), point_u8::Point::min());
        assert_eq!(delta(&Point::min(), &Point::max()), point_u8::Point::max());
    }
}
