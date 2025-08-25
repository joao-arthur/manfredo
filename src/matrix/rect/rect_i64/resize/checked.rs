use crate::matrix::{
    point::point_i64::PointI64,
    rect::rect_i64::{RectI64, delta_col, delta_row},
};

pub fn try_checked_resize_assign(r: &mut RectI64, size: u64) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_row = i128::from(delta_row(r)) + 1 - i128::from(size);
    let diff_col = i128::from(delta_col(r)) + 1 - i128::from(size);
    let temp_min_row = i128::from(r.min.row) + diff_row / 2;
    let temp_min_col = i128::from(r.min.col) + diff_col / 2;
    let min_row = i64::try_from(temp_min_row).ok()?;
    let min_col = i64::try_from(temp_min_col).ok()?;
    let max_row = min_row.checked_add_unsigned(size - 1)?;
    let max_col = min_col.checked_add_unsigned(size - 1)?;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Some(())
}

pub fn try_checked_resize(r: &RectI64, size: u64) -> Option<RectI64> {
    if size < 3 {
        return None;
    }
    let diff_row = i128::from(delta_row(r)) + 1 - i128::from(size);
    let diff_col = i128::from(delta_col(r)) + 1 - i128::from(size);
    let temp_min_row = i128::from(r.min.row) + diff_row / 2;
    let temp_min_col = i128::from(r.min.col) + diff_col / 2;
    let min_row = i64::try_from(temp_min_row).ok()?;
    let min_col = i64::try_from(temp_min_col).ok()?;
    let max_row = min_row.checked_add_unsigned(size - 1)?;
    let max_col = min_col.checked_add_unsigned(size - 1)?;
    Some(RectI64 { min: PointI64 { row: min_row, col: min_col }, max: PointI64 { row: max_row, col: max_col } })
}

pub fn checked_resize_assign(r: &mut RectI64, size: u64) {
    try_checked_resize_assign(r, size).unwrap()
}

