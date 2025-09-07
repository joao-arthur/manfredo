use crate::matrix::rect::rect_i64::RectI64;

pub fn contains_rect(outer: &RectI64, r: &RectI64) -> bool {
    r.min.row >= outer.min.row && r.max.row <= outer.max.row && r.min.col >= outer.min.col && r.max.col <= outer.max.col
}

#[cfg(test)]
mod tests {
    use super::contains_rect;
    use crate::matrix::rect::rect_i64::RectI64;

    const MIN: i64 = i64::MIN;
    const MAX: i64 = i64::MAX;

    #[test]
    fn inside() {
        let r = RectI64::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(contains_rect(&r, &RectI64::of(MIN + 2, MIN + 2, MAX - 2, MAX - 2)));
        assert!(contains_rect(&r, &RectI64::of(MIN + 3, MIN + 3, MAX - 3, MAX - 3)));
        assert!(contains_rect(&r, &RectI64::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10)));
    }

    #[test]
    fn borders() {
        let r = RectI64::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(contains_rect(&r, &RectI64::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1)));

        assert!(contains_rect(&r, &RectI64::of(MIN + 2, MIN + 1, MAX - 1, MAX - 1)));
        assert!(contains_rect(&r, &RectI64::of(MIN + 1, MIN + 2, MAX - 1, MAX - 1)));
        assert!(contains_rect(&r, &RectI64::of(MIN + 1, MIN + 1, MAX - 2, MAX - 1)));
        assert!(contains_rect(&r, &RectI64::of(MIN + 1, MIN + 1, MAX - 1, MAX - 2)));

        assert!(contains_rect(&r, &RectI64::of(MIN + 1, MIN + 1, MAX - 2, MAX - 2)));
        assert!(contains_rect(&r, &RectI64::of(MIN + 2, MIN + 2, MAX - 1, MAX - 1)));
    }

    #[test]
    fn outside() {
        let r = RectI64::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(!contains_rect(&r, &RectI64::largest()));

        assert!(!contains_rect(&r, &RectI64::of(MIN, MIN + 1, MAX - 1, MAX - 1)));
        assert!(!contains_rect(&r, &RectI64::of(MIN + 1, MIN, MAX - 1, MAX - 1)));
        assert!(!contains_rect(&r, &RectI64::of(MIN + 1, MIN + 1, MAX, MAX - 1)));
        assert!(!contains_rect(&r, &RectI64::of(MIN + 1, MIN + 1, MAX - 1, MAX)));

        assert!(!contains_rect(&r, &RectI64::of(MIN + 1, MIN + 1, MAX, MAX)));
        assert!(!contains_rect(&r, &RectI64::of(MIN, MIN, MAX - 1, MAX - 1)));
    }
}
