use std::f64;

use super::Circle;

pub fn area(c: &Circle) -> f64 {
    f64::consts::PI * c.r as f64 * c.r as f64
}

#[cfg(test)]
mod tests {
    use super::area;
    use crate::cartesian::d2::{circle::circle_i8::Circle, point::point_i8::Point};
    use std::f64::consts::PI;

    #[test]
    fn test_area() {
        assert_eq!(area(&Circle::of(Point::min(), 2)), 4.0 * PI);
        assert_eq!(area(&Circle::of(Point::min(), 3)), 9.0 * PI);
        assert_eq!(area(&Circle::of(Point::min(), 4)), 16.0 * PI);
        assert_eq!(area(&Circle::of(Point::min(), 5)), 25.0 * PI);
    }

    #[test]
    fn area_powers_of_10() {
        assert_eq!(area(&Circle::of(Point::min(), 1)), PI);
        assert_eq!(area(&Circle::of(Point::min(), 10)), 100.0 * PI);
        assert_eq!(area(&Circle::of(Point::min(), 100)), 10000.0 * PI);
    }

    #[test]
    fn area_bounds() {
        assert_eq!(area(&Circle::of(Point::zero(), 0)), 0.0);
        assert_eq!(area(&Circle::of(Point::zero(), u8::MAX)), 204282.0622996763);
    }
}
