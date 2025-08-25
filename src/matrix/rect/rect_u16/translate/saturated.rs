#[cfg(test)]
mod tests {
    use super::super::{saturating_translate, saturating_translate_assign};
    use crate::matrix::{point::point_i16::PointI16, rect::rect_u16::RectU16};

    #[test]
    fn test_saturating_translate_assign() {
        let mut r = RectU16::of(0, 0, 12, 15);
        saturating_translate_assign(&mut r, &PointI16::of(5, 4));
        assert_eq!(r, RectU16::of(5, 4, 17, 19));
        saturating_translate_assign(&mut r, &PointI16::of(-4, -2));
        assert_eq!(r, RectU16::of(1, 2, 13, 17));
    }

    #[test]
    fn saturating_translate_assign_small_rect_to_bounds() {
        let mut min_r = RectU16::of(2, 5, 12, 15);
        saturating_translate_assign(&mut min_r, &PointI16::of(-2, -5));
        assert_eq!(min_r, RectU16::of(0, 0, 10, 10));

        let mut max_r = RectU16::of(u16::MAX - 12, u16::MAX - 15, u16::MAX - 2, u16::MAX - 5);
        saturating_translate_assign(&mut max_r, &PointI16::of(2, 5));
        assert_eq!(max_r, RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX, u16::MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_to_bounds() {
        let mut min_r = RectU16::of(2, 5, u16::MAX, u16::MAX);
        saturating_translate_assign(&mut min_r, &PointI16::of(-2, -5));
        assert_eq!(min_r, RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5));

        let mut max_r = RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5);
        saturating_translate_assign(&mut max_r, &PointI16::of(2, 5));
        assert_eq!(max_r, RectU16::of(2, 5, u16::MAX, u16::MAX));
    }

    #[test]
    fn saturating_translate_assign_small_rect_out_of_bounds() {
        let mut r_min = RectU16::of(10, 5, 20, 30);
        saturating_translate_assign(&mut r_min, &PointI16::of(-20, -20));
        assert_eq!(r_min, RectU16::of(0, 0, 10, 25));

        let mut r_max = RectU16::of(u16::MAX - 20, u16::MAX - 30, u16::MAX - 5, u16::MAX - 10);
        saturating_translate_assign(&mut r_max, &PointI16::of(20, 20));
        assert_eq!(r_max, RectU16::of(u16::MAX - 15, u16::MAX - 20, u16::MAX, u16::MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_out_of_bounds() {
        let mut r_min = RectU16::of(10, 5, u16::MAX, u16::MAX);
        saturating_translate_assign(&mut r_min, &PointI16::of(-20, -20));
        assert_eq!(r_min, RectU16::of(0, 0, u16::MAX - 10, u16::MAX - 5));

        let mut r_max = RectU16::of(0, 0, u16::MAX - 5, u16::MAX - 10);
        saturating_translate_assign(&mut r_max, &PointI16::of(20, 20));
        assert_eq!(r_max, RectU16::of(5, 10, u16::MAX, u16::MAX));
    }

    #[test]
    fn saturating_translate_assign_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU16::of(1, 1, 10, 10);
        saturating_translate_assign(&mut r_min, &PointI16::min());
        assert_eq!(r_min, RectU16::of(0, 0, 9, 9));

        let mut r_max = RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX - 1, u16::MAX - 1);
        saturating_translate_assign(&mut r_max, &PointI16::max());
        assert_eq!(r_max, RectU16::of(u16::MAX - 9, u16::MAX - 9, u16::MAX, u16::MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_limits_out_of_bounds() {
        let mut r = RectU16::largest();
        saturating_translate_assign(&mut r, &PointI16::min());
        assert_eq!(r, RectU16::largest());
        saturating_translate_assign(&mut r, &PointI16::max());
        assert_eq!(r, RectU16::largest());

        let mut r_min = RectU16::of(1, 1, u16::MAX, u16::MAX);
        saturating_translate_assign(&mut r_min, &PointI16::min());
        assert_eq!(r_min, RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1));

        let mut r_max = RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1);
        saturating_translate_assign(&mut r_max, &PointI16::max());
        assert_eq!(r_max, RectU16::of(1, 1, u16::MAX, u16::MAX));
    }

    #[test]
    fn test_saturating_translate() {
        assert_eq!(saturating_translate(&RectU16::of(0, 0, 12, 15), &PointI16::of(5, 4)), RectU16::of(5, 4, 17, 19));
        assert_eq!(saturating_translate(&RectU16::of(5, 4, 17, 19), &PointI16::of(-4, -2)), RectU16::of(1, 2, 13, 17));
    }

    #[test]
    fn saturating_translate_small_rect_to_bounds() {
        assert_eq!(saturating_translate(&RectU16::of(2, 5, 12, 15), &PointI16::of(-2, -5)), RectU16::of(0, 0, 10, 10));
        assert_eq!(
            saturating_translate(&RectU16::of(u16::MAX - 12, u16::MAX - 15, u16::MAX - 2, u16::MAX - 5), &PointI16::of(2, 5)),
            RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX, u16::MAX)
        );
    }

    #[test]
    fn saturating_translate_big_rect_to_bounds() {
        assert_eq!(
            saturating_translate(&RectU16::of(2, 5, u16::MAX, u16::MAX), &PointI16::of(-2, -5)),
            RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5)
        );
        assert_eq!(saturating_translate(&RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5), &PointI16::of(2, 5)), RectU16::of(2, 5, u16::MAX, u16::MAX));
    }

    #[test]
    fn saturating_translate_small_rect_out_of_bounds() {
        assert_eq!(saturating_translate(&RectU16::of(10, 5, 20, 30), &PointI16::of(-20, -20)), RectU16::of(0, 0, 10, 25));
        assert_eq!(
            saturating_translate(&RectU16::of(u16::MAX - 20, u16::MAX - 30, u16::MAX - 5, u16::MAX - 10), &PointI16::of(20, 20)),
            RectU16::of(u16::MAX - 15, u16::MAX - 20, u16::MAX, u16::MAX)
        );
    }

    #[test]
    fn saturating_translate_big_rect_out_of_bounds() {
        assert_eq!(
            saturating_translate(&RectU16::of(10, 5, u16::MAX, u16::MAX), &PointI16::of(-20, -20)),
            RectU16::of(0, 0, u16::MAX - 10, u16::MAX - 5)
        );
        assert_eq!(
            saturating_translate(&RectU16::of(0, 0, u16::MAX - 5, u16::MAX - 10), &PointI16::of(20, 20)),
            RectU16::of(5, 10, u16::MAX, u16::MAX)
        );
    }

    #[test]
    fn saturating_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(saturating_translate(&RectU16::of(1, 1, 10, 10), &PointI16::min()), RectU16::of(0, 0, 9, 9));
        assert_eq!(
            saturating_translate(&RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX - 1, u16::MAX - 1), &PointI16::max()),
            RectU16::of(u16::MAX - 9, u16::MAX - 9, u16::MAX, u16::MAX)
        );
    }

    #[test]
    fn saturating_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(saturating_translate(&RectU16::of(1, 1, u16::MAX, u16::MAX), &PointI16::min()), RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1));
        assert_eq!(saturating_translate(&RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1), &PointI16::max()), RectU16::of(1, 1, u16::MAX, u16::MAX));
    }
}
