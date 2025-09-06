#[cfg(test)]
mod tests {
    use super::super::{saturating_resize, saturating_resize_assign, try_saturating_resize, try_saturating_resize_assign};
    use crate::matrix::rect::rect_i16::RectI16;

    #[test]
    fn try_saturating_resize_assign_odd() {
        let mut r = RectI16::of(-5, -5, 5, 5);
        assert_eq!(try_saturating_resize_assign(&mut r, 11), Some(()));
        assert_eq!(r, RectI16::of(-5, -5, 5, 5));
        assert_eq!(try_saturating_resize_assign(&mut r, 9), Some(()));
        assert_eq!(r, RectI16::of(-4, -4, 4, 4));
        assert_eq!(try_saturating_resize_assign(&mut r, 7), Some(()));
        assert_eq!(r, RectI16::of(-3, -3, 3, 3));
        assert_eq!(try_saturating_resize_assign(&mut r, 5), Some(()));
        assert_eq!(r, RectI16::of(-2, -2, 2, 2));
        assert_eq!(try_saturating_resize_assign(&mut r, 3), Some(()));
        assert_eq!(r, RectI16::of(-1, -1, 1, 1));
        assert_eq!(try_saturating_resize_assign(&mut r, 9), Some(()));
        assert_eq!(r, RectI16::of(-4, -4, 4, 4));
    }

    #[test]
    fn try_saturating_resize_assign_even() {
        let mut r = RectI16::of(-5, -5, 4, 4);
        assert_eq!(try_saturating_resize_assign(&mut r, 10), Some(()));
        assert_eq!(r, RectI16::of(-5, -5, 4, 4));
        assert_eq!(try_saturating_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectI16::of(-4, -4, 3, 3));
        assert_eq!(try_saturating_resize_assign(&mut r, 6), Some(()));
        assert_eq!(r, RectI16::of(-3, -3, 2, 2));
        assert_eq!(try_saturating_resize_assign(&mut r, 4), Some(()));
        assert_eq!(r, RectI16::of(-2, -2, 1, 1));
        assert_eq!(try_saturating_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectI16::of(-4, -4, 3, 3));
    }

    #[test]
    fn try_saturating_resize_assign_small_size() {
        let mut r = RectI16::of(10, 10, 20, 20);
        assert_eq!(try_saturating_resize_assign(&mut r, 0), None);
        assert_eq!(try_saturating_resize_assign(&mut r, 1), None);
        assert_eq!(try_saturating_resize_assign(&mut r, 2), None);
        assert_eq!(r, RectI16::of(10, 10, 20, 20));
    }

    #[test]
    fn try_saturating_resize_assign_same_size() {
        let mut r_11 = RectI16::of(10, 10, 20, 20);
        assert_eq!(try_saturating_resize_assign(&mut r_11, 11), Some(()));
        assert_eq!(r_11, RectI16::of(10, 10, 20, 20));

        let mut r_12 = RectI16::of(10, 10, 21, 21);
        assert_eq!(try_saturating_resize_assign(&mut r_12, 12), Some(()));
        assert_eq!(r_12, RectI16::of(10, 10, 21, 21));

        let mut r_13 = RectI16::of(9, 9, 21, 21);
        assert_eq!(try_saturating_resize_assign(&mut r_13, 13), Some(()));
        assert_eq!(r_13, RectI16::of(9, 9, 21, 21));
    }

