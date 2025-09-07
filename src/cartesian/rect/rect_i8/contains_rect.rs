use crate::cartesian::rect::rect_i8::RectI8;

pub fn contains_rect(outer: &RectI8, r: &RectI8) -> bool {
    r.min.x >= outer.min.x && r.max.x <= outer.max.x && r.min.y >= outer.min.y && r.max.y <= outer.max.y
}

#[cfg(test)]
mod tests {
    use super::contains_rect;
    use crate::cartesian::rect::rect_i8::RectI8;

    const MIN: i8 = i8::MIN;
    const MAX: i8 = i8::MAX;

    #[test]
    fn inside() {
        let r = RectI8::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(contains_rect(&r, &RectI8::of(MIN + 2, MIN + 2, MAX - 2, MAX - 2)));
        assert!(contains_rect(&r, &RectI8::of(MIN + 3, MIN + 3, MAX - 3, MAX - 3)));
        assert!(contains_rect(&r, &RectI8::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10)));
    }

    #[test]
    fn borders() {
        let r = RectI8::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(contains_rect(&r, &RectI8::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1)));

        assert!(contains_rect(&r, &RectI8::of(MIN + 2, MIN + 1, MAX - 1, MAX - 1)));
        assert!(contains_rect(&r, &RectI8::of(MIN + 1, MIN + 2, MAX - 1, MAX - 1)));
        assert!(contains_rect(&r, &RectI8::of(MIN + 1, MIN + 1, MAX - 2, MAX - 1)));
        assert!(contains_rect(&r, &RectI8::of(MIN + 1, MIN + 1, MAX - 1, MAX - 2)));

        assert!(contains_rect(&r, &RectI8::of(MIN + 1, MIN + 1, MAX - 2, MAX - 2)));
        assert!(contains_rect(&r, &RectI8::of(MIN + 2, MIN + 2, MAX - 1, MAX - 1)));
    }

    #[test]
    fn outside() {
        let r = RectI8::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(!contains_rect(&r, &RectI8::largest()));

        assert!(!contains_rect(&r, &RectI8::of(MIN, MIN + 1, MAX - 1, MAX - 1)));
        assert!(!contains_rect(&r, &RectI8::of(MIN + 1, MIN, MAX - 1, MAX - 1)));
        assert!(!contains_rect(&r, &RectI8::of(MIN + 1, MIN + 1, MAX, MAX - 1)));
        assert!(!contains_rect(&r, &RectI8::of(MIN + 1, MIN + 1, MAX - 1, MAX)));

        assert!(!contains_rect(&r, &RectI8::of(MIN + 1, MIN + 1, MAX, MAX)));
        assert!(!contains_rect(&r, &RectI8::of(MIN, MIN, MAX - 1, MAX - 1)));
    }
}
