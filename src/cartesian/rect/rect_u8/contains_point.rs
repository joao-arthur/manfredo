use crate::cartesian::{point::point_u8::PointU8, rect::rect_u8::RectU8};

pub fn contains_point(outer: &RectU8, p: &PointU8) -> bool {
    p.x >= outer.min.x && p.x <= outer.max.x && p.y >= outer.min.y && p.y <= outer.max.y
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{point::point_u8::PointU8, rect::rect_u8::RectU8};

    use super::contains_point;

    #[test]
    fn contains_point_inside_borders() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(contains_point(&r, &PointU8::of(1, 1)));
        assert!(contains_point(&r, &PointU8::of(1, u8::MAX - 1)));
        assert!(contains_point(&r, &PointU8::of(u8::MAX - 1, 1)));
        assert!(contains_point(&r, &PointU8::of(u8::MAX - 1, u8::MAX - 1)));
    }

    #[test]
    fn contains_point_outside_borders() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(!contains_point(&r, &PointU8::min()));
        assert!(!contains_point(&r, &PointU8::of(1, 0)));
        assert!(!contains_point(&r, &PointU8::of(0, 1)));
        
        assert!(!contains_point(&r, &PointU8::of(0, u8::MAX)));
        assert!(!contains_point(&r, &PointU8::of(0, u8::MAX - 1)));
        assert!(!contains_point(&r, &PointU8::of(1, u8::MAX)));
        
        assert!(!contains_point(&r, &PointU8::of(u8::MAX, 0)));
        assert!(!contains_point(&r, &PointU8::of(u8::MAX, 1)));
        assert!(!contains_point(&r, &PointU8::of(u8::MAX - 1, 0)));

        assert!(!contains_point(&r, &PointU8::max()));
        assert!(!contains_point(&r, &PointU8::of(u8::MAX - 1, u8::MAX)));
        assert!(!contains_point(&r, &PointU8::of(u8::MAX, u8::MAX - 1)));
    }

    #[test]
    fn contains_point_inside() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(contains_point(&r, &PointU8::of(10, 10)));
        assert!(contains_point(&r, &PointU8::of(u8::MAX - 10, 10)));
        assert!(contains_point(&r, &PointU8::of(10, u8::MAX - 10)));
        assert!(contains_point(&r, &PointU8::of(u8::MAX - 10, u8::MAX - 10)));
    }
}
