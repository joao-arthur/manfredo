use crate::cartesian::rect::rect_i64::RectI64;

pub fn contains_rect(outer: &RectI64, r: &RectI64) -> bool {
    r.min.x >= outer.min.x && r.max.x <= outer.max.x && r.min.y >= outer.min.y && r.max.y <= outer.max.y
}

#[cfg(test)]
mod tests {
    use super::contains_rect;
    use crate::cartesian::rect::rect_i64::RectI64;

    #[test]
    fn contains_rect_inside() {
        let r = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1);
        assert!(contains_rect(&r, &RectI64::of(i64::MIN + 2, i64::MIN + 2, i64::MAX - 2, i64::MAX - 2)));
        assert!(contains_rect(&r, &RectI64::of(i64::MIN + 3, i64::MIN + 3, i64::MAX - 3, i64::MAX - 3)));
        assert!(contains_rect(&r, &RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10)));
    }

    #[test]
    fn contains_rect_borders() {
        let r = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1);
        assert!(contains_rect(&r, &RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1)));

        assert!(contains_rect(&r, &RectI64::of(i64::MIN + 2, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1)));
        assert!(contains_rect(&r, &RectI64::of(i64::MIN + 1, i64::MIN + 2, i64::MAX - 1, i64::MAX - 1)));
        assert!(contains_rect(&r, &RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 2, i64::MAX - 1)));
        assert!(contains_rect(&r, &RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 2)));

        assert!(contains_rect(&r, &RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 2, i64::MAX - 2)));
        assert!(contains_rect(&r, &RectI64::of(i64::MIN + 2, i64::MIN + 2, i64::MAX - 1, i64::MAX - 1)));
    }

    #[test]
    fn contains_rect_outside() {
        let r = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1);
        assert!(!contains_rect(&r, &RectI64::largest()));

        assert!(!contains_rect(&r, &RectI64::of(i64::MIN, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1)));
        assert!(!contains_rect(&r, &RectI64::of(i64::MIN + 1, i64::MIN, i64::MAX - 1, i64::MAX - 1)));
        assert!(!contains_rect(&r, &RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX - 1)));
        assert!(!contains_rect(&r, &RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX)));

        assert!(!contains_rect(&r, &RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX)));
        assert!(!contains_rect(&r, &RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1)));
    }
}
