use crate::cartesian::d2::{point::point_i8::delta, rect::rect_i8::Rect};

pub fn area(r: &Rect) -> u16 {
    let d = delta(&r.min, &r.max);
    u16::from(d.x) * u16::from(d.y)
}

#[cfg(test)]
mod tests {
    use super::area;
    use crate::cartesian::d2::rect::rect_i8::Rect;

    #[test]
    fn test_area() {
        assert_eq!(area(&Rect::of(0, 0, 0, 0)), 0);
        assert_eq!(area(&Rect::of(0, 0, 1, 1)), 1);
        assert_eq!(area(&Rect::of(-5, 0, 5, 5)), 50);
        assert_eq!(area(&Rect::of(-5, -5, 5, 5)), 100);
        assert_eq!(area(&Rect::of(-25, -50, 25, 50)), 5000);
        assert_eq!(area(&Rect::of(-100, -100, 100, 100)), 40000);
        assert_eq!(area(&Rect::largest()), 65_025);
    }
}
