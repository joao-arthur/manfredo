use crate::cartesian::{point::point_i32::PointI32, rect::rect_i32::RectI32};

pub fn contains_point(r: &RectI32, p: &PointI32) -> bool {
    p.x >= r.min.x && p.x <= r.max.x && p.y >= r.min.y && p.y <= r.max.y
}

#[cfg(test)]
mod tests {
    use super::contains_point;
    use crate::cartesian::{point::point_i32::PointI32, rect::rect_i32::RectI32};

    #[test]
    fn contains_point_inside_borders() {
        let r = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1);
        assert!(contains_point(&r, &PointI32::of(i32::MIN + 1, i32::MIN + 1)));
        assert!(contains_point(&r, &PointI32::of(i32::MIN + 1, i32::MAX - 1)));
        assert!(contains_point(&r, &PointI32::of(i32::MAX - 1, i32::MIN + 1)));
        assert!(contains_point(&r, &PointI32::of(i32::MAX - 1, i32::MAX - 1)));
    }

    #[test]
    fn contains_point_outside_borders() {
        let r = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1);
        assert!(!contains_point(&r, &PointI32::min()));
        assert!(!contains_point(&r, &PointI32::of(i32::MIN + 1, i32::MIN)));
        assert!(!contains_point(&r, &PointI32::of(i32::MIN, i32::MIN + 1)));

        assert!(!contains_point(&r, &PointI32::of(i32::MIN, i32::MAX)));
        assert!(!contains_point(&r, &PointI32::of(i32::MIN, i32::MAX - 1)));
        assert!(!contains_point(&r, &PointI32::of(i32::MIN + 1, i32::MAX)));

        assert!(!contains_point(&r, &PointI32::of(i32::MAX, i32::MIN)));
        assert!(!contains_point(&r, &PointI32::of(i32::MAX, i32::MIN + 1)));
        assert!(!contains_point(&r, &PointI32::of(i32::MAX - 1, i32::MIN)));

        assert!(!contains_point(&r, &PointI32::max()));
        assert!(!contains_point(&r, &PointI32::of(i32::MAX - 1, i32::MAX)));
        assert!(!contains_point(&r, &PointI32::of(i32::MAX, i32::MAX - 1)));
    }

    #[test]
    fn contains_point_inside() {
        let r = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1);
        assert!(contains_point(&r, &PointI32::of(i32::MIN + 10, i32::MIN + 10)));
        assert!(contains_point(&r, &PointI32::of(i32::MAX - 10, i32::MIN + 10)));
        assert!(contains_point(&r, &PointI32::of(i32::MIN + 10, i32::MAX - 10)));
        assert!(contains_point(&r, &PointI32::of(i32::MAX - 10, i32::MAX - 10)));
    }
}
