use crate::matrix::{point::point_i64::PointI64, rect::rect_i64::RectI64};

pub fn contains_point(r: &RectI64, p: &PointI64) -> bool {
    p.row >= r.min.row && p.row <= r.max.row && p.col >= r.min.col && p.col <= r.max.col
}

#[cfg(test)]
mod tests {
    use super::contains_point;
    use crate::matrix::{point::point_i64::PointI64, rect::rect_i64::RectI64};

    #[test]
    fn contains_point_inside_borders() {
        let r = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1);
        assert!(contains_point(&r, &PointI64::of(i64::MIN + 1, i64::MIN + 1)));
        assert!(contains_point(&r, &PointI64::of(i64::MIN + 1, i64::MAX - 1)));
        assert!(contains_point(&r, &PointI64::of(i64::MAX - 1, i64::MIN + 1)));
        assert!(contains_point(&r, &PointI64::of(i64::MAX - 1, i64::MAX - 1)));
    }

    #[test]
    fn contains_point_outside_borders() {
        let r = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1);
        assert!(!contains_point(&r, &PointI64::min()));
        assert!(!contains_point(&r, &PointI64::of(i64::MIN + 1, i64::MIN)));
        assert!(!contains_point(&r, &PointI64::of(i64::MIN, i64::MIN + 1)));

        assert!(!contains_point(&r, &PointI64::of(i64::MIN, i64::MAX)));
        assert!(!contains_point(&r, &PointI64::of(i64::MIN, i64::MAX - 1)));
        assert!(!contains_point(&r, &PointI64::of(i64::MIN + 1, i64::MAX)));

        assert!(!contains_point(&r, &PointI64::of(i64::MAX, i64::MIN)));
        assert!(!contains_point(&r, &PointI64::of(i64::MAX, i64::MIN + 1)));
        assert!(!contains_point(&r, &PointI64::of(i64::MAX - 1, i64::MIN)));

        assert!(!contains_point(&r, &PointI64::max()));
        assert!(!contains_point(&r, &PointI64::of(i64::MAX - 1, i64::MAX)));
        assert!(!contains_point(&r, &PointI64::of(i64::MAX, i64::MAX - 1)));
    }

    #[test]
    fn contains_point_inside() {
        let r = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1);
        assert!(contains_point(&r, &PointI64::of(i64::MIN + 10, i64::MIN + 10)));
        assert!(contains_point(&r, &PointI64::of(i64::MAX - 10, i64::MIN + 10)));
        assert!(contains_point(&r, &PointI64::of(i64::MIN + 10, i64::MAX - 10)));
        assert!(contains_point(&r, &PointI64::of(i64::MAX - 10, i64::MAX - 10)));
    }
}
