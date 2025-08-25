#[cfg(test)]
mod tests {
    use super::super::{checked_translate, checked_translate_assign, try_checked_translate, try_checked_translate_assign};
    use crate::cartesian::{point::point_i8::PointI8, rect::rect_i8::RectI8};

    #[test]
    fn test_try_checked_translate_assign() {
        let mut r = RectI8::of(0, 0, 10, 10);
        assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(10, 20)), Some(()));
        assert_eq!(r, RectI8::of(10, 20, 20, 30));
        assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(-20, -15)), Some(()));
        assert_eq!(r, RectI8::of(-10, 5, 0, 15));
    }

    #[test]
    fn try_checked_translate_assign_small_rect_to_bounds() {
        let mut min_r = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MIN + 12, i8::MIN + 15);
        assert_eq!(try_checked_translate_assign(&mut min_r, &PointI8::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10));

        let mut max_r = RectI8::of(i8::MAX - 12, i8::MAX - 15, i8::MAX - 2, i8::MAX - 5);
        assert_eq!(try_checked_translate_assign(&mut max_r, &PointI8::of(2, 5)), Some(()));
        assert_eq!(max_r, RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX, i8::MAX));
    }

    #[test]
    fn try_checked_translate_assign_big_rect_to_bounds() {
        let mut min_r = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX);
        assert_eq!(try_checked_translate_assign(&mut min_r, &PointI8::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5));

        let mut max_r = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5);
        assert_eq!(try_checked_translate_assign(&mut max_r, &PointI8::of(2, 5)), Some(()));
        assert_eq!(max_r, RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX));
    }

    #[test]
    fn try_checked_translate_assign_small_rect_out_of_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MIN + 20, i8::MIN + 30);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI8::of(-20, -20)), None);
        assert_eq!(r_min, RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MIN + 20, i8::MIN + 30));

        let mut r_max = RectI8::of(i8::MAX - 20, i8::MAX - 30, i8::MAX - 5, i8::MAX - 10);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI8::of(20, 20)), None);
        assert_eq!(r_max, RectI8::of(i8::MAX - 20, i8::MAX - 30, i8::MAX - 5, i8::MAX - 10));
    }

    #[test]
    fn try_checked_translate_assign_big_rect_out_of_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MAX, i8::MAX);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI8::of(-20, -20)), None);
        assert_eq!(r_min, RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MAX, i8::MAX));

        let mut r_max = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 5, i8::MAX - 10);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI8::of(20, 20)), None);
        assert_eq!(r_max, RectI8::of(i8::MIN, i8::MIN, i8::MAX - 5, i8::MAX - 10));
    }

    #[test]
    fn try_checked_translate_assign_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MIN + 10, i8::MIN + 10);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI8::min()), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI8::of(i8::MIN, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI8::of(0, i8::MIN)), None);
        assert_eq!(r_min, RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MIN + 10, i8::MIN + 10));

        let mut r_max = RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX - 1, i8::MAX - 1);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI8::max()), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI8::of(i8::MAX, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI8::of(0, i8::MAX)), None);
        assert_eq!(r_max, RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX - 1, i8::MAX - 1));
    }

    #[test]
    fn try_checked_translate_assign_big_rect_limits_out_of_bounds() {
        let mut r = RectI8::largest();
        assert_eq!(try_checked_translate_assign(&mut r, &PointI8::min()), None);
        assert_eq!(try_checked_translate_assign(&mut r, &PointI8::max()), None);
        assert_eq!(r, RectI8::largest());

        let mut r_min = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI8::max()), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI8::of(i8::MAX, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI8::of(0, i8::MAX)), None);
        assert_eq!(r_min, RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1));

        let mut r_max = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI8::min()), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI8::of(i8::MIN, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI8::of(0, i8::MIN)), None);
        assert_eq!(r_max, RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX));
    }

    #[test]
    fn test_try_checked_translate() {
        assert_eq!(try_checked_translate(&RectI8::of(0, 0, 10, 10), &PointI8::of(10, 20)), Some(RectI8::of(10, 20, 20, 30)));
        assert_eq!(try_checked_translate(&RectI8::of(10, 20, 20, 30), &PointI8::of(-20, -15)), Some(RectI8::of(-10, 5, 0, 15)));
    }

    #[test]
    fn try_checked_translate_small_rect_to_bounds() {
        assert_eq!(
            try_checked_translate(&RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MIN + 12, i8::MIN + 15), &PointI8::of(-2, -5)),
            Some(RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10))
        );
        assert_eq!(
            try_checked_translate(&RectI8::of(i8::MAX - 12, i8::MAX - 15, i8::MAX - 2, i8::MAX - 5), &PointI8::of(2, 5)),
            Some(RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX, i8::MAX))
        );
    }

    #[test]
    fn try_checked_translate_big_rect_to_bounds() {
        assert_eq!(
            try_checked_translate(&RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX), &PointI8::of(-2, -5)),
            Some(RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5))
        );
        assert_eq!(
            try_checked_translate(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5), &PointI8::of(2, 5)),
            Some(RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX))
        );
    }

    #[test]
    fn try_checked_translate_small_rect_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MIN + 20, i8::MIN + 30), &PointI8::of(-20, -20)), None);
        assert_eq!(try_checked_translate(&RectI8::of(i8::MAX - 20, i8::MAX - 30, i8::MAX - 5, i8::MAX - 10), &PointI8::of(20, 20)), None);
    }

    #[test]
    fn try_checked_translate_big_rect_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MAX, i8::MAX), &PointI8::of(-20, -20)), None);
        assert_eq!(try_checked_translate(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 5, i8::MAX - 10), &PointI8::of(20, 20)), None);
    }

    #[test]
    fn try_checked_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MIN + 10, i8::MIN + 10), &PointI8::min()), None);
        assert_eq!(try_checked_translate(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MIN + 10, i8::MIN + 10), &PointI8::of(i8::MIN, 0)), None);
        assert_eq!(try_checked_translate(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MIN + 10, i8::MIN + 10), &PointI8::of(0, i8::MIN)), None);
        assert_eq!(try_checked_translate(&RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX - 1, i8::MAX - 1), &PointI8::max()), None);
        assert_eq!(try_checked_translate(&RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX - 1, i8::MAX - 1), &PointI8::of(i8::MAX, 0)), None);
        assert_eq!(try_checked_translate(&RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX - 1, i8::MAX - 1), &PointI8::of(0, i8::MAX)), None);
    }

    #[test]
    fn try_checked_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectI8::largest(), &PointI8::min()), None);
        assert_eq!(try_checked_translate(&RectI8::largest(), &PointI8::max()), None);
        assert_eq!(try_checked_translate(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1), &PointI8::max()), None);
        assert_eq!(try_checked_translate(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1), &PointI8::of(i8::MAX, 0)), None);
        assert_eq!(try_checked_translate(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1), &PointI8::of(0, i8::MAX)), None);
        assert_eq!(try_checked_translate(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX), &PointI8::min()), None);
        assert_eq!(try_checked_translate(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX), &PointI8::of(i8::MIN, 0)), None);
        assert_eq!(try_checked_translate(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX), &PointI8::of(0, i8::MIN)), None);
    }

    #[test]
    fn test_checked_translate_assign() {
        let mut r = RectI8::of(0, 0, 10, 10);
        checked_translate_assign(&mut r, &PointI8::of(10, 20));
        assert_eq!(r, RectI8::of(10, 20, 20, 30));
        checked_translate_assign(&mut r, &PointI8::of(-20, -15));
        assert_eq!(r, RectI8::of(-10, 5, 0, 15));
    }

    #[test]
    fn test_checked_translate() {
        assert_eq!(checked_translate(&RectI8::of(0, 0, 10, 10), &PointI8::of(10, 20)), RectI8::of(10, 20, 20, 30));
        assert_eq!(checked_translate(&RectI8::of(10, 20, 20, 30), &PointI8::of(-20, -15)), RectI8::of(-10, 5, 0, 15));
    }
}
