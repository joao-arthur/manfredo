#[cfg(test)]
mod tests {
    use super::super::{checked_resize, checked_resize_assign, try_checked_resize, try_checked_resize_assign};
    use crate::matrix::rect::rect_u32::RectU32;

    #[test]
    fn try_checked_resize_assign_odd() {
        let mut r = RectU32::of(5, 5, 15, 15);
        assert_eq!(try_checked_resize_assign(&mut r, 11), Some(()));
        assert_eq!(r, RectU32::of(5, 5, 15, 15));
        assert_eq!(try_checked_resize_assign(&mut r, 9), Some(()));
        assert_eq!(r, RectU32::of(6, 6, 14, 14));
        assert_eq!(try_checked_resize_assign(&mut r, 7), Some(()));
        assert_eq!(r, RectU32::of(7, 7, 13, 13));
        assert_eq!(try_checked_resize_assign(&mut r, 5), Some(()));
        assert_eq!(r, RectU32::of(8, 8, 12, 12));
        assert_eq!(try_checked_resize_assign(&mut r, 3), Some(()));
        assert_eq!(r, RectU32::of(9, 9, 11, 11));
        assert_eq!(try_checked_resize_assign(&mut r, 9), Some(()));
        assert_eq!(r, RectU32::of(6, 6, 14, 14));
    }

    #[test]
    fn try_checked_resize_assign_even() {
        let mut r = RectU32::of(5, 5, 14, 14);
        assert_eq!(try_checked_resize_assign(&mut r, 10), Some(()));
        assert_eq!(r, RectU32::of(5, 5, 14, 14));
        assert_eq!(try_checked_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectU32::of(6, 6, 13, 13));
        assert_eq!(try_checked_resize_assign(&mut r, 6), Some(()));
        assert_eq!(r, RectU32::of(7, 7, 12, 12));
        assert_eq!(try_checked_resize_assign(&mut r, 4), Some(()));
        assert_eq!(r, RectU32::of(8, 8, 11, 11));
        assert_eq!(try_checked_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectU32::of(6, 6, 13, 13));
    }

    #[test]
    fn try_checked_resize_assign_small_size() {
        let mut r = RectU32::of(10, 10, 20, 20);
        assert_eq!(try_checked_resize_assign(&mut r, 0), None);
        assert_eq!(try_checked_resize_assign(&mut r, 1), None);
        assert_eq!(try_checked_resize_assign(&mut r, 2), None);
        assert_eq!(r, RectU32::of(10, 10, 20, 20));
    }

    #[test]
    fn try_checked_resize_assign_same_size() {
        let mut r_11 = RectU32::of(10, 10, 20, 20);
        assert_eq!(try_checked_resize_assign(&mut r_11, 11), Some(()));
        assert_eq!(r_11, RectU32::of(10, 10, 20, 20));

        let mut r_12 = RectU32::of(10, 10, 21, 21);
        assert_eq!(try_checked_resize_assign(&mut r_12, 12), Some(()));
        assert_eq!(r_12, RectU32::of(10, 10, 21, 21));

        let mut r_13 = RectU32::of(9, 9, 21, 21);
        assert_eq!(try_checked_resize_assign(&mut r_13, 13), Some(()));
        assert_eq!(r_13, RectU32::of(9, 9, 21, 21));
    }

