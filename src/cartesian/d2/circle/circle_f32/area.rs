use super::Circle;
use std::f32;

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
    }

    #[test]
    fn area_bounds() {
        assert_eq!(area(&Circle::of(Point::zero(), 0.0)), 0.0);
        assert_eq!(area(&Circle::of(Point::zero(), MAX)), 884279600000000.0);
    }
}
