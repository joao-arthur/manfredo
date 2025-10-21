use super::Rect;
use crate::matrix::d2::point::point_u32::delta;

pub fn area(r: &Rect) -> u64 {
    let d = delta(&r.min, &r.max);
    u64::from(d.row) * u64::from(d.col)
}

#[cfg(test)]
mod tests {
    use super::area;
    use crate::matrix::d2::rect::rect_u32::Rect;

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
    }

    #[test]
    fn area_bounds() {
        assert_eq!(area(&Rect::min()), 0);
        assert_eq!(area(&Rect::largest()), 18_446_744_065_119_617_025);
    }
}
