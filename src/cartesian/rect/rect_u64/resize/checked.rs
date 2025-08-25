#[cfg(test)]
mod tests {
    use super::super::{checked_resize, checked_resize_assign, try_checked_resize, try_checked_resize_assign};
    use crate::cartesian::rect::rect_u64::RectU64;

    #[test]
    fn try_checked_resize_assign_odd() {
        let mut r = RectU64::of(5, 5, 15, 15);
        assert_eq!(try_checked_resize_assign(&mut r, 11), Some(()));
        assert_eq!(r, RectU64::of(5, 5, 15, 15));
        assert_eq!(try_checked_resize_assign(&mut r, 9), Some(()));
        assert_eq!(r, RectU64::of(6, 6, 14, 14));
        assert_eq!(try_checked_resize_assign(&mut r, 7), Some(()));
        assert_eq!(r, RectU64::of(7, 7, 13, 13));
        assert_eq!(try_checked_resize_assign(&mut r, 5), Some(()));
        assert_eq!(r, RectU64::of(8, 8, 12, 12));
        assert_eq!(try_checked_resize_assign(&mut r, 3), Some(()));
        assert_eq!(r, RectU64::of(9, 9, 11, 11));
        assert_eq!(try_checked_resize_assign(&mut r, 9), Some(()));
        assert_eq!(r, RectU64::of(6, 6, 14, 14));
    }

    #[test]
    fn try_checked_resize_assign_even() {
        let mut r = RectU64::of(5, 5, 14, 14);
        assert_eq!(try_checked_resize_assign(&mut r, 10), Some(()));
        assert_eq!(r, RectU64::of(5, 5, 14, 14));
        assert_eq!(try_checked_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectU64::of(6, 6, 13, 13));
        assert_eq!(try_checked_resize_assign(&mut r, 6), Some(()));
        assert_eq!(r, RectU64::of(7, 7, 12, 12));
        assert_eq!(try_checked_resize_assign(&mut r, 4), Some(()));
        assert_eq!(r, RectU64::of(8, 8, 11, 11));
        assert_eq!(try_checked_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectU64::of(6, 6, 13, 13));
    }

    #[test]
    fn try_checked_resize_assign_small_size() {
        let mut r = RectU64::of(10, 10, 20, 20);
        assert_eq!(try_checked_resize_assign(&mut r, 0), None);
        assert_eq!(try_checked_resize_assign(&mut r, 1), None);
        assert_eq!(try_checked_resize_assign(&mut r, 2), None);
        assert_eq!(r, RectU64::of(10, 10, 20, 20));
    }

    #[test]
    fn try_checked_resize_assign_same_size() {
        let mut r_11 = RectU64::of(10, 10, 20, 20);
        assert_eq!(try_checked_resize_assign(&mut r_11, 11), Some(()));
        assert_eq!(r_11, RectU64::of(10, 10, 20, 20));

        let mut r_12 = RectU64::of(10, 10, 21, 21);
        assert_eq!(try_checked_resize_assign(&mut r_12, 12), Some(()));
        assert_eq!(r_12, RectU64::of(10, 10, 21, 21));

        let mut r_13 = RectU64::of(9, 9, 21, 21);
        assert_eq!(try_checked_resize_assign(&mut r_13, 13), Some(()));
        assert_eq!(r_13, RectU64::of(9, 9, 21, 21));
    }

