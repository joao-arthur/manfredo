use crate::matrix::{point::point_u64::PointU64, rect::rect_u64::RectU64};

pub fn contains_point(r: &RectU64, p: &PointU64) -> bool {
    p.row >= r.min.row && p.row <= r.max.row && p.col >= r.min.col && p.col <= r.max.col
}

#[cfg(test)]
mod tests {
    use super::contains_point;
    use crate::matrix::{point::point_u64::PointU64, rect::rect_u64::RectU64};

    const MAX: u64 = u64::MAX;

    #[test]
    fn inside_borders() {
        let r = RectU64::of(1, 1, MAX - 1, MAX - 1);
        assert!(contains_point(&r, &PointU64::of(1, 1)));
        assert!(contains_point(&r, &PointU64::of(1, MAX - 1)));
        assert!(contains_point(&r, &PointU64::of(MAX - 1, 1)));
        assert!(contains_point(&r, &PointU64::of(MAX - 1, MAX - 1)));
    }

    #[test]
    fn outside_borders() {
        let r = RectU64::of(1, 1, MAX - 1, MAX - 1);
        assert!(!contains_point(&r, &PointU64::min()));
        assert!(!contains_point(&r, &PointU64::of(1, 0)));
        assert!(!contains_point(&r, &PointU64::of(0, 1)));

        assert!(!contains_point(&r, &PointU64::of(0, MAX)));
        assert!(!contains_point(&r, &PointU64::of(0, MAX - 1)));
        assert!(!contains_point(&r, &PointU64::of(1, MAX)));

        assert!(!contains_point(&r, &PointU64::of(MAX, 0)));
        assert!(!contains_point(&r, &PointU64::of(MAX, 1)));
        assert!(!contains_point(&r, &PointU64::of(MAX - 1, 0)));

        assert!(!contains_point(&r, &PointU64::max()));
        assert!(!contains_point(&r, &PointU64::of(MAX - 1, MAX)));
        assert!(!contains_point(&r, &PointU64::of(MAX, MAX - 1)));
    }

    #[test]
    fn inside() {
        let r = RectU64::of(1, 1, MAX - 1, MAX - 1);
        assert!(contains_point(&r, &PointU64::of(10, 10)));
        assert!(contains_point(&r, &PointU64::of(MAX - 10, 10)));
        assert!(contains_point(&r, &PointU64::of(10, MAX - 10)));
        assert!(contains_point(&r, &PointU64::of(MAX - 10, MAX - 10)));
    }
}
