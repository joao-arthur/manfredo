use crate::matrix::{point::point_i8::PointI8, rect::rect_i8::RectI8};

pub fn contains_point(r: &RectI8, p: &PointI8) -> bool {
    p.row >= r.min.row && p.row <= r.max.row && p.col >= r.min.col && p.col <= r.max.col
}

#[cfg(test)]
mod tests {
    use super::contains_point;
    use crate::matrix::{point::point_i8::PointI8, rect::rect_i8::RectI8};

    #[test]
    fn contains_point_inside_borders() {
        let r = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1);
        assert!(contains_point(&r, &PointI8::of(i8::MIN + 1, i8::MIN + 1)));
        assert!(contains_point(&r, &PointI8::of(i8::MIN + 1, i8::MAX - 1)));
        assert!(contains_point(&r, &PointI8::of(i8::MAX - 1, i8::MIN + 1)));
        assert!(contains_point(&r, &PointI8::of(i8::MAX - 1, i8::MAX - 1)));
    }

    #[test]
    fn contains_point_outside_borders() {
        let r = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1);
        assert!(!contains_point(&r, &PointI8::min()));
        assert!(!contains_point(&r, &PointI8::of(i8::MIN + 1, i8::MIN)));
        assert!(!contains_point(&r, &PointI8::of(i8::MIN, i8::MIN + 1)));

        assert!(!contains_point(&r, &PointI8::of(i8::MIN, i8::MAX)));
        assert!(!contains_point(&r, &PointI8::of(i8::MIN, i8::MAX - 1)));
        assert!(!contains_point(&r, &PointI8::of(i8::MIN + 1, i8::MAX)));

        assert!(!contains_point(&r, &PointI8::of(i8::MAX, i8::MIN)));
        assert!(!contains_point(&r, &PointI8::of(i8::MAX, i8::MIN + 1)));
        assert!(!contains_point(&r, &PointI8::of(i8::MAX - 1, i8::MIN)));

        assert!(!contains_point(&r, &PointI8::max()));
        assert!(!contains_point(&r, &PointI8::of(i8::MAX - 1, i8::MAX)));
        assert!(!contains_point(&r, &PointI8::of(i8::MAX, i8::MAX - 1)));
    }

    #[test]
    fn contains_point_inside() {
        let r = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1);
        assert!(contains_point(&r, &PointI8::of(i8::MIN + 10, i8::MIN + 10)));
        assert!(contains_point(&r, &PointI8::of(i8::MAX - 10, i8::MIN + 10)));
        assert!(contains_point(&r, &PointI8::of(i8::MIN + 10, i8::MAX - 10)));
        assert!(contains_point(&r, &PointI8::of(i8::MAX - 10, i8::MAX - 10)));
    }
}
