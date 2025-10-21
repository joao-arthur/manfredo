use super::Rect;
use crate::matrix::d2::point::point_i8::delta;

pub fn area(r: &Rect) -> u16 {
    let d = delta(&r.min, &r.max);
    u16::from(d.row) * u16::from(d.col)
}

#[cfg(test)]
mod tests {
    use super::area;
    use crate::matrix::d2::rect::rect_i8::Rect;

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
    }

    #[test]
    fn area_bounds() {
        assert_eq!(area(&Rect::zero()), 0);
        assert_eq!(area(&Rect::largest()), 65_025);
    }
}
