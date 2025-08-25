#[cfg(test)]
mod tests {
    use super::super::{wrapping_translate, wrapping_translate_assign};
    use crate::matrix::{point::point_i32::PointI32, rect::rect_i32::RectI32};

    #[test]
    fn test_wrapping_translate_assign() {
        let mut r = RectI32::of(0, 0, 12, 15);
        wrapping_translate_assign(&mut r, &PointI32::of(5, 4));
        assert_eq!(r, RectI32::of(5, 4, 17, 19));
        wrapping_translate_assign(&mut r, &PointI32::of(-4, -2));
        assert_eq!(r, RectI32::of(1, 2, 13, 17));
    }

    #[test]
    fn wrapping_translate_assign_small_rect_to_bounds() {
        let mut min_r = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MIN + 12, i32::MIN + 15);
        wrapping_translate_assign(&mut min_r, &PointI32::of(-2, -5));
        assert_eq!(min_r, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 10, i32::MIN + 10));

        let mut max_r = RectI32::of(i32::MAX - 12, i32::MAX - 15, i32::MAX - 2, i32::MAX - 5);
        wrapping_translate_assign(&mut max_r, &PointI32::of(2, 5));
        assert_eq!(max_r, RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX, i32::MAX));
    }

    #[test]
    fn wrapping_translate_assign_big_rect_to_bounds() {
        let mut min_r = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX);
        wrapping_translate_assign(&mut min_r, &PointI32::of(-2, -5));
        assert_eq!(min_r, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5));

        let mut max_r = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5);
        wrapping_translate_assign(&mut max_r, &PointI32::of(2, 5));
        assert_eq!(max_r, RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX));
    }

    #[test]
    fn wrapping_translate_assign_small_rect_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MIN + 20, i32::MIN + 30);
        wrapping_translate_assign(&mut r_min, &PointI32::of(-20, -20));
        assert_eq!(r_min, RectI32::of(i32::MAX - 9, i32::MAX - 14, i32::MIN, i32::MIN + 10));

        let mut r_max = RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX - 5, i32::MAX - 10);
        wrapping_translate_assign(&mut r_max, &PointI32::of(20, 20));
        assert_eq!(r_max, RectI32::of(i32::MAX, i32::MAX - 10, i32::MIN + 14, i32::MIN + 9));
    }

    #[test]
    fn wrapping_translate_assign_big_rect_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MAX, i32::MAX);
        wrapping_translate_assign(&mut r_min, &PointI32::of(-20, -20));
        assert_eq!(r_min, RectI32::of(i32::MAX - 9, i32::MAX - 14, i32::MAX - 20, i32::MAX - 20));

        let mut r_max = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 5, i32::MAX - 10);
        wrapping_translate_assign(&mut r_max, &PointI32::of(20, 20));
        assert_eq!(r_max, RectI32::of(i32::MIN + 20, i32::MIN + 20, i32::MIN + 14, i32::MIN + 9));
    }

    #[test]
    fn wrapping_translate_assign_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10);
        wrapping_translate_assign(&mut r_min, &PointI32::min());
        assert_eq!(r_min, RectI32::of(1, 1, 10, 10));

        let mut r_max = RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1);
        wrapping_translate_assign(&mut r_max, &PointI32::max());
        assert_eq!(r_max, RectI32::of(-12, -12, -3, -3));
    }

    #[test]
    fn wrapping_translate_assign_big_rect_limits_out_of_bounds() {
        let mut r1 = RectI32::largest();
        wrapping_translate_assign(&mut r1, &PointI32::min());
        assert_eq!(r1, RectI32::of(0, 0, -1, -1));

        let mut r2 = RectI32::largest();
        wrapping_translate_assign(&mut r2, &PointI32::max());
        assert_eq!(r2, RectI32::of(-1, -1, -2, -2));

        let mut r_min = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX);
        wrapping_translate_assign(&mut r_min, &PointI32::min());
        assert_eq!(r_min, RectI32::of(1, 1, -1, -1));

        let mut r_max = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1);
        wrapping_translate_assign(&mut r_max, &PointI32::max());
        assert_eq!(r_max, RectI32::of(-1, -1, -3, -3));
    }

    #[test]
    fn test_wrapping_translate() {
        assert_eq!(wrapping_translate(&RectI32::of(0, 0, 12, 15), &PointI32::of(5, 4)), RectI32::of(5, 4, 17, 19));
        assert_eq!(wrapping_translate(&RectI32::of(5, 4, 17, 19), &PointI32::of(-4, -2)), RectI32::of(1, 2, 13, 17));
    }

    #[test]
    fn wrapping_translate_small_rect_to_bounds() {
        assert_eq!(
            wrapping_translate(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MIN + 12, i32::MIN + 15), &PointI32::of(-2, -5)),
            RectI32::of(i32::MIN, i32::MIN, i32::MIN + 10, i32::MIN + 10)
        );
        assert_eq!(
            wrapping_translate(&RectI32::of(i32::MAX - 12, i32::MAX - 15, i32::MAX - 2, i32::MAX - 5), &PointI32::of(2, 5)),
            RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX, i32::MAX)
        );
    }

    #[test]
    fn wrapping_translate_big_rect_to_bounds() {
        assert_eq!(
            wrapping_translate(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX), &PointI32::of(-2, -5)),
            RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5)
        );
        assert_eq!(
            wrapping_translate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5), &PointI32::of(2, 5)),
            RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX)
        );
    }

    #[test]
    fn wrapping_translate_small_rect_out_of_bounds() {
        assert_eq!(
            wrapping_translate(&RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MIN + 20, i32::MIN + 30), &PointI32::of(-20, -20)),
            RectI32::of(i32::MAX - 9, i32::MAX - 14, i32::MIN, i32::MIN + 10)
        );
        assert_eq!(
            wrapping_translate(&RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX - 5, i32::MAX - 10), &PointI32::of(20, 20)),
            RectI32::of(i32::MAX, i32::MAX - 10, i32::MIN + 14, i32::MIN + 9)
        );
    }

    #[test]
    fn wrapping_translate_big_rect_out_of_bounds() {
        assert_eq!(
            wrapping_translate(&RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MAX, i32::MAX), &PointI32::of(-20, -20)),
            RectI32::of(i32::MAX - 9, i32::MAX - 14, i32::MAX - 20, i32::MAX - 20)
        );
        assert_eq!(
            wrapping_translate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 5, i32::MAX - 10), &PointI32::of(20, 20)),
            RectI32::of(i32::MIN + 20, i32::MIN + 20, i32::MIN + 14, i32::MIN + 9)
        );
    }

    #[test]
    fn wrapping_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(
            wrapping_translate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10), &PointI32::min()),
            RectI32::of(1, 1, 10, 10)
        );
        assert_eq!(
            wrapping_translate(&RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1), &PointI32::max()),
            RectI32::of(-12, -12, -3, -3)
        );
    }

    #[test]
    fn wrapping_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(wrapping_translate(&RectI32::largest(), &PointI32::min()), RectI32::of(0, 0, -1, -1));
        assert_eq!(wrapping_translate(&RectI32::largest(), &PointI32::max()), RectI32::of(-1, -1, -2, -2));
        assert_eq!(wrapping_translate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX), &PointI32::min()), RectI32::of(1, 1, -1, -1));
        assert_eq!(wrapping_translate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1), &PointI32::max()), RectI32::of(-1, -1, -3, -3));
    }
}