    #[test]
    fn try_checked_resize_assign_odd_small_rect_same_size() {
        let mut r_min = RectU32::of(0, 0, 2, 2);
        assert_eq!(try_checked_resize_assign(&mut r_min, 3), Some(()));
        assert_eq!(r_min, RectU32::of(0, 0, 2, 2));

        let mut r_max = RectU32::of(u32::MAX - 2, u32::MAX - 2, u32::MAX, u32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, 3), Some(()));
        assert_eq!(r_max, RectU32::of(u32::MAX - 2, u32::MAX - 2, u32::MAX, u32::MAX));
    }

    #[test]
    fn try_checked_resize_assign_even_small_rect_same_size() {
        let mut r = RectU32::of(0, 0, 3, 3);
        assert_eq!(try_checked_resize_assign(&mut r, 4), Some(()));
        assert_eq!(r, RectU32::of(0, 0, 3, 3));

        let mut r = RectU32::of(u32::MAX - 3, u32::MAX - 3, u32::MAX, u32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r, 4), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 3, u32::MAX - 3, u32::MAX, u32::MAX));
    }

    #[test]
    fn try_checked_resize_assign_odd_small_rect_to_bounds() {
        let mut r_min = RectU32::of(2, 2, 4, 4);
        assert_eq!(try_checked_resize_assign(&mut r_min, 7), Some(()));
        assert_eq!(r_min, RectU32::of(0, 0, 6, 6));

        let mut r_max = RectU32::of(u32::MAX - 4, u32::MAX - 4, u32::MAX - 2, u32::MAX - 2);
        assert_eq!(try_checked_resize_assign(&mut r_max, 7), Some(()));
        assert_eq!(r_max, RectU32::of(u32::MAX - 6, u32::MAX - 6, u32::MAX, u32::MAX));
    }

    #[test]
    fn try_checked_resize_assign_even_small_rect_to_bounds() {
        let mut r = RectU32::of(2, 2, 5, 5);
        assert_eq!(try_checked_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectU32::of(0, 0, 7, 7));

        let mut r = RectU32::of(u32::MAX - 5, u32::MAX - 5, u32::MAX - 2, u32::MAX - 2);
        assert_eq!(try_checked_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 7, u32::MAX - 7, u32::MAX, u32::MAX));
    }

    #[test]
    fn try_checked_resize_assign_odd_small_rect_out_of_bounds() {
        let mut r_min = RectU32::of(0, 0, 2, 2);
        assert_eq!(try_checked_resize_assign(&mut r_min, 5), None);
        assert_eq!(r_min, RectU32::of(0, 0, 2, 2));

        let mut r_min_row = RectU32::of(0, 2, 2, 4);
        assert_eq!(try_checked_resize_assign(&mut r_min_row, 5), None);
        assert_eq!(r_min_row, RectU32::of(0, 2, 2, 4));

        let mut r_min_col = RectU32::of(2, 0, 4, 2);
        assert_eq!(try_checked_resize_assign(&mut r_min_col, 5), None);
        assert_eq!(r_min_col, RectU32::of(2, 0, 4, 2));

        let mut r_max = RectU32::of(u32::MAX - 2, u32::MAX - 2, u32::MAX, u32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, 5), None);
        assert_eq!(r_max, RectU32::of(u32::MAX - 2, u32::MAX - 2, u32::MAX, u32::MAX));

        let mut r_max_row = RectU32::of(u32::MAX - 2, u32::MAX - 4, u32::MAX, u32::MAX - 2);
        assert_eq!(try_checked_resize_assign(&mut r_max_row, 5), None);
        assert_eq!(r_max_row, RectU32::of(u32::MAX - 2, u32::MAX - 4, u32::MAX, u32::MAX - 2));

        let mut r_max_col = RectU32::of(u32::MAX - 4, u32::MAX - 2, u32::MAX - 2, u32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max_col, 5), None);
        assert_eq!(r_max_col, RectU32::of(u32::MAX - 4, u32::MAX - 2, u32::MAX - 2, u32::MAX));
    }

    #[test]
    fn try_checked_resize_assign_even_small_rect_out_of_bounds() {
        let mut r_min = RectU32::of(0, 0, 3, 3);
        assert_eq!(try_checked_resize_assign(&mut r_min, 6), None);
        assert_eq!(r_min, RectU32::of(0, 0, 3, 3));

        let mut r_min_row = RectU32::of(0, 3, 3, 6);
        assert_eq!(try_checked_resize_assign(&mut r_min_row, 6), None);
        assert_eq!(r_min_row, RectU32::of(0, 3, 3, 6));

        let mut r_min_col = RectU32::of(3, 0, 6, 3);
        assert_eq!(try_checked_resize_assign(&mut r_min_col, 6), None);
        assert_eq!(r_min_col, RectU32::of(3, 0, 6, 3));

        let mut r_max = RectU32::of(u32::MAX - 3, u32::MAX - 3, u32::MAX, u32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, 6), None);
        assert_eq!(r_max, RectU32::of(u32::MAX - 3, u32::MAX - 3, u32::MAX, u32::MAX));

        let mut r_max_row = RectU32::of(u32::MAX - 3, u32::MAX - 6, u32::MAX, u32::MAX - 3);
        assert_eq!(try_checked_resize_assign(&mut r_max_row, 6), None);
        assert_eq!(r_max_row, RectU32::of(u32::MAX - 3, u32::MAX - 6, u32::MAX, u32::MAX - 3));

        let mut r_max_col = RectU32::of(u32::MAX - 6, u32::MAX - 3, u32::MAX - 3, u32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max_col, 6), None);
        assert_eq!(r_max_col, RectU32::of(u32::MAX - 6, u32::MAX - 3, u32::MAX - 3, u32::MAX));
    }

    #[test]
    fn try_checked_resize_assign_odd_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU32::of(0, 0, 2, 2);
        assert_eq!(try_checked_resize_assign(&mut r_min, u32::MAX), None);
        assert_eq!(r_min, RectU32::of(0, 0, 2, 2));

        let mut r_min_row = RectU32::of(0, 2, 2, 4);
        assert_eq!(try_checked_resize_assign(&mut r_min_row, u32::MAX), None);
        assert_eq!(r_min_row, RectU32::of(0, 2, 2, 4));

        let mut r_min_col = RectU32::of(2, 0, 4, 2);
        assert_eq!(try_checked_resize_assign(&mut r_min_col, u32::MAX), None);
        assert_eq!(r_min_col, RectU32::of(2, 0, 4, 2));

        let mut r_max = RectU32::of(u32::MAX - 2, u32::MAX - 2, u32::MAX, u32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, u32::MAX), None);
        assert_eq!(r_max, RectU32::of(u32::MAX - 2, u32::MAX - 2, u32::MAX, u32::MAX));

        let mut r_max_row = RectU32::of(u32::MAX - 2, u32::MAX - 4, u32::MAX, u32::MAX - 2);
        assert_eq!(try_checked_resize_assign(&mut r_max_row, u32::MAX), None);
        assert_eq!(r_max_row, RectU32::of(u32::MAX - 2, u32::MAX - 4, u32::MAX, u32::MAX - 2));

        let mut r_max_col = RectU32::of(u32::MAX - 4, u32::MAX - 2, u32::MAX - 2, u32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max_col, u32::MAX), None);
        assert_eq!(r_max_col, RectU32::of(u32::MAX - 4, u32::MAX - 2, u32::MAX - 2, u32::MAX));
    }

    #[test]
    fn try_checked_resize_assign_even_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU32::of(0, 0, 3, 3);
        assert_eq!(try_checked_resize_assign(&mut r_min, u32::MAX - 1), None);
        assert_eq!(r_min, RectU32::of(0, 0, 3, 3));

        let mut r_min_row = RectU32::of(0, 3, 3, 6);
        assert_eq!(try_checked_resize_assign(&mut r_min_row, u32::MAX - 1), None);
        assert_eq!(r_min_row, RectU32::of(0, 3, 3, 6));

        let mut r_min_col = RectU32::of(3, 0, 6, 3);
        assert_eq!(try_checked_resize_assign(&mut r_min_col, u32::MAX - 1), None);
        assert_eq!(r_min_col, RectU32::of(3, 0, 6, 3));

        let mut r_max = RectU32::of(u32::MAX - 3, u32::MAX - 3, u32::MAX, u32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, u32::MAX - 1), None);
        assert_eq!(r_max, RectU32::of(u32::MAX - 3, u32::MAX - 3, u32::MAX, u32::MAX));

        let mut r_max_row = RectU32::of(u32::MAX - 3, u32::MAX - 6, u32::MAX, u32::MAX - 3);
        assert_eq!(try_checked_resize_assign(&mut r_max_row, u32::MAX - 1), None);
        assert_eq!(r_max_row, RectU32::of(u32::MAX - 3, u32::MAX - 6, u32::MAX, u32::MAX - 3));

        let mut r_max_col = RectU32::of(u32::MAX - 6, u32::MAX - 3, u32::MAX - 3, u32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max_col, u32::MAX - 1), None);
        assert_eq!(r_max_col, RectU32::of(u32::MAX - 6, u32::MAX - 3, u32::MAX - 3, u32::MAX));
    }

    #[test]
    fn try_checked_resize_assign_big_rect_limits_out_of_bounds() {
        let mut r_odd_1 = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_checked_resize_assign(&mut r_odd_1, u32::MAX), Some(()));
        assert_eq!(r_odd_1, RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1));

        let mut r_odd_1 = RectU32::of(1, 1, u32::MAX, u32::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_odd_1, u32::MAX), Some(()));
        assert_eq!(r_odd_1, RectU32::of(1, 1, u32::MAX, u32::MAX));

        let mut r_even = RectU32::largest();
        assert_eq!(try_checked_resize_assign(&mut r_even, u32::MAX), Some(()));
        assert_eq!(r_even, RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1));
    }

    #[test]
    fn try_checked_resize_odd() {
        assert_eq!(try_checked_resize(&RectU32::of(5, 5, 15, 15), 11), Some(RectU32::of(5, 5, 15, 15)));
        assert_eq!(try_checked_resize(&RectU32::of(5, 5, 15, 15), 9), Some(RectU32::of(6, 6, 14, 14)));
        assert_eq!(try_checked_resize(&RectU32::of(6, 6, 14, 14), 7), Some(RectU32::of(7, 7, 13, 13)));
        assert_eq!(try_checked_resize(&RectU32::of(7, 7, 13, 13), 5), Some(RectU32::of(8, 8, 12, 12)));
        assert_eq!(try_checked_resize(&RectU32::of(8, 8, 12, 12), 3), Some(RectU32::of(9, 9, 11, 11)));
        assert_eq!(try_checked_resize(&RectU32::of(9, 9, 11, 11), 9), Some(RectU32::of(6, 6, 14, 14)));
    }

    #[test]
    fn try_checked_resize_even() {
        assert_eq!(try_checked_resize(&RectU32::of(5, 5, 14, 14), 10), Some(RectU32::of(5, 5, 14, 14)));
        assert_eq!(try_checked_resize(&RectU32::of(5, 5, 14, 14), 8), Some(RectU32::of(6, 6, 13, 13)));
        assert_eq!(try_checked_resize(&RectU32::of(6, 6, 13, 13), 6), Some(RectU32::of(7, 7, 12, 12)));
        assert_eq!(try_checked_resize(&RectU32::of(7, 7, 12, 12), 4), Some(RectU32::of(8, 8, 11, 11)));
        assert_eq!(try_checked_resize(&RectU32::of(8, 8, 11, 11), 8), Some(RectU32::of(6, 6, 13, 13)));
    }

    #[test]
    fn try_checked_resize_small_size() {
        let r = RectU32::of(10, 10, 20, 20);
        assert_eq!(try_checked_resize(&r, 0), None);
        assert_eq!(try_checked_resize(&r, 1), None);
        assert_eq!(try_checked_resize(&r, 2), None);
    }

    #[test]
    fn try_checked_resize_same_size() {
        assert_eq!(try_checked_resize(&RectU32::of(10, 10, 20, 20), 11), Some(RectU32::of(10, 10, 20, 20)));
        assert_eq!(try_checked_resize(&RectU32::of(10, 10, 21, 21), 12), Some(RectU32::of(10, 10, 21, 21)));
        assert_eq!(try_checked_resize(&RectU32::of(9, 9, 21, 21), 13), Some(RectU32::of(9, 9, 21, 21)));
    }

    #[test]
    fn try_checked_resize_odd_small_rect_same_size() {
        assert_eq!(try_checked_resize(&RectU32::of(0, 0, 2, 2), 3), Some(RectU32::of(0, 0, 2, 2)));
        assert_eq!(
            try_checked_resize(&RectU32::of(u32::MAX - 2, u32::MAX - 2, u32::MAX, u32::MAX), 3),
            Some(RectU32::of(u32::MAX - 2, u32::MAX - 2, u32::MAX, u32::MAX))
        );
    }

    #[test]
    fn try_checked_resize_even_small_rect_same_size() {
        assert_eq!(try_checked_resize(&RectU32::of(0, 0, 3, 3), 4), Some(RectU32::of(0, 0, 3, 3)));
        assert_eq!(
            try_checked_resize(&RectU32::of(u32::MAX - 3, u32::MAX - 3, u32::MAX, u32::MAX), 4),
            Some(RectU32::of(u32::MAX - 3, u32::MAX - 3, u32::MAX, u32::MAX))
        );
    }

    #[test]
    fn try_checked_resize_odd_small_rect_to_bounds() {
        assert_eq!(try_checked_resize(&RectU32::of(2, 2, 4, 4), 7), Some(RectU32::of(0, 0, 6, 6)));
        assert_eq!(
            try_checked_resize(&RectU32::of(u32::MAX - 4, u32::MAX - 4, u32::MAX - 2, u32::MAX - 2), 7),
            Some(RectU32::of(u32::MAX - 6, u32::MAX - 6, u32::MAX, u32::MAX))
        );
    }

    #[test]
    fn try_checked_resize_even_small_rect_to_bounds() {
        assert_eq!(try_checked_resize(&RectU32::of(2, 2, 5, 5), 8), Some(RectU32::of(0, 0, 7, 7)));
        assert_eq!(
            try_checked_resize(&RectU32::of(u32::MAX - 5, u32::MAX - 5, u32::MAX - 2, u32::MAX - 2), 8),
            Some(RectU32::of(u32::MAX - 7, u32::MAX - 7, u32::MAX, u32::MAX))
        );
    }

    #[test]
    fn try_checked_resize_odd_small_rect_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectU32::of(0, 0, 2, 2), 5), None);
        assert_eq!(try_checked_resize(&RectU32::of(0, 2, 2, 4), 5), None);
        assert_eq!(try_checked_resize(&RectU32::of(2, 0, 4, 2), 5), None);
        assert_eq!(try_checked_resize(&RectU32::of(u32::MAX - 2, u32::MAX - 2, u32::MAX, u32::MAX), 5), None);
        assert_eq!(try_checked_resize(&RectU32::of(u32::MAX - 2, u32::MAX - 4, u32::MAX, u32::MAX - 2), 5), None);
        assert_eq!(try_checked_resize(&RectU32::of(u32::MAX - 4, u32::MAX - 2, u32::MAX - 2, u32::MAX), 5), None);
    }

    #[test]
    fn try_checked_resize_even_small_rect_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectU32::of(0, 0, 3, 3), 6), None);
        assert_eq!(try_checked_resize(&RectU32::of(0, 3, 3, 6), 6), None);
        assert_eq!(try_checked_resize(&RectU32::of(0, 0, 3, 3), 6), None);
        assert_eq!(try_checked_resize(&RectU32::of(u32::MAX - 3, u32::MAX - 3, u32::MAX, u32::MAX), 6), None);
        assert_eq!(try_checked_resize(&RectU32::of(u32::MAX - 3, u32::MAX - 6, u32::MAX, u32::MAX - 3), 6), None);
        assert_eq!(try_checked_resize(&RectU32::of(u32::MAX - 6, u32::MAX - 3, u32::MAX - 3, u32::MAX), 6), None);
    }

    #[test]
    fn try_checked_resize_odd_small_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectU32::of(0, 0, 2, 2), u32::MAX), None);
        assert_eq!(try_checked_resize(&RectU32::of(0, 2, 2, 4), u32::MAX), None);
        assert_eq!(try_checked_resize(&RectU32::of(2, 0, 4, 2), u32::MAX), None);
        assert_eq!(try_checked_resize(&RectU32::of(u32::MAX - 2, u32::MAX - 2, u32::MAX, u32::MAX), u32::MAX), None);
        assert_eq!(try_checked_resize(&RectU32::of(u32::MAX - 2, u32::MAX - 4, u32::MAX, u32::MAX - 2), u32::MAX), None);
        assert_eq!(try_checked_resize(&RectU32::of(u32::MAX - 4, u32::MAX - 2, u32::MAX - 2, u32::MAX), u32::MAX), None);
    }

    #[test]
    fn try_checked_resize_even_small_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectU32::of(0, 0, 3, 3), u32::MAX - 1), None);
        assert_eq!(try_checked_resize(&RectU32::of(0, 3, 3, 6), u32::MAX - 1), None);
        assert_eq!(try_checked_resize(&RectU32::of(0, 0, 3, 3), u32::MAX - 1), None);
        assert_eq!(try_checked_resize(&RectU32::of(u32::MAX - 3, u32::MAX - 3, u32::MAX, u32::MAX), u32::MAX - 1), None);
        assert_eq!(try_checked_resize(&RectU32::of(u32::MAX - 3, u32::MAX - 6, u32::MAX, u32::MAX - 3), u32::MAX - 1), None);
        assert_eq!(try_checked_resize(&RectU32::of(u32::MAX - 6, u32::MAX - 3, u32::MAX - 3, u32::MAX), u32::MAX - 1), None);
    }

    #[test]
    fn try_checked_resize_big_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1), u32::MAX), Some(RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1)));
        assert_eq!(try_checked_resize(&RectU32::of(1, 1, u32::MAX, u32::MAX), u32::MAX), Some(RectU32::of(1, 1, u32::MAX, u32::MAX)));
        assert_eq!(try_checked_resize(&RectU32::largest(), u32::MAX), Some(RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1)));
    }

    #[test]
    fn checked_resize_assign_odd() {
        let mut r = RectU32::of(5, 5, 15, 15);
        checked_resize_assign(&mut r, 11);
        assert_eq!(r, RectU32::of(5, 5, 15, 15));
        checked_resize_assign(&mut r, 9);
        assert_eq!(r, RectU32::of(6, 6, 14, 14));
        checked_resize_assign(&mut r, 7);
        assert_eq!(r, RectU32::of(7, 7, 13, 13));
        checked_resize_assign(&mut r, 5);
        assert_eq!(r, RectU32::of(8, 8, 12, 12));
        checked_resize_assign(&mut r, 3);
        assert_eq!(r, RectU32::of(9, 9, 11, 11));
        checked_resize_assign(&mut r, 9);
        assert_eq!(r, RectU32::of(6, 6, 14, 14));
    }

    #[test]
    fn checked_resize_assign_even() {
        let mut r = RectU32::of(5, 5, 14, 14);
        checked_resize_assign(&mut r, 10);
        assert_eq!(r, RectU32::of(5, 5, 14, 14));
        checked_resize_assign(&mut r, 8);
        assert_eq!(r, RectU32::of(6, 6, 13, 13));
        checked_resize_assign(&mut r, 6);
        assert_eq!(r, RectU32::of(7, 7, 12, 12));
        checked_resize_assign(&mut r, 4);
        assert_eq!(r, RectU32::of(8, 8, 11, 11));
        checked_resize_assign(&mut r, 8);
        assert_eq!(r, RectU32::of(6, 6, 13, 13));
    }

    #[test]
    fn checked_resize_odd() {
        assert_eq!(checked_resize(&RectU32::of(5, 5, 15, 15), 11), RectU32::of(5, 5, 15, 15));
        assert_eq!(checked_resize(&RectU32::of(5, 5, 15, 15), 9), RectU32::of(6, 6, 14, 14));
        assert_eq!(checked_resize(&RectU32::of(6, 6, 14, 14), 7), RectU32::of(7, 7, 13, 13));
        assert_eq!(checked_resize(&RectU32::of(7, 7, 13, 13), 5), RectU32::of(8, 8, 12, 12));
        assert_eq!(checked_resize(&RectU32::of(8, 8, 12, 12), 3), RectU32::of(9, 9, 11, 11));
        assert_eq!(checked_resize(&RectU32::of(9, 9, 11, 11), 9), RectU32::of(6, 6, 14, 14));
    }

    #[test]
    fn checked_resize_even() {
        assert_eq!(checked_resize(&RectU32::of(5, 5, 14, 14), 10), RectU32::of(5, 5, 14, 14));
        assert_eq!(checked_resize(&RectU32::of(5, 5, 14, 14), 8), RectU32::of(6, 6, 13, 13));
        assert_eq!(checked_resize(&RectU32::of(6, 6, 13, 13), 6), RectU32::of(7, 7, 12, 12));
        assert_eq!(checked_resize(&RectU32::of(7, 7, 12, 12), 4), RectU32::of(8, 8, 11, 11));
        assert_eq!(checked_resize(&RectU32::of(8, 8, 11, 11), 8), RectU32::of(6, 6, 13, 13));
    }
}
