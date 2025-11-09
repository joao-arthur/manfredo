use super::Point;
use crate::cartesian::d2::point::point_u8;

pub fn delta_x(p1: &Point, p2: &Point) -> u8 {
    (i16::from(p2.x) - i16::from(p1.x)).unsigned_abs() as u8
}

pub fn delta_y(p1: &Point, p2: &Point) -> u8 {
    (i16::from(p2.y) - i16::from(p1.y)).unsigned_abs() as u8
}

pub fn delta_min(p1: &Point, p2: &Point) -> u8 {
    delta_x(p1, p2).min(delta_y(p1, p2))
}

pub fn delta_max(p1: &Point, p2: &Point) -> u8 {
    delta_x(p1, p2).max(delta_y(p1, p2))
}

pub fn delta(p1: &Point, p2: &Point) -> point_u8::Point {
    point_u8::Point { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{delta, delta_max, delta_min, delta_x, delta_y};
    use crate::cartesian::{
        d1::point::point_i8::{MAX, MIN},
        d2::point::{point_i8::Point, point_u8},
    };

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Point::new(MIN, 0), &Point::new(MAX, 0)), u8::MAX);
        assert_eq!(delta_x(&Point::new(0, MIN), &Point::new(0, MAX)), 0);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Point::new(MIN, 0), &Point::new(MAX, 0)), 0);
        assert_eq!(delta_y(&Point::new(0, MIN), &Point::new(0, MAX)), u8::MAX);
    }

    #[test]
    fn test_delta_max() {
        assert_eq!(delta_max(&Point::new(0, 1), &Point::new(10, 10)), 10);
        assert_eq!(delta_max(&Point::new(1, 0), &Point::new(9, 9)), 9);
    }

    #[test]
    fn test_delta_min() {
        assert_eq!(delta_min(&Point::new(0, 1), &Point::new(10, 10)), 9);
        assert_eq!(delta_min(&Point::new(1, 0), &Point::new(9, 9)), 8);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::zero(), &Point::zero()), point_u8::Point::min());
        assert_eq!(delta(&Point::min(), &Point::max()), point_u8::Point::max());
    }
}
