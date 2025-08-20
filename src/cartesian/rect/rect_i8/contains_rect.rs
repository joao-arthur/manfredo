use crate::cartesian::rect::rect_i8::RectI8;

pub fn contains_rect(outer: &RectI8, r: &RectI8) -> bool {
    r.min.x >= outer.min.x && r.max.x <= outer.max.x && r.min.y >= outer.min.y && r.max.y <= outer.max.y
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_i8::RectI8;

    use super::contains_rect;

    #[test]
    fn contains_rect_inside() {
        let r = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1);
        assert!(contains_rect(&r, &RectI8::of(i8::MIN + 2, i8::MIN + 2, i8::MAX - 2, i8::MAX - 2)));
        assert!(contains_rect(&r, &RectI8::of(i8::MIN + 3, i8::MIN + 3, i8::MAX - 3, i8::MAX - 3)));
        assert!(contains_rect(&r, &RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10)));
    }

    #[test]
    fn contains_rect_borders() {
        let r = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1);
        assert!(contains_rect(&r, &RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1)));

        assert!(contains_rect(&r, &RectI8::of(i8::MIN + 2, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1)));
        assert!(contains_rect(&r, &RectI8::of(i8::MIN + 1, i8::MIN + 2, i8::MAX - 1, i8::MAX - 1)));
        assert!(contains_rect(&r, &RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 2, i8::MAX - 1)));
        assert!(contains_rect(&r, &RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 2)));

        assert!(contains_rect(&r, &RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 2, i8::MAX - 2)));
        assert!(contains_rect(&r, &RectI8::of(i8::MIN + 2, i8::MIN + 2, i8::MAX - 1, i8::MAX - 1)));
    }

    #[test]
    fn contains_rect_outside() {
        let r = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1);
        assert!(!contains_rect(&r, &RectI8::largest()));

        assert!(!contains_rect(&r, &RectI8::of(i8::MIN, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1)));
        assert!(!contains_rect(&r, &RectI8::of(i8::MIN + 1, i8::MIN, i8::MAX - 1, i8::MAX - 1)));
        assert!(!contains_rect(&r, &RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX - 1)));
        assert!(!contains_rect(&r, &RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX)));

        assert!(!contains_rect(&r, &RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX)));
        assert!(!contains_rect(&r, &RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1)));
    }
}
