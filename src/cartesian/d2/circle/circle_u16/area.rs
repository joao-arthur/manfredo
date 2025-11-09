use super::Circle;
use std::f64;

pub fn area(c: &Circle) -> f64 {
    f64::consts::PI * c.r as f64 * c.r as f64
}

#[cfg(test)]
mod tests {
    use super::area;
    use crate::cartesian::d2::{circle::circle_u16::Circle, point::point_u16::Point};
    use std::f64::consts::PI;

    #[test]
    fn test_area() {
        assert_eq!(area(&Circle::new(Point::min(), 2)), 4.0 * PI);
        assert_eq!(area(&Circle::new(Point::min(), 3)), 9.0 * PI);
        assert_eq!(area(&Circle::new(Point::min(), 4)), 16.0 * PI);
        assert_eq!(area(&Circle::new(Point::min(), 5)), 25.0 * PI);
    }

    #[test]
    fn area_powers_of_10() {
        assert_eq!(area(&Circle::new(Point::min(), 1)), PI);
        assert_eq!(area(&Circle::new(Point::min(), 10)), 100.0 * PI);
        assert_eq!(area(&Circle::new(Point::min(), 100)), 10000.0 * PI);
        assert_eq!(area(&Circle::new(Point::min(), 1000)), 1000000.0 * PI);
        assert_eq!(area(&Circle::new(Point::min(), 10000)), 314159265.35897932);
    }

    #[test]
    fn area_bounds() {
        assert_eq!(area(&Circle::new(Point::min(), 0)), 0.0);
        assert_eq!(area(&Circle::new(Point::min(), u16::MAX)), 13492625932.83132);
    }
}
