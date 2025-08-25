#[cfg(test)]
mod tests {
    use super::super::{checked_translate, checked_translate_assign, try_checked_translate, try_checked_translate_assign};
    use crate::matrix::{point::point_i64::PointI64, rect::rect_u64::RectU64};

    #[test]
    fn test_try_checked_translate_assign() {
        let mut r = RectU64::of(0, 0, 12, 15);
        assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(5, 4)), Some(()));
        assert_eq!(r, RectU64::of(5, 4, 17, 19));
        assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(-4, -2)), Some(()));
        assert_eq!(r, RectU64::of(1, 2, 13, 17));
    }

    #[test]
    fn try_checked_translate_assign_small_rect_to_bounds() {
        let mut min_r = RectU64::of(2, 5, 12, 15);
        assert_eq!(try_checked_translate_assign(&mut min_r, &PointI64::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectU64::of(0, 0, 10, 10));

        let mut max_r = RectU64::of(u64::MAX - 12, u64::MAX - 15, u64::MAX - 2, u64::MAX - 5);
        assert_eq!(try_checked_translate_assign(&mut max_r, &PointI64::of(2, 5)), Some(()));
        assert_eq!(max_r, RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX));
    }

    #[test]
    fn try_checked_translate_assign_big_rect_to_bounds() {
        let mut min_r = RectU64::of(2, 5, u64::MAX, u64::MAX);
        assert_eq!(try_checked_translate_assign(&mut min_r, &PointI64::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5));

        let mut max_r = RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5);
        assert_eq!(try_checked_translate_assign(&mut max_r, &PointI64::of(2, 5)), Some(()));
        assert_eq!(max_r, RectU64::of(2, 5, u64::MAX, u64::MAX));
    }

    #[test]
    fn try_checked_translate_assign_small_rect_out_of_bounds() {
        let mut r_min = RectU64::of(10, 5, 20, 30);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI64::of(-20, -20)), None);
        assert_eq!(r_min, RectU64::of(10, 5, 20, 30));

        let mut r_max = RectU64::of(u64::MAX - 20, u64::MAX - 30, u64::MAX - 5, u64::MAX - 10);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI64::of(20, 20)), None);
        assert_eq!(r_max, RectU64::of(u64::MAX - 20, u64::MAX - 30, u64::MAX - 5, u64::MAX - 10));
    }

    #[test]
    fn try_checked_translate_assign_big_rect_out_of_bounds() {
        let mut r_min = RectU64::of(10, 5, u64::MAX, u64::MAX);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI64::of(-20, -20)), None);
        assert_eq!(r_min, RectU64::of(10, 5, u64::MAX, u64::MAX));

        let mut r_max = RectU64::of(0, 0, u64::MAX - 5, u64::MAX - 10);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI64::of(20, 20)), None);
        assert_eq!(r_max, RectU64::of(0, 0, u64::MAX - 5, u64::MAX - 10));
    }

    #[test]
    fn try_checked_translate_assign_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU64::of(1, 1, 10, 10);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI64::min()), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI64::of(i64::MIN, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI64::of(0, i64::MIN)), None);
        assert_eq!(r_min, RectU64::of(1, 1, 10, 10));

        let mut r_max = RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI64::max()), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI64::of(i64::MAX, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI64::of(0, i64::MAX)), None);
        assert_eq!(r_max, RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1));
    }

    #[test]
    fn try_checked_translate_assign_big_rect_limits_out_of_bounds() {
        let mut r = RectU64::largest();
        assert_eq!(try_checked_translate_assign(&mut r, &PointI64::min()), None);
        assert_eq!(try_checked_translate_assign(&mut r, &PointI64::max()), None);
        assert_eq!(r, RectU64::largest());

        let mut r_min = RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI64::max()), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI64::of(i64::MAX, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI64::of(0, i64::MAX)), None);
        assert_eq!(r_min, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));

        let mut r_max = RectU64::of(1, 1, u64::MAX, u64::MAX);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI64::min()), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI64::of(i64::MIN, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI64::of(0, i64::MIN)), None);
        assert_eq!(r_max, RectU64::of(1, 1, u64::MAX, u64::MAX));
    }

    #[test]
    fn test_try_checked_translate() {
        assert_eq!(try_checked_translate(&RectU64::of(0, 0, 12, 15), &PointI64::of(5, 4)), Some(RectU64::of(5, 4, 17, 19)));
        assert_eq!(try_checked_translate(&RectU64::of(5, 4, 17, 19), &PointI64::of(-4, -2)), Some(RectU64::of(1, 2, 13, 17)));
    }

    #[test]
    fn try_checked_translate_small_rect_to_bounds() {
        assert_eq!(try_checked_translate(&RectU64::of(2, 5, 12, 15), &PointI64::of(-2, -5)), Some(RectU64::of(0, 0, 10, 10)));
        assert_eq!(
            try_checked_translate(&RectU64::of(u64::MAX - 12, u64::MAX - 15, u64::MAX - 2, u64::MAX - 5), &PointI64::of(2, 5)),
            Some(RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX))
        );
    }

    #[test]
    fn try_checked_translate_big_rect_to_bounds() {
        assert_eq!(
            try_checked_translate(&RectU64::of(2, 5, u64::MAX, u64::MAX), &PointI64::of(-2, -5)),
            Some(RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5))
        );
        assert_eq!(
            try_checked_translate(&RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5), &PointI64::of(2, 5)),
            Some(RectU64::of(2, 5, u64::MAX, u64::MAX))
        );
    }

    #[test]
    fn try_checked_translate_small_rect_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectU64::of(10, 5, 20, 30), &PointI64::of(-20, -20)), None);
        assert_eq!(try_checked_translate(&RectU64::of(u64::MAX - 20, u64::MAX - 30, u64::MAX - 5, u64::MAX - 10), &PointI64::of(20, 20)), None);
    }

    #[test]
    fn try_checked_translate_big_rect_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectU64::of(10, 5, u64::MAX, u64::MAX), &PointI64::of(-20, -20)), None);
        assert_eq!(try_checked_translate(&RectU64::of(0, 0, u64::MAX - 5, u64::MAX - 10), &PointI64::of(20, 20)), None);
    }

    #[test]
    fn try_checked_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectU64::of(1, 1, 10, 10), &PointI64::min()), None);
        assert_eq!(try_checked_translate(&RectU64::of(1, 1, 10, 10), &PointI64::of(i64::MIN, 0)), None);
        assert_eq!(try_checked_translate(&RectU64::of(1, 1, 10, 10), &PointI64::of(0, i64::MIN)), None);
        assert_eq!(try_checked_translate(&RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1), &PointI64::max()), None);
        assert_eq!(try_checked_translate(&RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1), &PointI64::of(i64::MAX, 0)), None);
        assert_eq!(try_checked_translate(&RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1), &PointI64::of(0, i64::MAX)), None);
    }

    #[test]
    fn try_checked_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectU64::largest(), &PointI64::min()), None);
        assert_eq!(try_checked_translate(&RectU64::largest(), &PointI64::max()), None);
        assert_eq!(try_checked_translate(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1), &PointI64::max()), None);
        assert_eq!(try_checked_translate(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1), &PointI64::of(i64::MAX, 0)), None);
        assert_eq!(try_checked_translate(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1), &PointI64::of(0, i64::MAX)), None);
        assert_eq!(try_checked_translate(&RectU64::of(1, 1, u64::MAX, u64::MAX), &PointI64::min()), None);
        assert_eq!(try_checked_translate(&RectU64::of(1, 1, u64::MAX, u64::MAX), &PointI64::of(i64::MIN, 0)), None);
        assert_eq!(try_checked_translate(&RectU64::of(1, 1, u64::MAX, u64::MAX), &PointI64::of(0, i64::MIN)), None);
    }

    #[test]
    fn test_checked_translate_assign() {
        let mut r = RectU64::of(0, 0, 12, 15);
        checked_translate_assign(&mut r, &PointI64::of(5, 4));
        assert_eq!(r, RectU64::of(5, 4, 17, 19));
        checked_translate_assign(&mut r, &PointI64::of(-4, -2));
        assert_eq!(r, RectU64::of(1, 2, 13, 17));
    }

    #[test]
    fn test_checked_translate() {
        assert_eq!(checked_translate(&RectU64::of(0, 0, 12, 15), &PointI64::of(5, 4)), RectU64::of(5, 4, 17, 19));
        assert_eq!(checked_translate(&RectU64::of(5, 4, 17, 19), &PointI64::of(-4, -2)), RectU64::of(1, 2, 13, 17));
    }
}
