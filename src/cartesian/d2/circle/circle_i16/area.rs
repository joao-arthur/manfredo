use std::f64;

use super::Circle;

pub fn area(c: &Circle) -> f64 {
    f64::consts::PI * c.r as f64 * c.r as f64
}

#[cfg(test)]
mod tests {
    use super::area;
    use crate::cartesian::d2::{circle::circle_i16::Circle, point::point_i16::Point};
    use std::f64::consts::PI;

    #[test]
    fn test_area() {
        assert_eq!(area(&Circle::of(Point::zero(), 2)), 4.0 * PI);
        assert_eq!(area(&Circle::of(Point::zero(), 3)), 9.0 * PI);
        assert_eq!(area(&Circle::of(Point::zero(), 4)), 16.0 * PI);
        assert_eq!(area(&Circle::of(Point::zero(), 5)), 25.0 * PI);
    }

    #[test]
    fn area_powers_of_10() {
        assert_eq!(area(&Circle::of(Point::zero(), 1)), PI);
        assert_eq!(area(&Circle::of(Point::zero(), 10)), 100.0 * PI);
        assert_eq!(area(&Circle::of(Point::zero(), 100)), 10000.0 * PI);
        assert_eq!(area(&Circle::of(Point::zero(), 1000)), 1000000.0 * PI);
        assert_eq!(area(&Circle::of(Point::zero(), 10000)), 314159265.35897932);
    }

    #[test]
    fn area_bounds() {
        assert_eq!(area(&Circle::of(Point::zero(), 0)), 0.0);
        assert_eq!(area(&Circle::of(Point::zero(), u16::MAX)), 13492625932.83132);
    }
}
