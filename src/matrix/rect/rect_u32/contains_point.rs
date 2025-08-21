use crate::matrix::{point::point_u32::PointU32, rect::rect_u32::RectU32};

pub fn contains_point(r: &RectU32, p: &PointU32) -> bool {
    p.row >= r.min.row && p.row <= r.max.row && p.col >= r.min.col && p.col <= r.max.col
}

#[cfg(test)]
mod tests {
    use crate::matrix::{point::point_u32::PointU32, rect::rect_u32::RectU32};

    use super::contains_point;

    #[test]
    fn contains_point_inside_borders() {
        let r = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
        assert!(contains_point(&r, &PointU32::of(1, 1)));
        assert!(contains_point(&r, &PointU32::of(1, u32::MAX - 1)));
        assert!(contains_point(&r, &PointU32::of(u32::MAX - 1, 1)));
        assert!(contains_point(&r, &PointU32::of(u32::MAX - 1, u32::MAX - 1)));
    }

    #[test]
    fn contains_point_outside_borders() {
        let r = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
        assert!(!contains_point(&r, &PointU32::min()));
        assert!(!contains_point(&r, &PointU32::of(1, 0)));
        assert!(!contains_point(&r, &PointU32::of(0, 1)));

        assert!(!contains_point(&r, &PointU32::of(0, u32::MAX)));
        assert!(!contains_point(&r, &PointU32::of(0, u32::MAX - 1)));
        assert!(!contains_point(&r, &PointU32::of(1, u32::MAX)));

        assert!(!contains_point(&r, &PointU32::of(u32::MAX, 0)));
        assert!(!contains_point(&r, &PointU32::of(u32::MAX, 1)));
        assert!(!contains_point(&r, &PointU32::of(u32::MAX - 1, 0)));

        assert!(!contains_point(&r, &PointU32::max()));
        assert!(!contains_point(&r, &PointU32::of(u32::MAX - 1, u32::MAX)));
        assert!(!contains_point(&r, &PointU32::of(u32::MAX, u32::MAX - 1)));
    }

    #[test]
    fn contains_point_inside() {
        let r = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
        assert!(contains_point(&r, &PointU32::of(10, 10)));
        assert!(contains_point(&r, &PointU32::of(u32::MAX - 10, 10)));
        assert!(contains_point(&r, &PointU32::of(10, u32::MAX - 10)));
        assert!(contains_point(&r, &PointU32::of(u32::MAX - 10, u32::MAX - 10)));
    }
}
