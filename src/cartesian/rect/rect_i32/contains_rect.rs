use crate::cartesian::rect::rect_i32::RectI32;

pub fn contains_rect(outer: &RectI32, r: &RectI32) -> bool {
    r.min.x >= outer.min.x && r.max.x <= outer.max.x && r.min.y >= outer.min.y && r.max.y <= outer.max.y
}

#[cfg(test)]
mod tests {
    use super::contains_rect;
    use crate::cartesian::rect::rect_i32::RectI32;

    const MIN: i32 = i32::MIN;
    const MAX: i32 = i32::MAX;

    #[test]
    fn contains_rect_inside() {
        let r = RectI32::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(contains_rect(&r, &RectI32::of(MIN + 2, MIN + 2, MAX - 2, MAX - 2)));
        assert!(contains_rect(&r, &RectI32::of(MIN + 3, MIN + 3, MAX - 3, MAX - 3)));
        assert!(contains_rect(&r, &RectI32::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10)));
    }

    #[test]
    fn contains_rect_borders() {
        let r = RectI32::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(contains_rect(&r, &RectI32::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1)));

        assert!(contains_rect(&r, &RectI32::of(MIN + 2, MIN + 1, MAX - 1, MAX - 1)));
        assert!(contains_rect(&r, &RectI32::of(MIN + 1, MIN + 2, MAX - 1, MAX - 1)));
        assert!(contains_rect(&r, &RectI32::of(MIN + 1, MIN + 1, MAX - 2, MAX - 1)));
        assert!(contains_rect(&r, &RectI32::of(MIN + 1, MIN + 1, MAX - 1, MAX - 2)));

        assert!(contains_rect(&r, &RectI32::of(MIN + 1, MIN + 1, MAX - 2, MAX - 2)));
        assert!(contains_rect(&r, &RectI32::of(MIN + 2, MIN + 2, MAX - 1, MAX - 1)));
    }

    #[test]
    fn contains_rect_outside() {
        let r = RectI32::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(!contains_rect(&r, &RectI32::largest()));

        assert!(!contains_rect(&r, &RectI32::of(MIN, MIN + 1, MAX - 1, MAX - 1)));
        assert!(!contains_rect(&r, &RectI32::of(MIN + 1, MIN, MAX - 1, MAX - 1)));
        assert!(!contains_rect(&r, &RectI32::of(MIN + 1, MIN + 1, MAX, MAX - 1)));
        assert!(!contains_rect(&r, &RectI32::of(MIN + 1, MIN + 1, MAX - 1, MAX)));

        assert!(!contains_rect(&r, &RectI32::of(MIN + 1, MIN + 1, MAX, MAX)));
        assert!(!contains_rect(&r, &RectI32::of(MIN, MIN, MAX - 1, MAX - 1)));
    }
}
