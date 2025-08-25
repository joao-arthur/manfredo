#[cfg(test)]
mod tests {
    use super::super::{checked_translate, checked_translate_assign, try_checked_translate, try_checked_translate_assign};
    use crate::cartesian::{point::point_i32::PointI32, rect::rect_i32::RectI32};

    #[test]
    fn test_try_checked_translate_assign() {
        let mut r = RectI32::of(0, 0, 10, 10);
        assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(10, 20)), Some(()));
        assert_eq!(r, RectI32::of(10, 20, 20, 30));
        assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(-20, -15)), Some(()));
        assert_eq!(r, RectI32::of(-10, 5, 0, 15));
    }

    #[test]
    fn try_checked_translate_assign_small_rect_to_bounds() {
        let mut min_r = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MIN + 12, i32::MIN + 15);
        assert_eq!(try_checked_translate_assign(&mut min_r, &PointI32::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 10, i32::MIN + 10));

        let mut max_r = RectI32::of(i32::MAX - 12, i32::MAX - 15, i32::MAX - 2, i32::MAX - 5);
        assert_eq!(try_checked_translate_assign(&mut max_r, &PointI32::of(2, 5)), Some(()));
        assert_eq!(max_r, RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX, i32::MAX));
    }

    #[test]
    fn try_checked_translate_assign_big_rect_to_bounds() {
        let mut min_r = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX);
        assert_eq!(try_checked_translate_assign(&mut min_r, &PointI32::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5));

        let mut max_r = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5);
        assert_eq!(try_checked_translate_assign(&mut max_r, &PointI32::of(2, 5)), Some(()));
        assert_eq!(max_r, RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX));
    }

    #[test]
    fn try_checked_translate_assign_small_rect_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MIN + 20, i32::MIN + 30);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI32::of(-20, -20)), None);
        assert_eq!(r_min, RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MIN + 20, i32::MIN + 30));

        let mut r_max = RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX - 5, i32::MAX - 10);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI32::of(20, 20)), None);
        assert_eq!(r_max, RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX - 5, i32::MAX - 10));
    }

    #[test]
    fn try_checked_translate_assign_big_rect_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MAX, i32::MAX);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI32::of(-20, -20)), None);
        assert_eq!(r_min, RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MAX, i32::MAX));

        let mut r_max = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 5, i32::MAX - 10);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI32::of(20, 20)), None);
        assert_eq!(r_max, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 5, i32::MAX - 10));
    }

    #[test]
    fn try_checked_translate_assign_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI32::min()), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI32::of(0, i32::MIN)), None);
        assert_eq!(r_min, RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10));

        let mut r_max = RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI32::max()), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI32::of(0, i32::MAX)), None);
        assert_eq!(r_max, RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1));
    }

    #[test]
    fn try_checked_translate_assign_big_rect_limits_out_of_bounds() {
        let mut r = RectI32::largest();
        assert_eq!(try_checked_translate_assign(&mut r, &PointI32::min()), None);
        assert_eq!(try_checked_translate_assign(&mut r, &PointI32::max()), None);
        assert_eq!(r, RectI32::largest());

        let mut r_min = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI32::max()), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI32::of(0, i32::MAX)), None);
        assert_eq!(r_min, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1));

        let mut r_max = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI32::min()), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI32::of(0, i32::MIN)), None);
        assert_eq!(r_max, RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX));
    }

    #[test]
    fn test_try_checked_translate() {
        assert_eq!(try_checked_translate(&RectI32::of(0, 0, 10, 10), &PointI32::of(10, 20)), Some(RectI32::of(10, 20, 20, 30)));
        assert_eq!(try_checked_translate(&RectI32::of(10, 20, 20, 30), &PointI32::of(-20, -15)), Some(RectI32::of(-10, 5, 0, 15)));
    }

    #[test]
    fn try_checked_translate_small_rect_to_bounds() {
        assert_eq!(
            try_checked_translate(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MIN + 12, i32::MIN + 15), &PointI32::of(-2, -5)),
            Some(RectI32::of(i32::MIN, i32::MIN, i32::MIN + 10, i32::MIN + 10))
        );
        assert_eq!(
            try_checked_translate(&RectI32::of(i32::MAX - 12, i32::MAX - 15, i32::MAX - 2, i32::MAX - 5), &PointI32::of(2, 5)),
            Some(RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX, i32::MAX))
        );
    }

    #[test]
    fn try_checked_translate_big_rect_to_bounds() {
        assert_eq!(
            try_checked_translate(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX), &PointI32::of(-2, -5)),
            Some(RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5))
        );
        assert_eq!(
            try_checked_translate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5), &PointI32::of(2, 5)),
            Some(RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX))
        );
    }

    #[test]
    fn try_checked_translate_small_rect_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MIN + 20, i32::MIN + 30), &PointI32::of(-20, -20)), None);
        assert_eq!(try_checked_translate(&RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX - 5, i32::MAX - 10), &PointI32::of(20, 20)), None);
    }

    #[test]
    fn try_checked_translate_big_rect_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MAX, i32::MAX), &PointI32::of(-20, -20)), None);
        assert_eq!(try_checked_translate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 5, i32::MAX - 10), &PointI32::of(20, 20)), None);
    }

    #[test]
    fn try_checked_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10), &PointI32::min()), None);
        assert_eq!(try_checked_translate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10), &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(try_checked_translate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10), &PointI32::of(0, i32::MIN)), None);
        assert_eq!(try_checked_translate(&RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1), &PointI32::max()), None);
        assert_eq!(try_checked_translate(&RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1), &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(try_checked_translate(&RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1), &PointI32::of(0, i32::MAX)), None);
    }

    #[test]
    fn try_checked_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectI32::largest(), &PointI32::min()), None);
        assert_eq!(try_checked_translate(&RectI32::largest(), &PointI32::max()), None);
        assert_eq!(try_checked_translate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1), &PointI32::max()), None);
        assert_eq!(try_checked_translate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1), &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(try_checked_translate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1), &PointI32::of(0, i32::MAX)), None);
        assert_eq!(try_checked_translate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX), &PointI32::min()), None);
        assert_eq!(try_checked_translate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX), &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(try_checked_translate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX), &PointI32::of(0, i32::MIN)), None);
    }

    #[test]
    fn test_checked_translate_assign() {
        let mut r = RectI32::of(0, 0, 10, 10);
        checked_translate_assign(&mut r, &PointI32::of(10, 20));
        assert_eq!(r, RectI32::of(10, 20, 20, 30));
        checked_translate_assign(&mut r, &PointI32::of(-20, -15));
        assert_eq!(r, RectI32::of(-10, 5, 0, 15));
    }

    #[test]
    fn test_checked_translate() {
        assert_eq!(checked_translate(&RectI32::of(0, 0, 10, 10), &PointI32::of(10, 20)), RectI32::of(10, 20, 20, 30));
        assert_eq!(checked_translate(&RectI32::of(10, 20, 20, 30), &PointI32::of(-20, -15)), RectI32::of(-10, 5, 0, 15));
    }
}
