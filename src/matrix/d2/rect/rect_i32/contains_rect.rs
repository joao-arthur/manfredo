use super::Rect;

pub fn contains_rect(outer: &Rect, r: &Rect) -> bool {
    r.min.row >= outer.min.row && r.max.row <= outer.max.row && r.min.col >= outer.min.col && r.max.col <= outer.max.col
}

#[cfg(test)]
mod tests {
    use super::contains_rect;
    use crate::matrix::d2::rect::rect_i32::Rect;

    const MIN: i32 = i32::MIN;
    const MAX: i32 = i32::MAX;

    #[test]
    fn inside() {
        let r = Rect::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(contains_rect(&r, &Rect::of(MIN + 2, MIN + 2, MAX - 2, MAX - 2)));
        assert!(contains_rect(&r, &Rect::of(MIN + 3, MIN + 3, MAX - 3, MAX - 3)));
        assert!(contains_rect(&r, &Rect::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10)));
    }

    #[test]
    fn borders() {
        let r = Rect::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(contains_rect(&r, &Rect::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1)));

        assert!(contains_rect(&r, &Rect::of(MIN + 2, MIN + 1, MAX - 1, MAX - 1)));
        assert!(contains_rect(&r, &Rect::of(MIN + 1, MIN + 2, MAX - 1, MAX - 1)));
        assert!(contains_rect(&r, &Rect::of(MIN + 1, MIN + 1, MAX - 2, MAX - 1)));
        assert!(contains_rect(&r, &Rect::of(MIN + 1, MIN + 1, MAX - 1, MAX - 2)));

        assert!(contains_rect(&r, &Rect::of(MIN + 1, MIN + 1, MAX - 2, MAX - 2)));
        assert!(contains_rect(&r, &Rect::of(MIN + 2, MIN + 2, MAX - 1, MAX - 1)));
    }

    #[test]
    fn outside() {
        let r = Rect::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(!contains_rect(&r, &Rect::largest()));

        assert!(!contains_rect(&r, &Rect::of(MIN, MIN + 1, MAX - 1, MAX - 1)));
        assert!(!contains_rect(&r, &Rect::of(MIN + 1, MIN, MAX - 1, MAX - 1)));
        assert!(!contains_rect(&r, &Rect::of(MIN + 1, MIN + 1, MAX, MAX - 1)));
        assert!(!contains_rect(&r, &Rect::of(MIN + 1, MIN + 1, MAX - 1, MAX)));

        assert!(!contains_rect(&r, &Rect::of(MIN + 1, MIN + 1, MAX, MAX)));
        assert!(!contains_rect(&r, &Rect::of(MIN, MIN, MAX - 1, MAX - 1)));
    }
}
