#[cfg(test)]
mod tests {
    use super::super::{checked_resize, checked_resize_assign, try_checked_resize, try_checked_resize_assign};
    use crate::cartesian::{
        point::point_f32::{MAX, MIN},
        rect::rect_f32::RectF32,
    };

    #[test]
    fn try_checked_resize_assign_odd() {
        let mut r = RectF32::of(-5.0, -5.0, 5.0, 5.0);
        assert_eq!(try_checked_resize_assign(&mut r, 11.0), Some(()));
        assert_eq!(r, RectF32::of(-5.0, -5.0, 5.0, 5.0));
        assert_eq!(try_checked_resize_assign(&mut r, 9.0), Some(()));
        assert_eq!(r, RectF32::of(-4.0, -4.0, 4.0, 4.0));
        assert_eq!(try_checked_resize_assign(&mut r, 7.0), Some(()));
        assert_eq!(r, RectF32::of(-3.0, -3.0, 3.0, 3.0));
        assert_eq!(try_checked_resize_assign(&mut r, 5.0), Some(()));
        assert_eq!(r, RectF32::of(-2.0, -2.0, 2.0, 2.0));
        assert_eq!(try_checked_resize_assign(&mut r, 3.0), Some(()));
        assert_eq!(r, RectF32::of(-1.0, -1.0, 1.0, 1.0));
        assert_eq!(try_checked_resize_assign(&mut r, 9.0), Some(()));
        assert_eq!(r, RectF32::of(-4.0, -4.0, 4.0, 4.0));
    }

    #[test]
    fn try_checked_resize_assign_even() {
        let mut r = RectF32::of(-5.0, -5.0, 4.0, 4.0);
        assert_eq!(try_checked_resize_assign(&mut r, 10.0), Some(()));
        assert_eq!(r, RectF32::of(-5.0, -5.0, 4.0, 4.0));
        assert_eq!(try_checked_resize_assign(&mut r, 8.0), Some(()));
        assert_eq!(r, RectF32::of(-4.0, -4.0, 3.0, 3.0));
        assert_eq!(try_checked_resize_assign(&mut r, 6.0), Some(()));
        assert_eq!(r, RectF32::of(-3.0, -3.0, 2.0, 2.0));
        assert_eq!(try_checked_resize_assign(&mut r, 4.0), Some(()));
        assert_eq!(r, RectF32::of(-2.0, -2.0, 1.0, 1.0));
        assert_eq!(try_checked_resize_assign(&mut r, 8.0), Some(()));
        assert_eq!(r, RectF32::of(-4.0, -4.0, 3.0, 3.0));
    }

    #[test]
    fn try_checked_resize_assign_small_size() {
        let mut r = RectF32::of(10.0, 10.0, 20.0, 20.0);
        assert_eq!(try_checked_resize_assign(&mut r, 0.0), None);
        assert_eq!(try_checked_resize_assign(&mut r, 1.0), None);
        assert_eq!(try_checked_resize_assign(&mut r, 2.0), None);
        assert_eq!(try_checked_resize_assign(&mut r, MAX + 1.0), None);
        assert_eq!(try_checked_resize_assign(&mut r, MAX + 2.0), None);
        assert_eq!(try_checked_resize_assign(&mut r, MAX + 3.0), None);
        assert_eq!(r, RectF32::of(10.0, 10.0, 20.0, 20.0));
    }

    #[test]
    fn try_checked_resize_assign_same_size() {
        let mut r_11 = RectF32::of(10.0, 10.0, 20.0, 20.0);
        assert_eq!(try_checked_resize_assign(&mut r_11, 11.0), Some(()));
        assert_eq!(r_11, RectF32::of(10.0, 10.0, 20.0, 20.0));

        let mut r_12 = RectF32::of(10.0, 10.0, 21.0, 21.0);
        assert_eq!(try_checked_resize_assign(&mut r_12, 12.0), Some(()));
        assert_eq!(r_12, RectF32::of(10.0, 10.0, 21.0, 21.0));

        let mut r_13 = RectF32::of(9.0, 9.0, 21.0, 21.0);
        assert_eq!(try_checked_resize_assign(&mut r_13, 13.0), Some(()));
        assert_eq!(r_13, RectF32::of(9.0, 9.0, 21.0, 21.0));
    }

