use crate::matrix::rect::rect_u64::RectU64;

pub fn contains_rect(outer: &RectU64, r: &RectU64) -> bool {
    r.min.row >= outer.min.row && r.max.row <= outer.max.row && r.min.col >= outer.min.col && r.max.col <= outer.max.col
}

#[cfg(test)]
mod tests {
    use super::contains_rect;
    use crate::matrix::rect::rect_u64::RectU64;

    const MAX: u64 = u64::MAX;

    #[test]
    fn inside() {
        let r = RectU64::of(1, 1, MAX - 1, MAX - 1);
        assert!(contains_rect(&r, &RectU64::of(2, 2, MAX - 2, MAX - 2)));
        assert!(contains_rect(&r, &RectU64::of(3, 3, MAX - 3, MAX - 3)));
        assert!(contains_rect(&r, &RectU64::of(10, 10, MAX - 10, MAX - 10)));
    }

    #[test]
    fn borders() {
        let r = RectU64::of(1, 1, MAX - 1, MAX - 1);
        assert!(contains_rect(&r, &RectU64::of(1, 1, MAX - 1, MAX - 1)));

        assert!(contains_rect(&r, &RectU64::of(2, 1, MAX - 1, MAX - 1)));
        assert!(contains_rect(&r, &RectU64::of(1, 2, MAX - 1, MAX - 1)));
        assert!(contains_rect(&r, &RectU64::of(1, 1, MAX - 2, MAX - 1)));
        assert!(contains_rect(&r, &RectU64::of(1, 1, MAX - 1, MAX - 2)));

        assert!(contains_rect(&r, &RectU64::of(1, 1, MAX - 2, MAX - 2)));
        assert!(contains_rect(&r, &RectU64::of(2, 2, MAX - 1, MAX - 1)));
    }

    #[test]
    fn outside() {
        let r = RectU64::of(1, 1, MAX - 1, MAX - 1);
        assert!(!contains_rect(&r, &RectU64::largest()));

        assert!(!contains_rect(&r, &RectU64::of(0, 1, MAX - 1, MAX - 1)));
        assert!(!contains_rect(&r, &RectU64::of(1, 0, MAX - 1, MAX - 1)));
        assert!(!contains_rect(&r, &RectU64::of(1, 1, MAX, MAX - 1)));
        assert!(!contains_rect(&r, &RectU64::of(1, 1, MAX - 1, MAX)));

        assert!(!contains_rect(&r, &RectU64::of(1, 1, MAX, MAX)));
        assert!(!contains_rect(&r, &RectU64::of(0, 0, MAX - 1, MAX - 1)));
    }
}
