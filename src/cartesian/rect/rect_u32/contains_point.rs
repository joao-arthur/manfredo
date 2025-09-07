use crate::cartesian::{point::point_u32::PointU32, rect::rect_u32::RectU32};

pub fn contains_point(r: &RectU32, p: &PointU32) -> bool {
    p.x >= r.min.x && p.x <= r.max.x && p.y >= r.min.y && p.y <= r.max.y
}

#[cfg(test)]
mod tests {
    use super::contains_point;
    use crate::cartesian::{point::point_u32::PointU32, rect::rect_u32::RectU32};

    const MAX: u32 = u32::MAX;

    #[test]
    fn inside_borders() {
        let r = RectU32::of(1, 1, MAX - 1, MAX - 1);
        assert!(contains_point(&r, &PointU32::of(1, 1)));
        assert!(contains_point(&r, &PointU32::of(1, MAX - 1)));
        assert!(contains_point(&r, &PointU32::of(MAX - 1, 1)));
        assert!(contains_point(&r, &PointU32::of(MAX - 1, MAX - 1)));
    }

    #[test]
    fn outside_borders() {
        let r = RectU32::of(1, 1, MAX - 1, MAX - 1);
        assert!(!contains_point(&r, &PointU32::min()));
        assert!(!contains_point(&r, &PointU32::of(1, 0)));
        assert!(!contains_point(&r, &PointU32::of(0, 1)));

        assert!(!contains_point(&r, &PointU32::of(0, MAX)));
        assert!(!contains_point(&r, &PointU32::of(0, MAX - 1)));
        assert!(!contains_point(&r, &PointU32::of(1, MAX)));

        assert!(!contains_point(&r, &PointU32::of(MAX, 0)));
        assert!(!contains_point(&r, &PointU32::of(MAX, 1)));
        assert!(!contains_point(&r, &PointU32::of(MAX - 1, 0)));

        assert!(!contains_point(&r, &PointU32::max()));
        assert!(!contains_point(&r, &PointU32::of(MAX - 1, MAX)));
        assert!(!contains_point(&r, &PointU32::of(MAX, MAX - 1)));
    }

    #[test]
    fn inside() {
        let r = RectU32::of(1, 1, MAX - 1, MAX - 1);
        assert!(contains_point(&r, &PointU32::of(10, 10)));
        assert!(contains_point(&r, &PointU32::of(MAX - 10, 10)));
        assert!(contains_point(&r, &PointU32::of(10, MAX - 10)));
        assert!(contains_point(&r, &PointU32::of(MAX - 10, MAX - 10)));
    }
}
