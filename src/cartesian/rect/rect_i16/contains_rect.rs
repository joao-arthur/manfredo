use crate::cartesian::rect::rect_i16::RectI16;

pub fn contains_rect(outer: &RectI16, r: &RectI16) -> bool {
    r.min.x >= outer.min.x && r.max.x <= outer.max.x && r.min.y >= outer.min.y && r.max.y <= outer.max.y
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_i16::RectI16;

    use super::contains_rect;

    #[test]
    fn contains_rect_inside() {
        let r = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assert!(contains_rect(&r, &RectI16::of(i16::MIN + 2, i16::MIN + 2, i16::MAX - 2, i16::MAX - 2)));
        assert!(contains_rect(&r, &RectI16::of(i16::MIN + 3, i16::MIN + 3, i16::MAX - 3, i16::MAX - 3)));
        assert!(contains_rect(&r, &RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10)));
    }

    #[test]
    fn contains_rect_borders() {
        let r = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assert!(contains_rect(&r, &RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1)));

        assert!(contains_rect(&r, &RectI16::of(i16::MIN + 2, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1)));
        assert!(contains_rect(&r, &RectI16::of(i16::MIN + 1, i16::MIN + 2, i16::MAX - 1, i16::MAX - 1)));
        assert!(contains_rect(&r, &RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 2, i16::MAX - 1)));
        assert!(contains_rect(&r, &RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 2)));

        assert!(contains_rect(&r, &RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 2, i16::MAX - 2)));
        assert!(contains_rect(&r, &RectI16::of(i16::MIN + 2, i16::MIN + 2, i16::MAX - 1, i16::MAX - 1)));
    }

    #[test]
    fn contains_rect_outside() {
        let r = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assert!(!contains_rect(&r, &RectI16::largest()));

        assert!(!contains_rect(&r, &RectI16::of(i16::MIN, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1)));
        assert!(!contains_rect(&r, &RectI16::of(i16::MIN + 1, i16::MIN, i16::MAX - 1, i16::MAX - 1)));
        assert!(!contains_rect(&r, &RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX - 1)));
        assert!(!contains_rect(&r, &RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX)));

        assert!(!contains_rect(&r, &RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX)));
        assert!(!contains_rect(&r, &RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1)));
    }
}
