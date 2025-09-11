use crate::cartesian::{point::point_i8::PointI8, rect::rect_i8::Rect};

pub fn contains_point(r: &Rect, p: &PointI8) -> bool {
    p.x >= r.min.x && p.x <= r.max.x && p.y >= r.min.y && p.y <= r.max.y
}

#[cfg(test)]
mod tests {
    use super::contains_point;
    use crate::cartesian::{point::point_i8::PointI8, rect::rect_i8::Rect};

    const MIN: i8 = i8::MIN;
    const MAX: i8 = i8::MAX;

    #[test]
    fn inside_borders() {
        let r = Rect::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(contains_point(&r, &PointI8::of(MIN + 1, MIN + 1)));
        assert!(contains_point(&r, &PointI8::of(MIN + 1, MAX - 1)));
        assert!(contains_point(&r, &PointI8::of(MAX - 1, MIN + 1)));
        assert!(contains_point(&r, &PointI8::of(MAX - 1, MAX - 1)));
    }

    #[test]
    fn outside_borders() {
        let r = Rect::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(!contains_point(&r, &PointI8::min()));
        assert!(!contains_point(&r, &PointI8::of(MIN + 1, MIN)));
        assert!(!contains_point(&r, &PointI8::of(MIN, MIN + 1)));

        assert!(!contains_point(&r, &PointI8::of(MIN, MAX)));
        assert!(!contains_point(&r, &PointI8::of(MIN, MAX - 1)));
        assert!(!contains_point(&r, &PointI8::of(MIN + 1, MAX)));

        assert!(!contains_point(&r, &PointI8::of(MAX, MIN)));
        assert!(!contains_point(&r, &PointI8::of(MAX, MIN + 1)));
        assert!(!contains_point(&r, &PointI8::of(MAX - 1, MIN)));

        assert!(!contains_point(&r, &PointI8::max()));
        assert!(!contains_point(&r, &PointI8::of(MAX - 1, MAX)));
        assert!(!contains_point(&r, &PointI8::of(MAX, MAX - 1)));
    }

    #[test]
    fn inside() {
        let r = Rect::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(contains_point(&r, &PointI8::of(MIN + 10, MIN + 10)));
        assert!(contains_point(&r, &PointI8::of(MAX - 10, MIN + 10)));
        assert!(contains_point(&r, &PointI8::of(MIN + 10, MAX - 10)));
        assert!(contains_point(&r, &PointI8::of(MAX - 10, MAX - 10)));
    }
}
