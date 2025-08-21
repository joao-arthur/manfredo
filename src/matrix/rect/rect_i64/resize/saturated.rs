use crate::matrix::{
    point::point_i64::PointI64,
    rect::rect_i64::{RectI64, delta_col, delta_row},
};

pub fn try_assign_resize(r: &mut RectI64, size: u64) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_row = i128::from(delta_row(r)) + 1 - i128::from(size);
    let diff_col = i128::from(delta_col(r)) + 1 - i128::from(size);
    let temp_min_row = i128::from(r.min.row) + diff_row / 2;
    let temp_min_col = i128::from(r.min.col) + diff_col / 2;
    let min_row = temp_min_row.clamp(i128::from(i64::MIN), i128::from(i64::MAX) - i128::from(size) + 1);
    let min_col = temp_min_col.clamp(i128::from(i64::MIN), i128::from(i64::MAX) - i128::from(size) + 1);
    r.min.row = min_row as i64;
    r.min.col = min_col as i64;
    r.max.row = (min_row + i128::from(size) - 1) as i64;
    r.max.col = (min_col + i128::from(size) - 1) as i64;
    Some(())
}

pub fn try_resize(r: &RectI64, size: u64) -> Option<RectI64> {
    if size < 3 {
        return None;
    }
    let diff_row = i128::from(delta_row(r)) + 1 - i128::from(size);
    let diff_col = i128::from(delta_col(r)) + 1 - i128::from(size);
    let temp_min_row = i128::from(r.min.row) + diff_row / 2;
    let temp_min_col = i128::from(r.min.col) + diff_col / 2;
    let clamped_min_row = temp_min_row.clamp(i128::from(i64::MIN), i128::from(i64::MAX) - i128::from(size) + 1);
    let clamped_min_col = temp_min_col.clamp(i128::from(i64::MIN), i128::from(i64::MAX) - i128::from(size) + 1);
    let min_row = clamped_min_row as i64;
    let min_col = clamped_min_col as i64;
    let max_row = (clamped_min_row + i128::from(size) - 1) as i64;
    let max_col = (clamped_min_col + i128::from(size) - 1) as i64;
    Some(RectI64 { min: PointI64 { row: min_row, col: min_col }, max: PointI64 { row: max_row, col: max_col } })
}

pub fn assign_resize(r: &mut RectI64, size: u64) {
    try_assign_resize(r, size).unwrap()
}

