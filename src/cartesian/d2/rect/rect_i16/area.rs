use super::Rect;
use crate::cartesian::d2::point::point_i16::delta;

pub fn area(r: &Rect) -> u32 {
    let d = delta(&r.min, &r.max);
    u32::from(d.x) * u32::from(d.y)
}

#[cfg(test)]
mod tests {
    use super::area;
    use crate::cartesian::d2::rect::rect_i16::Rect;

    #[test]
    fn test_area() {
        assert_eq!(area(&Rect::of((-5, 0), (0, 2))), 10);
        assert_eq!(area(&Rect::of((-50, 50), (-30, 51))), 20);
        assert_eq!(area(&Rect::of((10, -30), (15, -20))), 50);
    }

    #[test]
    fn area_powers_of_10() {
        assert_eq!(area(&Rect::of((0, 0), (1, 1))), 1);
        assert_eq!(area(&Rect::of((-5, -5), (5, 5))), 100);
        assert_eq!(area(&Rect::of((-50, -50), (50, 50))), 10000);
        assert_eq!(area(&Rect::of((-500, -500), (500, 500))), 1000000);
        assert_eq!(area(&Rect::of((-5000, -5000), (5000, 5000))), 100000000);
    }

    #[test]
    fn area_bounds() {
        assert_eq!(area(&Rect::zero()), 0);
        assert_eq!(area(&Rect::largest()), 4_294_836_225);
    }
}
