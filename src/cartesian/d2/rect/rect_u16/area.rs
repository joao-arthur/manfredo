use super::Rect;
use crate::cartesian::d2::point::point_u16::delta;

pub fn area(r: &Rect) -> u32 {
    let d = delta(&r.min, &r.max);
    u32::from(d.x) * u32::from(d.y)
}

#[cfg(test)]
mod tests {
    use super::area;
    use crate::cartesian::d2::rect::rect_u16::Rect;

    #[test]
    fn test_area() {
        assert_eq!(area(&Rect::new((0, 0), (5, 2))), 10);
        assert_eq!(area(&Rect::new((30, 50), (50, 51))), 20);
        assert_eq!(area(&Rect::new((10, 20), (15, 30))), 50);
    }

    #[test]
    fn area_powers_of_10() {
        assert_eq!(area(&Rect::new((0, 0), (1, 1))), 1);
        assert_eq!(area(&Rect::new((0, 0), (10, 10))), 100);
        assert_eq!(area(&Rect::new((0, 0), (100, 100))), 10000);
        assert_eq!(area(&Rect::new((0, 0), (1000, 1000))), 1000000);
        assert_eq!(area(&Rect::new((0, 0), (10000, 10000))), 100000000);
    }

    #[test]
    fn area_bounds() {
        assert_eq!(area(&Rect::min()), 0);
        assert_eq!(area(&Rect::largest()), 4_294_836_225);
    }
}
