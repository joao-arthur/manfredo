use std::f32;

use super::Circle;

pub fn area(c: &Circle) -> f32 {
    f32::consts::PI * c.r as f32 * c.r as f32
}

#[cfg(test)]
mod tests {
    use super::area;
    use crate::cartesian::d2::{
        circle::circle_f32::Circle,
        point::point_f32::{MAX, Point},
    };

    #[test]
    fn test_area() {
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 2.0)), 12.5663706);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 3.0)), 28.2743338);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 4.0)), 50.2654824);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 5.0)), 78.5398163);
    }

    #[test]
    fn area_powers_of_10() {
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 1.0)), 3.1415927);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 10.0)), 314.15926);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 100.0)), 31415.928);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 1000.0)), 3141592.8);
    }

    #[test]
    fn area_bounds() {
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), 0.0)), 0.0);
        assert_eq!(area(&Circle::of(Point::of(0.0, 0.0), MAX)), 884279600000000.0);
    }
}
