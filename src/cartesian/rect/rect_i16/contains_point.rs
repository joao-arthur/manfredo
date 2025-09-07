use crate::cartesian::{point::point_i16::PointI16, rect::rect_i16::RectI16};

pub fn contains_point(r: &RectI16, p: &PointI16) -> bool {
    p.x >= r.min.x && p.x <= r.max.x && p.y >= r.min.y && p.y <= r.max.y
}

#[cfg(test)]
mod tests {
    use super::contains_point;
    use crate::cartesian::{point::point_i16::PointI16, rect::rect_i16::RectI16};

    const MIN: i16 = i16::MIN;
    const MAX: i16 = i16::MAX;

    #[test]
    fn inside_borders() {
        let r = RectI16::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(contains_point(&r, &PointI16::of(MIN + 1, MIN + 1)));
        assert!(contains_point(&r, &PointI16::of(MIN + 1, MAX - 1)));
        assert!(contains_point(&r, &PointI16::of(MAX - 1, MIN + 1)));
        assert!(contains_point(&r, &PointI16::of(MAX - 1, MAX - 1)));
    }

    #[test]
    fn outside_borders() {
        let r = RectI16::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(!contains_point(&r, &PointI16::min()));
        assert!(!contains_point(&r, &PointI16::of(MIN + 1, MIN)));
        assert!(!contains_point(&r, &PointI16::of(MIN, MIN + 1)));

        assert!(!contains_point(&r, &PointI16::of(MIN, MAX)));
        assert!(!contains_point(&r, &PointI16::of(MIN, MAX - 1)));
        assert!(!contains_point(&r, &PointI16::of(MIN + 1, MAX)));

        assert!(!contains_point(&r, &PointI16::of(MAX, MIN)));
        assert!(!contains_point(&r, &PointI16::of(MAX, MIN + 1)));
        assert!(!contains_point(&r, &PointI16::of(MAX - 1, MIN)));

        assert!(!contains_point(&r, &PointI16::max()));
        assert!(!contains_point(&r, &PointI16::of(MAX - 1, MAX)));
        assert!(!contains_point(&r, &PointI16::of(MAX, MAX - 1)));
    }

    #[test]
    fn inside() {
        let r = RectI16::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(contains_point(&r, &PointI16::of(MIN + 10, MIN + 10)));
        assert!(contains_point(&r, &PointI16::of(MAX - 10, MIN + 10)));
        assert!(contains_point(&r, &PointI16::of(MIN + 10, MAX - 10)));
        assert!(contains_point(&r, &PointI16::of(MAX - 10, MAX - 10)));
    }
}
