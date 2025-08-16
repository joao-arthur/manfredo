use crate::cartesian::{rect::rect_u8::RectU8, point::point_u8::PointU8};

pub fn contains_point(outer: &RectU8, p: &PointU8) -> bool {
    p.x >= outer.min.x &&
    p.x <= outer.max.x &&
    p.y >= outer.min.y &&
    p.y <= outer.max.y
}

pub fn contains_rect(outer: &RectU8, r: &RectU8) -> bool {
    r.min.x >= outer.min.x &&
    r.max.x <= outer.max.x &&
    r.min.y >= outer.min.y &&
    r.max.y <= outer.max.y
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{point::point_u8::PointU8, rect::rect_u8::RectU8};

    use super::{contains_point, contains_rect};

    #[test]
    fn contains_point_inside_borders() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(contains_point(&r, &PointU8::of(1, 1)));
        assert!(contains_point(&r, &PointU8::of(1, u8::MAX - 1)));
        assert!(contains_point(&r, &PointU8::of(u8::MAX - 1, 1)));
        assert!(contains_point(&r, &PointU8::of(u8::MAX - 1, u8::MAX - 1)));
    }

    #[test]
    fn contains_point_outside_borders() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(!contains_point(&r, &PointU8::of(0, 0)));
        assert!(!contains_point(&r, &PointU8::of(0, u8::MAX)));
        assert!(!contains_point(&r, &PointU8::of(u8::MAX, 0)));
        assert!(!contains_point(&r, &PointU8::max()));
        assert!(!contains_point(&r, &PointU8::of(1, 0)));
        assert!(!contains_point(&r, &PointU8::of(1, u8::MAX)));
        assert!(!contains_point(&r, &PointU8::of(u8::MAX - 1, 0)));
        assert!(!contains_point(&r, &PointU8::of(u8::MAX - 1, u8::MAX)));
        assert!(!contains_point(&r, &PointU8::of(0, 1)));
        assert!(!contains_point(&r, &PointU8::of(0, u8::MAX - 1)));
        assert!(!contains_point(&r, &PointU8::of(u8::MAX, 1)));
        assert!(!contains_point(&r, &PointU8::of(u8::MAX, u8::MAX - 1)));
    }

    #[test]
    fn contains_point_inside() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(contains_point(&r, &PointU8::of(10, 10)));
        assert!(contains_point(&r, &PointU8::of(u8::MAX - 10, u8::MAX - 10)));
    }

   #[test]
    fn contains_rect_inside_borders() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(contains_rect(&r, &RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1)));
        assert!(contains_rect(&r, &RectU8::of(1, 1, u8::MAX - 2, u8::MAX - 2)));
        assert!(contains_rect(&r, &RectU8::of(2, 2, u8::MAX - 1, u8::MAX - 1)));

    }

    #[test]
    fn contains_rect_outside_borders() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(!contains_rect(&r, &RectU8::of(0, 1, u8::MAX - 1, u8::MAX - 1)));
        assert!(!contains_rect(&r, &RectU8::of(1, 0, u8::MAX - 1, u8::MAX - 1)));
        assert!(!contains_rect(&r, &RectU8::of(1, 1, u8::MAX, u8::MAX - 1)));
        assert!(!contains_rect(&r, &RectU8::of(1, 1, u8::MAX - 1, u8::MAX)));
        assert!(!contains_rect(&r, &RectU8::of(1, 1, u8::MAX, u8::MAX)));
        assert!(!contains_rect(&r, &RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1)));
        assert!(!contains_rect(&r, &RectU8::of(0, 0, u8::MAX, u8::MAX)));
    }

    #[test]
    fn contains_rect_inside() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(contains_rect(&r, &RectU8::of(2, 2, u8::MAX - 2, u8::MAX - 2)));
        assert!(contains_rect(&r, &RectU8::of(3, 3, u8::MAX - 3, u8::MAX - 3)));
        assert!(contains_rect(&r, &RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10)));
    }
}