    #[test]
    fn try_saturating_resize_assign_odd_small_rect_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN, i16::MIN, i16::MIN + 2, i16::MIN + 2);
        assert_eq!(try_saturating_resize_assign(&mut r_min, 11), Some(()));
        assert_eq!(r_min, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 10));

        let mut r_max = RectI16::of(i16::MAX - 2, i16::MAX - 2, i16::MAX, i16::MAX);
        assert_eq!(try_saturating_resize_assign(&mut r_max, 11), Some(()));
        assert_eq!(r_max, RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX, i16::MAX));
    }

    #[test]
    fn try_saturating_resize_assign_even_small_rect_out_of_bounds() {
        let mut r = RectI16::of(i16::MIN, i16::MIN, i16::MIN + 3, i16::MIN + 3);
        assert_eq!(try_saturating_resize_assign(&mut r, 11), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 10));

        let mut r = RectI16::of(i16::MAX - 3, i16::MAX - 3, i16::MAX, i16::MAX);
        assert_eq!(try_saturating_resize_assign(&mut r, 11), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX, i16::MAX));
    }

    #[test]
    fn try_saturating_resize_assign_odd_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN, i16::MIN, i16::MIN + 2, i16::MIN + 2);
        assert_eq!(try_saturating_resize_assign(&mut r_min, u16::MAX), Some(()));
        assert_eq!(r_min, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1));

        let mut r_max = RectI16::of(i16::MAX - 2, i16::MAX - 2, i16::MAX, i16::MAX);
        assert_eq!(try_saturating_resize_assign(&mut r_max, u16::MAX), Some(()));
        assert_eq!(r_max, RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX));
    }

    #[test]
    fn try_saturating_resize_assign_even_small_rect_limits_out_of_bounds() {
        let mut r = RectI16::of(i16::MIN, i16::MIN, i16::MIN + 3, i16::MIN + 3);
        assert_eq!(try_saturating_resize_assign(&mut r, u16::MAX - 1), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 2));

        let mut r = RectI16::of(i16::MAX - 3, i16::MAX - 3, i16::MAX, i16::MAX);
        assert_eq!(try_saturating_resize_assign(&mut r, u16::MAX - 1), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN + 2, i16::MIN + 2, i16::MAX, i16::MAX));
    }

    #[test]
    fn try_saturating_resize_assign_big_rect_limits_out_of_bounds() {
        let mut r_odd_1 = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1);
        assert_eq!(try_saturating_resize_assign(&mut r_odd_1, u16::MAX), Some(()));
        assert_eq!(r_odd_1, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1));

        let mut r_odd_1 = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX);
        assert_eq!(try_saturating_resize_assign(&mut r_odd_1, u16::MAX), Some(()));
        assert_eq!(r_odd_1, RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX));

        let mut r_even = RectI16::largest();
        assert_eq!(try_saturating_resize_assign(&mut r_even, u16::MAX), Some(()));
        assert_eq!(r_even, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1));
    }

    #[test]
    fn try_saturating_resize_odd() {
        assert_eq!(try_saturating_resize(&RectI16::of(-5, -5, 5, 5), 11), Some(RectI16::of(-5, -5, 5, 5)));
        assert_eq!(try_saturating_resize(&RectI16::of(-5, -5, 5, 5), 9), Some(RectI16::of(-4, -4, 4, 4)));
        assert_eq!(try_saturating_resize(&RectI16::of(-4, -4, 4, 4), 7), Some(RectI16::of(-3, -3, 3, 3)));
        assert_eq!(try_saturating_resize(&RectI16::of(-3, -3, 3, 3), 5), Some(RectI16::of(-2, -2, 2, 2)));
        assert_eq!(try_saturating_resize(&RectI16::of(-2, -2, 2, 2), 3), Some(RectI16::of(-1, -1, 1, 1)));
        assert_eq!(try_saturating_resize(&RectI16::of(-1, -1, 1, 1), 9), Some(RectI16::of(-4, -4, 4, 4)));
    }

    #[test]
    fn try_saturating_resize_even() {
        assert_eq!(try_saturating_resize(&RectI16::of(-5, -5, 4, 4), 10), Some(RectI16::of(-5, -5, 4, 4)));
        assert_eq!(try_saturating_resize(&RectI16::of(-5, -5, 4, 4), 8), Some(RectI16::of(-4, -4, 3, 3)));
        assert_eq!(try_saturating_resize(&RectI16::of(-4, -4, 3, 3), 6), Some(RectI16::of(-3, -3, 2, 2)));
        assert_eq!(try_saturating_resize(&RectI16::of(-3, -3, 2, 2), 4), Some(RectI16::of(-2, -2, 1, 1)));
        assert_eq!(try_saturating_resize(&RectI16::of(-2, -2, 1, 1), 8), Some(RectI16::of(-4, -4, 3, 3)));
    }

    #[test]
    fn try_saturating_resize_small_size() {
        let r = RectI16::of(10, 10, 20, 20);
        assert_eq!(try_saturating_resize(&r, 0), None);
        assert_eq!(try_saturating_resize(&r, 1), None);
        assert_eq!(try_saturating_resize(&r, 2), None);
    }

    #[test]
    fn try_saturating_resize_same_size() {
        assert_eq!(try_saturating_resize(&RectI16::of(10, 10, 20, 20), 11), Some(RectI16::of(10, 10, 20, 20)));
        assert_eq!(try_saturating_resize(&RectI16::of(10, 10, 21, 21), 12), Some(RectI16::of(10, 10, 21, 21)));
        assert_eq!(try_saturating_resize(&RectI16::of(9, 9, 21, 21), 13), Some(RectI16::of(9, 9, 21, 21)));
    }

    #[test]
    fn try_saturating_resize_odd_small_rect_out_of_bounds() {
        assert_eq!(try_saturating_resize(&RectI16::of(i16::MIN, i16::MIN, i16::MIN + 2, i16::MIN + 2), 11), Some(RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 10)));
        assert_eq!(try_saturating_resize(&RectI16::of(i16::MAX - 2, i16::MAX - 2, i16::MAX, i16::MAX), 11), Some(RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX, i16::MAX)));
    }

    #[test]
    fn try_saturating_resize_even_small_rect_out_of_bounds() {
        assert_eq!(try_saturating_resize(&RectI16::of(i16::MIN, i16::MIN, i16::MIN + 3, i16::MIN + 3), 11), Some(RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 10)));
        assert_eq!(try_saturating_resize(&RectI16::of(i16::MAX - 3, i16::MAX - 3, i16::MAX, i16::MAX), 11), Some(RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX, i16::MAX)));
    }

    #[test]
    fn try_saturating_resize_odd_small_rect_limits_out_of_bounds() {
        assert_eq!(try_saturating_resize(&RectI16::of(i16::MIN, i16::MIN, i16::MIN + 2, i16::MIN + 2), u16::MAX), Some(RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1)));
        assert_eq!(try_saturating_resize(&RectI16::of(i16::MAX - 2, i16::MAX - 2, i16::MAX, i16::MAX), u16::MAX), Some(RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX)));
    }

    #[test]
    fn try_saturating_resize_even_small_rect_limits_out_of_bounds() {
        assert_eq!(try_saturating_resize(&RectI16::of(i16::MIN, i16::MIN, i16::MIN + 3, i16::MIN + 3), u16::MAX - 1), Some(RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 2)));
        assert_eq!(try_saturating_resize(&RectI16::of(i16::MAX - 3, i16::MAX - 3, i16::MAX, i16::MAX), u16::MAX - 1), Some(RectI16::of(i16::MIN + 2, i16::MIN + 2, i16::MAX, i16::MAX)));
    }

    #[test]
    fn try_saturating_resize_big_rect_limits_out_of_bounds() {
        assert_eq!(try_saturating_resize(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1), u16::MAX), Some(RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1)));
        assert_eq!(try_saturating_resize(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX), u16::MAX), Some(RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX)));
        assert_eq!(try_saturating_resize(&RectI16::largest(), u16::MAX), Some(RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1)));
    }

    #[test]
    fn saturating_resize_assign_odd() {
        let mut r = RectI16::of(-5, -5, 5, 5);
        saturating_resize_assign(&mut r, 11);
        assert_eq!(r, RectI16::of(-5, -5, 5, 5));
        saturating_resize_assign(&mut r, 9);
        assert_eq!(r, RectI16::of(-4, -4, 4, 4));
        saturating_resize_assign(&mut r, 7);
        assert_eq!(r, RectI16::of(-3, -3, 3, 3));
        saturating_resize_assign(&mut r, 5);
        assert_eq!(r, RectI16::of(-2, -2, 2, 2));
        saturating_resize_assign(&mut r, 3);
        assert_eq!(r, RectI16::of(-1, -1, 1, 1));
        saturating_resize_assign(&mut r, 9);
        assert_eq!(r, RectI16::of(-4, -4, 4, 4));
    }

    #[test]
    fn saturating_resize_assign_even() {
        let mut r = RectI16::of(-5, -5, 4, 4);
        saturating_resize_assign(&mut r, 10);
        assert_eq!(r, RectI16::of(-5, -5, 4, 4));
        saturating_resize_assign(&mut r, 8);
        assert_eq!(r, RectI16::of(-4, -4, 3, 3));
        saturating_resize_assign(&mut r, 6);
        assert_eq!(r, RectI16::of(-3, -3, 2, 2));
        saturating_resize_assign(&mut r, 4);
        assert_eq!(r, RectI16::of(-2, -2, 1, 1));
        saturating_resize_assign(&mut r, 8);
        assert_eq!(r, RectI16::of(-4, -4, 3, 3));
    }

    #[test]
    fn saturating_resize_odd() {
        assert_eq!(saturating_resize(&RectI16::of(-5, -5, 5, 5), 11), RectI16::of(-5, -5, 5, 5));
        assert_eq!(saturating_resize(&RectI16::of(-5, -5, 5, 5), 9), RectI16::of(-4, -4, 4, 4));
        assert_eq!(saturating_resize(&RectI16::of(-4, -4, 4, 4), 7), RectI16::of(-3, -3, 3, 3));
        assert_eq!(saturating_resize(&RectI16::of(-3, -3, 3, 3), 5), RectI16::of(-2, -2, 2, 2));
        assert_eq!(saturating_resize(&RectI16::of(-2, -2, 2, 2), 3), RectI16::of(-1, -1, 1, 1));
        assert_eq!(saturating_resize(&RectI16::of(-1, -1, 1, 1), 9), RectI16::of(-4, -4, 4, 4));
    }

    #[test]
    fn saturating_resize_even() {
        assert_eq!(saturating_resize(&RectI16::of(-5, -5, 4, 4), 10), RectI16::of(-5, -5, 4, 4));
        assert_eq!(saturating_resize(&RectI16::of(-5, -5, 4, 4), 8), RectI16::of(-4, -4, 3, 3));
        assert_eq!(saturating_resize(&RectI16::of(-4, -4, 3, 3), 6), RectI16::of(-3, -3, 2, 2));
        assert_eq!(saturating_resize(&RectI16::of(-3, -3, 2, 2), 4), RectI16::of(-2, -2, 1, 1));
        assert_eq!(saturating_resize(&RectI16::of(-2, -2, 1, 1), 8), RectI16::of(-4, -4, 3, 3));
    }
}
