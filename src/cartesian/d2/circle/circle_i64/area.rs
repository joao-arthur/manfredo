use super::Circle;
use std::f64;

pub fn area(c: &Circle) -> f64 {
    f64::consts::PI * c.r as f64 * c.r as f64
}

#[cfg(test)]
mod tests {
    use super::area;
    use crate::cartesian::d2::{circle::circle_i64::Circle, point::point_i64::Point};
    use std::f64::consts::PI;

    #[test]
    fn test_area() {
        assert_eq!(area(&Circle::new(Point::zero(), 2)), 4.0 * PI);
        assert_eq!(area(&Circle::new(Point::zero(), 3)), 9.0 * PI);
        assert_eq!(area(&Circle::new(Point::zero(), 4)), 16.0 * PI);
        assert_eq!(area(&Circle::new(Point::zero(), 5)), 25.0 * PI);
    }

    #[test]
    fn area_powers_of_10() {
        assert_eq!(area(&Circle::new(Point::zero(), 1)), PI);
        assert_eq!(area(&Circle::new(Point::zero(), 10)), 100.0 * PI);
        assert_eq!(area(&Circle::new(Point::zero(), 100)), 10000.0 * PI);
        assert_eq!(area(&Circle::new(Point::zero(), 1000)), 1000000.0 * PI);
        assert_eq!(area(&Circle::new(Point::zero(), 10000)), 314159265.35897932);
        assert_eq!(area(&Circle::new(Point::zero(), 100000)), 31_415_926_535.897_93);
        assert_eq!(area(&Circle::new(Point::zero(), 1000000)), 3_141_592_653_589.793);
        assert_eq!(area(&Circle::new(Point::zero(), 10000000)), 314_159_265_358_979.3);
    }

    #[test]
    fn area_bounds() {
        assert_eq!(area(&Circle::new(Point::zero(), 0)), 0.0);
        assert_eq!(area(&Circle::new(Point::zero(), u64::MAX)), 1.0690285840649667e39);
    }
}
