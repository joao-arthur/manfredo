use crate::cartesian::{point::point_u16::PointU16, rect::rect_u16::RectU16};

pub fn contains_point(outer: &RectU16, p: &PointU16) -> bool {
    p.x >= outer.min.x && p.x <= outer.max.x && p.y >= outer.min.y && p.y <= outer.max.y
}

pub fn contains_rect(outer: &RectU16, r: &RectU16) -> bool {
    r.min.x >= outer.min.x && r.max.x <= outer.max.x && r.min.y >= outer.min.y && r.max.y <= outer.max.y
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{point::point_u16::PointU16, rect::rect_u16::RectU16};

    use super::{contains_point, contains_rect};

    #[test]
    fn contains_point_inside_borders() {
        let r = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assert!(contains_point(&r, &PointU16::of(1, 1)));
        assert!(contains_point(&r, &PointU16::of(1, u16::MAX - 1)));
        assert!(contains_point(&r, &PointU16::of(u16::MAX - 1, 1)));
        assert!(contains_point(&r, &PointU16::of(u16::MAX - 1, u16::MAX - 1)));
    }

    #[test]
    fn contains_point_outside_borders() {
        let r = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assert!(!contains_point(&r, &PointU16::of(0, 0)));
        assert!(!contains_point(&r, &PointU16::of(0, u16::MAX)));
        assert!(!contains_point(&r, &PointU16::of(u16::MAX, 0)));
        assert!(!contains_point(&r, &PointU16::max()));
        assert!(!contains_point(&r, &PointU16::of(1, 0)));
        assert!(!contains_point(&r, &PointU16::of(1, u16::MAX)));
        assert!(!contains_point(&r, &PointU16::of(u16::MAX - 1, 0)));
        assert!(!contains_point(&r, &PointU16::of(u16::MAX - 1, u16::MAX)));
        assert!(!contains_point(&r, &PointU16::of(0, 1)));
        assert!(!contains_point(&r, &PointU16::of(0, u16::MAX - 1)));
        assert!(!contains_point(&r, &PointU16::of(u16::MAX, 1)));
        assert!(!contains_point(&r, &PointU16::of(u16::MAX, u16::MAX - 1)));
    }

    #[test]
    fn contains_point_inside() {
        let r = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assert!(contains_point(&r, &PointU16::of(10, 10)));
        assert!(contains_point(&r, &PointU16::of(u16::MAX - 10, u16::MAX - 10)));
    }

    #[test]
    fn contains_rect_inside_borders() {
        let r = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assert!(contains_rect(&r, &RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1)));
        assert!(contains_rect(&r, &RectU16::of(1, 1, u16::MAX - 2, u16::MAX - 2)));
        assert!(contains_rect(&r, &RectU16::of(2, 2, u16::MAX - 1, u16::MAX - 1)));
    }

    #[test]
    fn contains_rect_outside_borders() {
        let r = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assert!(!contains_rect(&r, &RectU16::of(0, 1, u16::MAX - 1, u16::MAX - 1)));
        assert!(!contains_rect(&r, &RectU16::of(1, 0, u16::MAX - 1, u16::MAX - 1)));
        assert!(!contains_rect(&r, &RectU16::of(1, 1, u16::MAX, u16::MAX - 1)));
        assert!(!contains_rect(&r, &RectU16::of(1, 1, u16::MAX - 1, u16::MAX)));
        assert!(!contains_rect(&r, &RectU16::of(1, 1, u16::MAX, u16::MAX)));
        assert!(!contains_rect(&r, &RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1)));
        assert!(!contains_rect(&r, &RectU16::of(0, 0, u16::MAX, u16::MAX)));
    }

    #[test]
    fn contains_rect_inside() {
        let r = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assert!(contains_rect(&r, &RectU16::of(2, 2, u16::MAX - 2, u16::MAX - 2)));
        assert!(contains_rect(&r, &RectU16::of(3, 3, u16::MAX - 3, u16::MAX - 3)));
        assert!(contains_rect(&r, &RectU16::of(10, 10, u16::MAX - 10, u16::MAX - 10)));
    }
}
