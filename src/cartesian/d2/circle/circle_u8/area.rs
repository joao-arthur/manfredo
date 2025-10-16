use std::f64;

use super::Circle;

pub fn area(c: &Circle) -> f64 {
    f64::consts::PI * c.r as f64 * c.r as f64
}

#[cfg(test)]
mod tests {
    use super::area;
    use crate::cartesian::d2::{circle::circle_u8::Circle, point::point_u8::Point};

    #[test]
    fn test_area() {
        assert_eq!(area(&Circle::of(Point::of(0, 0), 2)), 12.566370614359172);
        assert_eq!(area(&Circle::of(Point::of(0, 0), 3)), 28.274333882308138);
        assert_eq!(area(&Circle::of(Point::of(0, 0), 4)), 50.26548245743669);
        assert_eq!(area(&Circle::of(Point::of(0, 0), 5)), 78.53981633974483);
    }

    #[test]
    fn area_powers_of_10() {
        assert_eq!(area(&Circle::of(Point::of(0, 0), 1)), 3.141_592_653_589_793);
        assert_eq!(area(&Circle::of(Point::of(0, 0), 10)), 314.159_265_358_979_3);
        assert_eq!(area(&Circle::of(Point::of(0, 0), 100)), 31415.926535897932);
    }

    #[test]
    fn area_bounds() {
        assert_eq!(area(&Circle::of(Point::of(0, 0), 0)), 0.0);
        assert_eq!(area(&Circle::of(Point::of(0, 0), u8::MAX)), 204282.0622996763);
    }
}
