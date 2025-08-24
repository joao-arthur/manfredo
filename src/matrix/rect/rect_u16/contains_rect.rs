use crate::matrix::rect::rect_u16::RectU16;

pub fn contains_rect(outer: &RectU16, r: &RectU16) -> bool {
    r.min.row >= outer.min.row && r.max.row <= outer.max.row && r.min.col >= outer.min.col && r.max.col <= outer.max.col
}

#[cfg(test)]
mod tests {
    use super::contains_rect;
    use crate::matrix::rect::rect_u16::RectU16;

    #[test]
    fn contains_rect_inside() {
        let r = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assert!(contains_rect(&r, &RectU16::of(2, 2, u16::MAX - 2, u16::MAX - 2)));
        assert!(contains_rect(&r, &RectU16::of(3, 3, u16::MAX - 3, u16::MAX - 3)));
        assert!(contains_rect(&r, &RectU16::of(10, 10, u16::MAX - 10, u16::MAX - 10)));
    }

    #[test]
    fn contains_rect_borders() {
        let r = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assert!(contains_rect(&r, &RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1)));

        assert!(contains_rect(&r, &RectU16::of(2, 1, u16::MAX - 1, u16::MAX - 1)));
        assert!(contains_rect(&r, &RectU16::of(1, 2, u16::MAX - 1, u16::MAX - 1)));
        assert!(contains_rect(&r, &RectU16::of(1, 1, u16::MAX - 2, u16::MAX - 1)));
        assert!(contains_rect(&r, &RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 2)));

        assert!(contains_rect(&r, &RectU16::of(1, 1, u16::MAX - 2, u16::MAX - 2)));
        assert!(contains_rect(&r, &RectU16::of(2, 2, u16::MAX - 1, u16::MAX - 1)));
    }

    #[test]
    fn contains_rect_outside() {
        let r = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assert!(!contains_rect(&r, &RectU16::largest()));

        assert!(!contains_rect(&r, &RectU16::of(0, 1, u16::MAX - 1, u16::MAX - 1)));
        assert!(!contains_rect(&r, &RectU16::of(1, 0, u16::MAX - 1, u16::MAX - 1)));
        assert!(!contains_rect(&r, &RectU16::of(1, 1, u16::MAX, u16::MAX - 1)));
        assert!(!contains_rect(&r, &RectU16::of(1, 1, u16::MAX - 1, u16::MAX)));

        assert!(!contains_rect(&r, &RectU16::of(1, 1, u16::MAX, u16::MAX)));
        assert!(!contains_rect(&r, &RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1)));
    }
}
