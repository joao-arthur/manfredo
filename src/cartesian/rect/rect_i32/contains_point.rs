use crate::cartesian::{point::point_i32::PointI32, rect::rect_i32::RectI32};

pub fn contains_point(r: &RectI32, p: &PointI32) -> bool {
    p.x >= r.min.x && p.x <= r.max.x && p.y >= r.min.y && p.y <= r.max.y
}

#[cfg(test)]
mod tests {
    use super::contains_point;
    use crate::cartesian::{point::point_i32::PointI32, rect::rect_i32::RectI32};

    const MIN: i32 = i32::MIN;
    const MAX: i32 = i32::MAX;

    #[test]
    fn contains_point_inside_borders() {
        let r = RectI32::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(contains_point(&r, &PointI32::of(MIN + 1, MIN + 1)));
        assert!(contains_point(&r, &PointI32::of(MIN + 1, MAX - 1)));
        assert!(contains_point(&r, &PointI32::of(MAX - 1, MIN + 1)));
        assert!(contains_point(&r, &PointI32::of(MAX - 1, MAX - 1)));
    }

    #[test]
    fn contains_point_outside_borders() {
        let r = RectI32::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(!contains_point(&r, &PointI32::min()));
        assert!(!contains_point(&r, &PointI32::of(MIN + 1, MIN)));
        assert!(!contains_point(&r, &PointI32::of(MIN, MIN + 1)));

        assert!(!contains_point(&r, &PointI32::of(MIN, MAX)));
        assert!(!contains_point(&r, &PointI32::of(MIN, MAX - 1)));
        assert!(!contains_point(&r, &PointI32::of(MIN + 1, MAX)));

        assert!(!contains_point(&r, &PointI32::of(MAX, MIN)));
        assert!(!contains_point(&r, &PointI32::of(MAX, MIN + 1)));
        assert!(!contains_point(&r, &PointI32::of(MAX - 1, MIN)));

        assert!(!contains_point(&r, &PointI32::max()));
        assert!(!contains_point(&r, &PointI32::of(MAX - 1, MAX)));
        assert!(!contains_point(&r, &PointI32::of(MAX, MAX - 1)));
    }

    #[test]
    fn contains_point_inside() {
        let r = RectI32::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(contains_point(&r, &PointI32::of(MIN + 10, MIN + 10)));
        assert!(contains_point(&r, &PointI32::of(MAX - 10, MIN + 10)));
        assert!(contains_point(&r, &PointI32::of(MIN + 10, MAX - 10)));
        assert!(contains_point(&r, &PointI32::of(MAX - 10, MAX - 10)));
    }
}
