use std::f32;

use super::Circle;

pub fn area(c: &Circle) -> f32 {
    f32::consts::PI * c.r * c.r
}

#[cfg(test)]
mod tests {
    use super::area;
    use crate::cartesian::{
        d1::point::point_f32::MAX,
        d2::{circle::circle_f32::Circle, point::point_f32::Point},
    };
    use std::f32::consts::PI;

    #[test]
    fn test_area() {
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 2.0)), 4.0 * PI);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 3.0)), 9.0 * PI);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 4.0)), 16.0 * PI);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 5.0)), 25.0 * PI);
    }

    #[test]
    fn area_powers_of_10() {
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 1.0)), PI);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 10.0)), 100.0 * PI);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 100.0)), 10000.0 * PI);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 1000.0)), 1000000.0 * PI);
    }

    #[test]
    fn area_bounds() {
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 0.0)), 0.0);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), MAX)), 884279600000000.0);
    }
}
