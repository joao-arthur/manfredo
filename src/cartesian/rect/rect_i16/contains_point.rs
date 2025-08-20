use crate::cartesian::{point::point_i16::PointI16, rect::rect_i16::RectI16};

pub fn contains_point(outer: &RectI16, p: &PointI16) -> bool {
    p.x >= outer.min.x && p.x <= outer.max.x && p.y >= outer.min.y && p.y <= outer.max.y
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{point::point_i16::PointI16, rect::rect_i16::RectI16};

    use super::contains_point;

    #[test]
    fn contains_point_inside_borders() {
        let r = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assert!(contains_point(&r, &PointI16::of(i16::MIN + 1, i16::MIN + 1)));
        assert!(contains_point(&r, &PointI16::of(i16::MIN + 1, i16::MAX - 1)));
        assert!(contains_point(&r, &PointI16::of(i16::MAX - 1, i16::MIN + 1)));
        assert!(contains_point(&r, &PointI16::of(i16::MAX - 1, i16::MAX - 1)));
    }

    #[test]
    fn contains_point_outside_borders() {
        let r = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assert!(!contains_point(&r, &PointI16::min()));
        assert!(!contains_point(&r, &PointI16::of(i16::MIN + 1, i16::MIN)));
        assert!(!contains_point(&r, &PointI16::of(i16::MIN, i16::MIN + 1)));

        assert!(!contains_point(&r, &PointI16::of(i16::MIN, i16::MAX)));
        assert!(!contains_point(&r, &PointI16::of(i16::MIN, i16::MAX - 1)));
        assert!(!contains_point(&r, &PointI16::of(i16::MIN + 1, i16::MAX)));

        assert!(!contains_point(&r, &PointI16::of(i16::MAX, i16::MIN)));
        assert!(!contains_point(&r, &PointI16::of(i16::MAX, i16::MIN + 1)));
        assert!(!contains_point(&r, &PointI16::of(i16::MAX - 1, i16::MIN)));

        assert!(!contains_point(&r, &PointI16::max()));
        assert!(!contains_point(&r, &PointI16::of(i16::MAX - 1, i16::MAX)));
        assert!(!contains_point(&r, &PointI16::of(i16::MAX, i16::MAX - 1)));
    }

    #[test]
    fn contains_point_inside() {
        let r = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assert!(contains_point(&r, &PointI16::of(i16::MIN + 10, i16::MIN + 10)));
        assert!(contains_point(&r, &PointI16::of(i16::MAX - 10, i16::MIN + 10)));
        assert!(contains_point(&r, &PointI16::of(i16::MIN + 10, i16::MAX - 10)));
        assert!(contains_point(&r, &PointI16::of(i16::MAX - 10, i16::MAX - 10)));
    }
}
