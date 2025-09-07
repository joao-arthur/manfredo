use crate::cartesian::{point::point_f64::PointF64, rect::rect_f64::RectF64};

pub fn contains_point(r: &RectF64, p: &PointF64) -> bool {
    p.x >= r.min.x && p.x <= r.max.x && p.y >= r.min.y && p.y <= r.max.y
}

#[cfg(test)]
mod tests {
    use super::contains_point;
    use crate::cartesian::{
        point::point_f64::{MAX, MIN, PointF64},
        rect::rect_f64::RectF64,
    };

    #[test]
    fn inside_borders() {
        let r_negative = RectF64::of(MIN + 1.0, MIN + 1.0, -1.0, -1.0);
        assert!(contains_point(&r_negative, &PointF64::of(MIN + 1.0, MIN + 1.0)));
        assert!(contains_point(&r_negative, &PointF64::of(MIN + 1.0, -1.0)));
        assert!(contains_point(&r_negative, &PointF64::of(-1.0, MIN + 1.0)));
        assert!(contains_point(&r_negative, &PointF64::of(-1.0, -1.0)));

        let r_positive = RectF64::of(1.0, 1.0, MAX - 1.0, MAX - 1.0);
        assert!(contains_point(&r_positive, &PointF64::of(1.0, 1.0)));
        assert!(contains_point(&r_positive, &PointF64::of(1.0, MAX - 1.0)));
        assert!(contains_point(&r_positive, &PointF64::of(MAX - 1.0, 1.0)));
        assert!(contains_point(&r_positive, &PointF64::of(MAX - 1.0, MAX - 1.0)));
    }

    #[test]
    fn outside_borders() {
        let r_negative = RectF64::of(MIN + 1.0, MIN + 1.0, -1.0, -1.0);
        assert!(!contains_point(&r_negative, &PointF64::min()));
        assert!(!contains_point(&r_negative, &PointF64::of(MIN + 1.0, MIN)));
        assert!(!contains_point(&r_negative, &PointF64::of(MIN, MIN + 1.0)));

        assert!(!contains_point(&r_negative, &PointF64::of(MIN, 0.0)));
        assert!(!contains_point(&r_negative, &PointF64::of(MIN, -1.0)));
        assert!(!contains_point(&r_negative, &PointF64::of(MIN + 1.0, 0.0)));

        assert!(!contains_point(&r_negative, &PointF64::of(MAX, MIN)));
        assert!(!contains_point(&r_negative, &PointF64::of(MAX, MIN + 1.0)));
        assert!(!contains_point(&r_negative, &PointF64::of(-1.0, MIN)));

        assert!(!contains_point(&r_negative, &PointF64::max()));
        assert!(!contains_point(&r_negative, &PointF64::of(-1.0, 0.0)));
        assert!(!contains_point(&r_negative, &PointF64::of(0.0, -1.0)));

        let r_positive = RectF64::of(1.0, 1.0, MAX - 1.0, MAX - 1.0);
        assert!(!contains_point(&r_positive, &PointF64::min()));
        assert!(!contains_point(&r_positive, &PointF64::of(1.0, 0.0)));
        assert!(!contains_point(&r_positive, &PointF64::of(0.0, 1.0)));

        assert!(!contains_point(&r_positive, &PointF64::of(0.0, MAX)));
        assert!(!contains_point(&r_positive, &PointF64::of(0.0, MAX - 1.0)));
        assert!(!contains_point(&r_positive, &PointF64::of(1.0, MAX)));

        assert!(!contains_point(&r_positive, &PointF64::of(MAX, 0.0)));
        assert!(!contains_point(&r_positive, &PointF64::of(MAX, 1.0)));
        assert!(!contains_point(&r_positive, &PointF64::of(MAX - 1.0, 0.0)));

        assert!(!contains_point(&r_positive, &PointF64::max()));
        assert!(!contains_point(&r_positive, &PointF64::of(MAX - 1.0, MAX)));
        assert!(!contains_point(&r_positive, &PointF64::of(MAX, MAX - 1.0)));
    }

    #[test]
    fn inside() {
        let r_negative = RectF64::of(MIN + 1.0, MIN + 1.0, -1.0, -1.0);
        assert!(contains_point(&r_negative, &PointF64::of(MIN + 10.0, MIN + 10.0)));
        assert!(contains_point(&r_negative, &PointF64::of(-10.0, MIN + 10.0)));
        assert!(contains_point(&r_negative, &PointF64::of(MIN + 10.0, -10.0)));
        assert!(contains_point(&r_negative, &PointF64::of(-10.0, -10.0)));

        let r_positive = RectF64::of(1.0, 1.0, MAX - 1.0, MAX - 1.0);
        assert!(contains_point(&r_positive, &PointF64::of(10.0, 10.0)));
        assert!(contains_point(&r_positive, &PointF64::of(MAX - 10.0, 10.0)));
        assert!(contains_point(&r_positive, &PointF64::of(10.0, MAX - 10.0)));
        assert!(contains_point(&r_positive, &PointF64::of(MAX - 10.0, MAX - 10.0)));
    }
}
