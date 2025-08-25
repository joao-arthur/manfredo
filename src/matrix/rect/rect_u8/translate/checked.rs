#[cfg(test)]
mod tests {
    use super::super::{checked_translate, checked_translate_assign, try_checked_translate, try_checked_translate_assign};
    use crate::matrix::{point::point_i8::PointI8, rect::rect_u8::RectU8};

    #[test]
    fn test_try_checked_translate_assign() {
        let mut r = RectU8::of(0, 0, 12, 15);
        assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(5, 4)), Some(()));
        assert_eq!(r, RectU8::of(5, 4, 17, 19));
        assert_eq!(try_checked_translate_assign(&mut r, &PointI8::of(-4, -2)), Some(()));
        assert_eq!(r, RectU8::of(1, 2, 13, 17));
    }

    #[test]
    fn try_checked_translate_assign_small_rect_to_bounds() {
        let mut min_r = RectU8::of(2, 5, 12, 15);
        assert_eq!(try_checked_translate_assign(&mut min_r, &PointI8::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectU8::of(0, 0, 10, 10));

        let mut max_r = RectU8::of(u8::MAX - 12, u8::MAX - 15, u8::MAX - 2, u8::MAX - 5);
        assert_eq!(try_checked_translate_assign(&mut max_r, &PointI8::of(2, 5)), Some(()));
        assert_eq!(max_r, RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX, u8::MAX));
    }

    #[test]
    fn try_checked_translate_assign_big_rect_to_bounds() {
        let mut min_r = RectU8::of(2, 5, u8::MAX, u8::MAX);
        assert_eq!(try_checked_translate_assign(&mut min_r, &PointI8::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5));

        let mut max_r = RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5);
        assert_eq!(try_checked_translate_assign(&mut max_r, &PointI8::of(2, 5)), Some(()));
        assert_eq!(max_r, RectU8::of(2, 5, u8::MAX, u8::MAX));
    }

    #[test]
    fn try_checked_translate_assign_small_rect_out_of_bounds() {
        let mut r_min = RectU8::of(10, 5, 20, 30);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI8::of(-20, -20)), None);
        assert_eq!(r_min, RectU8::of(10, 5, 20, 30));

        let mut r_max = RectU8::of(u8::MAX - 20, u8::MAX - 30, u8::MAX - 5, u8::MAX - 10);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI8::of(20, 20)), None);
        assert_eq!(r_max, RectU8::of(u8::MAX - 20, u8::MAX - 30, u8::MAX - 5, u8::MAX - 10));
    }

    #[test]
    fn try_checked_translate_assign_big_rect_out_of_bounds() {
        let mut r_min = RectU8::of(10, 5, u8::MAX, u8::MAX);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI8::of(-20, -20)), None);
        assert_eq!(r_min, RectU8::of(10, 5, u8::MAX, u8::MAX));

        let mut r_max = RectU8::of(0, 0, u8::MAX - 5, u8::MAX - 10);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI8::of(20, 20)), None);
        assert_eq!(r_max, RectU8::of(0, 0, u8::MAX - 5, u8::MAX - 10));
    }

    #[test]
    fn try_checked_translate_assign_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU8::of(1, 1, 10, 10);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI8::min()), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI8::of(i8::MIN, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI8::of(0, i8::MIN)), None);
        assert_eq!(r_min, RectU8::of(1, 1, 10, 10));

        let mut r_max = RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX - 1, u8::MAX - 1);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI8::max()), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI8::of(i8::MAX, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI8::of(0, i8::MAX)), None);
        assert_eq!(r_max, RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX - 1, u8::MAX - 1));
    }

    #[test]
    fn try_checked_translate_assign_big_rect_limits_out_of_bounds() {
        let mut r = RectU8::largest();
        assert_eq!(try_checked_translate_assign(&mut r, &PointI8::min()), None);
        assert_eq!(try_checked_translate_assign(&mut r, &PointI8::max()), None);
        assert_eq!(r, RectU8::largest());

        let mut r_min = RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI8::max()), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI8::of(i8::MAX, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI8::of(0, i8::MAX)), None);
        assert_eq!(r_min, RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1));

        let mut r_max = RectU8::of(1, 1, u8::MAX, u8::MAX);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI8::min()), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI8::of(i8::MIN, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI8::of(0, i8::MIN)), None);
        assert_eq!(r_max, RectU8::of(1, 1, u8::MAX, u8::MAX));
    }

    #[test]
    fn test_try_checked_translate() {
        assert_eq!(try_checked_translate(&RectU8::of(0, 0, 12, 15), &PointI8::of(5, 4)), Some(RectU8::of(5, 4, 17, 19)));
        assert_eq!(try_checked_translate(&RectU8::of(5, 4, 17, 19), &PointI8::of(-4, -2)), Some(RectU8::of(1, 2, 13, 17)));
    }

    #[test]
    fn try_checked_translate_small_rect_to_bounds() {
        assert_eq!(try_checked_translate(&RectU8::of(2, 5, 12, 15), &PointI8::of(-2, -5)), Some(RectU8::of(0, 0, 10, 10)));
        assert_eq!(
            try_checked_translate(&RectU8::of(u8::MAX - 12, u8::MAX - 15, u8::MAX - 2, u8::MAX - 5), &PointI8::of(2, 5)),
            Some(RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX, u8::MAX))
        );
    }

    #[test]
    fn try_checked_translate_big_rect_to_bounds() {
        assert_eq!(
            try_checked_translate(&RectU8::of(2, 5, u8::MAX, u8::MAX), &PointI8::of(-2, -5)),
            Some(RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5))
        );
        assert_eq!(try_checked_translate(&RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5), &PointI8::of(2, 5)), Some(RectU8::of(2, 5, u8::MAX, u8::MAX)));
    }

    #[test]
    fn try_checked_translate_small_rect_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectU8::of(10, 5, 20, 30), &PointI8::of(-20, -20)), None);
        assert_eq!(try_checked_translate(&RectU8::of(u8::MAX - 20, u8::MAX - 30, u8::MAX - 5, u8::MAX - 10), &PointI8::of(20, 20)), None);
    }

    #[test]
    fn try_checked_translate_big_rect_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectU8::of(10, 5, u8::MAX, u8::MAX), &PointI8::of(-20, -20)), None);
        assert_eq!(try_checked_translate(&RectU8::of(0, 0, u8::MAX - 5, u8::MAX - 10), &PointI8::of(20, 20)), None);
    }

    #[test]
    fn try_checked_translate_small_rect_limits_out_of_bounds() {
        let r_min = RectU8::of(1, 1, 10, 10);
        assert_eq!(try_checked_translate(&r_min, &PointI8::min()), None);
        assert_eq!(try_checked_translate(&r_min, &PointI8::of(i8::MIN, 0)), None);
        assert_eq!(try_checked_translate(&r_min, &PointI8::of(0, i8::MIN)), None);

        let r_max = RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX - 1, u8::MAX - 1);
        assert_eq!(try_checked_translate(&r_max, &PointI8::max()), None);
        assert_eq!(try_checked_translate(&r_max, &PointI8::of(i8::MAX, 0)), None);
        assert_eq!(try_checked_translate(&r_max, &PointI8::of(0, i8::MAX)), None);
    }

    #[test]
    fn try_checked_translate_big_rect_limits_out_of_bounds() {
        let r = RectU8::largest();
        assert_eq!(try_checked_translate(&r, &PointI8::min()), None);
        assert_eq!(try_checked_translate(&r, &PointI8::max()), None);

        let r_min = RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1);
        assert_eq!(try_checked_translate(&r_min, &PointI8::max()), None);
        assert_eq!(try_checked_translate(&r_min, &PointI8::of(i8::MAX, 0)), None);
        assert_eq!(try_checked_translate(&r_min, &PointI8::of(0, i8::MAX)), None);

        let r_max = RectU8::of(1, 1, u8::MAX, u8::MAX);
        assert_eq!(try_checked_translate(&r_max, &PointI8::min()), None);
        assert_eq!(try_checked_translate(&r_max, &PointI8::of(i8::MIN, 0)), None);
        assert_eq!(try_checked_translate(&r_max, &PointI8::of(0, i8::MIN)), None);
    }

    #[test]
    fn test_checked_translate_assign() {
        let mut r = RectU8::of(0, 0, 12, 15);
        checked_translate_assign(&mut r, &PointI8::of(5, 4));
        assert_eq!(r, RectU8::of(5, 4, 17, 19));
        checked_translate_assign(&mut r, &PointI8::of(-4, -2));
        assert_eq!(r, RectU8::of(1, 2, 13, 17));
    }

    #[test]
    fn test_checked_translate() {
        assert_eq!(checked_translate(&RectU8::of(0, 0, 12, 15), &PointI8::of(5, 4)), RectU8::of(5, 4, 17, 19));
        assert_eq!(checked_translate(&RectU8::of(5, 4, 17, 19), &PointI8::of(-4, -2)), RectU8::of(1, 2, 13, 17));
    }
}