pub fn checked_resize(r: &RectI64, size: u64) -> RectI64 {
    try_checked_resize(r, size).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{checked_resize, checked_resize_assign, try_checked_resize, try_checked_resize_assign};
    use crate::matrix::rect::rect_i64::RectI64;

    #[test]
    fn try_checked_resize_assign_odd() {
        let mut r = RectI64::of(-5, -5, 5, 5);
        assert_eq!(try_checked_resize_assign(&mut r, 11), Some(()));
        assert_eq!(r, RectI64::of(-5, -5, 5, 5));
        assert_eq!(try_checked_resize_assign(&mut r, 9), Some(()));
        assert_eq!(r, RectI64::of(-4, -4, 4, 4));
        assert_eq!(try_checked_resize_assign(&mut r, 7), Some(()));
        assert_eq!(r, RectI64::of(-3, -3, 3, 3));
        assert_eq!(try_checked_resize_assign(&mut r, 5), Some(()));
        assert_eq!(r, RectI64::of(-2, -2, 2, 2));
        assert_eq!(try_checked_resize_assign(&mut r, 3), Some(()));
        assert_eq!(r, RectI64::of(-1, -1, 1, 1));
        assert_eq!(try_checked_resize_assign(&mut r, 9), Some(()));
        assert_eq!(r, RectI64::of(-4, -4, 4, 4));
    }

    #[test]
    fn try_checked_resize_assign_even() {
        let mut r = RectI64::of(-5, -5, 4, 4);
        assert_eq!(try_checked_resize_assign(&mut r, 10), Some(()));
        assert_eq!(r, RectI64::of(-5, -5, 4, 4));
        assert_eq!(try_checked_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectI64::of(-4, -4, 3, 3));
        assert_eq!(try_checked_resize_assign(&mut r, 6), Some(()));
        assert_eq!(r, RectI64::of(-3, -3, 2, 2));
        assert_eq!(try_checked_resize_assign(&mut r, 4), Some(()));
        assert_eq!(r, RectI64::of(-2, -2, 1, 1));
        assert_eq!(try_checked_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectI64::of(-4, -4, 3, 3));
    }

    #[test]
    fn try_checked_resize_assign_small_size() {
        let mut r = RectI64::of(10, 10, 20, 20);
        assert_eq!(try_checked_resize_assign(&mut r, 0), None);
        assert_eq!(try_checked_resize_assign(&mut r, 1), None);
        assert_eq!(try_checked_resize_assign(&mut r, 2), None);
        assert_eq!(r, RectI64::of(10, 10, 20, 20));
    }

    #[test]
    fn try_checked_resize_assign_same_size() {
        let mut r_11 = RectI64::of(10, 10, 20, 20);
        assert_eq!(try_checked_resize_assign(&mut r_11, 11), Some(()));
        assert_eq!(r_11, RectI64::of(10, 10, 20, 20));

        let mut r_12 = RectI64::of(10, 10, 21, 21);
        assert_eq!(try_checked_resize_assign(&mut r_12, 12), Some(()));
        assert_eq!(r_12, RectI64::of(10, 10, 21, 21));

        let mut r_13 = RectI64::of(9, 9, 21, 21);
        assert_eq!(try_checked_resize_assign(&mut r_13, 13), Some(()));
        assert_eq!(r_13, RectI64::of(9, 9, 21, 21));
    }

    #[test]
    fn try_checked_resize_assign_odd_small_rect_same_size() {
        let mut r_min = RectI64::of(i64::MIN, i64::MIN, i64::MIN + 2, i64::MIN + 2);
        assert_eq!(try_checked_resize_assign(&mut r_min, 3), Some(()));
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 2, i64::MIN + 2));

        let mut r_max = RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, 3), Some(()));
        assert_eq!(r_max, RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX));
    }

    #[test]
    fn try_checked_resize_assign_even_small_rect_same_size() {
        let mut r = RectI64::of(i64::MIN, i64::MIN, i64::MIN + 3, i64::MIN + 3);
        assert_eq!(try_checked_resize_assign(&mut r, 4), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 3, i64::MIN + 3));

        let mut r = RectI64::of(i64::MAX - 3, i64::MAX - 3, i64::MAX, i64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r, 4), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 3, i64::MAX - 3, i64::MAX, i64::MAX));
    }

    #[test]
    fn try_checked_resize_assign_odd_small_rect_to_bounds() {
        let mut r_min = RectI64::of(2, 2, 4, 4);
        assert_eq!(try_checked_resize_assign(&mut r_min, 7), Some(()));
        assert_eq!(r_min, RectI64::of(0, 0, 6, 6));

        let mut r_max = RectI64::of(i64::MAX - 4, i64::MAX - 4, i64::MAX - 2, i64::MAX - 2);
        assert_eq!(try_checked_resize_assign(&mut r_max, 7), Some(()));
        assert_eq!(r_max, RectI64::of(i64::MAX - 6, i64::MAX - 6, i64::MAX, i64::MAX));
    }

    #[test]
    fn try_checked_resize_assign_even_small_rect_to_bounds() {
        let mut r = RectI64::of(2, 2, 5, 5);
        assert_eq!(try_checked_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectI64::of(0, 0, 7, 7));

        let mut r = RectI64::of(i64::MAX - 5, i64::MAX - 5, i64::MAX - 2, i64::MAX - 2);
        assert_eq!(try_checked_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 7, i64::MAX - 7, i64::MAX, i64::MAX));
    }

    #[test]
    fn try_checked_resize_assign_odd_small_rect_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN, i64::MIN, i64::MIN + 2, i64::MIN + 2);
        assert_eq!(try_checked_resize_assign(&mut r_min, 5), None);
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 2, i64::MIN + 2));

        let mut r_min_row = RectI64::of(i64::MIN, i64::MIN + 2, i64::MIN + 2, i64::MIN + 4);
        assert_eq!(try_checked_resize_assign(&mut r_min_row, 5), None);
        assert_eq!(r_min_row, RectI64::of(i64::MIN, i64::MIN + 2, i64::MIN + 2, i64::MIN + 4));

        let mut r_min_col = RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 4, i64::MIN + 2);
        assert_eq!(try_checked_resize_assign(&mut r_min_col, 5), None);
        assert_eq!(r_min_col, RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 4, i64::MIN + 2));

        let mut r_max = RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, 5), None);
        assert_eq!(r_max, RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX));

        let mut r_max_row = RectI64::of(i64::MAX - 2, i64::MAX - 4, i64::MAX, i64::MAX - 2);
        assert_eq!(try_checked_resize_assign(&mut r_max_row, 5), None);
        assert_eq!(r_max_row, RectI64::of(i64::MAX - 2, i64::MAX - 4, i64::MAX, i64::MAX - 2));

        let mut r_max_col = RectI64::of(i64::MAX - 4, i64::MAX - 2, i64::MAX - 2, i64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max_col, 5), None);
        assert_eq!(r_max_col, RectI64::of(i64::MAX - 4, i64::MAX - 2, i64::MAX - 2, i64::MAX));
    }

    #[test]
    fn try_checked_resize_assign_even_small_rect_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN, i64::MIN, i64::MIN + 3, i64::MIN + 3);
        assert_eq!(try_checked_resize_assign(&mut r_min, 6), None);
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 3, i64::MIN + 3));

        let mut r_min_row = RectI64::of(i64::MIN, i64::MIN + 3, i64::MIN + 3, i64::MIN + 6);
        assert_eq!(try_checked_resize_assign(&mut r_min_row, 6), None);
        assert_eq!(r_min_row, RectI64::of(i64::MIN, i64::MIN + 3, i64::MIN + 3, i64::MIN + 6));

        let mut r_min_col = RectI64::of(i64::MIN + 3, i64::MIN, i64::MIN + 6, i64::MIN + 3);
        assert_eq!(try_checked_resize_assign(&mut r_min_col, 6), None);
        assert_eq!(r_min_col, RectI64::of(i64::MIN + 3, i64::MIN, i64::MIN + 6, i64::MIN + 3));

        let mut r_max = RectI64::of(i64::MAX - 3, i64::MAX - 3, i64::MAX, i64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, 6), None);
        assert_eq!(r_max, RectI64::of(i64::MAX - 3, i64::MAX - 3, i64::MAX, i64::MAX));

        let mut r_max_row = RectI64::of(i64::MAX - 3, i64::MAX - 6, i64::MAX, i64::MAX - 3);
        assert_eq!(try_checked_resize_assign(&mut r_max_row, 6), None);
        assert_eq!(r_max_row, RectI64::of(i64::MAX - 3, i64::MAX - 6, i64::MAX, i64::MAX - 3));

        let mut r_max_col = RectI64::of(i64::MAX - 6, i64::MAX - 3, i64::MAX - 3, i64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max_col, 6), None);
        assert_eq!(r_max_col, RectI64::of(i64::MAX - 6, i64::MAX - 3, i64::MAX - 3, i64::MAX));
    }

    #[test]
    fn try_checked_resize_assign_odd_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN, i64::MIN, i64::MIN + 2, i64::MIN + 2);
        assert_eq!(try_checked_resize_assign(&mut r_min, u64::MAX), None);
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 2, i64::MIN + 2));

        let mut r_min_row = RectI64::of(i64::MIN, i64::MIN + 2, i64::MIN + 2, i64::MIN + 4);
        assert_eq!(try_checked_resize_assign(&mut r_min_row, u64::MAX), None);
        assert_eq!(r_min_row, RectI64::of(i64::MIN, i64::MIN + 2, i64::MIN + 2, i64::MIN + 4));

        let mut r_min_col = RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 4, i64::MIN + 2);
        assert_eq!(try_checked_resize_assign(&mut r_min_col, u64::MAX), None);
        assert_eq!(r_min_col, RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 4, i64::MIN + 2));

        let mut r_max = RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, u64::MAX), None);
        assert_eq!(r_max, RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX));

        let mut r_max_row = RectI64::of(i64::MAX - 2, i64::MAX - 4, i64::MAX, i64::MAX - 2);
        assert_eq!(try_checked_resize_assign(&mut r_max_row, u64::MAX), None);
        assert_eq!(r_max_row, RectI64::of(i64::MAX - 2, i64::MAX - 4, i64::MAX, i64::MAX - 2));

        let mut r_max_col = RectI64::of(i64::MAX - 4, i64::MAX - 2, i64::MAX - 2, i64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max_col, u64::MAX), None);
        assert_eq!(r_max_col, RectI64::of(i64::MAX - 4, i64::MAX - 2, i64::MAX - 2, i64::MAX));
    }

    #[test]
    fn try_checked_resize_assign_even_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN, i64::MIN, i64::MIN + 3, i64::MIN + 3);
        assert_eq!(try_checked_resize_assign(&mut r_min, u64::MAX - 1), None);
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 3, i64::MIN + 3));

        let mut r_min_row = RectI64::of(i64::MIN, i64::MIN + 3, i64::MIN + 3, i64::MIN + 6);
        assert_eq!(try_checked_resize_assign(&mut r_min_row, u64::MAX - 1), None);
        assert_eq!(r_min_row, RectI64::of(i64::MIN, i64::MIN + 3, i64::MIN + 3, i64::MIN + 6));

        let mut r_min_col = RectI64::of(i64::MIN + 3, i64::MIN, i64::MIN + 6, i64::MIN + 3);
        assert_eq!(try_checked_resize_assign(&mut r_min_col, u64::MAX - 1), None);
        assert_eq!(r_min_col, RectI64::of(i64::MIN + 3, i64::MIN, i64::MIN + 6, i64::MIN + 3));

        let mut r_max = RectI64::of(i64::MAX - 3, i64::MAX - 3, i64::MAX, i64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max, u64::MAX - 1), None);
        assert_eq!(r_max, RectI64::of(i64::MAX - 3, i64::MAX - 3, i64::MAX, i64::MAX));

        let mut r_max_row = RectI64::of(i64::MAX - 3, i64::MAX - 6, i64::MAX, i64::MAX - 3);
        assert_eq!(try_checked_resize_assign(&mut r_max_row, u64::MAX - 1), None);
        assert_eq!(r_max_row, RectI64::of(i64::MAX - 3, i64::MAX - 6, i64::MAX, i64::MAX - 3));

        let mut r_max_col = RectI64::of(i64::MAX - 6, i64::MAX - 3, i64::MAX - 3, i64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_max_col, u64::MAX - 1), None);
        assert_eq!(r_max_col, RectI64::of(i64::MAX - 6, i64::MAX - 3, i64::MAX - 3, i64::MAX));
    }

    #[test]
    fn try_checked_resize_assign_big_rect_limits_out_of_bounds() {
        let mut r_odd_1 = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1);
        assert_eq!(try_checked_resize_assign(&mut r_odd_1, u64::MAX), Some(()));
        assert_eq!(r_odd_1, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1));

        let mut r_odd_1 = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX);
        assert_eq!(try_checked_resize_assign(&mut r_odd_1, u64::MAX), Some(()));
        assert_eq!(r_odd_1, RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX));

        let mut r_even = RectI64::largest();
        assert_eq!(try_checked_resize_assign(&mut r_even, u64::MAX), Some(()));
        assert_eq!(r_even, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1));
    }

    #[test]
    fn try_checked_resize_odd() {
        assert_eq!(try_checked_resize(&RectI64::of(-5, -5, 5, 5), 11), Some(RectI64::of(-5, -5, 5, 5)));
        assert_eq!(try_checked_resize(&RectI64::of(-5, -5, 5, 5), 9), Some(RectI64::of(-4, -4, 4, 4)));
        assert_eq!(try_checked_resize(&RectI64::of(-4, -4, 4, 4), 7), Some(RectI64::of(-3, -3, 3, 3)));
        assert_eq!(try_checked_resize(&RectI64::of(-3, -3, 3, 3), 5), Some(RectI64::of(-2, -2, 2, 2)));
        assert_eq!(try_checked_resize(&RectI64::of(-2, -2, 2, 2), 3), Some(RectI64::of(-1, -1, 1, 1)));
        assert_eq!(try_checked_resize(&RectI64::of(-1, -1, 1, 1), 9), Some(RectI64::of(-4, -4, 4, 4)));
    }

    #[test]
    fn try_checked_resize_even() {
        assert_eq!(try_checked_resize(&RectI64::of(-5, -5, 4, 4), 10), Some(RectI64::of(-5, -5, 4, 4)));
        assert_eq!(try_checked_resize(&RectI64::of(-5, -5, 4, 4), 8), Some(RectI64::of(-4, -4, 3, 3)));
        assert_eq!(try_checked_resize(&RectI64::of(-4, -4, 3, 3), 6), Some(RectI64::of(-3, -3, 2, 2)));
        assert_eq!(try_checked_resize(&RectI64::of(-3, -3, 2, 2), 4), Some(RectI64::of(-2, -2, 1, 1)));
        assert_eq!(try_checked_resize(&RectI64::of(-2, -2, 1, 1), 8), Some(RectI64::of(-4, -4, 3, 3)));
    }

    #[test]
    fn try_checked_resize_small_size() {
        let r = RectI64::of(10, 10, 20, 20);
        assert_eq!(try_checked_resize(&r, 0), None);
        assert_eq!(try_checked_resize(&r, 1), None);
        assert_eq!(try_checked_resize(&r, 2), None);
    }

    #[test]
    fn try_checked_resize_same_size() {
        assert_eq!(try_checked_resize(&RectI64::of(10, 10, 20, 20), 11), Some(RectI64::of(10, 10, 20, 20)));
        assert_eq!(try_checked_resize(&RectI64::of(10, 10, 21, 21), 12), Some(RectI64::of(10, 10, 21, 21)));
        assert_eq!(try_checked_resize(&RectI64::of(9, 9, 21, 21), 13), Some(RectI64::of(9, 9, 21, 21)));
    }

    #[test]
    fn try_checked_resize_odd_small_rect_same_size() {
        assert_eq!(
            try_checked_resize(&RectI64::of(i64::MIN, i64::MIN, i64::MIN + 2, i64::MIN + 2), 3),
            Some(RectI64::of(i64::MIN, i64::MIN, i64::MIN + 2, i64::MIN + 2))
        );
        assert_eq!(
            try_checked_resize(&RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX), 3),
            Some(RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX))
        );
    }

    #[test]
    fn try_checked_resize_even_small_rect_same_size() {
        assert_eq!(
            try_checked_resize(&RectI64::of(i64::MIN, i64::MIN, i64::MIN + 3, i64::MIN + 3), 4),
            Some(RectI64::of(i64::MIN, i64::MIN, i64::MIN + 3, i64::MIN + 3))
        );
        assert_eq!(
            try_checked_resize(&RectI64::of(i64::MAX - 3, i64::MAX - 3, i64::MAX, i64::MAX), 4),
            Some(RectI64::of(i64::MAX - 3, i64::MAX - 3, i64::MAX, i64::MAX))
        );
    }

    #[test]
    fn try_checked_resize_odd_small_rect_to_bounds() {
        assert_eq!(
            try_checked_resize(&RectI64::of(i64::MIN + 2, i64::MIN + 2, i64::MIN + 4, i64::MIN + 4), 7),
            Some(RectI64::of(i64::MIN, i64::MIN, i64::MIN + 6, i64::MIN + 6))
        );
        assert_eq!(
            try_checked_resize(&RectI64::of(i64::MAX - 4, i64::MAX - 4, i64::MAX - 2, i64::MAX - 2), 7),
            Some(RectI64::of(i64::MAX - 6, i64::MAX - 6, i64::MAX, i64::MAX))
        );
    }

    #[test]
    fn try_checked_resize_even_small_rect_to_bounds() {
        assert_eq!(
            try_checked_resize(&RectI64::of(i64::MIN + 2, i64::MIN + 2, i64::MIN + 5, i64::MIN + 5), 8),
            Some(RectI64::of(i64::MIN, i64::MIN, i64::MIN + 7, i64::MIN + 7))
        );
        assert_eq!(
            try_checked_resize(&RectI64::of(i64::MAX - 5, i64::MAX - 5, i64::MAX - 2, i64::MAX - 2), 8),
            Some(RectI64::of(i64::MAX - 7, i64::MAX - 7, i64::MAX, i64::MAX))
        );
    }

    #[test]
    fn try_checked_resize_odd_small_rect_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectI64::of(i64::MIN, i64::MIN, i64::MIN + 2, i64::MIN + 2), 5), None);
        assert_eq!(try_checked_resize(&RectI64::of(i64::MIN, i64::MIN + 2, i64::MIN + 2, i64::MIN + 4), 5), None);
        assert_eq!(try_checked_resize(&RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 4, i64::MIN + 2), 5), None);
        assert_eq!(try_checked_resize(&RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX), 5), None);
        assert_eq!(try_checked_resize(&RectI64::of(i64::MAX - 2, i64::MAX - 4, i64::MAX, i64::MAX - 2), 5), None);
        assert_eq!(try_checked_resize(&RectI64::of(i64::MAX - 4, i64::MAX - 2, i64::MAX - 2, i64::MAX), 5), None);
    }

    #[test]
    fn try_checked_resize_even_small_rect_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectI64::of(i64::MIN, i64::MIN, i64::MIN + 3, i64::MIN + 3), 6), None);
        assert_eq!(try_checked_resize(&RectI64::of(i64::MIN, i64::MIN + 3, i64::MIN + 3, i64::MIN + 6), 6), None);
        assert_eq!(try_checked_resize(&RectI64::of(i64::MIN, i64::MIN, i64::MIN + 3, i64::MIN + 3), 6), None);
        assert_eq!(try_checked_resize(&RectI64::of(i64::MAX - 3, i64::MAX - 3, i64::MAX, i64::MAX), 6), None);
        assert_eq!(try_checked_resize(&RectI64::of(i64::MAX - 3, i64::MAX - 6, i64::MAX, i64::MAX - 3), 6), None);
        assert_eq!(try_checked_resize(&RectI64::of(i64::MAX - 6, i64::MAX - 3, i64::MAX - 3, i64::MAX), 6), None);
    }

    #[test]
    fn try_checked_resize_odd_small_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectI64::of(i64::MIN, i64::MIN, i64::MIN + 2, i64::MIN + 2), u64::MAX), None);
        assert_eq!(try_checked_resize(&RectI64::of(i64::MIN, i64::MIN + 2, i64::MIN + 2, i64::MIN + 4), u64::MAX), None);
        assert_eq!(try_checked_resize(&RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 4, i64::MIN + 2), u64::MAX), None);
        assert_eq!(try_checked_resize(&RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX), u64::MAX), None);
        assert_eq!(try_checked_resize(&RectI64::of(i64::MAX - 2, i64::MAX - 4, i64::MAX, i64::MAX - 2), u64::MAX), None);
        assert_eq!(try_checked_resize(&RectI64::of(i64::MAX - 4, i64::MAX - 2, i64::MAX - 2, i64::MAX), u64::MAX), None);
    }

    #[test]
    fn try_checked_resize_even_small_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_resize(&RectI64::of(i64::MIN, i64::MIN, i64::MIN + 3, i64::MIN + 3), u64::MAX - 1), None);
        assert_eq!(try_checked_resize(&RectI64::of(i64::MIN, i64::MIN + 3, i64::MIN + 3, i64::MIN + 6), u64::MAX - 1), None);
        assert_eq!(try_checked_resize(&RectI64::of(i64::MIN, i64::MIN, i64::MIN + 3, i64::MIN + 3), u64::MAX - 1), None);
        assert_eq!(try_checked_resize(&RectI64::of(i64::MAX - 3, i64::MAX - 3, i64::MAX, i64::MAX), u64::MAX - 1), None);
        assert_eq!(try_checked_resize(&RectI64::of(i64::MAX - 3, i64::MAX - 6, i64::MAX, i64::MAX - 3), u64::MAX - 1), None);
        assert_eq!(try_checked_resize(&RectI64::of(i64::MAX - 6, i64::MAX - 3, i64::MAX - 3, i64::MAX), u64::MAX - 1), None);
    }

    #[test]
    fn try_checked_resize_big_rect_limits_out_of_bounds() {
        assert_eq!(
            try_checked_resize(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1), u64::MAX),
            Some(RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1))
        );
        assert_eq!(
            try_checked_resize(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX), u64::MAX),
            Some(RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX))
        );
        assert_eq!(try_checked_resize(&RectI64::largest(), u64::MAX), Some(RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1)));
    }

    #[test]
    fn checked_resize_assign_odd() {
        let mut r = RectI64::of(-5, -5, 5, 5);
        checked_resize_assign(&mut r, 11);
        assert_eq!(r, RectI64::of(-5, -5, 5, 5));
        checked_resize_assign(&mut r, 9);
        assert_eq!(r, RectI64::of(-4, -4, 4, 4));
        checked_resize_assign(&mut r, 7);
        assert_eq!(r, RectI64::of(-3, -3, 3, 3));
        checked_resize_assign(&mut r, 5);
        assert_eq!(r, RectI64::of(-2, -2, 2, 2));
        checked_resize_assign(&mut r, 3);
        assert_eq!(r, RectI64::of(-1, -1, 1, 1));
        checked_resize_assign(&mut r, 9);
        assert_eq!(r, RectI64::of(-4, -4, 4, 4));
    }

    #[test]
    fn checked_resize_assign_even() {
        let mut r = RectI64::of(-5, -5, 4, 4);
        checked_resize_assign(&mut r, 10);
        assert_eq!(r, RectI64::of(-5, -5, 4, 4));
        checked_resize_assign(&mut r, 8);
        assert_eq!(r, RectI64::of(-4, -4, 3, 3));
        checked_resize_assign(&mut r, 6);
        assert_eq!(r, RectI64::of(-3, -3, 2, 2));
        checked_resize_assign(&mut r, 4);
        assert_eq!(r, RectI64::of(-2, -2, 1, 1));
        checked_resize_assign(&mut r, 8);
        assert_eq!(r, RectI64::of(-4, -4, 3, 3));
    }

    #[test]
    fn checked_resize_odd() {
        assert_eq!(checked_resize(&RectI64::of(-5, -5, 5, 5), 11), RectI64::of(-5, -5, 5, 5));
        assert_eq!(checked_resize(&RectI64::of(-5, -5, 5, 5), 9), RectI64::of(-4, -4, 4, 4));
        assert_eq!(checked_resize(&RectI64::of(-4, -4, 4, 4), 7), RectI64::of(-3, -3, 3, 3));
        assert_eq!(checked_resize(&RectI64::of(-3, -3, 3, 3), 5), RectI64::of(-2, -2, 2, 2));
        assert_eq!(checked_resize(&RectI64::of(-2, -2, 2, 2), 3), RectI64::of(-1, -1, 1, 1));
        assert_eq!(checked_resize(&RectI64::of(-1, -1, 1, 1), 9), RectI64::of(-4, -4, 4, 4));
    }

    #[test]
    fn checked_resize_even() {
        assert_eq!(checked_resize(&RectI64::of(-5, -5, 4, 4), 10), RectI64::of(-5, -5, 4, 4));
        assert_eq!(checked_resize(&RectI64::of(-5, -5, 4, 4), 8), RectI64::of(-4, -4, 3, 3));
        assert_eq!(checked_resize(&RectI64::of(-4, -4, 3, 3), 6), RectI64::of(-3, -3, 2, 2));
        assert_eq!(checked_resize(&RectI64::of(-3, -3, 2, 2), 4), RectI64::of(-2, -2, 1, 1));
        assert_eq!(checked_resize(&RectI64::of(-2, -2, 1, 1), 8), RectI64::of(-4, -4, 3, 3));
    }
}
