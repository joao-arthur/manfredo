use crate::matrix::{
    point::point_u64::PointU64,
    rect::rect_u64::{RectU64, delta_col, delta_row},
};

pub fn try_saturating_resize_assign(r: &mut RectU64, size: u64) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_row = i128::from(delta_row(r)) + 1 - i128::from(size);
    let diff_col = i128::from(delta_col(r)) + 1 - i128::from(size);
    let temp_min_row = i128::from(r.min.row) + diff_row / 2;
    let temp_min_col = i128::from(r.min.col) + diff_col / 2;
    let min_row = temp_min_row.clamp(0, i128::from(u64::MAX) - i128::from(size) + 1);
    let min_col = temp_min_col.clamp(0, i128::from(u64::MAX) - i128::from(size) + 1);
    r.min.row = min_row as u64;
    r.min.col = min_col as u64;
    r.max.row = (min_row + i128::from(size) - 1) as u64;
    r.max.col = (min_col + i128::from(size) - 1) as u64;
    Some(())
}

pub fn try_saturating_resize(r: &RectU64, size: u64) -> Option<RectU64> {
    if size < 3 {
        return None;
    }
    let diff_row = i128::from(delta_row(r)) + 1 - i128::from(size);
    let diff_col = i128::from(delta_col(r)) + 1 - i128::from(size);
    let temp_min_row = i128::from(r.min.row) + diff_row / 2;
    let temp_min_col = i128::from(r.min.col) + diff_col / 2;
    let clamped_min_row = temp_min_row.clamp(0, i128::from(u64::MAX) - i128::from(size) + 1);
    let clamped_min_col = temp_min_col.clamp(0, i128::from(u64::MAX) - i128::from(size) + 1);
    let min_row = clamped_min_row as u64;
    let min_col = clamped_min_col as u64;
    let max_row = (clamped_min_row + i128::from(size) - 1) as u64;
    let max_col = (clamped_min_col + i128::from(size) - 1) as u64;
    Some(RectU64 { min: PointU64 { row: min_row, col: min_col }, max: PointU64 { row: max_row, col: max_col } })
}

pub fn saturating_resize_assign(r: &mut RectU64, size: u64) {
    try_saturating_resize_assign(r, size).unwrap()
}

