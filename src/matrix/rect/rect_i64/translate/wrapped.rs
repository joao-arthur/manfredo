#[cfg(test)]
mod tests {
    use super::super::{wrapping_translate, wrapping_translate_assign};
    use crate::matrix::{point::point_i64::PointI64, rect::rect_i64::RectI64};

    #[test]
    fn test_wrapping_translate_assign() {
        let mut r = RectI64::of(0, 0, 12, 15);
        wrapping_translate_assign(&mut r, &PointI64::of(5, 4));
        assert_eq!(r, RectI64::of(5, 4, 17, 19));
        wrapping_translate_assign(&mut r, &PointI64::of(-4, -2));
        assert_eq!(r, RectI64::of(1, 2, 13, 17));
    }

    #[test]
    fn wrapping_translate_assign_small_rect_to_bounds() {
        let mut min_r = RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MIN + 12, i64::MIN + 15);
        wrapping_translate_assign(&mut min_r, &PointI64::of(-2, -5));
        assert_eq!(min_r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10));

        let mut max_r = RectI64::of(i64::MAX - 12, i64::MAX - 15, i64::MAX - 2, i64::MAX - 5);
        wrapping_translate_assign(&mut max_r, &PointI64::of(2, 5));
        assert_eq!(max_r, RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX, i64::MAX));
    }

    #[test]
    fn wrapping_translate_assign_big_rect_to_bounds() {
        let mut min_r = RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX);
        wrapping_translate_assign(&mut min_r, &PointI64::of(-2, -5));
        assert_eq!(min_r, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5));

        let mut max_r = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5);
        wrapping_translate_assign(&mut max_r, &PointI64::of(2, 5));
        assert_eq!(max_r, RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX));
    }

    #[test]
    fn wrapping_translate_assign_small_rect_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MIN + 20, i64::MIN + 30);
        wrapping_translate_assign(&mut r_min, &PointI64::of(-20, -20));
        assert_eq!(r_min, RectI64::of(i64::MAX - 9, i64::MAX - 14, i64::MIN, i64::MIN + 10));

        let mut r_max = RectI64::of(i64::MAX - 20, i64::MAX - 30, i64::MAX - 5, i64::MAX - 10);
        wrapping_translate_assign(&mut r_max, &PointI64::of(20, 20));
        assert_eq!(r_max, RectI64::of(i64::MAX, i64::MAX - 10, i64::MIN + 14, i64::MIN + 9));
    }

    #[test]
    fn wrapping_translate_assign_big_rect_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MAX, i64::MAX);
        wrapping_translate_assign(&mut r_min, &PointI64::of(-20, -20));
        assert_eq!(r_min, RectI64::of(i64::MAX - 9, i64::MAX - 14, i64::MAX - 20, i64::MAX - 20));

        let mut r_max = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 5, i64::MAX - 10);
        wrapping_translate_assign(&mut r_max, &PointI64::of(20, 20));
        assert_eq!(r_max, RectI64::of(i64::MIN + 20, i64::MIN + 20, i64::MIN + 14, i64::MIN + 9));
    }

    #[test]
    fn wrapping_translate_assign_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MIN + 10, i64::MIN + 10);
        wrapping_translate_assign(&mut r_min, &PointI64::min());
        assert_eq!(r_min, RectI64::of(1, 1, 10, 10));

        let mut r_max = RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX - 1, i64::MAX - 1);
        wrapping_translate_assign(&mut r_max, &PointI64::max());
        assert_eq!(r_max, RectI64::of(-12, -12, -3, -3));
    }

    #[test]
    fn wrapping_translate_assign_big_rect_limits_out_of_bounds() {
        let mut r1 = RectI64::largest();
        wrapping_translate_assign(&mut r1, &PointI64::min());
        assert_eq!(r1, RectI64::of(0, 0, -1, -1));

        let mut r2 = RectI64::largest();
        wrapping_translate_assign(&mut r2, &PointI64::max());
        assert_eq!(r2, RectI64::of(-1, -1, -2, -2));

        let mut r_min = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX);
        wrapping_translate_assign(&mut r_min, &PointI64::min());
        assert_eq!(r_min, RectI64::of(1, 1, -1, -1));

        let mut r_max = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1);
        wrapping_translate_assign(&mut r_max, &PointI64::max());
        assert_eq!(r_max, RectI64::of(-1, -1, -3, -3));
    }

    #[test]
    fn test_wrapping_translate() {
        assert_eq!(wrapping_translate(&RectI64::of(0, 0, 12, 15), &PointI64::of(5, 4)), RectI64::of(5, 4, 17, 19));
        assert_eq!(wrapping_translate(&RectI64::of(5, 4, 17, 19), &PointI64::of(-4, -2)), RectI64::of(1, 2, 13, 17));
    }

    #[test]
    fn wrapping_translate_small_rect_to_bounds() {
        assert_eq!(
            wrapping_translate(&RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MIN + 12, i64::MIN + 15), &PointI64::of(-2, -5)),
            RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10)
        );
        assert_eq!(
            wrapping_translate(&RectI64::of(i64::MAX - 12, i64::MAX - 15, i64::MAX - 2, i64::MAX - 5), &PointI64::of(2, 5)),
            RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX, i64::MAX)
        );
    }

    #[test]
    fn wrapping_translate_big_rect_to_bounds() {
        assert_eq!(
            wrapping_translate(&RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX), &PointI64::of(-2, -5)),
            RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5)
        );
        assert_eq!(
            wrapping_translate(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5), &PointI64::of(2, 5)),
            RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX)
        );
    }

    #[test]
    fn wrapping_translate_small_rect_out_of_bounds() {
        assert_eq!(
            wrapping_translate(&RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MIN + 20, i64::MIN + 30), &PointI64::of(-20, -20)),
            RectI64::of(i64::MAX - 9, i64::MAX - 14, i64::MIN, i64::MIN + 10)
        );
        assert_eq!(
            wrapping_translate(&RectI64::of(i64::MAX - 20, i64::MAX - 30, i64::MAX - 5, i64::MAX - 10), &PointI64::of(20, 20)),
            RectI64::of(i64::MAX, i64::MAX - 10, i64::MIN + 14, i64::MIN + 9)
        );
    }

    #[test]
    fn wrapping_translate_big_rect_out_of_bounds() {
        assert_eq!(
            wrapping_translate(&RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MAX, i64::MAX), &PointI64::of(-20, -20)),
            RectI64::of(i64::MAX - 9, i64::MAX - 14, i64::MAX - 20, i64::MAX - 20)
        );
        assert_eq!(
            wrapping_translate(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 5, i64::MAX - 10), &PointI64::of(20, 20)),
            RectI64::of(i64::MIN + 20, i64::MIN + 20, i64::MIN + 14, i64::MIN + 9)
        );
    }

    #[test]
    fn wrapping_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(
            wrapping_translate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MIN + 10, i64::MIN + 10), &PointI64::min()),
            RectI64::of(1, 1, 10, 10)
        );
        assert_eq!(
            wrapping_translate(&RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX - 1, i64::MAX - 1), &PointI64::max()),
            RectI64::of(-12, -12, -3, -3)
        );
    }

    #[test]
    fn wrapping_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(wrapping_translate(&RectI64::largest(), &PointI64::min()), RectI64::of(0, 0, -1, -1));
        assert_eq!(wrapping_translate(&RectI64::largest(), &PointI64::max()), RectI64::of(-1, -1, -2, -2));
        assert_eq!(wrapping_translate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX), &PointI64::min()), RectI64::of(1, 1, -1, -1));
        assert_eq!(wrapping_translate(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1), &PointI64::max()), RectI64::of(-1, -1, -3, -3));
    }
}