pub fn resize(r: &RectI64, size: u64) -> RectI64 {
    try_resize(r, size).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::matrix::rect::rect_i64::RectI64;

    use super::{assign_resize, resize, try_assign_resize, try_resize};

    #[test]
    fn try_assign_resize_odd() {
        let mut r = RectI64::of(-5, -5, 5, 5);
        assert_eq!(try_assign_resize(&mut r, 11), Some(()));
        assert_eq!(r, RectI64::of(-5, -5, 5, 5));
        assert_eq!(try_assign_resize(&mut r, 9), Some(()));
        assert_eq!(r, RectI64::of(-4, -4, 4, 4));
        assert_eq!(try_assign_resize(&mut r, 7), Some(()));
        assert_eq!(r, RectI64::of(-3, -3, 3, 3));
        assert_eq!(try_assign_resize(&mut r, 5), Some(()));
        assert_eq!(r, RectI64::of(-2, -2, 2, 2));
        assert_eq!(try_assign_resize(&mut r, 3), Some(()));
        assert_eq!(r, RectI64::of(-1, -1, 1, 1));
        assert_eq!(try_assign_resize(&mut r, 9), Some(()));
        assert_eq!(r, RectI64::of(-4, -4, 4, 4));
    }

    #[test]
    fn try_assign_resize_even() {
        let mut r = RectI64::of(-5, -5, 4, 4);
        assert_eq!(try_assign_resize(&mut r, 10), Some(()));
        assert_eq!(r, RectI64::of(-5, -5, 4, 4));
        assert_eq!(try_assign_resize(&mut r, 8), Some(()));
        assert_eq!(r, RectI64::of(-4, -4, 3, 3));
        assert_eq!(try_assign_resize(&mut r, 6), Some(()));
        assert_eq!(r, RectI64::of(-3, -3, 2, 2));
        assert_eq!(try_assign_resize(&mut r, 4), Some(()));
        assert_eq!(r, RectI64::of(-2, -2, 1, 1));
        assert_eq!(try_assign_resize(&mut r, 8), Some(()));
        assert_eq!(r, RectI64::of(-4, -4, 3, 3));
    }

    #[test]
    fn try_assign_resize_small_size() {
        let mut r = RectI64::of(10, 10, 20, 20);
        assert_eq!(try_assign_resize(&mut r, 0), None);
        assert_eq!(try_assign_resize(&mut r, 1), None);
        assert_eq!(try_assign_resize(&mut r, 2), None);
        assert_eq!(r, RectI64::of(10, 10, 20, 20));
    }

    #[test]
    fn try_assign_resize_same_size() {
        let mut r_11 = RectI64::of(10, 10, 20, 20);
        assert_eq!(try_assign_resize(&mut r_11, 11), Some(()));
        assert_eq!(r_11, RectI64::of(10, 10, 20, 20));

        let mut r_12 = RectI64::of(10, 10, 21, 21);
        assert_eq!(try_assign_resize(&mut r_12, 12), Some(()));
        assert_eq!(r_12, RectI64::of(10, 10, 21, 21));

        let mut r_13 = RectI64::of(9, 9, 21, 21);
        assert_eq!(try_assign_resize(&mut r_13, 13), Some(()));
        assert_eq!(r_13, RectI64::of(9, 9, 21, 21));
    }

    #[test]
    fn try_assign_resize_odd_small_rect_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN, i64::MIN, i64::MIN + 2, i64::MIN + 2);
        assert_eq!(try_assign_resize(&mut r_min, 11), Some(()));
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10));

        let mut r_max = RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX);
        assert_eq!(try_assign_resize(&mut r_max, 11), Some(()));
        assert_eq!(r_max, RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX, i64::MAX));
    }

    #[test]
    fn try_assign_resize_even_small_rect_out_of_bounds() {
        let mut r = RectI64::of(i64::MIN, i64::MIN, i64::MIN + 3, i64::MIN + 3);
        assert_eq!(try_assign_resize(&mut r, 11), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10));

        let mut r = RectI64::of(i64::MAX - 3, i64::MAX - 3, i64::MAX, i64::MAX);
        assert_eq!(try_assign_resize(&mut r, 11), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX, i64::MAX));
    }

    #[test]
    fn try_assign_resize_odd_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN, i64::MIN, i64::MIN + 2, i64::MIN + 2);
        assert_eq!(try_assign_resize(&mut r_min, u64::MAX), Some(()));
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1));

        let mut r_max = RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX);
        assert_eq!(try_assign_resize(&mut r_max, u64::MAX), Some(()));
        assert_eq!(r_max, RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX));
    }

    #[test]
    fn try_assign_resize_even_small_rect_limits_out_of_bounds() {
        let mut r = RectI64::of(i64::MIN, i64::MIN, i64::MIN + 3, i64::MIN + 3);
        assert_eq!(try_assign_resize(&mut r, u64::MAX - 1), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 2));

        let mut r = RectI64::of(i64::MAX - 3, i64::MAX - 3, i64::MAX, i64::MAX);
        assert_eq!(try_assign_resize(&mut r, u64::MAX - 1), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN + 2, i64::MIN + 2, i64::MAX, i64::MAX));
    }

    #[test]
    fn try_assign_resize_big_rect_limits_out_of_bounds() {
        let mut r_odd_1 = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1);
        assert_eq!(try_assign_resize(&mut r_odd_1, u64::MAX), Some(()));
        assert_eq!(r_odd_1, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1));

        let mut r_odd_1 = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX);
        assert_eq!(try_assign_resize(&mut r_odd_1, u64::MAX), Some(()));
        assert_eq!(r_odd_1, RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX));

        let mut r_even = RectI64::largest();
        assert_eq!(try_assign_resize(&mut r_even, u64::MAX), Some(()));
        assert_eq!(r_even, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1));
    }

    #[test]
    fn try_resize_odd() {
        assert_eq!(try_resize(&RectI64::of(-5, -5, 5, 5), 11), Some(RectI64::of(-5, -5, 5, 5)));
        assert_eq!(try_resize(&RectI64::of(-5, -5, 5, 5), 9), Some(RectI64::of(-4, -4, 4, 4)));
        assert_eq!(try_resize(&RectI64::of(-4, -4, 4, 4), 7), Some(RectI64::of(-3, -3, 3, 3)));
        assert_eq!(try_resize(&RectI64::of(-3, -3, 3, 3), 5), Some(RectI64::of(-2, -2, 2, 2)));
        assert_eq!(try_resize(&RectI64::of(-2, -2, 2, 2), 3), Some(RectI64::of(-1, -1, 1, 1)));
        assert_eq!(try_resize(&RectI64::of(-1, -1, 1, 1), 9), Some(RectI64::of(-4, -4, 4, 4)));
    }

    #[test]
    fn try_resize_even() {
        assert_eq!(try_resize(&RectI64::of(-5, -5, 4, 4), 10), Some(RectI64::of(-5, -5, 4, 4)));
        assert_eq!(try_resize(&RectI64::of(-5, -5, 4, 4), 8), Some(RectI64::of(-4, -4, 3, 3)));
        assert_eq!(try_resize(&RectI64::of(-4, -4, 3, 3), 6), Some(RectI64::of(-3, -3, 2, 2)));
        assert_eq!(try_resize(&RectI64::of(-3, -3, 2, 2), 4), Some(RectI64::of(-2, -2, 1, 1)));
        assert_eq!(try_resize(&RectI64::of(-2, -2, 1, 1), 8), Some(RectI64::of(-4, -4, 3, 3)));
    }

    #[test]
    fn try_resize_small_size() {
        let r = RectI64::of(10, 10, 20, 20);
        assert_eq!(try_resize(&r, 0), None);
        assert_eq!(try_resize(&r, 1), None);
        assert_eq!(try_resize(&r, 2), None);
    }

    #[test]
    fn try_resize_same_size() {
        assert_eq!(try_resize(&RectI64::of(10, 10, 20, 20), 11), Some(RectI64::of(10, 10, 20, 20)));
        assert_eq!(try_resize(&RectI64::of(10, 10, 21, 21), 12), Some(RectI64::of(10, 10, 21, 21)));
        assert_eq!(try_resize(&RectI64::of(9, 9, 21, 21), 13), Some(RectI64::of(9, 9, 21, 21)));
    }

    #[test]
    fn try_resize_odd_small_rect_out_of_bounds() {
        assert_eq!(
            try_resize(&RectI64::of(i64::MIN, i64::MIN, i64::MIN + 2, i64::MIN + 2), 11),
            Some(RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10))
        );
        assert_eq!(
            try_resize(&RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX), 11),
            Some(RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX, i64::MAX))
        );
    }

    #[test]
    fn try_resize_even_small_rect_out_of_bounds() {
        assert_eq!(
            try_resize(&RectI64::of(i64::MIN, i64::MIN, i64::MIN + 3, i64::MIN + 3), 11),
            Some(RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10))
        );
        assert_eq!(
            try_resize(&RectI64::of(i64::MAX - 3, i64::MAX - 3, i64::MAX, i64::MAX), 11),
            Some(RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX, i64::MAX))
        );
    }

    #[test]
    fn try_resize_odd_small_rect_limits_out_of_bounds() {
        assert_eq!(
            try_resize(&RectI64::of(i64::MIN, i64::MIN, i64::MIN + 2, i64::MIN + 2), u64::MAX),
            Some(RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1))
        );
        assert_eq!(
            try_resize(&RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX), u64::MAX),
            Some(RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX))
        );
    }

    #[test]
    fn try_resize_even_small_rect_limits_out_of_bounds() {
        assert_eq!(
            try_resize(&RectI64::of(i64::MIN, i64::MIN, i64::MIN + 3, i64::MIN + 3), u64::MAX - 1),
            Some(RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 2))
        );
        assert_eq!(
            try_resize(&RectI64::of(i64::MAX - 3, i64::MAX - 3, i64::MAX, i64::MAX), u64::MAX - 1),
            Some(RectI64::of(i64::MIN + 2, i64::MIN + 2, i64::MAX, i64::MAX))
        );
    }

    #[test]
    fn try_resize_big_rect_limits_out_of_bounds() {
        assert_eq!(
            try_resize(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1), u64::MAX),
            Some(RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1))
        );
        assert_eq!(
            try_resize(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX), u64::MAX),
            Some(RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX))
        );
        assert_eq!(try_resize(&RectI64::largest(), u64::MAX), Some(RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1)));
    }

    #[test]
    fn assign_resize_odd() {
        let mut r = RectI64::of(-5, -5, 5, 5);
        assign_resize(&mut r, 11);
        assert_eq!(r, RectI64::of(-5, -5, 5, 5));
        assign_resize(&mut r, 9);
        assert_eq!(r, RectI64::of(-4, -4, 4, 4));
        assign_resize(&mut r, 7);
        assert_eq!(r, RectI64::of(-3, -3, 3, 3));
        assign_resize(&mut r, 5);
        assert_eq!(r, RectI64::of(-2, -2, 2, 2));
        assign_resize(&mut r, 3);
        assert_eq!(r, RectI64::of(-1, -1, 1, 1));
        assign_resize(&mut r, 9);
        assert_eq!(r, RectI64::of(-4, -4, 4, 4));
    }

    #[test]
    fn assign_resize_even() {
        let mut r = RectI64::of(-5, -5, 4, 4);
        assign_resize(&mut r, 10);
        assert_eq!(r, RectI64::of(-5, -5, 4, 4));
        assign_resize(&mut r, 8);
        assert_eq!(r, RectI64::of(-4, -4, 3, 3));
        assign_resize(&mut r, 6);
        assert_eq!(r, RectI64::of(-3, -3, 2, 2));
        assign_resize(&mut r, 4);
        assert_eq!(r, RectI64::of(-2, -2, 1, 1));
        assign_resize(&mut r, 8);
        assert_eq!(r, RectI64::of(-4, -4, 3, 3));
    }

    #[test]
    fn resize_odd() {
        assert_eq!(resize(&RectI64::of(-5, -5, 5, 5), 11), RectI64::of(-5, -5, 5, 5));
        assert_eq!(resize(&RectI64::of(-5, -5, 5, 5), 9), RectI64::of(-4, -4, 4, 4));
        assert_eq!(resize(&RectI64::of(-4, -4, 4, 4), 7), RectI64::of(-3, -3, 3, 3));
        assert_eq!(resize(&RectI64::of(-3, -3, 3, 3), 5), RectI64::of(-2, -2, 2, 2));
        assert_eq!(resize(&RectI64::of(-2, -2, 2, 2), 3), RectI64::of(-1, -1, 1, 1));
        assert_eq!(resize(&RectI64::of(-1, -1, 1, 1), 9), RectI64::of(-4, -4, 4, 4));
    }

    #[test]
    fn resize_even() {
        assert_eq!(resize(&RectI64::of(-5, -5, 4, 4), 10), RectI64::of(-5, -5, 4, 4));
        assert_eq!(resize(&RectI64::of(-5, -5, 4, 4), 8), RectI64::of(-4, -4, 3, 3));
        assert_eq!(resize(&RectI64::of(-4, -4, 3, 3), 6), RectI64::of(-3, -3, 2, 2));
        assert_eq!(resize(&RectI64::of(-3, -3, 2, 2), 4), RectI64::of(-2, -2, 1, 1));
        assert_eq!(resize(&RectI64::of(-2, -2, 1, 1), 8), RectI64::of(-4, -4, 3, 3));
    }
}