pub fn saturating_resize(r: &RectU64, size: u64) -> RectU64 {
    try_saturating_resize(r, size).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{saturating_resize_assign, saturating_resize, try_saturating_resize_assign, try_saturating_resize};
    use crate::matrix::rect::rect_u64::RectU64;

    #[test]
    fn try_saturating_resize_assign_odd() {
        let mut r = RectU64::of(5, 5, 15, 15);
        assert_eq!(try_saturating_resize_assign(&mut r, 11), Some(()));
        assert_eq!(r, RectU64::of(5, 5, 15, 15));
        assert_eq!(try_saturating_resize_assign(&mut r, 9), Some(()));
        assert_eq!(r, RectU64::of(6, 6, 14, 14));
        assert_eq!(try_saturating_resize_assign(&mut r, 7), Some(()));
        assert_eq!(r, RectU64::of(7, 7, 13, 13));
        assert_eq!(try_saturating_resize_assign(&mut r, 5), Some(()));
        assert_eq!(r, RectU64::of(8, 8, 12, 12));
        assert_eq!(try_saturating_resize_assign(&mut r, 3), Some(()));
        assert_eq!(r, RectU64::of(9, 9, 11, 11));
        assert_eq!(try_saturating_resize_assign(&mut r, 9), Some(()));
        assert_eq!(r, RectU64::of(6, 6, 14, 14));
    }

    #[test]
    fn try_saturating_resize_assign_even() {
        let mut r = RectU64::of(5, 5, 14, 14);
        assert_eq!(try_saturating_resize_assign(&mut r, 10), Some(()));
        assert_eq!(r, RectU64::of(5, 5, 14, 14));
        assert_eq!(try_saturating_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectU64::of(6, 6, 13, 13));
        assert_eq!(try_saturating_resize_assign(&mut r, 6), Some(()));
        assert_eq!(r, RectU64::of(7, 7, 12, 12));
        assert_eq!(try_saturating_resize_assign(&mut r, 4), Some(()));
        assert_eq!(r, RectU64::of(8, 8, 11, 11));
        assert_eq!(try_saturating_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectU64::of(6, 6, 13, 13));
    }

    #[test]
    fn try_saturating_resize_assign_small_size() {
        let mut r = RectU64::of(10, 10, 20, 20);
        assert_eq!(try_saturating_resize_assign(&mut r, 0), None);
        assert_eq!(try_saturating_resize_assign(&mut r, 1), None);
        assert_eq!(try_saturating_resize_assign(&mut r, 2), None);
        assert_eq!(r, RectU64::of(10, 10, 20, 20));
    }

    #[test]
    fn try_saturating_resize_assign_same_size() {
        let mut r_11 = RectU64::of(10, 10, 20, 20);
        assert_eq!(try_saturating_resize_assign(&mut r_11, 11), Some(()));
        assert_eq!(r_11, RectU64::of(10, 10, 20, 20));

        let mut r_12 = RectU64::of(10, 10, 21, 21);
        assert_eq!(try_saturating_resize_assign(&mut r_12, 12), Some(()));
        assert_eq!(r_12, RectU64::of(10, 10, 21, 21));

        let mut r_13 = RectU64::of(9, 9, 21, 21);
        assert_eq!(try_saturating_resize_assign(&mut r_13, 13), Some(()));
        assert_eq!(r_13, RectU64::of(9, 9, 21, 21));
    }

    #[test]
    fn try_saturating_resize_assign_odd_small_rect_out_of_bounds() {
        let mut r_min = RectU64::of(0, 0, 2, 2);
        assert_eq!(try_saturating_resize_assign(&mut r_min, 11), Some(()));
        assert_eq!(r_min, RectU64::of(0, 0, 10, 10));

        let mut r_max = RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX);
        assert_eq!(try_saturating_resize_assign(&mut r_max, 11), Some(()));
        assert_eq!(r_max, RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX));
    }

    #[test]
    fn try_saturating_resize_assign_even_small_rect_out_of_bounds() {
        let mut r = RectU64::of(0, 0, 3, 3);
        assert_eq!(try_saturating_resize_assign(&mut r, 11), Some(()));
        assert_eq!(r, RectU64::of(0, 0, 10, 10));

        let mut r = RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX);
        assert_eq!(try_saturating_resize_assign(&mut r, 11), Some(()));
        assert_eq!(r, RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX));
    }

    #[test]
    fn try_saturating_resize_assign_odd_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU64::of(0, 0, 2, 2);
        assert_eq!(try_saturating_resize_assign(&mut r_min, u64::MAX), Some(()));
        assert_eq!(r_min, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));

        let mut r_max = RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX);
        assert_eq!(try_saturating_resize_assign(&mut r_max, u64::MAX), Some(()));
        assert_eq!(r_max, RectU64::of(1, 1, u64::MAX, u64::MAX));
    }

    #[test]
    fn try_saturating_resize_assign_even_small_rect_limits_out_of_bounds() {
        let mut r = RectU64::of(0, 0, 3, 3);
        assert_eq!(try_saturating_resize_assign(&mut r, u64::MAX - 1), Some(()));
        assert_eq!(r, RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 2));

        let mut r = RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX);
        assert_eq!(try_saturating_resize_assign(&mut r, u64::MAX - 1), Some(()));
        assert_eq!(r, RectU64::of(2, 2, u64::MAX, u64::MAX));
    }

    #[test]
    fn try_saturating_resize_assign_big_rect_limits_out_of_bounds() {
        let mut r_odd_1 = RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1);
        assert_eq!(try_saturating_resize_assign(&mut r_odd_1, u64::MAX), Some(()));
        assert_eq!(r_odd_1, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));

        let mut r_odd_1 = RectU64::of(1, 1, u64::MAX, u64::MAX);
        assert_eq!(try_saturating_resize_assign(&mut r_odd_1, u64::MAX), Some(()));
        assert_eq!(r_odd_1, RectU64::of(1, 1, u64::MAX, u64::MAX));

        let mut r_even = RectU64::largest();
        assert_eq!(try_saturating_resize_assign(&mut r_even, u64::MAX), Some(()));
        assert_eq!(r_even, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));
    }

    #[test]
    fn try_saturating_resize_odd() {
        assert_eq!(try_saturating_resize(&RectU64::of(5, 5, 15, 15), 11), Some(RectU64::of(5, 5, 15, 15)));
        assert_eq!(try_saturating_resize(&RectU64::of(5, 5, 15, 15), 9), Some(RectU64::of(6, 6, 14, 14)));
        assert_eq!(try_saturating_resize(&RectU64::of(6, 6, 14, 14), 7), Some(RectU64::of(7, 7, 13, 13)));
        assert_eq!(try_saturating_resize(&RectU64::of(7, 7, 13, 13), 5), Some(RectU64::of(8, 8, 12, 12)));
        assert_eq!(try_saturating_resize(&RectU64::of(8, 8, 12, 12), 3), Some(RectU64::of(9, 9, 11, 11)));
        assert_eq!(try_saturating_resize(&RectU64::of(9, 9, 11, 11), 9), Some(RectU64::of(6, 6, 14, 14)));
    }

    #[test]
    fn try_saturating_resize_even() {
        assert_eq!(try_saturating_resize(&RectU64::of(5, 5, 14, 14), 10), Some(RectU64::of(5, 5, 14, 14)));
        assert_eq!(try_saturating_resize(&RectU64::of(5, 5, 14, 14), 8), Some(RectU64::of(6, 6, 13, 13)));
        assert_eq!(try_saturating_resize(&RectU64::of(6, 6, 13, 13), 6), Some(RectU64::of(7, 7, 12, 12)));
        assert_eq!(try_saturating_resize(&RectU64::of(7, 7, 12, 12), 4), Some(RectU64::of(8, 8, 11, 11)));
        assert_eq!(try_saturating_resize(&RectU64::of(8, 8, 11, 11), 8), Some(RectU64::of(6, 6, 13, 13)));
    }

    #[test]
    fn try_saturating_resize_small_size() {
        let r = RectU64::of(10, 10, 20, 20);
        assert_eq!(try_saturating_resize(&r, 0), None);
        assert_eq!(try_saturating_resize(&r, 1), None);
        assert_eq!(try_saturating_resize(&r, 2), None);
    }

    #[test]
    fn try_saturating_resize_same_size() {
        assert_eq!(try_saturating_resize(&RectU64::of(10, 10, 20, 20), 11), Some(RectU64::of(10, 10, 20, 20)));
        assert_eq!(try_saturating_resize(&RectU64::of(10, 10, 21, 21), 12), Some(RectU64::of(10, 10, 21, 21)));
        assert_eq!(try_saturating_resize(&RectU64::of(9, 9, 21, 21), 13), Some(RectU64::of(9, 9, 21, 21)));
    }

    #[test]
    fn try_saturating_resize_odd_small_rect_out_of_bounds() {
        assert_eq!(try_saturating_resize(&RectU64::of(0, 0, 2, 2), 11), Some(RectU64::of(0, 0, 10, 10)));
        assert_eq!(
            try_saturating_resize(&RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX), 11),
            Some(RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX))
        );
    }

    #[test]
    fn try_saturating_resize_even_small_rect_out_of_bounds() {
        assert_eq!(try_saturating_resize(&RectU64::of(0, 0, 3, 3), 11), Some(RectU64::of(0, 0, 10, 10)));
        assert_eq!(
            try_saturating_resize(&RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX), 11),
            Some(RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX))
        );
    }

    #[test]
    fn try_saturating_resize_odd_small_rect_limits_out_of_bounds() {
        assert_eq!(try_saturating_resize(&RectU64::of(0, 0, 2, 2), u64::MAX), Some(RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1)));
        assert_eq!(try_saturating_resize(&RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX), u64::MAX), Some(RectU64::of(1, 1, u64::MAX, u64::MAX)));
    }

    #[test]
    fn try_saturating_resize_even_small_rect_limits_out_of_bounds() {
        assert_eq!(try_saturating_resize(&RectU64::of(0, 0, 3, 3), u64::MAX - 1), Some(RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 2)));
        assert_eq!(
            try_saturating_resize(&RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX), u64::MAX - 1),
            Some(RectU64::of(2, 2, u64::MAX, u64::MAX))
        );
    }

    #[test]
    fn try_saturating_resize_big_rect_limits_out_of_bounds() {
        assert_eq!(try_saturating_resize(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1), u64::MAX), Some(RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1)));
        assert_eq!(try_saturating_resize(&RectU64::of(1, 1, u64::MAX, u64::MAX), u64::MAX), Some(RectU64::of(1, 1, u64::MAX, u64::MAX)));
        assert_eq!(try_saturating_resize(&RectU64::largest(), u64::MAX), Some(RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1)));
    }

    #[test]
    fn saturating_resize_assign_odd() {
        let mut r = RectU64::of(5, 5, 15, 15);
        saturating_resize_assign(&mut r, 11);
        assert_eq!(r, RectU64::of(5, 5, 15, 15));
        saturating_resize_assign(&mut r, 9);
        assert_eq!(r, RectU64::of(6, 6, 14, 14));
        saturating_resize_assign(&mut r, 7);
        assert_eq!(r, RectU64::of(7, 7, 13, 13));
        saturating_resize_assign(&mut r, 5);
        assert_eq!(r, RectU64::of(8, 8, 12, 12));
        saturating_resize_assign(&mut r, 3);
        assert_eq!(r, RectU64::of(9, 9, 11, 11));
        saturating_resize_assign(&mut r, 9);
        assert_eq!(r, RectU64::of(6, 6, 14, 14));
    }

    #[test]
    fn saturating_resize_assign_even() {
        let mut r = RectU64::of(5, 5, 14, 14);
        saturating_resize_assign(&mut r, 10);
        assert_eq!(r, RectU64::of(5, 5, 14, 14));
        saturating_resize_assign(&mut r, 8);
        assert_eq!(r, RectU64::of(6, 6, 13, 13));
        saturating_resize_assign(&mut r, 6);
        assert_eq!(r, RectU64::of(7, 7, 12, 12));
        saturating_resize_assign(&mut r, 4);
        assert_eq!(r, RectU64::of(8, 8, 11, 11));
        saturating_resize_assign(&mut r, 8);
        assert_eq!(r, RectU64::of(6, 6, 13, 13));
    }

    #[test]
    fn saturating_resize_odd() {
        assert_eq!(saturating_resize(&RectU64::of(5, 5, 15, 15), 11), RectU64::of(5, 5, 15, 15));
        assert_eq!(saturating_resize(&RectU64::of(5, 5, 15, 15), 9), RectU64::of(6, 6, 14, 14));
        assert_eq!(saturating_resize(&RectU64::of(6, 6, 14, 14), 7), RectU64::of(7, 7, 13, 13));
        assert_eq!(saturating_resize(&RectU64::of(7, 7, 13, 13), 5), RectU64::of(8, 8, 12, 12));
        assert_eq!(saturating_resize(&RectU64::of(8, 8, 12, 12), 3), RectU64::of(9, 9, 11, 11));
        assert_eq!(saturating_resize(&RectU64::of(9, 9, 11, 11), 9), RectU64::of(6, 6, 14, 14));
    }

    #[test]
    fn saturating_resize_even() {
        assert_eq!(saturating_resize(&RectU64::of(5, 5, 14, 14), 10), RectU64::of(5, 5, 14, 14));
        assert_eq!(saturating_resize(&RectU64::of(5, 5, 14, 14), 8), RectU64::of(6, 6, 13, 13));
        assert_eq!(saturating_resize(&RectU64::of(6, 6, 13, 13), 6), RectU64::of(7, 7, 12, 12));
        assert_eq!(saturating_resize(&RectU64::of(7, 7, 12, 12), 4), RectU64::of(8, 8, 11, 11));
        assert_eq!(saturating_resize(&RectU64::of(8, 8, 11, 11), 8), RectU64::of(6, 6, 13, 13));
    }
}
