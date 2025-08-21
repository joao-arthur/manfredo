use crate::matrix::rect::rect_i32::RectI32;

pub fn contains_rect(outer: &RectI32, r: &RectI32) -> bool {
    r.min.row >= outer.min.row && r.max.row <= outer.max.row && r.min.col >= outer.min.col && r.max.col <= outer.max.col
}

#[cfg(test)]
mod tests {
    use crate::matrix::rect::rect_i32::RectI32;

    use super::contains_rect;

    #[test]
    fn contains_rect_inside() {
        let r = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1);
        assert!(contains_rect(&r, &RectI32::of(i32::MIN + 2, i32::MIN + 2, i32::MAX - 2, i32::MAX - 2)));
        assert!(contains_rect(&r, &RectI32::of(i32::MIN + 3, i32::MIN + 3, i32::MAX - 3, i32::MAX - 3)));
        assert!(contains_rect(&r, &RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 10, i32::MAX - 10)));
    }

    #[test]
    fn contains_rect_borders() {
        let r = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1);
        assert!(contains_rect(&r, &RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1)));

        assert!(contains_rect(&r, &RectI32::of(i32::MIN + 2, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1)));
        assert!(contains_rect(&r, &RectI32::of(i32::MIN + 1, i32::MIN + 2, i32::MAX - 1, i32::MAX - 1)));
        assert!(contains_rect(&r, &RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 2, i32::MAX - 1)));
        assert!(contains_rect(&r, &RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX - 2)));

        assert!(contains_rect(&r, &RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 2, i32::MAX - 2)));
        assert!(contains_rect(&r, &RectI32::of(i32::MIN + 2, i32::MIN + 2, i32::MAX - 1, i32::MAX - 1)));
    }

    #[test]
    fn contains_rect_outside() {
        let r = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1);
        assert!(!contains_rect(&r, &RectI32::largest()));

        assert!(!contains_rect(&r, &RectI32::of(i32::MIN, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1)));
        assert!(!contains_rect(&r, &RectI32::of(i32::MIN + 1, i32::MIN, i32::MAX - 1, i32::MAX - 1)));
        assert!(!contains_rect(&r, &RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX - 1)));
        assert!(!contains_rect(&r, &RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX)));

        assert!(!contains_rect(&r, &RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX)));
        assert!(!contains_rect(&r, &RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1)));
    }
}
