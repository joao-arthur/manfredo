#[cfg(test)]
mod tests {
    use super::super::{saturating_translate, saturating_translate_assign};
    use crate::cartesian::{
        point::point_f32::{MAX, MIN, PointF32},
        rect::rect_f32::RectF32,
    };

    #[test]
    fn test_saturating_translate_assign() {
        let mut r = RectF32::of(0.0, 0.0, 10.0, 10.0);
        saturating_translate_assign(&mut r, &PointF32::of(10.0, 20.0));
        assert_eq!(r, RectF32::of(10.0, 20.0, 20.0, 30.0));
        saturating_translate_assign(&mut r, &PointF32::of(-20.0, -15.0));
        assert_eq!(r, RectF32::of(-10.0, 5.0, 0.0, 15.0));
    }

    #[test]
    fn saturating_translate_assign_small_rect_to_bounds() {
        let mut min_r = RectF32::of(MIN + 2.0, MIN + 5.0, MIN + 12.0, MIN + 15.0);
        saturating_translate_assign(&mut min_r, &PointF32::of(-2.0, -5.0));
        assert_eq!(min_r, RectF32::of(MIN, MIN, MIN + 10.0, MIN + 10.0));

        let mut max_r = RectF32::of(MAX - 12.0, MAX - 15.0, MAX - 2.0, MAX - 5.0);
        saturating_translate_assign(&mut max_r, &PointF32::of(2.0, 5.0));
        assert_eq!(max_r, RectF32::of(MAX - 10.0, MAX - 10.0, MAX, MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_to_bounds() {
        let mut min_r = RectF32::of(MIN + 2.0, MIN + 5.0, 0.0, 0.0);
        saturating_translate_assign(&mut min_r, &PointF32::of(-2.0, -5.0));
        assert_eq!(min_r, RectF32::of(MIN, MIN, -2.0, -5.0));

        let mut max_r = RectF32::of(0.0, 0.0, MAX - 2.0, MAX - 5.0);
        saturating_translate_assign(&mut max_r, &PointF32::of(2.0, 5.0));
        assert_eq!(max_r, RectF32::of(2.0, 5.0, MAX, MAX));
    }

    #[test]
    fn saturating_translate_assign_small_rect_out_of_bounds() {
        let mut r_min = RectF32::of(MIN + 10.0, MIN + 5.0, MIN + 20.0, MIN + 30.0);
        saturating_translate_assign(&mut r_min, &PointF32::of(-20.0, -20.0));
        assert_eq!(r_min, RectF32::of(MIN, MIN, MIN + 10.0, MIN + 25.0));

        let mut r_max = RectF32::of(MAX - 20.0, MAX - 30.0, MAX - 5.0, MAX - 10.0);
        saturating_translate_assign(&mut r_max, &PointF32::of(20.0, 20.0));
        assert_eq!(r_max, RectF32::of(MAX - 15.0, MAX - 20.0, MAX, MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_out_of_bounds() {
        let mut r_min = RectF32::of(MIN + 10.0, MIN + 5.0, 0.0, 0.0);
        saturating_translate_assign(&mut r_min, &PointF32::of(-20.0, -20.0));
        assert_eq!(r_min, RectF32::of(MIN, MIN, -10.0, -5.0));

        let mut r_max = RectF32::of(0.0, 0.0, MAX - 5.0, MAX - 10.0);
        saturating_translate_assign(&mut r_max, &PointF32::of(20.0, 20.0));
        assert_eq!(r_max, RectF32::of(5.0, 10.0, MAX, MAX));
    }

    #[test]
    fn saturating_translate_assign_small_rect_limits_out_of_bounds() {
        let mut r_min = RectF32::of(MIN + 1.0, MIN + 1.0, MIN + 10.0, MIN + 10.0);
        saturating_translate_assign(&mut r_min, &PointF32::min());
        assert_eq!(r_min, RectF32::of(MIN, MIN, MIN + 9.0, MIN + 9.0));

        let mut r_max = RectF32::of(MAX - 10.0, MAX - 10.0, MAX - 1.0, MAX - 1.0);
        saturating_translate_assign(&mut r_max, &PointF32::max());
        assert_eq!(r_max, RectF32::of(MAX - 9.0, MAX - 9.0, MAX, MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_limits_out_of_bounds() {
        let mut r_min = RectF32::largest_min();
        saturating_translate_assign(&mut r_min, &PointF32::min());
        assert_eq!(r_min, RectF32::largest_min());

        let mut r_max = RectF32::largest_max();
        saturating_translate_assign(&mut r_max, &PointF32::max());
        assert_eq!(r_max, RectF32::largest_max());

        let mut r_almost_min = RectF32::of(MIN + 1.0, MIN + 1.0, 0.0, 0.0);
        saturating_translate_assign(&mut r_almost_min, &PointF32::min());
        assert_eq!(r_almost_min, RectF32::of(MIN, MIN, -1.0, -1.0));

        let mut r_almost_max = RectF32::of(0.0, 0.0, MAX - 1.0, MAX - 1.0);
        saturating_translate_assign(&mut r_almost_max, &PointF32::max());
        assert_eq!(r_almost_max, RectF32::of(1.0, 1.0, MAX, MAX));
    }

    #[test]
    fn test_saturating_translate() {
        assert_eq!(saturating_translate(&RectF32::of(0.0, 0.0, 10.0, 10.0), &PointF32::of(10.0, 20.0)), RectF32::of(10.0, 20.0, 20.0, 30.0));
        assert_eq!(saturating_translate(&RectF32::of(10.0, 20.0, 20.0, 30.0), &PointF32::of(-20.0, -15.0)), RectF32::of(-10.0, 5.0, 0.0, 15.0));
    }

    #[test]
    fn saturating_translate_small_rect_to_bounds() {
        assert_eq!(
            saturating_translate(&RectF32::of(MIN + 2.0, MIN + 5.0, MIN + 12.0, MIN + 15.0), &PointF32::of(-2.0, -5.0)),
            RectF32::of(MIN, MIN, MIN + 10.0, MIN + 10.0)
        );
        assert_eq!(
            saturating_translate(&RectF32::of(MAX - 12.0, MAX - 15.0, MAX - 2.0, MAX - 5.0), &PointF32::of(2.0, 5.0)),
            RectF32::of(MAX - 10.0, MAX - 10.0, MAX, MAX)
        );
    }

    #[test]
    fn saturating_translate_big_rect_to_bounds() {
        assert_eq!(saturating_translate(&RectF32::of(MIN + 2.0, MIN + 5.0, 0.0, 0.0), &PointF32::of(-2.0, -5.0)), RectF32::of(MIN, MIN, -2.0, -5.0));
        assert_eq!(saturating_translate(&RectF32::of(0.0, 0.0, MAX - 2.0, MAX - 5.0), &PointF32::of(2.0, 5.0)), RectF32::of(2.0, 5.0, MAX, MAX));
    }

    #[test]
    fn saturating_translate_small_rect_out_of_bounds() {
        assert_eq!(
            saturating_translate(&RectF32::of(MIN + 10.0, MIN + 5.0, MIN + 20.0, MIN + 30.0), &PointF32::of(-20.0, -20.0)),
            RectF32::of(MIN, MIN, MIN + 10.0, MIN + 25.0)
        );
        assert_eq!(
            saturating_translate(&RectF32::of(MAX - 20.0, MAX - 30.0, MAX - 5.0, MAX - 10.0), &PointF32::of(20.0, 20.0)),
            RectF32::of(MAX - 15.0, MAX - 20.0, MAX, MAX)
        );
    }

    #[test]
    fn saturating_translate_big_rect_out_of_bounds() {
        assert_eq!(
            saturating_translate(&RectF32::of(MIN + 10.0, MIN + 5.0, 0.0, 0.0), &PointF32::of(-20.0, -20.0)),
            RectF32::of(MIN, MIN, -10.0, -5.0)
        );
        assert_eq!(saturating_translate(&RectF32::of(0.0, 0.0, MAX - 5.0, MAX - 10.0), &PointF32::of(20.0, 20.0)), RectF32::of(5.0, 10.0, MAX, MAX));
    }

    #[test]
    fn saturating_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(
            saturating_translate(&RectF32::of(MIN + 1.0, MIN + 1.0, MIN + 10.0, MIN + 10.0), &PointF32::min()),
            RectF32::of(MIN, MIN, MIN + 9.0, MIN + 9.0)
        );
        assert_eq!(
            saturating_translate(&RectF32::of(MAX - 10.0, MAX - 10.0, MAX - 1.0, MAX - 1.0), &PointF32::max()),
            RectF32::of(MAX - 9.0, MAX - 9.0, MAX, MAX)
        );
    }

    #[test]
    fn saturating_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(saturating_translate(&RectF32::largest_min(), &PointF32::min()), RectF32::largest_min());
        assert_eq!(saturating_translate(&RectF32::largest_max(), &PointF32::max()), RectF32::largest_max());
        assert_eq!(saturating_translate(&RectF32::of(MIN + 1.0, MIN + 1.0, 0.0, 0.0), &PointF32::min()), RectF32::of(MIN, MIN, -1.0, -1.0));
        assert_eq!(saturating_translate(&RectF32::of(0.0, 0.0, MAX - 1.0, MAX - 1.0), &PointF32::max()), RectF32::of(1.0, 1.0, MAX, MAX));
    }
}
