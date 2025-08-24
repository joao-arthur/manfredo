use crate::cartesian::rect::rect_u8::RectU8;

pub fn contains_rect(outer: &RectU8, r: &RectU8) -> bool {
    r.min.x >= outer.min.x && r.max.x <= outer.max.x && r.min.y >= outer.min.y && r.max.y <= outer.max.y
}

#[cfg(test)]
mod tests {
    use super::contains_rect;
    use crate::cartesian::rect::rect_u8::RectU8;

    #[test]
    fn contains_rect_inside() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(contains_rect(&r, &RectU8::of(2, 2, u8::MAX - 2, u8::MAX - 2)));
        assert!(contains_rect(&r, &RectU8::of(3, 3, u8::MAX - 3, u8::MAX - 3)));
        assert!(contains_rect(&r, &RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10)));
    }

    #[test]
    fn contains_rect_borders() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(contains_rect(&r, &RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1)));

        assert!(contains_rect(&r, &RectU8::of(2, 1, u8::MAX - 1, u8::MAX - 1)));
        assert!(contains_rect(&r, &RectU8::of(1, 2, u8::MAX - 1, u8::MAX - 1)));
        assert!(contains_rect(&r, &RectU8::of(1, 1, u8::MAX - 2, u8::MAX - 1)));
        assert!(contains_rect(&r, &RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 2)));

        assert!(contains_rect(&r, &RectU8::of(1, 1, u8::MAX - 2, u8::MAX - 2)));
        assert!(contains_rect(&r, &RectU8::of(2, 2, u8::MAX - 1, u8::MAX - 1)));
    }

    #[test]
    fn contains_rect_outside() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(!contains_rect(&r, &RectU8::largest()));

        assert!(!contains_rect(&r, &RectU8::of(0, 1, u8::MAX - 1, u8::MAX - 1)));
        assert!(!contains_rect(&r, &RectU8::of(1, 0, u8::MAX - 1, u8::MAX - 1)));
        assert!(!contains_rect(&r, &RectU8::of(1, 1, u8::MAX, u8::MAX - 1)));
        assert!(!contains_rect(&r, &RectU8::of(1, 1, u8::MAX - 1, u8::MAX)));

        assert!(!contains_rect(&r, &RectU8::of(1, 1, u8::MAX, u8::MAX)));
        assert!(!contains_rect(&r, &RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1)));
    }
}
