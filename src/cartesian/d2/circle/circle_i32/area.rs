use std::f64;

use super::Circle;

pub fn area(c: &Circle) -> f64 {
    f64::consts::PI * c.r as f64 * c.r as f64
}

#[cfg(test)]
mod tests {
    use super::area;
    use crate::cartesian::d2::{circle::circle_i32::Circle, point::point_i32::Point};

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
        assert_eq!(area(&Circle::of(Point::of(0, 0), 1000)), 3_141_592.653_589_793);
        assert_eq!(area(&Circle::of(Point::of(0, 0), 10000)), 314159265.35897932);
        assert_eq!(area(&Circle::of(Point::of(0, 0), 100000)), 31_415_926_535.897_93);
        assert_eq!(area(&Circle::of(Point::of(0, 0), 1000000)), 3_141_592_653_589.793);
        assert_eq!(area(&Circle::of(Point::of(0, 0), 10000000)), 314_159_265_358_979.3);
    }

    #[test]
    fn area_bounds() {
        assert_eq!(area(&Circle::of(Point::of(0, 0), 0)), 0.0);
        assert_eq!(area(&Circle::of(Point::of(0, 0), u32::MAX)), 5.795215563763091e19);
    }
}
