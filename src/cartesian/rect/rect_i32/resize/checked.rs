#[cfg(test)]
mod tests {
    use super::super::{checked_resize, checked_resize_assign, try_checked_resize, try_checked_resize_assign};
    use crate::cartesian::rect::rect_i32::RectI32;

    #[test]
    fn try_checked_resize_assign_odd() {
        let mut r = RectI32::of(-5, -5, 5, 5);
        assert_eq!(try_checked_resize_assign(&mut r, 11), Some(()));
        assert_eq!(r, RectI32::of(-5, -5, 5, 5));
        assert_eq!(try_checked_resize_assign(&mut r, 9), Some(()));
        assert_eq!(r, RectI32::of(-4, -4, 4, 4));
        assert_eq!(try_checked_resize_assign(&mut r, 7), Some(()));
        assert_eq!(r, RectI32::of(-3, -3, 3, 3));
        assert_eq!(try_checked_resize_assign(&mut r, 5), Some(()));
        assert_eq!(r, RectI32::of(-2, -2, 2, 2));
        assert_eq!(try_checked_resize_assign(&mut r, 3), Some(()));
        assert_eq!(r, RectI32::of(-1, -1, 1, 1));
        assert_eq!(try_checked_resize_assign(&mut r, 9), Some(()));
        assert_eq!(r, RectI32::of(-4, -4, 4, 4));
    }

    #[test]
    fn try_checked_resize_assign_even() {
        let mut r = RectI32::of(-5, -5, 4, 4);
        assert_eq!(try_checked_resize_assign(&mut r, 10), Some(()));
        assert_eq!(r, RectI32::of(-5, -5, 4, 4));
        assert_eq!(try_checked_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectI32::of(-4, -4, 3, 3));
        assert_eq!(try_checked_resize_assign(&mut r, 6), Some(()));
        assert_eq!(r, RectI32::of(-3, -3, 2, 2));
        assert_eq!(try_checked_resize_assign(&mut r, 4), Some(()));
        assert_eq!(r, RectI32::of(-2, -2, 1, 1));
        assert_eq!(try_checked_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectI32::of(-4, -4, 3, 3));
    }

    #[test]
    fn try_checked_resize_assign_small_size() {
        let mut r = RectI32::of(10, 10, 20, 20);
        assert_eq!(try_checked_resize_assign(&mut r, 0), None);
        assert_eq!(try_checked_resize_assign(&mut r, 1), None);
        assert_eq!(try_checked_resize_assign(&mut r, 2), None);
        assert_eq!(r, RectI32::of(10, 10, 20, 20));
    }

    #[test]
    fn try_checked_resize_assign_same_size() {
        let mut r_11 = RectI32::of(10, 10, 20, 20);
        assert_eq!(try_checked_resize_assign(&mut r_11, 11), Some(()));
        assert_eq!(r_11, RectI32::of(10, 10, 20, 20));

        let mut r_12 = RectI32::of(10, 10, 21, 21);
        assert_eq!(try_checked_resize_assign(&mut r_12, 12), Some(()));
        assert_eq!(r_12, RectI32::of(10, 10, 21, 21));

        let mut r_13 = RectI32::of(9, 9, 21, 21);
        assert_eq!(try_checked_resize_assign(&mut r_13, 13), Some(()));
        assert_eq!(r_13, RectI32::of(9, 9, 21, 21));
    }

    #[test]
    fn try_checked_resize_assign_odd_small_rect_same_size() {
        let mut r_min = RectI32::of(i32::MIN, i32::MIN, i32::MIN + 2, i32::MIN + 2);
        assert_eq!(try_checked_resize_assign(&mut r_min, 3), Some(()));
        assert_eq!(r_min, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 2, i32::MIN + 2));

