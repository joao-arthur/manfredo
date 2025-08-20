use crate::matrix::{point::point_u64::PointU64, rect::rect_u64::RectU64};

pub fn contains_point(outer: &RectU64, p: &PointU64) -> bool {
    p.row >= outer.min.row && p.row <= outer.max.row && p.col >= outer.min.col && p.col <= outer.max.col
}

#[cfg(test)]
mod tests {
    use crate::matrix::{point::point_u64::PointU64, rect::rect_u64::RectU64};

    use super::contains_point;

    #[test]
    fn contains_point_inside_borders() {
        let r = RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1);
        assert!(contains_point(&r, &PointU64::of(1, 1)));
        assert!(contains_point(&r, &PointU64::of(1, u64::MAX - 1)));
        assert!(contains_point(&r, &PointU64::of(u64::MAX - 1, 1)));
        assert!(contains_point(&r, &PointU64::of(u64::MAX - 1, u64::MAX - 1)));
    }

    #[test]
    fn contains_point_outside_borders() {
        let r = RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1);
        assert!(!contains_point(&r, &PointU64::min()));
        assert!(!contains_point(&r, &PointU64::of(1, 0)));
        assert!(!contains_point(&r, &PointU64::of(0, 1)));

        assert!(!contains_point(&r, &PointU64::of(0, u64::MAX)));
        assert!(!contains_point(&r, &PointU64::of(0, u64::MAX - 1)));
        assert!(!contains_point(&r, &PointU64::of(1, u64::MAX)));

        assert!(!contains_point(&r, &PointU64::of(u64::MAX, 0)));
        assert!(!contains_point(&r, &PointU64::of(u64::MAX, 1)));
        assert!(!contains_point(&r, &PointU64::of(u64::MAX - 1, 0)));

        assert!(!contains_point(&r, &PointU64::max()));
        assert!(!contains_point(&r, &PointU64::of(u64::MAX - 1, u64::MAX)));
        assert!(!contains_point(&r, &PointU64::of(u64::MAX, u64::MAX - 1)));
    }

    #[test]
    fn contains_point_inside() {
        let r = RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1);
        assert!(contains_point(&r, &PointU64::of(10, 10)));
        assert!(contains_point(&r, &PointU64::of(u64::MAX - 10, 10)));
        assert!(contains_point(&r, &PointU64::of(10, u64::MAX - 10)));
        assert!(contains_point(&r, &PointU64::of(u64::MAX - 10, u64::MAX - 10)));
    }
}
