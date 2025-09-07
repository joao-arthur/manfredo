use crate::cartesian::{point::point_u8::PointU8, rect::rect_u8::RectU8};

pub fn contains_point(r: &RectU8, p: &PointU8) -> bool {
    p.x >= r.min.x && p.x <= r.max.x && p.y >= r.min.y && p.y <= r.max.y
}

#[cfg(test)]
mod tests {
    use super::contains_point;
    use crate::cartesian::{point::point_u8::PointU8, rect::rect_u8::RectU8};

    const MAX: u8 = u8::MAX;

    #[test]
    fn inside_borders() {
        let r = RectU8::of(1, 1, MAX - 1, MAX - 1);
        assert!(contains_point(&r, &PointU8::of(1, 1)));
        assert!(contains_point(&r, &PointU8::of(1, MAX - 1)));
        assert!(contains_point(&r, &PointU8::of(MAX - 1, 1)));
        assert!(contains_point(&r, &PointU8::of(MAX - 1, MAX - 1)));
    }

    #[test]
    fn outside_borders() {
        let r = RectU8::of(1, 1, MAX - 1, MAX - 1);
        assert!(!contains_point(&r, &PointU8::min()));
        assert!(!contains_point(&r, &PointU8::of(1, 0)));
        assert!(!contains_point(&r, &PointU8::of(0, 1)));

        assert!(!contains_point(&r, &PointU8::of(0, MAX)));
        assert!(!contains_point(&r, &PointU8::of(0, MAX - 1)));
        assert!(!contains_point(&r, &PointU8::of(1, MAX)));

        assert!(!contains_point(&r, &PointU8::of(MAX, 0)));
        assert!(!contains_point(&r, &PointU8::of(MAX, 1)));
        assert!(!contains_point(&r, &PointU8::of(MAX - 1, 0)));

        assert!(!contains_point(&r, &PointU8::max()));
        assert!(!contains_point(&r, &PointU8::of(MAX - 1, MAX)));
        assert!(!contains_point(&r, &PointU8::of(MAX, MAX - 1)));
    }

    #[test]
    fn inside() {
        let r = RectU8::of(1, 1, MAX - 1, MAX - 1);
        assert!(contains_point(&r, &PointU8::of(10, 10)));
        assert!(contains_point(&r, &PointU8::of(MAX - 10, 10)));
        assert!(contains_point(&r, &PointU8::of(10, MAX - 10)));
        assert!(contains_point(&r, &PointU8::of(MAX - 10, MAX - 10)));
    }
}
