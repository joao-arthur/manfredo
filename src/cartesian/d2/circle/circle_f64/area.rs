use std::f64;

use super::Circle;

pub fn area(c: &Circle) -> f64 {
    f64::consts::PI * c.r as f64 * c.r as f64
}

#[cfg(test)]
mod tests {
    use super::area;
    use crate::cartesian::d2::{
        circle::circle_f64::Circle,
        point::point_f64::{MAX, Point},
    };

    #[test]
    fn test_area() {
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 2.0)), 12.566370614359172);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 3.0)), 28.274333882308138);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 4.0)), 50.26548245743669);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 5.0)), 78.53981633974483);
    }

    #[test]
    fn area_powers_of_10() {
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 1.0)), 3.1415926535897932);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 10.0)), 314.15926535897932);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 100.0)), 31415.926535897932);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 1000.0)), 3141592.6535897932);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 10000.0)), 314159265.35897932);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 100000.0)), 31415926535.897932);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 1000000.0)), 3141592653589.7932);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 10000000.0)), 314159265358979.32);
    }

    #[test]
    fn area_bounds() {
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 0.0)), 0.0);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), MAX)), 2.5487627603172455e32);
    }
}
