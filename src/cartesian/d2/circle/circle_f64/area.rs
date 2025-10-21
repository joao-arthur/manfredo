use super::Circle;
use std::f64;

pub fn area(c: &Circle) -> f64 {
    f64::consts::PI * c.r * c.r
}

#[cfg(test)]
mod tests {
    use super::area;
    use crate::cartesian::{
        d1::point::point_f64::MAX,
        d2::{circle::circle_f64::Circle, point::point_f64::Point},
    };
    use std::f64::consts::PI;

    #[test]
    fn test_area() {
        assert_eq!(area(&Circle::of(Point::zero(), 2.0)), 4.0 * PI);
        assert_eq!(area(&Circle::of(Point::zero(), 3.0)), 9.0 * PI);
        assert_eq!(area(&Circle::of(Point::zero(), 4.0)), 16.0 * PI);
        assert_eq!(area(&Circle::of(Point::zero(), 5.0)), 25.0 * PI);
    }

    #[test]
    fn area_powers_of_10() {
        assert_eq!(area(&Circle::of(Point::zero(), 1.0)), PI);
        assert_eq!(area(&Circle::of(Point::zero(), 10.0)), 100.0 * PI);
        assert_eq!(area(&Circle::of(Point::zero(), 100.0)), 10000.0 * PI);
        assert_eq!(area(&Circle::of(Point::zero(), 1000.0)), 1000000.0 * PI);
        assert_eq!(area(&Circle::of(Point::zero(), 10000.0)), 314159265.35897932);
        assert_eq!(area(&Circle::of(Point::zero(), 100000.0)), 31_415_926_535.897_93);
        assert_eq!(area(&Circle::of(Point::zero(), 1000000.0)), 3_141_592_653_589.793);
        assert_eq!(area(&Circle::of(Point::zero(), 10000000.0)), 314_159_265_358_979.3);
    }

    #[test]
    fn area_bounds() {
        assert_eq!(area(&Circle::of(Point::zero(), 0.0)), 0.0);
        assert_eq!(area(&Circle::of(Point::zero(), MAX)), 2.5487627603172455e32);
    }
}
