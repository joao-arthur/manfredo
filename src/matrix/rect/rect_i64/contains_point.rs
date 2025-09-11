use crate::matrix::{point::point_i64::PointI64, rect::rect_i64::Rect};

pub fn contains_point(r: &Rect, p: &PointI64) -> bool {
    p.row >= r.min.row && p.row <= r.max.row && p.col >= r.min.col && p.col <= r.max.col
}

#[cfg(test)]
mod tests {
    use super::contains_point;
    use crate::matrix::{point::point_i64::PointI64, rect::rect_i64::Rect};

    const MIN: i64 = i64::MIN;
    const MAX: i64 = i64::MAX;

    #[test]
    fn inside_borders() {
        let r = Rect::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(contains_point(&r, &PointI64::of(MIN + 1, MIN + 1)));
        assert!(contains_point(&r, &PointI64::of(MIN + 1, MAX - 1)));
        assert!(contains_point(&r, &PointI64::of(MAX - 1, MIN + 1)));
        assert!(contains_point(&r, &PointI64::of(MAX - 1, MAX - 1)));
    }

    #[test]
    fn outside_borders() {
        let r = Rect::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(!contains_point(&r, &PointI64::min()));
        assert!(!contains_point(&r, &PointI64::of(MIN + 1, MIN)));
        assert!(!contains_point(&r, &PointI64::of(MIN, MIN + 1)));

        assert!(!contains_point(&r, &PointI64::of(MIN, MAX)));
        assert!(!contains_point(&r, &PointI64::of(MIN, MAX - 1)));
        assert!(!contains_point(&r, &PointI64::of(MIN + 1, MAX)));

        assert!(!contains_point(&r, &PointI64::of(MAX, MIN)));
        assert!(!contains_point(&r, &PointI64::of(MAX, MIN + 1)));
        assert!(!contains_point(&r, &PointI64::of(MAX - 1, MIN)));

        assert!(!contains_point(&r, &PointI64::max()));
        assert!(!contains_point(&r, &PointI64::of(MAX - 1, MAX)));
        assert!(!contains_point(&r, &PointI64::of(MAX, MAX - 1)));
    }

    #[test]
    fn inside() {
        let r = Rect::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(contains_point(&r, &PointI64::of(MIN + 10, MIN + 10)));
        assert!(contains_point(&r, &PointI64::of(MAX - 10, MIN + 10)));
        assert!(contains_point(&r, &PointI64::of(MIN + 10, MAX - 10)));
        assert!(contains_point(&r, &PointI64::of(MAX - 10, MAX - 10)));
    }
}
