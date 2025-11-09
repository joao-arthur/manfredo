use super::Point;
use crate::cartesian::d4::point::point_u16;

pub fn delta_x(p1: &Point, p2: &Point) -> u16 {
    (i32::from(p2.x) - i32::from(p1.x)).unsigned_abs() as u16
}

pub fn delta_y(p1: &Point, p2: &Point) -> u16 {
    (i32::from(p2.y) - i32::from(p1.y)).unsigned_abs() as u16
}

pub fn delta_z(p1: &Point, p2: &Point) -> u16 {
    (i32::from(p2.z) - i32::from(p1.z)).unsigned_abs() as u16
}

pub fn delta_w(p1: &Point, p2: &Point) -> u16 {
    (i32::from(p2.w) - i32::from(p1.w)).unsigned_abs() as u16
}

pub fn delta_min(p1: &Point, p2: &Point) -> u16 {
    delta_x(p1, p2).min(delta_y(p1, p2)).min(delta_z(p1, p2)).min(delta_w(p1, p2))
}

pub fn delta_max(p1: &Point, p2: &Point) -> u16 {
    delta_x(p1, p2).max(delta_y(p1, p2)).max(delta_z(p1, p2)).max(delta_w(p1, p2))
}

pub fn delta(p1: &Point, p2: &Point) -> point_u16::Point {
    point_u16::Point { x: delta_x(p1, p2), y: delta_y(p1, p2), z: delta_z(p1, p2), w: delta_w(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{delta, delta_max, delta_min, delta_w, delta_x, delta_y, delta_z};
    use crate::cartesian::{
        d1::point::point_i16::{MAX, MIN},
        d4::point::{point_i16::Point, point_u16},
    };

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Point::new(MIN, 0, 0, 0), &Point::new(MAX, 0, 0, 0)), u16::MAX);
        assert_eq!(delta_x(&Point::new(0, MIN, 0, 0), &Point::new(0, MAX, 0, 0)), 0);
        assert_eq!(delta_x(&Point::new(0, 0, MIN, 0), &Point::new(0, 0, MAX, 0)), 0);
        assert_eq!(delta_x(&Point::new(0, 0, 0, MIN), &Point::new(0, 0, 0, MAX)), 0);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Point::new(MIN, 0, 0, 0), &Point::new(MAX, 0, 0, 0)), 0);
        assert_eq!(delta_y(&Point::new(0, MIN, 0, 0), &Point::new(0, MAX, 0, 0)), u16::MAX);
        assert_eq!(delta_y(&Point::new(0, 0, MIN, 0), &Point::new(0, 0, MAX, 0)), 0);
        assert_eq!(delta_y(&Point::new(0, 0, 0, MIN), &Point::new(0, 0, 0, MAX)), 0);
    }

    #[test]
    fn test_delta_z() {
        assert_eq!(delta_z(&Point::new(MIN, 0, 0, 0), &Point::new(MAX, 0, 0, 0)), 0);
        assert_eq!(delta_z(&Point::new(0, MIN, 0, 0), &Point::new(0, MAX, 0, 0)), 0);
        assert_eq!(delta_z(&Point::new(0, 0, MIN, 0), &Point::new(0, 0, MAX, 0)), u16::MAX);
        assert_eq!(delta_z(&Point::new(0, 0, 0, MIN), &Point::new(0, 0, 0, MAX)), 0);
    }

    #[test]
    fn test_delta_w() {
        assert_eq!(delta_w(&Point::new(MIN, 0, 0, 0), &Point::new(MAX, 0, 0, 0)), 0);
        assert_eq!(delta_w(&Point::new(0, MIN, 0, 0), &Point::new(0, MAX, 0, 0)), 0);
        assert_eq!(delta_w(&Point::new(0, 0, MIN, 0), &Point::new(0, 0, MAX, 0)), 0);
        assert_eq!(delta_w(&Point::new(0, 0, 0, MIN), &Point::new(0, 0, 0, MAX)), u16::MAX);
    }

    #[test]
    fn test_delta_max() {
        assert_eq!(delta_max(&Point::new(0, 1, 2, 3), &Point::new(10, 10, 10, 10)), 10);
        assert_eq!(delta_max(&Point::new(1, 2, 3, 0), &Point::new(9, 9, 9, 9)), 9);
        assert_eq!(delta_max(&Point::new(2, 3, 0, 1), &Point::new(8, 8, 8, 8)), 8);
        assert_eq!(delta_max(&Point::new(3, 0, 1, 2), &Point::new(7, 7, 7, 7)), 7);
    }

    #[test]
    fn test_delta_min() {
        assert_eq!(delta_min(&Point::new(0, 1, 2, 3), &Point::new(10, 10, 10, 10)), 7);
        assert_eq!(delta_min(&Point::new(1, 2, 3, 0), &Point::new(9, 9, 9, 9)), 6);
        assert_eq!(delta_min(&Point::new(2, 3, 0, 1), &Point::new(8, 8, 8, 8)), 5);
        assert_eq!(delta_min(&Point::new(3, 0, 1, 1), &Point::new(7, 7, 7, 7)), 4);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::zero(), &Point::zero()), point_u16::Point::min());
        assert_eq!(delta(&Point::min(), &Point::max()), point_u16::Point::max());
    }
}