        let mut r_max = RectI32::of(i32::MAX - 2, i32::MAX - 2, i32::MAX, i32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, 3), Some(()));
        assert_eq!(r_max, RectI32::of(i32::MAX - 2, i32::MAX - 2, i32::MAX, i32::MAX));
    }

    #[test]
    fn try_checked_resize_assign_even_small_rect_same_size() {
        let mut r = RectI32::of(i32::MIN, i32::MIN, i32::MIN + 3, i32::MIN + 3);
        assert_eq!(try_checked_resize_assign(&mut r, 4), Some(()));
        assert_eq!(r, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 3, i32::MIN + 3));

        let mut r = RectI32::of(i32::MAX - 3, i32::MAX - 3, i32::MAX, i32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r, 4), Some(()));
        assert_eq!(r, RectI32::of(i32::MAX - 3, i32::MAX - 3, i32::MAX, i32::MAX));
    }

    #[test]
    fn try_checked_resize_assign_odd_small_rect_to_bounds() {
        let mut r_min = RectI32::of(2, 2, 4, 4);
        assert_eq!(try_checked_resize_assign(&mut r_min, 7), Some(()));
        assert_eq!(r_min, RectI32::of(0, 0, 6, 6));

        let mut r_max = RectI32::of(i32::MAX - 4, i32::MAX - 4, i32::MAX - 2, i32::MAX - 2);
        assert_eq!(try_checked_resize_assign(&mut r_max, 7), Some(()));
        assert_eq!(r_max, RectI32::of(i32::MAX - 6, i32::MAX - 6, i32::MAX, i32::MAX));
    }

    #[test]
    fn try_checked_resize_assign_even_small_rect_to_bounds() {
        let mut r = RectI32::of(2, 2, 5, 5);
        assert_eq!(try_checked_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectI32::of(0, 0, 7, 7));

        let mut r = RectI32::of(i32::MAX - 5, i32::MAX - 5, i32::MAX - 2, i32::MAX - 2);
        assert_eq!(try_checked_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectI32::of(i32::MAX - 7, i32::MAX - 7, i32::MAX, i32::MAX));
    }

    #[test]
    fn try_checked_resize_assign_odd_small_rect_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN, i32::MIN, i32::MIN + 2, i32::MIN + 2);
        assert_eq!(try_checked_resize_assign(&mut r_min, 5), None);
        assert_eq!(r_min, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 2, i32::MIN + 2));

        let mut r_min_x = RectI32::of(i32::MIN, i32::MIN + 2, i32::MIN + 2, i32::MIN + 4);
        assert_eq!(try_checked_resize_assign(&mut r_min_x, 5), None);
        assert_eq!(r_min_x, RectI32::of(i32::MIN, i32::MIN + 2, i32::MIN + 2, i32::MIN + 4));

        let mut r_min_y = RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 4, i32::MIN + 2);
        assert_eq!(try_checked_resize_assign(&mut r_min_y, 5), None);
        assert_eq!(r_min_y, RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 4, i32::MIN + 2));

        let mut r_max = RectI32::of(i32::MAX - 2, i32::MAX - 2, i32::MAX, i32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, 5), None);
        assert_eq!(r_max, RectI32::of(i32::MAX - 2, i32::MAX - 2, i32::MAX, i32::MAX));

        let mut r_max_x = RectI32::of(i32::MAX - 2, i32::MAX - 4, i32::MAX, i32::MAX - 2);
        assert_eq!(try_checked_resize_assign(&mut r_max_x, 5), None);
        assert_eq!(r_max_x, RectI32::of(i32::MAX - 2, i32::MAX - 4, i32::MAX, i32::MAX - 2));

        let mut r_max_y = RectI32::of(i32::MAX - 4, i32::MAX - 2, i32::MAX - 2, i32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max_y, 5), None);
        assert_eq!(r_max_y, RectI32::of(i32::MAX - 4, i32::MAX - 2, i32::MAX - 2, i32::MAX));
    }

    #[test]
    fn try_checked_resize_assign_even_small_rect_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN, i32::MIN, i32::MIN + 3, i32::MIN + 3);
        assert_eq!(try_checked_resize_assign(&mut r_min, 6), None);
        assert_eq!(r_min, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 3, i32::MIN + 3));

        let mut r_min_x = RectI32::of(i32::MIN, i32::MIN + 3, i32::MIN + 3, i32::MIN + 6);
        assert_eq!(try_checked_resize_assign(&mut r_min_x, 6), None);
        assert_eq!(r_min_x, RectI32::of(i32::MIN, i32::MIN + 3, i32::MIN + 3, i32::MIN + 6));

        let mut r_min_y = RectI32::of(i32::MIN + 3, i32::MIN, i32::MIN + 6, i32::MIN + 3);
        assert_eq!(try_checked_resize_assign(&mut r_min_y, 6), None);
        assert_eq!(r_min_y, RectI32::of(i32::MIN + 3, i32::MIN, i32::MIN + 6, i32::MIN + 3));

        let mut r_max = RectI32::of(i32::MAX - 3, i32::MAX - 3, i32::MAX, i32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, 6), None);
        assert_eq!(r_max, RectI32::of(i32::MAX - 3, i32::MAX - 3, i32::MAX, i32::MAX));

        let mut r_max_x = RectI32::of(i32::MAX - 3, i32::MAX - 6, i32::MAX, i32::MAX - 3);
        assert_eq!(try_checked_resize_assign(&mut r_max_x, 6), None);
        assert_eq!(r_max_x, RectI32::of(i32::MAX - 3, i32::MAX - 6, i32::MAX, i32::MAX - 3));

        let mut r_max_y = RectI32::of(i32::MAX - 6, i32::MAX - 3, i32::MAX - 3, i32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max_y, 6), None);
        assert_eq!(r_max_y, RectI32::of(i32::MAX - 6, i32::MAX - 3, i32::MAX - 3, i32::MAX));
    }

    #[test]
    fn try_checked_resize_assign_odd_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN, i32::MIN, i32::MIN + 2, i32::MIN + 2);
        assert_eq!(try_checked_resize_assign(&mut r_min, u32::MAX), None);
        assert_eq!(r_min, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 2, i32::MIN + 2));

        let mut r_min_x = RectI32::of(i32::MIN, i32::MIN + 2, i32::MIN + 2, i32::MIN + 4);
        assert_eq!(try_checked_resize_assign(&mut r_min_x, u32::MAX), None);
        assert_eq!(r_min_x, RectI32::of(i32::MIN, i32::MIN + 2, i32::MIN + 2, i32::MIN + 4));

        let mut r_min_y = RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 4, i32::MIN + 2);
        assert_eq!(try_checked_resize_assign(&mut r_min_y, u32::MAX), None);
        assert_eq!(r_min_y, RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 4, i32::MIN + 2));

        let mut r_max = RectI32::of(i32::MAX - 2, i32::MAX - 2, i32::MAX, i32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, u32::MAX), None);
        assert_eq!(r_max, RectI32::of(i32::MAX - 2, i32::MAX - 2, i32::MAX, i32::MAX));

        let mut r_max_x = RectI32::of(i32::MAX - 2, i32::MAX - 4, i32::MAX, i32::MAX - 2);
        assert_eq!(try_checked_resize_assign(&mut r_max_x, u32::MAX), None);
        assert_eq!(r_max_x, RectI32::of(i32::MAX - 2, i32::MAX - 4, i32::MAX, i32::MAX - 2));

        let mut r_max_y = RectI32::of(i32::MAX - 4, i32::MAX - 2, i32::MAX - 2, i32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max_y, u32::MAX), None);
        assert_eq!(r_max_y, RectI32::of(i32::MAX - 4, i32::MAX - 2, i32::MAX - 2, i32::MAX));
    }

    #[test]
    fn try_checked_resize_assign_even_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN, i32::MIN, i32::MIN + 3, i32::MIN + 3);
        assert_eq!(try_checked_resize_assign(&mut r_min, u32::MAX - 1), None);
        assert_eq!(r_min, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 3, i32::MIN + 3));

        let mut r_min_x = RectI32::of(i32::MIN, i32::MIN + 3, i32::MIN + 3, i32::MIN + 6);
        assert_eq!(try_checked_resize_assign(&mut r_min_x, u32::MAX - 1), None);
        assert_eq!(r_min_x, RectI32::of(i32::MIN, i32::MIN + 3, i32::MIN + 3, i32::MIN + 6));

        let mut r_min_y = RectI32::of(i32::MIN + 3, i32::MIN, i32::MIN + 6, i32::MIN + 3);
        assert_eq!(try_checked_resize_assign(&mut r_min_y, u32::MAX - 1), None);
        assert_eq!(r_min_y, RectI32::of(i32::MIN + 3, i32::MIN, i32::MIN + 6, i32::MIN + 3));

        let mut r_max = RectI32::of(i32::MAX - 3, i32::MAX - 3, i32::MAX, i32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, u32::MAX - 1), None);
        assert_eq!(r_max, RectI32::of(i32::MAX - 3, i32::MAX - 3, i32::MAX, i32::MAX));

        let mut r_max_x = RectI32::of(i32::MAX - 3, i32::MAX - 6, i32::MAX, i32::MAX - 3);
        assert_eq!(try_checked_resize_assign(&mut r_max_x, u32::MAX - 1), None);
        assert_eq!(r_max_x, RectI32::of(i32::MAX - 3, i32::MAX - 6, i32::MAX, i32::MAX - 3));

        let mut r_max_y = RectI32::of(i32::MAX - 6, i32::MAX - 3, i32::MAX - 3, i32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max_y, u32::MAX - 1), None);
        assert_eq!(r_max_y, RectI32::of(i32::MAX - 6, i32::MAX - 3, i32::MAX - 3, i32::MAX));
    }

    #[test]
    fn try_checked_resize_assign_big_rect_limits_out_of_bounds() {
        let mut r_odd_1 = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1);
        assert_eq!(try_checked_resize_assign(&mut r_odd_1, u32::MAX), Some(()));
        assert_eq!(r_odd_1, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1));

        let mut r_odd_1 = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_odd_1, u32::MAX), Some(()));
        assert_eq!(r_odd_1, RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX));

        let mut r_even = RectI32::largest();
        assert_eq!(try_checked_resize_assign(&mut r_even, u32::MAX), Some(()));
        assert_eq!(r_even, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1));
    }

    #[test]
    fn try_checked_resize_odd() {
        assert_eq!(try_checked_resize(&RectI32::of(-5, -5, 5, 5), 11), Some(RectI32::of(-5, -5, 5, 5)));
        assert_eq!(try_checked_resize(&RectI32::of(-5, -5, 5, 5), 9), Some(RectI32::of(-4, -4, 4, 4)));
        assert_eq!(try_checked_resize(&RectI32::of(-4, -4, 4, 4), 7), Some(RectI32::of(-3, -3, 3, 3)));
        assert_eq!(try_checked_resize(&RectI32::of(-3, -3, 3, 3), 5), Some(RectI32::of(-2, -2, 2, 2)));
        assert_eq!(try_checked_resize(&RectI32::of(-2, -2, 2, 2), 3), Some(RectI32::of(-1, -1, 1, 1)));
        assert_eq!(try_checked_resize(&RectI32::of(-1, -1, 1, 1), 9), Some(RectI32::of(-4, -4, 4, 4)));
    }

    #[test]
    fn try_checked_resize_even() {
        assert_eq!(try_checked_resize(&RectI32::of(-5, -5, 4, 4), 10), Some(RectI32::of(-5, -5, 4, 4)));
        assert_eq!(try_checked_resize(&RectI32::of(-5, -5, 4, 4), 8), Some(RectI32::of(-4, -4, 3, 3)));
        assert_eq!(try_checked_resize(&RectI32::of(-4, -4, 3, 3), 6), Some(RectI32::of(-3, -3, 2, 2)));
        assert_eq!(try_checked_resize(&RectI32::of(-3, -3, 2, 2), 4), Some(RectI32::of(-2, -2, 1, 1)));
        assert_eq!(try_checked_resize(&RectI32::of(-2, -2, 1, 1), 8), Some(RectI32::of(-4, -4, 3, 3)));
    }

    #[test]
    fn try_checked_resize_small_size() {
        let r = RectI32::of(10, 10, 20, 20);
        assert_eq!(try_checked_resize(&r, 0), None);
        assert_eq!(try_checked_resize(&r, 1), None);
        assert_eq!(try_checked_resize(&r, 2), None);
    }

    #[test]
    fn try_checked_resize_same_size() {
        assert_eq!(try_checked_resize(&RectI32::of(10, 10, 20, 20), 11), Some(RectI32::of(10, 10, 20, 20)));
        assert_eq!(try_checked_resize(&RectI32::of(10, 10, 21, 21), 12), Some(RectI32::of(10, 10, 21, 21)));
        assert_eq!(try_checked_resize(&RectI32::of(9, 9, 21, 21), 13), Some(RectI32::of(9, 9, 21, 21)));
    }

    #[test]
    fn try_checked_resize_odd_small_rect_same_size() {
        assert_eq!(try_checked_resize(&RectI32::of(i32::MIN, i32::MIN, i32::MIN + 2, i32::MIN + 2), 3), Some(RectI32::of(i32::MIN, i32::MIN, i32::MIN + 2, i32::MIN + 2)));
        assert_eq!(try_checked_resize(&RectI32::of(i32::MAX - 2, i32::MAX - 2, i32::MAX, i32::MAX), 3), Some(RectI32::of(i32::MAX - 2, i32::MAX - 2, i32::MAX, i32::MAX)));
    }

    #[test]
    fn try_checked_resize_even_small_rect_same_size() {
        assert_eq!(try_checked_resize(&RectI32::of(i32::MIN, i32::MIN, i32::MIN + 3, i32::MIN + 3), 4), Some(RectI32::of(i32::MIN, i32::MIN, i32::MIN + 3, i32::MIN + 3)));
        assert_eq!(try_checked_resize(&RectI32::of(i32::MAX - 3, i32::MAX - 3, i32::MAX, i32::MAX), 4), Some(RectI32::of(i32::MAX - 3, i32::MAX - 3, i32::MAX, i32::MAX)));
    }

    #[test]
    fn try_checked_resize_odd_small_rect_to_bounds() {
        assert_eq!(try_checked_resize(&RectI32::of(i32::MIN + 2, i32::MIN + 2, i32::MIN + 4, i32::MIN + 4), 7), Some(RectI32::of(i32::MIN, i32::MIN, i32::MIN + 6, i32::MIN + 6)));
        assert_eq!(try_checked_resize(&RectI32::of(i32::MAX - 4, i32::MAX - 4, i32::MAX - 2, i32::MAX - 2), 7), Some(RectI32::of(i32::MAX - 6, i32::MAX - 6, i32::MAX, i32::MAX)));
    }

    #[test]
    fn try_checked_resize_even_small_rect_to_bounds() {
        assert_eq!(try_checked_resize(&RectI32::of(i32::MIN + 2, i32::MIN + 2, i32::MIN + 5, i32::MIN + 5), 8), Some(RectI32::of(i32::MIN, i32::MIN, i32::MIN + 7, i32::MIN + 7)));
        assert_eq!(try_checked_resize(&RectI32::of(i32::MAX - 5, i32::MAX - 5, i32::MAX - 2, i32::MAX - 2), 8), Some(RectI32::of(i32::MAX - 7, i32::MAX - 7, i32::MAX, i32::MAX)));
    }

    #[test]
    fn try_checked_resize_odd_small_rect_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectI32::of(i32::MIN, i32::MIN, i32::MIN + 2, i32::MIN + 2), 5), None);
        assert_eq!(try_checked_resize(&RectI32::of(i32::MIN, i32::MIN + 2, i32::MIN + 2, i32::MIN + 4), 5), None);
        assert_eq!(try_checked_resize(&RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 4, i32::MIN + 2), 5), None);
        assert_eq!(try_checked_resize(&RectI32::of(i32::MAX - 2, i32::MAX - 2, i32::MAX, i32::MAX), 5), None);
        assert_eq!(try_checked_resize(&RectI32::of(i32::MAX - 2, i32::MAX - 4, i32::MAX, i32::MAX - 2), 5), None);
        assert_eq!(try_checked_resize(&RectI32::of(i32::MAX - 4, i32::MAX - 2, i32::MAX - 2, i32::MAX), 5), None);
    }

    #[test]
    fn try_checked_resize_even_small_rect_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectI32::of(i32::MIN, i32::MIN, i32::MIN + 3, i32::MIN + 3), 6), None);
        assert_eq!(try_checked_resize(&RectI32::of(i32::MIN, i32::MIN + 3, i32::MIN + 3, i32::MIN + 6), 6), None);
        assert_eq!(try_checked_resize(&RectI32::of(i32::MIN, i32::MIN, i32::MIN + 3, i32::MIN + 3), 6), None);
        assert_eq!(try_checked_resize(&RectI32::of(i32::MAX - 3, i32::MAX - 3, i32::MAX, i32::MAX), 6), None);
        assert_eq!(try_checked_resize(&RectI32::of(i32::MAX - 3, i32::MAX - 6, i32::MAX, i32::MAX - 3), 6), None);
        assert_eq!(try_checked_resize(&RectI32::of(i32::MAX - 6, i32::MAX - 3, i32::MAX - 3, i32::MAX), 6), None);
    }

    #[test]
    fn try_checked_resize_odd_small_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectI32::of(i32::MIN, i32::MIN, i32::MIN + 2, i32::MIN + 2), u32::MAX), None);
        assert_eq!(try_checked_resize(&RectI32::of(i32::MIN, i32::MIN + 2, i32::MIN + 2, i32::MIN + 4), u32::MAX), None);
        assert_eq!(try_checked_resize(&RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 4, i32::MIN + 2), u32::MAX), None);
        assert_eq!(try_checked_resize(&RectI32::of(i32::MAX - 2, i32::MAX - 2, i32::MAX, i32::MAX), u32::MAX), None);
        assert_eq!(try_checked_resize(&RectI32::of(i32::MAX - 2, i32::MAX - 4, i32::MAX, i32::MAX - 2), u32::MAX), None);
        assert_eq!(try_checked_resize(&RectI32::of(i32::MAX - 4, i32::MAX - 2, i32::MAX - 2, i32::MAX), u32::MAX), None);
    }

    #[test]
    fn try_checked_resize_even_small_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectI32::of(i32::MIN, i32::MIN, i32::MIN + 3, i32::MIN + 3), u32::MAX - 1), None);
        assert_eq!(try_checked_resize(&RectI32::of(i32::MIN, i32::MIN + 3, i32::MIN + 3, i32::MIN + 6), u32::MAX - 1), None);
        assert_eq!(try_checked_resize(&RectI32::of(i32::MIN, i32::MIN, i32::MIN + 3, i32::MIN + 3), u32::MAX - 1), None);
        assert_eq!(try_checked_resize(&RectI32::of(i32::MAX - 3, i32::MAX - 3, i32::MAX, i32::MAX), u32::MAX - 1), None);
        assert_eq!(try_checked_resize(&RectI32::of(i32::MAX - 3, i32::MAX - 6, i32::MAX, i32::MAX - 3), u32::MAX - 1), None);
        assert_eq!(try_checked_resize(&RectI32::of(i32::MAX - 6, i32::MAX - 3, i32::MAX - 3, i32::MAX), u32::MAX - 1), None);
    }

    #[test]
    fn try_checked_resize_big_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1), u32::MAX), Some(RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1)));
        assert_eq!(try_checked_resize(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX), u32::MAX), Some(RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX)));
        assert_eq!(try_checked_resize(&RectI32::largest(), u32::MAX), Some(RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1)));
    }

    #[test]
    fn checked_resize_assign_odd() {
        let mut r = RectI32::of(-5, -5, 5, 5);
        checked_resize_assign(&mut r, 11);
        assert_eq!(r, RectI32::of(-5, -5, 5, 5));
        checked_resize_assign(&mut r, 9);
        assert_eq!(r, RectI32::of(-4, -4, 4, 4));
        checked_resize_assign(&mut r, 7);
        assert_eq!(r, RectI32::of(-3, -3, 3, 3));
        checked_resize_assign(&mut r, 5);
        assert_eq!(r, RectI32::of(-2, -2, 2, 2));
        checked_resize_assign(&mut r, 3);
        assert_eq!(r, RectI32::of(-1, -1, 1, 1));
        checked_resize_assign(&mut r, 9);
        assert_eq!(r, RectI32::of(-4, -4, 4, 4));
    }

    #[test]
    fn checked_resize_assign_even() {
        let mut r = RectI32::of(-5, -5, 4, 4);
        checked_resize_assign(&mut r, 10);
        assert_eq!(r, RectI32::of(-5, -5, 4, 4));
        checked_resize_assign(&mut r, 8);
        assert_eq!(r, RectI32::of(-4, -4, 3, 3));
        checked_resize_assign(&mut r, 6);
        assert_eq!(r, RectI32::of(-3, -3, 2, 2));
        checked_resize_assign(&mut r, 4);
        assert_eq!(r, RectI32::of(-2, -2, 1, 1));
        checked_resize_assign(&mut r, 8);
        assert_eq!(r, RectI32::of(-4, -4, 3, 3));
    }

    #[test]
    fn checked_resize_odd() {
        assert_eq!(checked_resize(&RectI32::of(-5, -5, 5, 5), 11), RectI32::of(-5, -5, 5, 5));
        assert_eq!(checked_resize(&RectI32::of(-5, -5, 5, 5), 9), RectI32::of(-4, -4, 4, 4));
        assert_eq!(checked_resize(&RectI32::of(-4, -4, 4, 4), 7), RectI32::of(-3, -3, 3, 3));
        assert_eq!(checked_resize(&RectI32::of(-3, -3, 3, 3), 5), RectI32::of(-2, -2, 2, 2));
        assert_eq!(checked_resize(&RectI32::of(-2, -2, 2, 2), 3), RectI32::of(-1, -1, 1, 1));
        assert_eq!(checked_resize(&RectI32::of(-1, -1, 1, 1), 9), RectI32::of(-4, -4, 4, 4));
    }

    #[test]
    fn checked_resize_even() {
        assert_eq!(checked_resize(&RectI32::of(-5, -5, 4, 4), 10), RectI32::of(-5, -5, 4, 4));
        assert_eq!(checked_resize(&RectI32::of(-5, -5, 4, 4), 8), RectI32::of(-4, -4, 3, 3));
        assert_eq!(checked_resize(&RectI32::of(-4, -4, 3, 3), 6), RectI32::of(-3, -3, 2, 2));
        assert_eq!(checked_resize(&RectI32::of(-3, -3, 2, 2), 4), RectI32::of(-2, -2, 1, 1));
        assert_eq!(checked_resize(&RectI32::of(-2, -2, 1, 1), 8), RectI32::of(-4, -4, 3, 3));
    }
}
