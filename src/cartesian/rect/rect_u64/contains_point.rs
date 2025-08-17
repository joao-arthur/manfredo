use crate::cartesian::{point::point_u64::PointU64, rect::rect_u64::RectU64};

pub fn contains_point(outer: &RectU64, p: &PointU64) -> bool {
    p.x >= outer.min.x && p.x <= outer.max.x && p.y >= outer.min.y && p.y <= outer.max.y
}

pub fn contains_rect(outer: &RectU64, r: &RectU64) -> bool {
    r.min.x >= outer.min.x && r.max.x <= outer.max.x && r.min.y >= outer.min.y && r.max.y <= outer.max.y
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{point::point_u64::PointU64, rect::rect_u64::RectU64};

    use super::{contains_point, contains_rect};

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
        assert!(!contains_point(&r, &PointU64::of(0, 0)));
        assert!(!contains_point(&r, &PointU64::of(0, u64::MAX)));
        assert!(!contains_point(&r, &PointU64::of(u64::MAX, 0)));
        assert!(!contains_point(&r, &PointU64::max()));
        assert!(!contains_point(&r, &PointU64::of(1, 0)));
        assert!(!contains_point(&r, &PointU64::of(1, u64::MAX)));
        assert!(!contains_point(&r, &PointU64::of(u64::MAX - 1, 0)));
        assert!(!contains_point(&r, &PointU64::of(u64::MAX - 1, u64::MAX)));
        assert!(!contains_point(&r, &PointU64::of(0, 1)));
        assert!(!contains_point(&r, &PointU64::of(0, u64::MAX - 1)));
        assert!(!contains_point(&r, &PointU64::of(u64::MAX, 1)));
        assert!(!contains_point(&r, &PointU64::of(u64::MAX, u64::MAX - 1)));
    }

    #[test]
    fn contains_point_inside() {
        let r = RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1);
        assert!(contains_point(&r, &PointU64::of(10, 10)));
        assert!(contains_point(&r, &PointU64::of(u64::MAX - 10, u64::MAX - 10)));
    }

    #[test]
    fn contains_rect_inside_borders() {
        let r = RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1);
        assert!(contains_rect(&r, &RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1)));
        assert!(contains_rect(&r, &RectU64::of(1, 1, u64::MAX - 2, u64::MAX - 2)));
        assert!(contains_rect(&r, &RectU64::of(2, 2, u64::MAX - 1, u64::MAX - 1)));
    }

    #[test]
    fn contains_rect_outside_borders() {
        let r = RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1);
        assert!(!contains_rect(&r, &RectU64::of(0, 1, u64::MAX - 1, u64::MAX - 1)));
        assert!(!contains_rect(&r, &RectU64::of(1, 0, u64::MAX - 1, u64::MAX - 1)));
        assert!(!contains_rect(&r, &RectU64::of(1, 1, u64::MAX, u64::MAX - 1)));
        assert!(!contains_rect(&r, &RectU64::of(1, 1, u64::MAX - 1, u64::MAX)));
        assert!(!contains_rect(&r, &RectU64::of(1, 1, u64::MAX, u64::MAX)));
        assert!(!contains_rect(&r, &RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1)));
        assert!(!contains_rect(&r, &RectU64::largest()));
    }

    #[test]
    fn contains_rect_inside() {
        let r = RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1);
        assert!(contains_rect(&r, &RectU64::of(2, 2, u64::MAX - 2, u64::MAX - 2)));
        assert!(contains_rect(&r, &RectU64::of(3, 3, u64::MAX - 3, u64::MAX - 3)));
        assert!(contains_rect(&r, &RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10)));
    }
}
