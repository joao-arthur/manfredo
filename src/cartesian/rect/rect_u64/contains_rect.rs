use crate::cartesian::rect::rect_u64::RectU64;

pub fn contains_rect(outer: &RectU64, r: &RectU64) -> bool {
    r.min.x >= outer.min.x && r.max.x <= outer.max.x && r.min.y >= outer.min.y && r.max.y <= outer.max.y
}

#[cfg(test)]
mod tests {
    use super::contains_rect;
    use crate::cartesian::rect::rect_u64::RectU64;

    #[test]
    fn contains_rect_inside() {
        let r = RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1);
        assert!(contains_rect(&r, &RectU64::of(2, 2, u64::MAX - 2, u64::MAX - 2)));
        assert!(contains_rect(&r, &RectU64::of(3, 3, u64::MAX - 3, u64::MAX - 3)));
        assert!(contains_rect(&r, &RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10)));
    }

    #[test]
    fn contains_rect_borders() {
        let r = RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1);
        assert!(contains_rect(&r, &RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1)));

        assert!(contains_rect(&r, &RectU64::of(2, 1, u64::MAX - 1, u64::MAX - 1)));
        assert!(contains_rect(&r, &RectU64::of(1, 2, u64::MAX - 1, u64::MAX - 1)));
        assert!(contains_rect(&r, &RectU64::of(1, 1, u64::MAX - 2, u64::MAX - 1)));
        assert!(contains_rect(&r, &RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 2)));

        assert!(contains_rect(&r, &RectU64::of(1, 1, u64::MAX - 2, u64::MAX - 2)));
        assert!(contains_rect(&r, &RectU64::of(2, 2, u64::MAX - 1, u64::MAX - 1)));
    }

    #[test]
    fn contains_rect_outside() {
        let r = RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1);
        assert!(!contains_rect(&r, &RectU64::largest()));

        assert!(!contains_rect(&r, &RectU64::of(0, 1, u64::MAX - 1, u64::MAX - 1)));
        assert!(!contains_rect(&r, &RectU64::of(1, 0, u64::MAX - 1, u64::MAX - 1)));
        assert!(!contains_rect(&r, &RectU64::of(1, 1, u64::MAX, u64::MAX - 1)));
        assert!(!contains_rect(&r, &RectU64::of(1, 1, u64::MAX - 1, u64::MAX)));

        assert!(!contains_rect(&r, &RectU64::of(1, 1, u64::MAX, u64::MAX)));
        assert!(!contains_rect(&r, &RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1)));
    }
}
