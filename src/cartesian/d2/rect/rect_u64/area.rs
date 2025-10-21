use super::Rect;
use crate::cartesian::d2::point::point_u64::delta;

pub fn area(r: &Rect) -> u128 {
    let d = delta(&r.min, &r.max);
    u128::from(d.x) * u128::from(d.y)
}

#[cfg(test)]
mod tests {
    use super::area;
    use crate::cartesian::d2::rect::rect_u64::Rect;

    #[test]
    fn test_area() {
        assert_eq!(area(&Rect::of(0, 0, 5, 2)), 10);
        assert_eq!(area(&Rect::of(30, 50, 50, 51)), 20);
        assert_eq!(area(&Rect::of(10, 20, 15, 30)), 50);
    }

    #[test]
    fn area_powers_of_10() {
        assert_eq!(area(&Rect::of(0, 0, 1, 1)), 1);
        assert_eq!(area(&Rect::of(0, 0, 10, 10)), 100);
        assert_eq!(area(&Rect::of(0, 0, 100, 100)), 10000);
        assert_eq!(area(&Rect::of(0, 0, 1000, 1000)), 1000000);
        assert_eq!(area(&Rect::of(0, 0, 10000, 10000)), 100000000);
        assert_eq!(area(&Rect::of(0, 0, 100000, 100000)), 10000000000);
        assert_eq!(area(&Rect::of(0, 0, 1000000, 1000000)), 1000000000000);
        assert_eq!(area(&Rect::of(0, 0, 10000000, 10000000)), 100000000000000);
        assert_eq!(area(&Rect::of(0, 0, 100000000, 100000000)), 10000000000000000);
        assert_eq!(area(&Rect::of(0, 0, 1000000000, 1000000000)), 1000000000000000000);
        assert_eq!(area(&Rect::of(0, 0, 10000000000, 10000000000)), 100000000000000000000);
        assert_eq!(area(&Rect::of(0, 0, 100000000000, 100000000000)), 10000000000000000000000);
        assert_eq!(area(&Rect::of(0, 0, 1000000000000, 1000000000000)), 1000000000000000000000000);
        assert_eq!(area(&Rect::of(0, 0, 10000000000000, 10000000000000)), 100000000000000000000000000);
        assert_eq!(area(&Rect::of(0, 0, 100000000000000, 100000000000000)), 10000000000000000000000000000);
        assert_eq!(area(&Rect::of(0, 0, 1000000000000000, 1000000000000000)), 1000000000000000000000000000000);
        assert_eq!(area(&Rect::of(0, 0, 10000000000000000, 10000000000000000)), 100000000000000000000000000000000);
        assert_eq!(area(&Rect::of(0, 0, 100000000000000000, 100000000000000000)), 10000000000000000000000000000000000);
        assert_eq!(area(&Rect::of(0, 0, 1000000000000000000, 1000000000000000000)), 1000000000000000000000000000000000000);
        assert_eq!(area(&Rect::of(0, 0, 10000000000000000000, 10000000000000000000)), 100000000000000000000000000000000000000);
    }

    #[test]
    fn area_bounds() {
        assert_eq!(area(&Rect::min()), 0);
        assert_eq!(area(&Rect::largest()), 340_282_366_920_938_463_426_481_119_284_349_108_225);
    }
}
