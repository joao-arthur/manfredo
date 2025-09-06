#[cfg(test)]
mod tests {
    use super::super::{saturating_translate, saturating_translate_assign};
    use crate::matrix::{point::point_i16::PointI16, rect::rect_i16::RectI16};

    #[test]
    fn test_saturating_translate_assign() {
        let mut r = RectI16::of(0, 0, 10, 10);
        saturating_translate_assign(&mut r, &PointI16::of(10, 20));
        assert_eq!(r, RectI16::of(10, 20, 20, 30));
        saturating_translate_assign(&mut r, &PointI16::of(-20, -15));
        assert_eq!(r, RectI16::of(-10, 5, 0, 15));
    }

    #[test]
    fn saturating_translate_assign_small_rect_to_bounds() {
        let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MIN + 12, i16::MIN + 15);
        saturating_translate_assign(&mut min_r, &PointI16::of(-2, -5));
        assert_eq!(min_r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 10));

        let mut max_r = RectI16::of(i16::MAX - 12, i16::MAX - 15, i16::MAX - 2, i16::MAX - 5);
        saturating_translate_assign(&mut max_r, &PointI16::of(2, 5));
        assert_eq!(max_r, RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX, i16::MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_to_bounds() {
        let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX);
        saturating_translate_assign(&mut min_r, &PointI16::of(-2, -5));
        assert_eq!(min_r, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5));

        let mut max_r = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5);
        saturating_translate_assign(&mut max_r, &PointI16::of(2, 5));
        assert_eq!(max_r, RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX));
    }

    #[test]
    fn saturating_translate_assign_small_rect_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30);
        saturating_translate_assign(&mut r_min, &PointI16::of(-20, -20));
        assert_eq!(r_min, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 25));

        let mut r_max = RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10);
        saturating_translate_assign(&mut r_max, &PointI16::of(20, 20));
        assert_eq!(r_max, RectI16::of(i16::MAX - 15, i16::MAX - 20, i16::MAX, i16::MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX);
        saturating_translate_assign(&mut r_min, &PointI16::of(-20, -20));
        assert_eq!(r_min, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 10, i16::MAX - 5));

        let mut r_max = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10);
        saturating_translate_assign(&mut r_max, &PointI16::of(20, 20));
        assert_eq!(r_max, RectI16::of(i16::MIN + 5, i16::MIN + 10, i16::MAX, i16::MAX));
    }

    #[test]
    fn saturating_translate_assign_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10);
        saturating_translate_assign(&mut r_min, &PointI16::min());
        assert_eq!(r_min, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 9, i16::MIN + 9));

        let mut r_max = RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1);
        saturating_translate_assign(&mut r_max, &PointI16::max());
        assert_eq!(r_max, RectI16::of(i16::MAX - 9, i16::MAX - 9, i16::MAX, i16::MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_limits_out_of_bounds() {
        let mut r_min = RectI16::largest();
        saturating_translate_assign(&mut r_min, &PointI16::min());
        assert_eq!(r_min, RectI16::largest());
        saturating_translate_assign(&mut r_min, &PointI16::max());
        assert_eq!(r_min, RectI16::largest());

        let mut r_min = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX);
        saturating_translate_assign(&mut r_min, &PointI16::min());
        assert_eq!(r_min, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1));

        let mut r_max = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1);
        saturating_translate_assign(&mut r_max, &PointI16::max());
        assert_eq!(r_max, RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX));
    }

    #[test]
    fn test_saturating_translate() {
        assert_eq!(saturating_translate(&RectI16::of(0, 0, 10, 10), &PointI16::of(10, 20)), RectI16::of(10, 20, 20, 30));
        assert_eq!(saturating_translate(&RectI16::of(10, 20, 20, 30), &PointI16::of(-20, -15)), RectI16::of(-10, 5, 0, 15));
    }

    #[test]
    fn saturating_translate_small_rect_to_bounds() {
        assert_eq!(saturating_translate(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MIN + 12, i16::MIN + 15), &PointI16::of(-2, -5)), RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 10));
        assert_eq!(saturating_translate(&RectI16::of(i16::MAX - 12, i16::MAX - 15, i16::MAX - 2, i16::MAX - 5), &PointI16::of(2, 5)), RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX, i16::MAX));
    }

    #[test]
    fn saturating_translate_big_rect_to_bounds() {
        assert_eq!(saturating_translate(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX), &PointI16::of(-2, -5)), RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5));
        assert_eq!(saturating_translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5), &PointI16::of(2, 5)), RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX));
    }

    #[test]
    fn saturating_translate_small_rect_out_of_bounds() {
        assert_eq!(
            saturating_translate(&RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30), &PointI16::of(-20, -20)),
            RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 25)
        );
        assert_eq!(saturating_translate(&RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10), &PointI16::of(20, 20)), RectI16::of(i16::MAX - 15, i16::MAX - 20, i16::MAX, i16::MAX));
    }

    #[test]
    fn saturating_translate_big_rect_out_of_bounds() {
        assert_eq!(saturating_translate(&RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX), &PointI16::of(-20, -20)), RectI16::of(i16::MIN, i16::MIN, i16::MAX - 10, i16::MAX - 5));
        assert_eq!(saturating_translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10), &PointI16::of(20, 20)), RectI16::of(i16::MIN + 5, i16::MIN + 10, i16::MAX, i16::MAX));
    }

    #[test]
    fn saturating_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(saturating_translate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10), &PointI16::min()), RectI16::of(i16::MIN, i16::MIN, i16::MIN + 9, i16::MIN + 9));
        assert_eq!(saturating_translate(&RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1), &PointI16::max()), RectI16::of(i16::MAX - 9, i16::MAX - 9, i16::MAX, i16::MAX));
    }

    #[test]
    fn saturating_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(saturating_translate(&RectI16::largest(), &PointI16::min()), RectI16::largest());
        assert_eq!(saturating_translate(&RectI16::largest(), &PointI16::max()), RectI16::largest());
        assert_eq!(saturating_translate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX), &PointI16::min()), RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1));
        assert_eq!(saturating_translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1), &PointI16::max()), RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX));
    }
}
