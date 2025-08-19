use crate::cartesian::rect::rect_u32::RectU32;

pub fn contains_rect(outer: &RectU32, r: &RectU32) -> bool {
    r.min.x >= outer.min.x && r.max.x <= outer.max.x && r.min.y >= outer.min.y && r.max.y <= outer.max.y
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_u32::RectU32;

    use super::contains_rect;

    #[test]
    fn contains_rect_inside() {
        let r = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
        assert!(contains_rect(&r, &RectU32::of(2, 2, u32::MAX - 2, u32::MAX - 2)));
        assert!(contains_rect(&r, &RectU32::of(3, 3, u32::MAX - 3, u32::MAX - 3)));
        assert!(contains_rect(&r, &RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 10)));
    }

    #[test]
    fn contains_rect_borders() {
        let r = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
        assert!(contains_rect(&r, &RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1)));

        assert!(contains_rect(&r, &RectU32::of(2, 1, u32::MAX - 1, u32::MAX - 1)));
        assert!(contains_rect(&r, &RectU32::of(1, 2, u32::MAX - 1, u32::MAX - 1)));
        assert!(contains_rect(&r, &RectU32::of(1, 1, u32::MAX - 2, u32::MAX - 1)));
        assert!(contains_rect(&r, &RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 2)));

        assert!(contains_rect(&r, &RectU32::of(1, 1, u32::MAX - 2, u32::MAX - 2)));
        assert!(contains_rect(&r, &RectU32::of(2, 2, u32::MAX - 1, u32::MAX - 1)));
    }

    #[test]
    fn contains_rect_outside() {
        let r = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
        assert!(!contains_rect(&r, &RectU32::largest()));

        assert!(!contains_rect(&r, &RectU32::of(0, 1, u32::MAX - 1, u32::MAX - 1)));
        assert!(!contains_rect(&r, &RectU32::of(1, 0, u32::MAX - 1, u32::MAX - 1)));
        assert!(!contains_rect(&r, &RectU32::of(1, 1, u32::MAX, u32::MAX - 1)));
        assert!(!contains_rect(&r, &RectU32::of(1, 1, u32::MAX - 1, u32::MAX)));

        assert!(!contains_rect(&r, &RectU32::of(1, 1, u32::MAX, u32::MAX)));
        assert!(!contains_rect(&r, &RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1)));
    }
}
