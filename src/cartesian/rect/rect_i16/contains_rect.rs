use crate::cartesian::rect::rect_i16::RectI16;

pub fn contains_rect(outer: &RectI16, r: &RectI16) -> bool {
    r.min.x >= outer.min.x && r.max.x <= outer.max.x && r.min.y >= outer.min.y && r.max.y <= outer.max.y
}

#[cfg(test)]
mod tests {
    use super::contains_rect;
    use crate::cartesian::rect::rect_i16::RectI16;

    const MIN: i16 = i16::MIN;
    const MAX: i16 = i16::MAX;

    #[test]
    fn inside() {
        let r = RectI16::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(contains_rect(&r, &RectI16::of(MIN + 2, MIN + 2, MAX - 2, MAX - 2)));
        assert!(contains_rect(&r, &RectI16::of(MIN + 3, MIN + 3, MAX - 3, MAX - 3)));
        assert!(contains_rect(&r, &RectI16::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10)));
    }

    #[test]
    fn borders() {
        let r = RectI16::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(contains_rect(&r, &RectI16::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1)));

        assert!(contains_rect(&r, &RectI16::of(MIN + 2, MIN + 1, MAX - 1, MAX - 1)));
        assert!(contains_rect(&r, &RectI16::of(MIN + 1, MIN + 2, MAX - 1, MAX - 1)));
        assert!(contains_rect(&r, &RectI16::of(MIN + 1, MIN + 1, MAX - 2, MAX - 1)));
        assert!(contains_rect(&r, &RectI16::of(MIN + 1, MIN + 1, MAX - 1, MAX - 2)));

        assert!(contains_rect(&r, &RectI16::of(MIN + 1, MIN + 1, MAX - 2, MAX - 2)));
        assert!(contains_rect(&r, &RectI16::of(MIN + 2, MIN + 2, MAX - 1, MAX - 1)));
    }

    #[test]
    fn outside() {
        let r = RectI16::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(!contains_rect(&r, &RectI16::largest()));

        assert!(!contains_rect(&r, &RectI16::of(MIN, MIN + 1, MAX - 1, MAX - 1)));
        assert!(!contains_rect(&r, &RectI16::of(MIN + 1, MIN, MAX - 1, MAX - 1)));
        assert!(!contains_rect(&r, &RectI16::of(MIN + 1, MIN + 1, MAX, MAX - 1)));
        assert!(!contains_rect(&r, &RectI16::of(MIN + 1, MIN + 1, MAX - 1, MAX)));

        assert!(!contains_rect(&r, &RectI16::of(MIN + 1, MIN + 1, MAX, MAX)));
        assert!(!contains_rect(&r, &RectI16::of(MIN, MIN, MAX - 1, MAX - 1)));
    }
}
