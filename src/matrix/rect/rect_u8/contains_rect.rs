use crate::matrix::rect::rect_u8::RectU8;

pub fn contains_rect(outer: &RectU8, r: &RectU8) -> bool {
    r.min.row >= outer.min.row && r.max.row <= outer.max.row && r.min.col >= outer.min.col && r.max.col <= outer.max.col
}

#[cfg(test)]
mod tests {
    use super::contains_rect;
    use crate::matrix::rect::rect_u8::RectU8;

    const MAX: u8 = u8::MAX;

    #[test]
    fn inside() {
        let r = RectU8::of(1, 1, MAX - 1, MAX - 1);
        assert!(contains_rect(&r, &RectU8::of(2, 2, MAX - 2, MAX - 2)));
        assert!(contains_rect(&r, &RectU8::of(3, 3, MAX - 3, MAX - 3)));
        assert!(contains_rect(&r, &RectU8::of(10, 10, MAX - 10, MAX - 10)));
    }

    #[test]
    fn borders() {
        let r = RectU8::of(1, 1, MAX - 1, MAX - 1);
        assert!(contains_rect(&r, &RectU8::of(1, 1, MAX - 1, MAX - 1)));

        assert!(contains_rect(&r, &RectU8::of(2, 1, MAX - 1, MAX - 1)));
        assert!(contains_rect(&r, &RectU8::of(1, 2, MAX - 1, MAX - 1)));
        assert!(contains_rect(&r, &RectU8::of(1, 1, MAX - 2, MAX - 1)));
        assert!(contains_rect(&r, &RectU8::of(1, 1, MAX - 1, MAX - 2)));

        assert!(contains_rect(&r, &RectU8::of(1, 1, MAX - 2, MAX - 2)));
        assert!(contains_rect(&r, &RectU8::of(2, 2, MAX - 1, MAX - 1)));
    }

    #[test]
    fn outside() {
        let r = RectU8::of(1, 1, MAX - 1, MAX - 1);
        assert!(!contains_rect(&r, &RectU8::largest()));

        assert!(!contains_rect(&r, &RectU8::of(0, 1, MAX - 1, MAX - 1)));
        assert!(!contains_rect(&r, &RectU8::of(1, 0, MAX - 1, MAX - 1)));
        assert!(!contains_rect(&r, &RectU8::of(1, 1, MAX, MAX - 1)));
        assert!(!contains_rect(&r, &RectU8::of(1, 1, MAX - 1, MAX)));

        assert!(!contains_rect(&r, &RectU8::of(1, 1, MAX, MAX)));
        assert!(!contains_rect(&r, &RectU8::of(0, 0, MAX - 1, MAX - 1)));
    }
}