    #[test]
    fn try_checked_resize_assign_odd_small_rect_same_size() {
        let mut r_min = RectU64::of(0, 0, 2, 2);
        assert_eq!(try_checked_resize_assign(&mut r_min, 3), Some(()));
        assert_eq!(r_min, RectU64::of(0, 0, 2, 2));

        let mut r_max = RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, 3), Some(()));
        assert_eq!(r_max, RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX));
    }

    #[test]
    fn try_checked_resize_assign_even_small_rect_same_size() {
        let mut r = RectU64::of(0, 0, 3, 3);
        assert_eq!(try_checked_resize_assign(&mut r, 4), Some(()));
        assert_eq!(r, RectU64::of(0, 0, 3, 3));

        let mut r = RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r, 4), Some(()));
        assert_eq!(r, RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX));
    }

    #[test]
    fn try_checked_resize_assign_odd_small_rect_to_bounds() {
        let mut r_min = RectU64::of(2, 2, 4, 4);
        assert_eq!(try_checked_resize_assign(&mut r_min, 7), Some(()));
        assert_eq!(r_min, RectU64::of(0, 0, 6, 6));

        let mut r_max = RectU64::of(u64::MAX - 4, u64::MAX - 4, u64::MAX - 2, u64::MAX - 2);
        assert_eq!(try_checked_resize_assign(&mut r_max, 7), Some(()));
        assert_eq!(r_max, RectU64::of(u64::MAX - 6, u64::MAX - 6, u64::MAX, u64::MAX));
    }

    #[test]
    fn try_checked_resize_assign_even_small_rect_to_bounds() {
        let mut r = RectU64::of(2, 2, 5, 5);
        assert_eq!(try_checked_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectU64::of(0, 0, 7, 7));

        let mut r = RectU64::of(u64::MAX - 5, u64::MAX - 5, u64::MAX - 2, u64::MAX - 2);
        assert_eq!(try_checked_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectU64::of(u64::MAX - 7, u64::MAX - 7, u64::MAX, u64::MAX));
    }

    #[test]
    fn try_checked_resize_assign_odd_small_rect_out_of_bounds() {
        let mut r_min = RectU64::of(0, 0, 2, 2);
        assert_eq!(try_checked_resize_assign(&mut r_min, 5), None);
        assert_eq!(r_min, RectU64::of(0, 0, 2, 2));

        let mut r_min_x = RectU64::of(0, 2, 2, 4);
        assert_eq!(try_checked_resize_assign(&mut r_min_x, 5), None);
        assert_eq!(r_min_x, RectU64::of(0, 2, 2, 4));

        let mut r_min_y = RectU64::of(2, 0, 4, 2);
        assert_eq!(try_checked_resize_assign(&mut r_min_y, 5), None);
        assert_eq!(r_min_y, RectU64::of(2, 0, 4, 2));

        let mut r_max = RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, 5), None);
        assert_eq!(r_max, RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX));

        let mut r_max_x = RectU64::of(u64::MAX - 2, u64::MAX - 4, u64::MAX, u64::MAX - 2);
        assert_eq!(try_checked_resize_assign(&mut r_max_x, 5), None);
        assert_eq!(r_max_x, RectU64::of(u64::MAX - 2, u64::MAX - 4, u64::MAX, u64::MAX - 2));

        let mut r_max_y = RectU64::of(u64::MAX - 4, u64::MAX - 2, u64::MAX - 2, u64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max_y, 5), None);
        assert_eq!(r_max_y, RectU64::of(u64::MAX - 4, u64::MAX - 2, u64::MAX - 2, u64::MAX));
    }

    #[test]
    fn try_checked_resize_assign_even_small_rect_out_of_bounds() {
        let mut r_min = RectU64::of(0, 0, 3, 3);
        assert_eq!(try_checked_resize_assign(&mut r_min, 6), None);
        assert_eq!(r_min, RectU64::of(0, 0, 3, 3));

        let mut r_min_x = RectU64::of(0, 3, 3, 6);
        assert_eq!(try_checked_resize_assign(&mut r_min_x, 6), None);
        assert_eq!(r_min_x, RectU64::of(0, 3, 3, 6));

        let mut r_min_y = RectU64::of(3, 0, 6, 3);
        assert_eq!(try_checked_resize_assign(&mut r_min_y, 6), None);
        assert_eq!(r_min_y, RectU64::of(3, 0, 6, 3));

        let mut r_max = RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, 6), None);
        assert_eq!(r_max, RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX));

        let mut r_max_x = RectU64::of(u64::MAX - 3, u64::MAX - 6, u64::MAX, u64::MAX - 3);
        assert_eq!(try_checked_resize_assign(&mut r_max_x, 6), None);
        assert_eq!(r_max_x, RectU64::of(u64::MAX - 3, u64::MAX - 6, u64::MAX, u64::MAX - 3));

        let mut r_max_y = RectU64::of(u64::MAX - 6, u64::MAX - 3, u64::MAX - 3, u64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max_y, 6), None);
        assert_eq!(r_max_y, RectU64::of(u64::MAX - 6, u64::MAX - 3, u64::MAX - 3, u64::MAX));
    }

    #[test]
    fn try_checked_resize_assign_odd_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU64::of(0, 0, 2, 2);
        assert_eq!(try_checked_resize_assign(&mut r_min, u64::MAX), None);
        assert_eq!(r_min, RectU64::of(0, 0, 2, 2));

        let mut r_min_x = RectU64::of(0, 2, 2, 4);
        assert_eq!(try_checked_resize_assign(&mut r_min_x, u64::MAX), None);
        assert_eq!(r_min_x, RectU64::of(0, 2, 2, 4));

        let mut r_min_y = RectU64::of(2, 0, 4, 2);
        assert_eq!(try_checked_resize_assign(&mut r_min_y, u64::MAX), None);
        assert_eq!(r_min_y, RectU64::of(2, 0, 4, 2));

        let mut r_max = RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, u64::MAX), None);
        assert_eq!(r_max, RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX));

        let mut r_max_x = RectU64::of(u64::MAX - 2, u64::MAX - 4, u64::MAX, u64::MAX - 2);
        assert_eq!(try_checked_resize_assign(&mut r_max_x, u64::MAX), None);
        assert_eq!(r_max_x, RectU64::of(u64::MAX - 2, u64::MAX - 4, u64::MAX, u64::MAX - 2));

        let mut r_max_y = RectU64::of(u64::MAX - 4, u64::MAX - 2, u64::MAX - 2, u64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max_y, u64::MAX), None);
        assert_eq!(r_max_y, RectU64::of(u64::MAX - 4, u64::MAX - 2, u64::MAX - 2, u64::MAX));
    }

    #[test]
    fn try_checked_resize_assign_even_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU64::of(0, 0, 3, 3);
        assert_eq!(try_checked_resize_assign(&mut r_min, u64::MAX - 1), None);
        assert_eq!(r_min, RectU64::of(0, 0, 3, 3));

        let mut r_min_x = RectU64::of(0, 3, 3, 6);
        assert_eq!(try_checked_resize_assign(&mut r_min_x, u64::MAX - 1), None);
        assert_eq!(r_min_x, RectU64::of(0, 3, 3, 6));

        let mut r_min_y = RectU64::of(3, 0, 6, 3);
        assert_eq!(try_checked_resize_assign(&mut r_min_y, u64::MAX - 1), None);
        assert_eq!(r_min_y, RectU64::of(3, 0, 6, 3));

        let mut r_max = RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, u64::MAX - 1), None);
        assert_eq!(r_max, RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX));

        let mut r_max_x = RectU64::of(u64::MAX - 3, u64::MAX - 6, u64::MAX, u64::MAX - 3);
        assert_eq!(try_checked_resize_assign(&mut r_max_x, u64::MAX - 1), None);
        assert_eq!(r_max_x, RectU64::of(u64::MAX - 3, u64::MAX - 6, u64::MAX, u64::MAX - 3));

        let mut r_max_y = RectU64::of(u64::MAX - 6, u64::MAX - 3, u64::MAX - 3, u64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max_y, u64::MAX - 1), None);
        assert_eq!(r_max_y, RectU64::of(u64::MAX - 6, u64::MAX - 3, u64::MAX - 3, u64::MAX));
    }

    #[test]
    fn try_checked_resize_assign_big_rect_limits_out_of_bounds() {
        let mut r_odd_1 = RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1);
        assert_eq!(try_checked_resize_assign(&mut r_odd_1, u64::MAX), Some(()));
        assert_eq!(r_odd_1, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));

        let mut r_odd_1 = RectU64::of(1, 1, u64::MAX, u64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_odd_1, u64::MAX), Some(()));
        assert_eq!(r_odd_1, RectU64::of(1, 1, u64::MAX, u64::MAX));

        let mut r_even = RectU64::largest();
        assert_eq!(try_checked_resize_assign(&mut r_even, u64::MAX), Some(()));
        assert_eq!(r_even, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));
    }

    #[test]
    fn try_checked_resize_odd() {
        assert_eq!(try_checked_resize(&RectU64::of(5, 5, 15, 15), 11), Some(RectU64::of(5, 5, 15, 15)));
        assert_eq!(try_checked_resize(&RectU64::of(5, 5, 15, 15), 9), Some(RectU64::of(6, 6, 14, 14)));
        assert_eq!(try_checked_resize(&RectU64::of(6, 6, 14, 14), 7), Some(RectU64::of(7, 7, 13, 13)));
        assert_eq!(try_checked_resize(&RectU64::of(7, 7, 13, 13), 5), Some(RectU64::of(8, 8, 12, 12)));
        assert_eq!(try_checked_resize(&RectU64::of(8, 8, 12, 12), 3), Some(RectU64::of(9, 9, 11, 11)));
        assert_eq!(try_checked_resize(&RectU64::of(9, 9, 11, 11), 9), Some(RectU64::of(6, 6, 14, 14)));
    }

    #[test]
    fn try_checked_resize_even() {
        assert_eq!(try_checked_resize(&RectU64::of(5, 5, 14, 14), 10), Some(RectU64::of(5, 5, 14, 14)));
        assert_eq!(try_checked_resize(&RectU64::of(5, 5, 14, 14), 8), Some(RectU64::of(6, 6, 13, 13)));
        assert_eq!(try_checked_resize(&RectU64::of(6, 6, 13, 13), 6), Some(RectU64::of(7, 7, 12, 12)));
        assert_eq!(try_checked_resize(&RectU64::of(7, 7, 12, 12), 4), Some(RectU64::of(8, 8, 11, 11)));
        assert_eq!(try_checked_resize(&RectU64::of(8, 8, 11, 11), 8), Some(RectU64::of(6, 6, 13, 13)));
    }

    #[test]
    fn try_checked_resize_small_size() {
        let r = RectU64::of(10, 10, 20, 20);
        assert_eq!(try_checked_resize(&r, 0), None);
        assert_eq!(try_checked_resize(&r, 1), None);
        assert_eq!(try_checked_resize(&r, 2), None);
    }

    #[test]
    fn try_checked_resize_same_size() {
        assert_eq!(try_checked_resize(&RectU64::of(10, 10, 20, 20), 11), Some(RectU64::of(10, 10, 20, 20)));
        assert_eq!(try_checked_resize(&RectU64::of(10, 10, 21, 21), 12), Some(RectU64::of(10, 10, 21, 21)));
        assert_eq!(try_checked_resize(&RectU64::of(9, 9, 21, 21), 13), Some(RectU64::of(9, 9, 21, 21)));
    }

    #[test]
    fn try_checked_resize_odd_small_rect_same_size() {
        assert_eq!(try_checked_resize(&RectU64::of(0, 0, 2, 2), 3), Some(RectU64::of(0, 0, 2, 2)));
        assert_eq!(
            try_checked_resize(&RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX), 3),
            Some(RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX))
        );
    }

    #[test]
    fn try_checked_resize_even_small_rect_same_size() {
        assert_eq!(try_checked_resize(&RectU64::of(0, 0, 3, 3), 4), Some(RectU64::of(0, 0, 3, 3)));
        assert_eq!(
            try_checked_resize(&RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX), 4),
            Some(RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX))
        );
    }

    #[test]
    fn try_checked_resize_odd_small_rect_to_bounds() {
        assert_eq!(try_checked_resize(&RectU64::of(2, 2, 4, 4), 7), Some(RectU64::of(0, 0, 6, 6)));
        assert_eq!(
            try_checked_resize(&RectU64::of(u64::MAX - 4, u64::MAX - 4, u64::MAX - 2, u64::MAX - 2), 7),
            Some(RectU64::of(u64::MAX - 6, u64::MAX - 6, u64::MAX, u64::MAX))
        );
    }

    #[test]
    fn try_checked_resize_even_small_rect_to_bounds() {
        assert_eq!(try_checked_resize(&RectU64::of(2, 2, 5, 5), 8), Some(RectU64::of(0, 0, 7, 7)));
        assert_eq!(
            try_checked_resize(&RectU64::of(u64::MAX - 5, u64::MAX - 5, u64::MAX - 2, u64::MAX - 2), 8),
            Some(RectU64::of(u64::MAX - 7, u64::MAX - 7, u64::MAX, u64::MAX))
        );
    }

    #[test]
    fn try_checked_resize_odd_small_rect_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectU64::of(0, 0, 2, 2), 5), None);
        assert_eq!(try_checked_resize(&RectU64::of(0, 2, 2, 4), 5), None);
        assert_eq!(try_checked_resize(&RectU64::of(2, 0, 4, 2), 5), None);
        assert_eq!(try_checked_resize(&RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX), 5), None);
        assert_eq!(try_checked_resize(&RectU64::of(u64::MAX - 2, u64::MAX - 4, u64::MAX, u64::MAX - 2), 5), None);
        assert_eq!(try_checked_resize(&RectU64::of(u64::MAX - 4, u64::MAX - 2, u64::MAX - 2, u64::MAX), 5), None);
    }

    #[test]
    fn try_checked_resize_even_small_rect_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectU64::of(0, 0, 3, 3), 6), None);
        assert_eq!(try_checked_resize(&RectU64::of(0, 3, 3, 6), 6), None);
        assert_eq!(try_checked_resize(&RectU64::of(0, 0, 3, 3), 6), None);
        assert_eq!(try_checked_resize(&RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX), 6), None);
        assert_eq!(try_checked_resize(&RectU64::of(u64::MAX - 3, u64::MAX - 6, u64::MAX, u64::MAX - 3), 6), None);
        assert_eq!(try_checked_resize(&RectU64::of(u64::MAX - 6, u64::MAX - 3, u64::MAX - 3, u64::MAX), 6), None);
    }

    #[test]
    fn try_checked_resize_odd_small_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectU64::of(0, 0, 2, 2), u64::MAX), None);
        assert_eq!(try_checked_resize(&RectU64::of(0, 2, 2, 4), u64::MAX), None);
        assert_eq!(try_checked_resize(&RectU64::of(2, 0, 4, 2), u64::MAX), None);
        assert_eq!(try_checked_resize(&RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX), u64::MAX), None);
        assert_eq!(try_checked_resize(&RectU64::of(u64::MAX - 2, u64::MAX - 4, u64::MAX, u64::MAX - 2), u64::MAX), None);
        assert_eq!(try_checked_resize(&RectU64::of(u64::MAX - 4, u64::MAX - 2, u64::MAX - 2, u64::MAX), u64::MAX), None);
    }

    #[test]
    fn try_checked_resize_even_small_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectU64::of(0, 0, 3, 3), u64::MAX - 1), None);
        assert_eq!(try_checked_resize(&RectU64::of(0, 3, 3, 6), u64::MAX - 1), None);
        assert_eq!(try_checked_resize(&RectU64::of(0, 0, 3, 3), u64::MAX - 1), None);
        assert_eq!(try_checked_resize(&RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX), u64::MAX - 1), None);
        assert_eq!(try_checked_resize(&RectU64::of(u64::MAX - 3, u64::MAX - 6, u64::MAX, u64::MAX - 3), u64::MAX - 1), None);
        assert_eq!(try_checked_resize(&RectU64::of(u64::MAX - 6, u64::MAX - 3, u64::MAX - 3, u64::MAX), u64::MAX - 1), None);
    }

    #[test]
    fn try_checked_resize_big_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1), u64::MAX), Some(RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1)));
        assert_eq!(try_checked_resize(&RectU64::of(1, 1, u64::MAX, u64::MAX), u64::MAX), Some(RectU64::of(1, 1, u64::MAX, u64::MAX)));
        assert_eq!(try_checked_resize(&RectU64::largest(), u64::MAX), Some(RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1)));
    }

    #[test]
    fn checked_resize_assign_odd() {
        let mut r = RectU64::of(5, 5, 15, 15);
        checked_resize_assign(&mut r, 11);
        assert_eq!(r, RectU64::of(5, 5, 15, 15));
        checked_resize_assign(&mut r, 9);
        assert_eq!(r, RectU64::of(6, 6, 14, 14));
        checked_resize_assign(&mut r, 7);
        assert_eq!(r, RectU64::of(7, 7, 13, 13));
        checked_resize_assign(&mut r, 5);
        assert_eq!(r, RectU64::of(8, 8, 12, 12));
        checked_resize_assign(&mut r, 3);
        assert_eq!(r, RectU64::of(9, 9, 11, 11));
        checked_resize_assign(&mut r, 9);
        assert_eq!(r, RectU64::of(6, 6, 14, 14));
    }

    #[test]
    fn checked_resize_assign_even() {
        let mut r = RectU64::of(5, 5, 14, 14);
        checked_resize_assign(&mut r, 10);
        assert_eq!(r, RectU64::of(5, 5, 14, 14));
        checked_resize_assign(&mut r, 8);
        assert_eq!(r, RectU64::of(6, 6, 13, 13));
        checked_resize_assign(&mut r, 6);
        assert_eq!(r, RectU64::of(7, 7, 12, 12));
        checked_resize_assign(&mut r, 4);
        assert_eq!(r, RectU64::of(8, 8, 11, 11));
        checked_resize_assign(&mut r, 8);
        assert_eq!(r, RectU64::of(6, 6, 13, 13));
    }

    #[test]
    fn checked_resize_odd() {
        assert_eq!(checked_resize(&RectU64::of(5, 5, 15, 15), 11), RectU64::of(5, 5, 15, 15));
        assert_eq!(checked_resize(&RectU64::of(5, 5, 15, 15), 9), RectU64::of(6, 6, 14, 14));
        assert_eq!(checked_resize(&RectU64::of(6, 6, 14, 14), 7), RectU64::of(7, 7, 13, 13));
        assert_eq!(checked_resize(&RectU64::of(7, 7, 13, 13), 5), RectU64::of(8, 8, 12, 12));
        assert_eq!(checked_resize(&RectU64::of(8, 8, 12, 12), 3), RectU64::of(9, 9, 11, 11));
        assert_eq!(checked_resize(&RectU64::of(9, 9, 11, 11), 9), RectU64::of(6, 6, 14, 14));
    }

    #[test]
    fn checked_resize_even() {
        assert_eq!(checked_resize(&RectU64::of(5, 5, 14, 14), 10), RectU64::of(5, 5, 14, 14));
        assert_eq!(checked_resize(&RectU64::of(5, 5, 14, 14), 8), RectU64::of(6, 6, 13, 13));
        assert_eq!(checked_resize(&RectU64::of(6, 6, 13, 13), 6), RectU64::of(7, 7, 12, 12));
        assert_eq!(checked_resize(&RectU64::of(7, 7, 12, 12), 4), RectU64::of(8, 8, 11, 11));
        assert_eq!(checked_resize(&RectU64::of(8, 8, 11, 11), 8), RectU64::of(6, 6, 13, 13));
    }
}
