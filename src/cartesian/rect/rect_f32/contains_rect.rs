use crate::cartesian::rect::rect_f32::Rect;

pub fn contains_rect(outer: &Rect, r: &Rect) -> bool {
    r.min.x >= outer.min.x && r.max.x <= outer.max.x && r.min.y >= outer.min.y && r.max.y <= outer.max.y
}

#[cfg(test)]
mod tests {
    use super::contains_rect;
    use crate::cartesian::{
        point::point_f32::{MAX, MIN},
        rect::rect_f32::Rect,
    };

    #[test]
    fn inside() {
        let r_negative = Rect::of(MIN + 1.0, MIN + 1.0, -1.0, -1.0);
        assert!(contains_rect(&r_negative, &Rect::of(MIN + 2.0, MIN + 2.0, -2.0, -2.0)));
        assert!(contains_rect(&r_negative, &Rect::of(MIN + 3.0, MIN + 3.0, -3.0, -3.0)));
        assert!(contains_rect(&r_negative, &Rect::of(MIN + 10.0, MIN + 10.0, -10.0, -10.0)));

        let r_positive = Rect::of(1.0, 1.0, MAX - 1.0, MAX - 1.0);
        assert!(contains_rect(&r_positive, &Rect::of(2.0, 2.0, MAX - 2.0, MAX - 2.0)));
        assert!(contains_rect(&r_positive, &Rect::of(3.0, 3.0, MAX - 3.0, MAX - 3.0)));
        assert!(contains_rect(&r_positive, &Rect::of(10.0, 10.0, MAX - 10.0, MAX - 10.0)));
    }

    #[test]
    fn borders() {
        let r_negative = Rect::of(MIN + 1.0, MIN + 1.0, -1.0, -1.0);
        assert!(contains_rect(&r_negative, &Rect::of(MIN + 1.0, MIN + 1.0, -1.0, -1.0)));

        assert!(contains_rect(&r_negative, &Rect::of(MIN + 2.0, MIN + 1.0, -1.0, -1.0)));
        assert!(contains_rect(&r_negative, &Rect::of(MIN + 1.0, MIN + 2.0, -1.0, -1.0)));
        assert!(contains_rect(&r_negative, &Rect::of(MIN + 1.0, MIN + 1.0, -2.0, -1.0)));
        assert!(contains_rect(&r_negative, &Rect::of(MIN + 1.0, MIN + 1.0, -1.0, -2.0)));

        assert!(contains_rect(&r_negative, &Rect::of(MIN + 1.0, MIN + 1.0, -2.0, -2.0)));
        assert!(contains_rect(&r_negative, &Rect::of(MIN + 2.0, MIN + 2.0, -1.0, -1.0)));

        let r_positive = Rect::of(1.0, 1.0, MAX - 1.0, MAX - 1.0);
        assert!(contains_rect(&r_positive, &Rect::of(1.0, 1.0, MAX - 1.0, MAX - 1.0)));

        assert!(contains_rect(&r_positive, &Rect::of(2.0, 1.0, MAX - 1.0, MAX - 1.0)));
        assert!(contains_rect(&r_positive, &Rect::of(1.0, 2.0, MAX - 1.0, MAX - 1.0)));
        assert!(contains_rect(&r_positive, &Rect::of(1.0, 1.0, MAX - 2.0, MAX - 1.0)));
        assert!(contains_rect(&r_positive, &Rect::of(1.0, 1.0, MAX - 1.0, MAX - 2.0)));

        assert!(contains_rect(&r_positive, &Rect::of(1.0, 1.0, MAX - 2.0, MAX - 2.0)));
        assert!(contains_rect(&r_positive, &Rect::of(2.0, 2.0, MAX - 1.0, MAX - 1.0)));
    }

    #[test]
    fn outside() {
        let r_negative = Rect::of(MIN + 1.0, MIN + 1.0, -1.0, -1.0);
        assert!(!contains_rect(&r_negative, &Rect::largest()));

        assert!(!contains_rect(&r_negative, &Rect::of(MIN, MIN + 1.0, -1.0, -1.0)));
        assert!(!contains_rect(&r_negative, &Rect::of(MIN + 1.0, MIN, -1.0, -1.0)));
        assert!(!contains_rect(&r_negative, &Rect::of(MIN + 1.0, MIN + 1.0, 0.0, -1.0)));
        assert!(!contains_rect(&r_negative, &Rect::of(MIN + 1.0, MIN + 1.0, -1.0, 0.0)));

        assert!(!contains_rect(&r_negative, &Rect::of(MIN + 1.0, MIN + 1.0, 0.0, 0.0)));
        assert!(!contains_rect(&r_negative, &Rect::of(MIN, MIN, -1.0, -1.0)));

        let r_positive = Rect::of(1.0, 1.0, MAX - 1.0, MAX - 1.0);
        assert!(!contains_rect(&r_positive, &Rect::largest()));

        assert!(!contains_rect(&r_positive, &Rect::of(0.0, 1.0, MAX - 1.0, MAX - 1.0)));
        assert!(!contains_rect(&r_positive, &Rect::of(1.0, 0.0, MAX - 1.0, MAX - 1.0)));
        assert!(!contains_rect(&r_positive, &Rect::of(1.0, 1.0, MAX, MAX - 1.0)));
        assert!(!contains_rect(&r_positive, &Rect::of(1.0, 1.0, MAX - 1.0, MAX)));

        assert!(!contains_rect(&r_positive, &Rect::of(1.0, 1.0, MAX, MAX)));
        assert!(!contains_rect(&r_positive, &Rect::of(0.0, 0.0, MAX - 1.0, MAX - 1.0)));
    }
}
