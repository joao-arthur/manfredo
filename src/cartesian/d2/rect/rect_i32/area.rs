use super::Rect;
use crate::cartesian::d2::point::point_i32::delta;

pub fn area(r: &Rect) -> u64 {
    let d = delta(&r.min, &r.max);
    u64::from(d.x) * u64::from(d.y)
}

#[cfg(test)]
mod tests {
    use super::area;
    use crate::cartesian::d2::rect::rect_i32::Rect;

    #[test]
    fn test_area() {
        assert_eq!(area(&Rect::of(-5, 0, 0, 2)), 10);
        assert_eq!(area(&Rect::of(-50, 50, -30, 51)), 20);
        assert_eq!(area(&Rect::of(10, -30, 15, -20)), 50);
    }

    #[test]
    fn area_powers_of_10() {
        assert_eq!(area(&Rect::of(0, 0, 1, 1)), 1);
        assert_eq!(area(&Rect::of(-5, -5, 5, 5)), 100);
        assert_eq!(area(&Rect::of(-50, -50, 50, 50)), 10000);
        assert_eq!(area(&Rect::of(-500, -500, 500, 500)), 1000000);
        assert_eq!(area(&Rect::of(-5000, -5000, 5000, 5000)), 100000000);
        assert_eq!(area(&Rect::of(-50000, -50000, 50000, 50000)), 10000000000);
        assert_eq!(area(&Rect::of(-500000, -500000, 500000, 500000)), 1000000000000);
        assert_eq!(area(&Rect::of(-5000000, -5000000, 5000000, 5000000)), 100000000000000);
        assert_eq!(area(&Rect::of(-50000000, -50000000, 50000000, 50000000)), 10000000000000000);
        assert_eq!(area(&Rect::of(-500000000, -500000000, 500000000, 500000000)), 1000000000000000000);
    }

    #[test]
    fn area_bounds() {
        assert_eq!(area(&Rect::zero()), 0);
        assert_eq!(area(&Rect::largest()), 18_446_744_065_119_617_025);
    }
}