    #[test]
    fn try_checked_resize_assign_odd_small_rect_same_size() {
        let mut r_min = RectF32::of(MIN, MIN, MIN + 2.0, MIN + 2.0);
        assert_eq!(try_checked_resize_assign(&mut r_min, 3.0), Some(()));
        assert_eq!(r_min, RectF32::of(MIN, MIN, MIN + 2.0, MIN + 2.0));

        let mut r_max = RectF32::of(MAX - 2.0, MAX - 2.0, MAX, MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, 3.0), Some(()));
        assert_eq!(r_max, RectF32::of(MAX - 2.0, MAX - 2.0, MAX, MAX));
    }

    #[test]
    fn try_checked_resize_assign_even_small_rect_same_size() {
        let mut r = RectF32::of(MIN, MIN, MIN + 3.0, MIN + 3.0);
        assert_eq!(try_checked_resize_assign(&mut r, 4.0), Some(()));
        assert_eq!(r, RectF32::of(MIN, MIN, MIN + 3.0, MIN + 3.0));

        let mut r = RectF32::of(MAX - 3.0, MAX - 3.0, MAX, MAX);
        assert_eq!(try_checked_resize_assign(&mut r, 4.0), Some(()));
        assert_eq!(r, RectF32::of(MAX - 3.0, MAX - 3.0, MAX, MAX));
    }

    #[test]
    fn try_checked_resize_assign_odd_small_rect_to_bounds() {
        let mut r_min = RectF32::of(2.0, 2.0, 4.0, 4.0);
        assert_eq!(try_checked_resize_assign(&mut r_min, 7.0), Some(()));
        assert_eq!(r_min, RectF32::of(0.0, 0.0, 6.0, 6.0));

        let mut r_max = RectF32::of(MAX - 4.0, MAX - 4.0, MAX - 2.0, MAX - 2.0);
        assert_eq!(try_checked_resize_assign(&mut r_max, 7.0), Some(()));
        assert_eq!(r_max, RectF32::of(MAX - 6.0, MAX - 6.0, MAX, MAX));
    }

    #[test]
    fn try_checked_resize_assign_even_small_rect_to_bounds() {
        let mut r = RectF32::of(2.0, 2.0, 5.0, 5.0);
        assert_eq!(try_checked_resize_assign(&mut r, 8.0), Some(()));
        assert_eq!(r, RectF32::of(0.0, 0.0, 7.0, 7.0));

        let mut r = RectF32::of(MAX - 5.0, MAX - 5.0, MAX - 2.0, MAX - 2.0);
        assert_eq!(try_checked_resize_assign(&mut r, 8.0), Some(()));
        assert_eq!(r, RectF32::of(MAX - 7.0, MAX - 7.0, MAX, MAX));
    }

    #[test]
    fn try_checked_resize_assign_odd_small_rect_out_of_bounds() {
        let mut r_min = RectF32::of(MIN, MIN, MIN + 2.0, MIN + 2.0);
        assert_eq!(try_checked_resize_assign(&mut r_min, 5.0), None);
        assert_eq!(r_min, RectF32::of(MIN, MIN, MIN + 2.0, MIN + 2.0));

        let mut r_min_x = RectF32::of(MIN, MIN + 2.0, MIN + 2.0, MIN + 4.0);
        assert_eq!(try_checked_resize_assign(&mut r_min_x, 5.0), None);
        assert_eq!(r_min_x, RectF32::of(MIN, MIN + 2.0, MIN + 2.0, MIN + 4.0));

        let mut r_min_y = RectF32::of(MIN + 2.0, MIN, MIN + 4.0, MIN + 2.0);
        assert_eq!(try_checked_resize_assign(&mut r_min_y, 5.0), None);
        assert_eq!(r_min_y, RectF32::of(MIN + 2.0, MIN, MIN + 4.0, MIN + 2.0));

        let mut r_max = RectF32::of(MAX - 2.0, MAX - 2.0, MAX, MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, 5.0), None);
        assert_eq!(r_max, RectF32::of(MAX - 2.0, MAX - 2.0, MAX, MAX));

        let mut r_max_x = RectF32::of(MAX - 2.0, MAX - 4.0, MAX, MAX - 2.0);
        assert_eq!(try_checked_resize_assign(&mut r_max_x, 5.0), None);
        assert_eq!(r_max_x, RectF32::of(MAX - 2.0, MAX - 4.0, MAX, MAX - 2.0));

        let mut r_max_y = RectF32::of(MAX - 4.0, MAX - 2.0, MAX - 2.0, MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max_y, 5.0), None);
        assert_eq!(r_max_y, RectF32::of(MAX - 4.0, MAX - 2.0, MAX - 2.0, MAX));
    }

    #[test]
    fn try_checked_resize_assign_even_small_rect_out_of_bounds() {
        let mut r_min = RectF32::of(MIN, MIN, MIN + 3.0, MIN + 3.0);
        assert_eq!(try_checked_resize_assign(&mut r_min, 6.0), None);
        assert_eq!(r_min, RectF32::of(MIN, MIN, MIN + 3.0, MIN + 3.0));

        let mut r_min_x = RectF32::of(MIN, MIN + 3.0, MIN + 3.0, MIN + 6.0);
        assert_eq!(try_checked_resize_assign(&mut r_min_x, 6.0), None);
        assert_eq!(r_min_x, RectF32::of(MIN, MIN + 3.0, MIN + 3.0, MIN + 6.0));

        let mut r_min_y = RectF32::of(MIN + 3.0, MIN, MIN + 6.0, MIN + 3.0);
        assert_eq!(try_checked_resize_assign(&mut r_min_y, 6.0), None);
        assert_eq!(r_min_y, RectF32::of(MIN + 3.0, MIN, MIN + 6.0, MIN + 3.0));

        let mut r_max = RectF32::of(MAX - 3.0, MAX - 3.0, MAX, MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, 6.0), None);
        assert_eq!(r_max, RectF32::of(MAX - 3.0, MAX - 3.0, MAX, MAX));

        let mut r_max_x = RectF32::of(MAX - 3.0, MAX - 6.0, MAX, MAX - 3.0);
        assert_eq!(try_checked_resize_assign(&mut r_max_x, 6.0), None);
        assert_eq!(r_max_x, RectF32::of(MAX - 3.0, MAX - 6.0, MAX, MAX - 3.0));

        let mut r_max_y = RectF32::of(MAX - 6.0, MAX - 3.0, MAX - 3.0, MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max_y, 6.0), None);
        assert_eq!(r_max_y, RectF32::of(MAX - 6.0, MAX - 3.0, MAX - 3.0, MAX));
    }

    #[test]
    fn try_checked_resize_assign_odd_small_rect_limits_out_of_bounds() {
        let mut r_min = RectF32::of(MIN, MIN, MIN + 2.0, MIN + 2.0);
        assert_eq!(try_checked_resize_assign(&mut r_min, MAX), None);
        assert_eq!(r_min, RectF32::of(MIN, MIN, MIN + 2.0, MIN + 2.0));

        let mut r_min_x = RectF32::of(MIN, MIN + 2.0, MIN + 2.0, MIN + 4.0);
        assert_eq!(try_checked_resize_assign(&mut r_min_x, MAX), None);
        assert_eq!(r_min_x, RectF32::of(MIN, MIN + 2.0, MIN + 2.0, MIN + 4.0));

        let mut r_min_y = RectF32::of(MIN + 2.0, MIN, MIN + 4.0, MIN + 2.0);
        assert_eq!(try_checked_resize_assign(&mut r_min_y, MAX), None);
        assert_eq!(r_min_y, RectF32::of(MIN + 2.0, MIN, MIN + 4.0, MIN + 2.0));

        let mut r_max = RectF32::of(MAX - 2.0, MAX - 2.0, MAX, MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, MAX), None);
        assert_eq!(r_max, RectF32::of(MAX - 2.0, MAX - 2.0, MAX, MAX));

        let mut r_max_x = RectF32::of(MAX - 2.0, MAX - 4.0, MAX, MAX - 2.0);
        assert_eq!(try_checked_resize_assign(&mut r_max_x, MAX), None);
        assert_eq!(r_max_x, RectF32::of(MAX - 2.0, MAX - 4.0, MAX, MAX - 2.0));

        let mut r_max_y = RectF32::of(MAX - 4.0, MAX - 2.0, MAX - 2.0, MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max_y, MAX), None);
        assert_eq!(r_max_y, RectF32::of(MAX - 4.0, MAX - 2.0, MAX - 2.0, MAX));
    }

    #[test]
    fn try_checked_resize_assign_even_small_rect_limits_out_of_bounds() {
        let mut r_min = RectF32::of(MIN, MIN, MIN + 3.0, MIN + 3.0);
        assert_eq!(try_checked_resize_assign(&mut r_min, MAX - 1.0), None);
        assert_eq!(r_min, RectF32::of(MIN, MIN, MIN + 3.0, MIN + 3.0));

        let mut r_min_x = RectF32::of(MIN, MIN + 3.0, MIN + 3.0, MIN + 6.0);
        assert_eq!(try_checked_resize_assign(&mut r_min_x, MAX - 1.0), None);
        assert_eq!(r_min_x, RectF32::of(MIN, MIN + 3.0, MIN + 3.0, MIN + 6.0));

        let mut r_min_y = RectF32::of(MIN + 3.0, MIN, MIN + 6.0, MIN + 3.0);
        assert_eq!(try_checked_resize_assign(&mut r_min_y, MAX - 1.0), None);
        assert_eq!(r_min_y, RectF32::of(MIN + 3.0, MIN, MIN + 6.0, MIN + 3.0));

        let mut r_max = RectF32::of(MAX - 3.0, MAX - 3.0, MAX, MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, MAX - 1.0), None);
        assert_eq!(r_max, RectF32::of(MAX - 3.0, MAX - 3.0, MAX, MAX));

        let mut r_max_x = RectF32::of(MAX - 3.0, MAX - 6.0, MAX, MAX - 3.0);
        assert_eq!(try_checked_resize_assign(&mut r_max_x, MAX - 1.0), None);
        assert_eq!(r_max_x, RectF32::of(MAX - 3.0, MAX - 6.0, MAX, MAX - 3.0));

        let mut r_max_y = RectF32::of(MAX - 6.0, MAX - 3.0, MAX - 3.0, MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max_y, MAX - 1.0), None);
        assert_eq!(r_max_y, RectF32::of(MAX - 6.0, MAX - 3.0, MAX - 3.0, MAX));
    }

    #[test]
    fn try_checked_resize_assign_big_rect_limits_out_of_bounds() {
        let mut r_min = RectF32::of(MIN, MIN, -1.0, -1.0);
        assert_eq!(try_checked_resize_assign(&mut r_min, MAX), Some(()));
        assert_eq!(r_min, RectF32::of(MIN, MIN, -2.0, -2.0));

        let mut r_max = RectF32::of(1.0, 1.0, MAX, MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, MAX), Some(()));
        assert_eq!(r_max, RectF32::of(1.0, 1.0, MAX, MAX));

        let mut r_even = RectF32::largest();
        assert_eq!(try_checked_resize_assign(&mut r_even, MAX), Some(()));
        assert_eq!(r_even, RectF32::of(MIN / 2.0, MIN / 2.0, (MAX - 3.0) / 2.0, (MAX - 3.0) / 2.0));
    }

    #[test]
    fn try_checked_resize_odd() {
        assert_eq!(try_checked_resize(&RectF32::of(-5.0, -5.0, 5.0, 5.0), 11.0), Some(RectF32::of(-5.0, -5.0, 5.0, 5.0)));
        assert_eq!(try_checked_resize(&RectF32::of(-5.0, -5.0, 5.0, 5.0), 9.0), Some(RectF32::of(-4.0, -4.0, 4.0, 4.0)));
        assert_eq!(try_checked_resize(&RectF32::of(-4.0, -4.0, 4.0, 4.0), 7.0), Some(RectF32::of(-3.0, -3.0, 3.0, 3.0)));
        assert_eq!(try_checked_resize(&RectF32::of(-3.0, -3.0, 3.0, 3.0), 5.0), Some(RectF32::of(-2.0, -2.0, 2.0, 2.0)));
        assert_eq!(try_checked_resize(&RectF32::of(-2.0, -2.0, 2.0, 2.0), 3.0), Some(RectF32::of(-1.0, -1.0, 1.0, 1.0)));
        assert_eq!(try_checked_resize(&RectF32::of(-1.0, -1.0, 1.0, 1.0), 9.0), Some(RectF32::of(-4.0, -4.0, 4.0, 4.0)));
    }

    #[test]
    fn try_checked_resize_even() {
        assert_eq!(try_checked_resize(&RectF32::of(-5.0, -5.0, 4.0, 4.0), 10.0), Some(RectF32::of(-5.0, -5.0, 4.0, 4.0)));
        assert_eq!(try_checked_resize(&RectF32::of(-5.0, -5.0, 4.0, 4.0), 8.0), Some(RectF32::of(-4.0, -4.0, 3.0, 3.0)));
        assert_eq!(try_checked_resize(&RectF32::of(-4.0, -4.0, 3.0, 3.0), 6.0), Some(RectF32::of(-3.0, -3.0, 2.0, 2.0)));
        assert_eq!(try_checked_resize(&RectF32::of(-3.0, -3.0, 2.0, 2.0), 4.0), Some(RectF32::of(-2.0, -2.0, 1.0, 1.0)));
        assert_eq!(try_checked_resize(&RectF32::of(-2.0, -2.0, 1.0, 1.0), 8.0), Some(RectF32::of(-4.0, -4.0, 3.0, 3.0)));
    }

    #[test]
    fn try_checked_resize_small_size() {
        let r = RectF32::of(10.0, 10.0, 20.0, 20.0);
        assert_eq!(try_checked_resize(&r, 0.0), None);
        assert_eq!(try_checked_resize(&r, 1.0), None);
        assert_eq!(try_checked_resize(&r, 2.0), None);
        assert_eq!(try_checked_resize(&r, MAX + 1.0), None);
        assert_eq!(try_checked_resize(&r, MAX + 2.0), None);
        assert_eq!(try_checked_resize(&r, MAX + 3.0), None);
    }

    #[test]
    fn try_checked_resize_same_size() {
        assert_eq!(try_checked_resize(&RectF32::of(10.0, 10.0, 20.0, 20.0), 11.0), Some(RectF32::of(10.0, 10.0, 20.0, 20.0)));
        assert_eq!(try_checked_resize(&RectF32::of(10.0, 10.0, 21.0, 21.0), 12.0), Some(RectF32::of(10.0, 10.0, 21.0, 21.0)));
        assert_eq!(try_checked_resize(&RectF32::of(9.0, 9.0, 21.0, 21.0), 13.0), Some(RectF32::of(9.0, 9.0, 21.0, 21.0)));
    }

    #[test]
    fn try_checked_resize_odd_small_rect_same_size() {
        assert_eq!(try_checked_resize(&RectF32::of(MIN, MIN, MIN + 2.0, MIN + 2.0), 3.0), Some(RectF32::of(MIN, MIN, MIN + 2.0, MIN + 2.0)));
        assert_eq!(try_checked_resize(&RectF32::of(MAX - 2.0, MAX - 2.0, MAX, MAX), 3.0), Some(RectF32::of(MAX - 2.0, MAX - 2.0, MAX, MAX)));
    }

    #[test]
    fn try_checked_resize_even_small_rect_same_size() {
        assert_eq!(try_checked_resize(&RectF32::of(MIN, MIN, MIN + 3.0, MIN + 3.0), 4.0), Some(RectF32::of(MIN, MIN, MIN + 3.0, MIN + 3.0)));
        assert_eq!(try_checked_resize(&RectF32::of(MAX - 3.0, MAX - 3.0, MAX, MAX), 4.0), Some(RectF32::of(MAX - 3.0, MAX - 3.0, MAX, MAX)));
    }

    #[test]
    fn try_checked_resize_odd_small_rect_to_bounds() {
        assert_eq!(try_checked_resize(&RectF32::of(MIN + 2.0, MIN + 2.0, MIN + 4.0, MIN + 4.0), 7.0), Some(RectF32::of(MIN, MIN, MIN + 6.0, MIN + 6.0)));
        assert_eq!(try_checked_resize(&RectF32::of(MAX - 4.0, MAX - 4.0, MAX - 2.0, MAX - 2.0), 7.0), Some(RectF32::of(MAX - 6.0, MAX - 6.0, MAX, MAX)));
    }

    #[test]
    fn try_checked_resize_even_small_rect_to_bounds() {
        assert_eq!(try_checked_resize(&RectF32::of(MIN + 2.0, MIN + 2.0, MIN + 5.0, MIN + 5.0), 8.0), Some(RectF32::of(MIN, MIN, MIN + 7.0, MIN + 7.0)));
        assert_eq!(try_checked_resize(&RectF32::of(MAX - 5.0, MAX - 5.0, MAX - 2.0, MAX - 2.0), 8.0), Some(RectF32::of(MAX - 7.0, MAX - 7.0, MAX, MAX)));
    }

    #[test]
    fn try_checked_resize_odd_small_rect_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectF32::of(MIN, MIN, MIN + 2.0, MIN + 2.0), 5.0), None);
        assert_eq!(try_checked_resize(&RectF32::of(MIN, MIN + 2.0, MIN + 2.0, MIN + 4.0), 5.0), None);
        assert_eq!(try_checked_resize(&RectF32::of(MIN + 2.0, MIN, MIN + 4.0, MIN + 2.0), 5.0), None);
        assert_eq!(try_checked_resize(&RectF32::of(MAX - 2.0, MAX - 2.0, MAX, MAX), 5.0), None);
        assert_eq!(try_checked_resize(&RectF32::of(MAX - 2.0, MAX - 4.0, MAX, MAX - 2.0), 5.0), None);
        assert_eq!(try_checked_resize(&RectF32::of(MAX - 4.0, MAX - 2.0, MAX - 2.0, MAX), 5.0), None);
    }

    #[test]
    fn try_checked_resize_even_small_rect_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectF32::of(MIN, MIN, MIN + 3.0, MIN + 3.0), 6.0), None);
        assert_eq!(try_checked_resize(&RectF32::of(MIN, MIN + 3.0, MIN + 3.0, MIN + 6.0), 6.0), None);
        assert_eq!(try_checked_resize(&RectF32::of(MIN, MIN, MIN + 3.0, MIN + 3.0), 6.0), None);
        assert_eq!(try_checked_resize(&RectF32::of(MAX - 3.0, MAX - 3.0, MAX, MAX), 6.0), None);
        assert_eq!(try_checked_resize(&RectF32::of(MAX - 3.0, MAX - 6.0, MAX, MAX - 3.0), 6.0), None);
        assert_eq!(try_checked_resize(&RectF32::of(MAX - 6.0, MAX - 3.0, MAX - 3.0, MAX), 6.0), None);
    }

    #[test]
    fn try_checked_resize_odd_small_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectF32::of(MIN, MIN, MIN + 2.0, MIN + 2.0), MAX), None);
        assert_eq!(try_checked_resize(&RectF32::of(MIN, MIN + 2.0, MIN + 2.0, MIN + 4.0), MAX), None);
        assert_eq!(try_checked_resize(&RectF32::of(MIN + 2.0, MIN, MIN + 4.0, MIN + 2.0), MAX), None);
        assert_eq!(try_checked_resize(&RectF32::of(MAX - 2.0, MAX - 2.0, MAX, MAX), MAX), None);
        assert_eq!(try_checked_resize(&RectF32::of(MAX - 2.0, MAX - 4.0, MAX, MAX - 2.0), MAX), None);
        assert_eq!(try_checked_resize(&RectF32::of(MAX - 4.0, MAX - 2.0, MAX - 2.0, MAX), MAX), None);
    }

    #[test]
    fn try_checked_resize_even_small_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectF32::of(MIN, MIN, MIN + 3.0, MIN + 3.0), MAX - 1.0), None);
        assert_eq!(try_checked_resize(&RectF32::of(MIN, MIN + 3.0, MIN + 3.0, MIN + 6.0), MAX - 1.0), None);
        assert_eq!(try_checked_resize(&RectF32::of(MIN, MIN, MIN + 3.0, MIN + 3.0), MAX - 1.0), None);
        assert_eq!(try_checked_resize(&RectF32::of(MAX - 3.0, MAX - 3.0, MAX, MAX), MAX - 1.0), None);
        assert_eq!(try_checked_resize(&RectF32::of(MAX - 3.0, MAX - 6.0, MAX, MAX - 3.0), MAX - 1.0), None);
        assert_eq!(try_checked_resize(&RectF32::of(MAX - 6.0, MAX - 3.0, MAX - 3.0, MAX), MAX - 1.0), None);
    }

    #[test]
    fn try_checked_resize_big_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectF32::of(MIN, MIN, -1.0, -1.0), MAX), Some(RectF32::of(MIN, MIN, -2.0, -2.0)));
        assert_eq!(try_checked_resize(&RectF32::of(1.0, 1.0, MAX, MAX), MAX), Some(RectF32::of(1.0, 1.0, MAX, MAX)));
        assert_eq!(try_checked_resize(&RectF32::largest(), MAX), Some(RectF32::of(MIN / 2.0, MIN / 2.0, (MAX - 3.0) / 2.0, (MAX - 3.0) / 2.0)));
    }

    #[test]
    fn checked_resize_assign_odd() {
        let mut r = RectF32::of(-5.0, -5.0, 5.0, 5.0);
        checked_resize_assign(&mut r, 11.0);
        assert_eq!(r, RectF32::of(-5.0, -5.0, 5.0, 5.0));
        checked_resize_assign(&mut r, 9.0);
        assert_eq!(r, RectF32::of(-4.0, -4.0, 4.0, 4.0));
        checked_resize_assign(&mut r, 7.0);
        assert_eq!(r, RectF32::of(-3.0, -3.0, 3.0, 3.0));
        checked_resize_assign(&mut r, 5.0);
        assert_eq!(r, RectF32::of(-2.0, -2.0, 2.0, 2.0));
        checked_resize_assign(&mut r, 3.0);
        assert_eq!(r, RectF32::of(-1.0, -1.0, 1.0, 1.0));
        checked_resize_assign(&mut r, 9.0);
        assert_eq!(r, RectF32::of(-4.0, -4.0, 4.0, 4.0));
    }

    #[test]
    fn checked_resize_assign_even() {
        let mut r = RectF32::of(-5.0, -5.0, 4.0, 4.0);
        checked_resize_assign(&mut r, 10.0);
        assert_eq!(r, RectF32::of(-5.0, -5.0, 4.0, 4.0));
        checked_resize_assign(&mut r, 8.0);
        assert_eq!(r, RectF32::of(-4.0, -4.0, 3.0, 3.0));
        checked_resize_assign(&mut r, 6.0);
        assert_eq!(r, RectF32::of(-3.0, -3.0, 2.0, 2.0));
        checked_resize_assign(&mut r, 4.0);
        assert_eq!(r, RectF32::of(-2.0, -2.0, 1.0, 1.0));
        checked_resize_assign(&mut r, 8.0);
        assert_eq!(r, RectF32::of(-4.0, -4.0, 3.0, 3.0));
    }

    #[test]
    fn checked_resize_odd() {
        assert_eq!(checked_resize(&RectF32::of(-5.0, -5.0, 5.0, 5.0), 11.0), RectF32::of(-5.0, -5.0, 5.0, 5.0));
        assert_eq!(checked_resize(&RectF32::of(-5.0, -5.0, 5.0, 5.0), 9.0), RectF32::of(-4.0, -4.0, 4.0, 4.0));
        assert_eq!(checked_resize(&RectF32::of(-4.0, -4.0, 4.0, 4.0), 7.0), RectF32::of(-3.0, -3.0, 3.0, 3.0));
        assert_eq!(checked_resize(&RectF32::of(-3.0, -3.0, 3.0, 3.0), 5.0), RectF32::of(-2.0, -2.0, 2.0, 2.0));
        assert_eq!(checked_resize(&RectF32::of(-2.0, -2.0, 2.0, 2.0), 3.0), RectF32::of(-1.0, -1.0, 1.0, 1.0));
        assert_eq!(checked_resize(&RectF32::of(-1.0, -1.0, 1.0, 1.0), 9.0), RectF32::of(-4.0, -4.0, 4.0, 4.0));
    }

    #[test]
    fn checked_resize_even() {
        assert_eq!(checked_resize(&RectF32::of(-5.0, -5.0, 4.0, 4.0), 10.0), RectF32::of(-5.0, -5.0, 4.0, 4.0));
        assert_eq!(checked_resize(&RectF32::of(-5.0, -5.0, 4.0, 4.0), 8.0), RectF32::of(-4.0, -4.0, 3.0, 3.0));
        assert_eq!(checked_resize(&RectF32::of(-4.0, -4.0, 3.0, 3.0), 6.0), RectF32::of(-3.0, -3.0, 2.0, 2.0));
        assert_eq!(checked_resize(&RectF32::of(-3.0, -3.0, 2.0, 2.0), 4.0), RectF32::of(-2.0, -2.0, 1.0, 1.0));
        assert_eq!(checked_resize(&RectF32::of(-2.0, -2.0, 1.0, 1.0), 8.0), RectF32::of(-4.0, -4.0, 3.0, 3.0));
    }
}
