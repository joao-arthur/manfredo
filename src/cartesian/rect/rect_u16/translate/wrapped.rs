#[cfg(test)]
mod tests {
    use super::super::{wrapping_translate, wrapping_translate_assign};
    use crate::cartesian::{point::point_i16::PointI16, rect::rect_u16::RectU16};

    #[test]
    fn test_wrapping_translate_assign() {
        let mut r = RectU16::of(0, 0, 12, 15);
        wrapping_translate_assign(&mut r, &PointI16::of(5, 4));
        assert_eq!(r, RectU16::of(5, 4, 17, 19));
        wrapping_translate_assign(&mut r, &PointI16::of(-4, -2));
        assert_eq!(r, RectU16::of(1, 2, 13, 17));
    }

    #[test]
    fn wrapping_translate_assign_small_rect_to_bounds() {
        let mut min_r = RectU16::of(2, 5, 12, 15);
        wrapping_translate_assign(&mut min_r, &PointI16::of(-2, -5));
        assert_eq!(min_r, RectU16::of(0, 0, 10, 10));

        let mut max_r = RectU16::of(u16::MAX - 12, u16::MAX - 15, u16::MAX - 2, u16::MAX - 5);
        wrapping_translate_assign(&mut max_r, &PointI16::of(2, 5));
        assert_eq!(max_r, RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX, u16::MAX));
    }

    #[test]
    fn wrapping_translate_assign_big_rect_to_bounds() {
        let mut min_r = RectU16::of(2, 5, u16::MAX, u16::MAX);
        wrapping_translate_assign(&mut min_r, &PointI16::of(-2, -5));
        assert_eq!(min_r, RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5));

        let mut max_r = RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5);
        wrapping_translate_assign(&mut max_r, &PointI16::of(2, 5));
        assert_eq!(max_r, RectU16::of(2, 5, u16::MAX, u16::MAX));
    }

    #[test]
    fn wrapping_translate_assign_small_rect_out_of_bounds() {
        let mut r_min = RectU16::of(10, 5, 20, 30);
        wrapping_translate_assign(&mut r_min, &PointI16::of(-20, -20));
        assert_eq!(r_min, RectU16::of(u16::MAX - 9, u16::MAX - 14, 0, 10));

        let mut r_max = RectU16::of(u16::MAX - 20, u16::MAX - 30, u16::MAX - 5, u16::MAX - 10);
        wrapping_translate_assign(&mut r_max, &PointI16::of(20, 20));
        assert_eq!(r_max, RectU16::of(u16::MAX, u16::MAX - 10, 14, 9));
    }

    #[test]
    fn wrapping_translate_assign_big_rect_out_of_bounds() {
        let mut r_min = RectU16::of(10, 5, u16::MAX, u16::MAX);
        wrapping_translate_assign(&mut r_min, &PointI16::of(-20, -20));
        assert_eq!(r_min, RectU16::of(u16::MAX - 9, u16::MAX - 14, u16::MAX - 20, u16::MAX - 20));

        let mut r_max = RectU16::of(0, 0, u16::MAX - 5, u16::MAX - 10);
        wrapping_translate_assign(&mut r_max, &PointI16::of(20, 20));
        assert_eq!(r_max, RectU16::of(20, 20, 14, 9));
    }

    #[test]
    fn wrapping_translate_assign_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU16::of(1, 1, 10, 10);
        wrapping_translate_assign(&mut r_min, &PointI16::min());
        assert_eq!(r_min, RectU16::of((i16::MAX as u16) + 2, (i16::MAX as u16) + 2, (i16::MAX as u16) + 11, (i16::MAX as u16) + 11));

        let mut r_max = RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX - 1, u16::MAX - 1);
        wrapping_translate_assign(&mut r_max, &PointI16::max());
        assert_eq!(r_max, RectU16::of((i16::MAX as u16) - 11, (i16::MAX as u16) - 11, (i16::MAX as u16) - 2, (i16::MAX as u16) - 2));
    }

    #[test]
    fn wrapping_translate_assign_big_rect_limits_out_of_bounds() {
        let mut r1 = RectU16::largest();
        wrapping_translate_assign(&mut r1, &PointI16::min());
        assert_eq!(r1, RectU16::of((i16::MAX as u16) + 1, (i16::MAX as u16) + 1, i16::MAX as u16, i16::MAX as u16));

        let mut r2 = RectU16::largest();
        wrapping_translate_assign(&mut r2, &PointI16::max());
        assert_eq!(r2, RectU16::of(i16::MAX as u16, i16::MAX as u16, (i16::MAX as u16) - 1, (i16::MAX as u16) - 1));

        let mut r_min = RectU16::of(1, 1, u16::MAX, u16::MAX);
        wrapping_translate_assign(&mut r_min, &PointI16::min());
        assert_eq!(r_min, RectU16::of((i16::MAX as u16) + 2, (i16::MAX as u16) + 2, i16::MAX as u16, i16::MAX as u16));

        let mut r_max = RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1);
        wrapping_translate_assign(&mut r_max, &PointI16::max());
        assert_eq!(r_max, RectU16::of(i16::MAX as u16, i16::MAX as u16, (i16::MAX as u16) - 2, (i16::MAX as u16) - 2));
    }

    #[test]
    fn test_wrapping_translate() {
        assert_eq!(wrapping_translate(&RectU16::of(0, 0, 12, 15), &PointI16::of(5, 4)), RectU16::of(5, 4, 17, 19));
        assert_eq!(wrapping_translate(&RectU16::of(5, 4, 17, 19), &PointI16::of(-4, -2)), RectU16::of(1, 2, 13, 17));
    }

    #[test]
    fn wrapping_translate_small_rect_to_bounds() {
        assert_eq!(wrapping_translate(&RectU16::of(2, 5, 12, 15), &PointI16::of(-2, -5)), RectU16::of(0, 0, 10, 10));
        assert_eq!(
            wrapping_translate(&RectU16::of(u16::MAX - 12, u16::MAX - 15, u16::MAX - 2, u16::MAX - 5), &PointI16::of(2, 5)),
            RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX, u16::MAX)
        );
    }

    #[test]
    fn wrapping_translate_big_rect_to_bounds() {
        assert_eq!(wrapping_translate(&RectU16::of(2, 5, u16::MAX, u16::MAX), &PointI16::of(-2, -5)), RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5));
        assert_eq!(wrapping_translate(&RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5), &PointI16::of(2, 5)), RectU16::of(2, 5, u16::MAX, u16::MAX));
    }

    #[test]
    fn wrapping_translate_small_rect_out_of_bounds() {
        assert_eq!(wrapping_translate(&RectU16::of(10, 5, 20, 30), &PointI16::of(-20, -20)), RectU16::of(u16::MAX - 9, u16::MAX - 14, 0, 10));
        assert_eq!(
            wrapping_translate(&RectU16::of(u16::MAX - 20, u16::MAX - 30, u16::MAX - 5, u16::MAX - 10), &PointI16::of(20, 20)),
            RectU16::of(u16::MAX, u16::MAX - 10, 14, 9)
        );
    }

    #[test]
    fn wrapping_translate_big_rect_out_of_bounds() {
        assert_eq!(
            wrapping_translate(&RectU16::of(10, 5, u16::MAX, u16::MAX), &PointI16::of(-20, -20)),
            RectU16::of(u16::MAX - 9, u16::MAX - 14, u16::MAX - 20, u16::MAX - 20)
        );
        assert_eq!(wrapping_translate(&RectU16::of(0, 0, u16::MAX - 5, u16::MAX - 10), &PointI16::of(20, 20)), RectU16::of(20, 20, 14, 9));
    }

    #[test]
    fn wrapping_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(
            wrapping_translate(&RectU16::of(1, 1, 10, 10), &PointI16::min()),
            RectU16::of((i16::MAX as u16) + 2, (i16::MAX as u16) + 2, (i16::MAX as u16) + 11, (i16::MAX as u16) + 11)
        );
        assert_eq!(
            wrapping_translate(&RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX - 1, u16::MAX - 1), &PointI16::max()),
            RectU16::of((i16::MAX as u16) - 11, (i16::MAX as u16) - 11, (i16::MAX as u16) - 2, (i16::MAX as u16) - 2)
        );
    }

    #[test]
    fn wrapping_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(
            wrapping_translate(&RectU16::largest(), &PointI16::min()),
            RectU16::of((i16::MAX as u16) + 1, (i16::MAX as u16) + 1, i16::MAX as u16, i16::MAX as u16)
        );
        assert_eq!(
            wrapping_translate(&RectU16::largest(), &PointI16::max()),
            RectU16::of(i16::MAX as u16, i16::MAX as u16, (i16::MAX as u16) - 1, (i16::MAX as u16) - 1)
        );
        assert_eq!(
            wrapping_translate(&RectU16::of(1, 1, u16::MAX, u16::MAX), &PointI16::min()),
            RectU16::of((i16::MAX as u16) + 2, (i16::MAX as u16) + 2, i16::MAX as u16, i16::MAX as u16)
        );
        assert_eq!(
            wrapping_translate(&RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1), &PointI16::max()),
            RectU16::of(i16::MAX as u16, i16::MAX as u16, (i16::MAX as u16) - 2, (i16::MAX as u16) - 2)
        );
    }
}
