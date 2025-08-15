use crate::cartesian::{rect::rect_u8::RectU8, point::point_u8};

// contains_p
// contains_r
pub fn contains(r: &RectU8, p: &point_u8::PointU8) -> bool {
    p.x >= r.min.x && p.x <= r.max.x && p.y >= r.min.y && p.y <= r.max.y
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{point::point_u8::PointU8, rect::rect_u8::RectU8};

    use super::contains;

    #[test]
    fn contains_inside_borders() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(contains(&r, &PointU8::of(1, 1)));
        assert!(contains(&r, &PointU8::of(1, u8::MAX - 1)));
        assert!(contains(&r, &PointU8::of(u8::MAX - 1, 1)));
        assert!(contains(&r, &PointU8::of(u8::MAX - 1, u8::MAX - 1)));
    }

    #[test]
    fn contains_outside_borders() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(!contains(&r, &PointU8::of(0, 0)));
        assert!(!contains(&r, &PointU8::of(0, u8::MAX)));
        assert!(!contains(&r, &PointU8::of(u8::MAX, 0)));
        assert!(!contains(&r, &PointU8::max()));
        assert!(!contains(&r, &PointU8::of(1, 0)));
        assert!(!contains(&r, &PointU8::of(1, u8::MAX)));
        assert!(!contains(&r, &PointU8::of(u8::MAX - 1, 0)));
        assert!(!contains(&r, &PointU8::of(u8::MAX - 1, u8::MAX)));
        assert!(!contains(&r, &PointU8::of(0, 1)));
        assert!(!contains(&r, &PointU8::of(0, u8::MAX - 1)));
        assert!(!contains(&r, &PointU8::of(u8::MAX, 1)));
        assert!(!contains(&r, &PointU8::of(u8::MAX, u8::MAX - 1)));
    }

    #[test]
    fn contains_inside() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(contains(&r, &PointU8::of(10, 10)));
        assert!(contains(&r, &PointU8::of(u8::MAX - 10, u8::MAX - 10)));
    }
}
