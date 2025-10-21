use super::Rect;
use crate::cartesian::d2::point::point_f32::delta;

pub fn area(r: &Rect) -> f32 {
    let d = delta(&r.min, &r.max);
    d.x * d.y
}

#[cfg(test)]
mod tests {
    use super::area;
    use crate::cartesian::{d1::point::point_f32::MAX, d2::rect::rect_f32::Rect};

    #[test]
    fn test_area() {
        assert_eq!(area(&Rect::of(-5.0, 0.0, 0.0, 2.0)), 10.0);
        assert_eq!(area(&Rect::of(-50.0, 50.0, -30.0, 51.0)), 20.0);
        assert_eq!(area(&Rect::of(10.0, -30.0, 15.0, -20.0)), 50.0);
    }

    #[test]
    fn area_powers_of_10() {
        assert_eq!(area(&Rect::of(0.0, 0.0, 1.0, 1.0)), 1.0);
        assert_eq!(area(&Rect::of(-5.0, -5.0, 5.0, 5.0)), 100.0);
        assert_eq!(area(&Rect::of(-50.0, -50.0, 50.0, 50.0)), 10000.0);
        assert_eq!(area(&Rect::of(-500.0, -500.0, 500.0, 500.0)), 1000000.0);
        assert_eq!(area(&Rect::of(-5000.0, -5000.0, 5000.0, 5000.0)), 100000000.0);
        assert_eq!(area(&Rect::of(-50000.0, -50000.0, 50000.0, 50000.0)), 10000000000.0);
        assert_eq!(area(&Rect::of(-500000.0, -500000.0, 500000.0, 500000.0)), 1000000000000.0);
        assert_eq!(area(&Rect::of(-5000000.0, -5000000.0, 5000000.0, 5000000.0)), 100000000000000.0);
        assert_eq!(area(&Rect::of(-50000000.0, -50000000.0, 50000000.0, 50000000.0)), 10000000000000000.0);
        assert_eq!(area(&Rect::of(-500000000.0, -500000000.0, 500000000.0, 500000000.0)), 1000000000000000000.0);
    }

    #[test]
    fn area_bounds() {
        assert_eq!(area(&Rect::zero()), 0.0);
        assert_eq!(area(&Rect::of(-4096.0, -4096.0, 0.0, 0.0)), MAX + 1.0);
        assert_eq!(area(&Rect::of(0.0, 0.0, 4096.0, 4096.0)), MAX + 1.0);
    }
}
