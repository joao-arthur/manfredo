use crate::matrix::{
    point::point_u8::PointU8,
    rect::rect_u8::{RectU8, delta_col, delta_row},
};

pub fn try_assign_resize(r: &mut RectU8, size: u8) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_row = i16::from(delta_row(r)) + 1 - i16::from(size);
    let diff_col = i16::from(delta_col(r)) + 1 - i16::from(size);
    let temp_min_row = i16::from(r.min.row) + diff_row / 2;
    let temp_min_col = i16::from(r.min.col) + diff_col / 2;
    let min_row = temp_min_row.clamp(0, i16::from(u8::MAX) - i16::from(size) + 1);
    let min_col = temp_min_col.clamp(0, i16::from(u8::MAX) - i16::from(size) + 1);
    r.min.row = min_row as u8;
    r.min.col = min_col as u8;
    r.max.row = (min_row + i16::from(size) - 1) as u8;
    r.max.col = (min_col + i16::from(size) - 1) as u8;
    Some(())
}

pub fn try_resize(r: &RectU8, size: u8) -> Option<RectU8> {
    if size < 3 {
        return None;
    }
    let diff_row = i16::from(delta_row(r)) + 1 - i16::from(size);
    let diff_col = i16::from(delta_col(r)) + 1 - i16::from(size);
    let temp_min_row = i16::from(r.min.row) + diff_row / 2;
    let temp_min_col = i16::from(r.min.col) + diff_col / 2;
    let clamped_min_row = temp_min_row.clamp(0, i16::from(u8::MAX) - i16::from(size) + 1);
    let clamped_min_col = temp_min_col.clamp(0, i16::from(u8::MAX) - i16::from(size) + 1);
    let min_row = clamped_min_row as u8;
    let min_col = clamped_min_col as u8;
    let max_row = (clamped_min_row + i16::from(size) - 1) as u8;
    let max_col = (clamped_min_col + i16::from(size) - 1) as u8;
    Some(RectU8 { min: PointU8 { row: min_row, col: min_col }, max: PointU8 { row: max_row, col: max_col } })
}

pub fn assign_resize(r: &mut RectU8, size: u8) {
    try_assign_resize(r, size).unwrap()
}

pub fn resize(r: &RectU8, size: u8) -> RectU8 {
    try_resize(r, size).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{assign_resize, resize, try_assign_resize, try_resize};
    use crate::matrix::rect::rect_u8::RectU8;

    #[test]
    fn try_assign_resize_odd() {
        let mut r = RectU8::of(5, 5, 15, 15);
        assert_eq!(try_assign_resize(&mut r, 11), Some(()));
        assert_eq!(r, RectU8::of(5, 5, 15, 15));
        assert_eq!(try_assign_resize(&mut r, 9), Some(()));
        assert_eq!(r, RectU8::of(6, 6, 14, 14));
        assert_eq!(try_assign_resize(&mut r, 7), Some(()));
        assert_eq!(r, RectU8::of(7, 7, 13, 13));
        assert_eq!(try_assign_resize(&mut r, 5), Some(()));
        assert_eq!(r, RectU8::of(8, 8, 12, 12));
        assert_eq!(try_assign_resize(&mut r, 3), Some(()));
        assert_eq!(r, RectU8::of(9, 9, 11, 11));
        assert_eq!(try_assign_resize(&mut r, 9), Some(()));
        assert_eq!(r, RectU8::of(6, 6, 14, 14));
    }

    #[test]
    fn try_assign_resize_even() {
        let mut r = RectU8::of(5, 5, 14, 14);
        assert_eq!(try_assign_resize(&mut r, 10), Some(()));
        assert_eq!(r, RectU8::of(5, 5, 14, 14));
        assert_eq!(try_assign_resize(&mut r, 8), Some(()));
        assert_eq!(r, RectU8::of(6, 6, 13, 13));
        assert_eq!(try_assign_resize(&mut r, 6), Some(()));
        assert_eq!(r, RectU8::of(7, 7, 12, 12));
        assert_eq!(try_assign_resize(&mut r, 4), Some(()));
        assert_eq!(r, RectU8::of(8, 8, 11, 11));
        assert_eq!(try_assign_resize(&mut r, 8), Some(()));
        assert_eq!(r, RectU8::of(6, 6, 13, 13));
    }

    #[test]
    fn try_assign_resize_small_size() {
        let mut r = RectU8::of(10, 10, 20, 20);
        assert_eq!(try_assign_resize(&mut r, 0), None);
        assert_eq!(try_assign_resize(&mut r, 1), None);
        assert_eq!(try_assign_resize(&mut r, 2), None);
        assert_eq!(r, RectU8::of(10, 10, 20, 20));
    }

    #[test]
    fn try_assign_resize_same_size() {
        let mut r_11 = RectU8::of(10, 10, 20, 20);
        assert_eq!(try_assign_resize(&mut r_11, 11), Some(()));
        assert_eq!(r_11, RectU8::of(10, 10, 20, 20));

        let mut r_12 = RectU8::of(10, 10, 21, 21);
        assert_eq!(try_assign_resize(&mut r_12, 12), Some(()));
        assert_eq!(r_12, RectU8::of(10, 10, 21, 21));

        let mut r_13 = RectU8::of(9, 9, 21, 21);
        assert_eq!(try_assign_resize(&mut r_13, 13), Some(()));
        assert_eq!(r_13, RectU8::of(9, 9, 21, 21));
    }

    #[test]
    fn try_assign_resize_odd_small_rect_out_of_bounds() {
        let mut r_min = RectU8::of(0, 0, 2, 2);
        assert_eq!(try_assign_resize(&mut r_min, 11), Some(()));
        assert_eq!(r_min, RectU8::of(0, 0, 10, 10));

        let mut r_max = RectU8::of(u8::MAX - 2, u8::MAX - 2, u8::MAX, u8::MAX);
        assert_eq!(try_assign_resize(&mut r_max, 11), Some(()));
        assert_eq!(r_max, RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX, u8::MAX));
    }

    #[test]
    fn try_assign_resize_even_small_rect_out_of_bounds() {
        let mut r = RectU8::of(0, 0, 3, 3);
        assert_eq!(try_assign_resize(&mut r, 11), Some(()));
        assert_eq!(r, RectU8::of(0, 0, 10, 10));

        let mut r = RectU8::of(u8::MAX - 3, u8::MAX - 3, u8::MAX, u8::MAX);
        assert_eq!(try_assign_resize(&mut r, 11), Some(()));
        assert_eq!(r, RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX, u8::MAX));
    }

    #[test]
    fn try_assign_resize_odd_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU8::of(0, 0, 2, 2);
        assert_eq!(try_assign_resize(&mut r_min, u8::MAX), Some(()));
        assert_eq!(r_min, RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1));

        let mut r_max = RectU8::of(u8::MAX - 2, u8::MAX - 2, u8::MAX, u8::MAX);
        assert_eq!(try_assign_resize(&mut r_max, u8::MAX), Some(()));
        assert_eq!(r_max, RectU8::of(1, 1, u8::MAX, u8::MAX));
    }

    #[test]
    fn try_assign_resize_even_small_rect_limits_out_of_bounds() {
        let mut r = RectU8::of(0, 0, 3, 3);
        assert_eq!(try_assign_resize(&mut r, u8::MAX - 1), Some(()));
        assert_eq!(r, RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 2));

        let mut r = RectU8::of(u8::MAX - 3, u8::MAX - 3, u8::MAX, u8::MAX);
        assert_eq!(try_assign_resize(&mut r, u8::MAX - 1), Some(()));
        assert_eq!(r, RectU8::of(2, 2, u8::MAX, u8::MAX));
    }

    #[test]
    fn try_assign_resize_big_rect_limits_out_of_bounds() {
        let mut r_odd_1 = RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1);
        assert_eq!(try_assign_resize(&mut r_odd_1, u8::MAX), Some(()));
        assert_eq!(r_odd_1, RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1));

        let mut r_odd_1 = RectU8::of(1, 1, u8::MAX, u8::MAX);
        assert_eq!(try_assign_resize(&mut r_odd_1, u8::MAX), Some(()));
        assert_eq!(r_odd_1, RectU8::of(1, 1, u8::MAX, u8::MAX));

        let mut r_even = RectU8::largest();
        assert_eq!(try_assign_resize(&mut r_even, u8::MAX), Some(()));
        assert_eq!(r_even, RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1));
    }

    #[test]
    fn try_resize_odd() {
        assert_eq!(try_resize(&RectU8::of(5, 5, 15, 15), 11), Some(RectU8::of(5, 5, 15, 15)));
        assert_eq!(try_resize(&RectU8::of(5, 5, 15, 15), 9), Some(RectU8::of(6, 6, 14, 14)));
        assert_eq!(try_resize(&RectU8::of(6, 6, 14, 14), 7), Some(RectU8::of(7, 7, 13, 13)));
        assert_eq!(try_resize(&RectU8::of(7, 7, 13, 13), 5), Some(RectU8::of(8, 8, 12, 12)));
        assert_eq!(try_resize(&RectU8::of(8, 8, 12, 12), 3), Some(RectU8::of(9, 9, 11, 11)));
        assert_eq!(try_resize(&RectU8::of(9, 9, 11, 11), 9), Some(RectU8::of(6, 6, 14, 14)));
    }

    #[test]
    fn try_resize_even() {
        assert_eq!(try_resize(&RectU8::of(5, 5, 14, 14), 10), Some(RectU8::of(5, 5, 14, 14)));
        assert_eq!(try_resize(&RectU8::of(5, 5, 14, 14), 8), Some(RectU8::of(6, 6, 13, 13)));
        assert_eq!(try_resize(&RectU8::of(6, 6, 13, 13), 6), Some(RectU8::of(7, 7, 12, 12)));
        assert_eq!(try_resize(&RectU8::of(7, 7, 12, 12), 4), Some(RectU8::of(8, 8, 11, 11)));
        assert_eq!(try_resize(&RectU8::of(8, 8, 11, 11), 8), Some(RectU8::of(6, 6, 13, 13)));
    }

    #[test]
    fn try_resize_small_size() {
        let r = RectU8::of(10, 10, 20, 20);
        assert_eq!(try_resize(&r, 0), None);
        assert_eq!(try_resize(&r, 1), None);
        assert_eq!(try_resize(&r, 2), None);
    }

    #[test]
    fn try_resize_same_size() {
        assert_eq!(try_resize(&RectU8::of(10, 10, 20, 20), 11), Some(RectU8::of(10, 10, 20, 20)));
        assert_eq!(try_resize(&RectU8::of(10, 10, 21, 21), 12), Some(RectU8::of(10, 10, 21, 21)));
        assert_eq!(try_resize(&RectU8::of(9, 9, 21, 21), 13), Some(RectU8::of(9, 9, 21, 21)));
    }

    #[test]
    fn try_resize_odd_small_rect_out_of_bounds() {
        assert_eq!(try_resize(&RectU8::of(0, 0, 2, 2), 11), Some(RectU8::of(0, 0, 10, 10)));
        assert_eq!(
            try_resize(&RectU8::of(u8::MAX - 2, u8::MAX - 2, u8::MAX, u8::MAX), 11),
            Some(RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX, u8::MAX))
        );
    }

    #[test]
    fn try_resize_even_small_rect_out_of_bounds() {
        assert_eq!(try_resize(&RectU8::of(0, 0, 3, 3), 11), Some(RectU8::of(0, 0, 10, 10)));
        assert_eq!(
            try_resize(&RectU8::of(u8::MAX - 3, u8::MAX - 3, u8::MAX, u8::MAX), 11),
            Some(RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX, u8::MAX))
        );
    }

    #[test]
    fn try_resize_odd_small_rect_limits_out_of_bounds() {
        assert_eq!(try_resize(&RectU8::of(0, 0, 2, 2), u8::MAX), Some(RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1)));
        assert_eq!(try_resize(&RectU8::of(u8::MAX - 2, u8::MAX - 2, u8::MAX, u8::MAX), u8::MAX), Some(RectU8::of(1, 1, u8::MAX, u8::MAX)));
    }

    #[test]
    fn try_resize_even_small_rect_limits_out_of_bounds() {
        assert_eq!(try_resize(&RectU8::of(0, 0, 3, 3), u8::MAX - 1), Some(RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 2)));
        assert_eq!(try_resize(&RectU8::of(u8::MAX - 3, u8::MAX - 3, u8::MAX, u8::MAX), u8::MAX - 1), Some(RectU8::of(2, 2, u8::MAX, u8::MAX)));
    }

    #[test]
    fn try_resize_big_rect_limits_out_of_bounds() {
        assert_eq!(try_resize(&RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1), u8::MAX), Some(RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1)));
        assert_eq!(try_resize(&RectU8::of(1, 1, u8::MAX, u8::MAX), u8::MAX), Some(RectU8::of(1, 1, u8::MAX, u8::MAX)));
        assert_eq!(try_resize(&RectU8::largest(), u8::MAX), Some(RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1)));
    }

    #[test]
    fn assign_resize_odd() {
        let mut r = RectU8::of(5, 5, 15, 15);
        assign_resize(&mut r, 11);
        assert_eq!(r, RectU8::of(5, 5, 15, 15));
        assign_resize(&mut r, 9);
        assert_eq!(r, RectU8::of(6, 6, 14, 14));
        assign_resize(&mut r, 7);
        assert_eq!(r, RectU8::of(7, 7, 13, 13));
        assign_resize(&mut r, 5);
        assert_eq!(r, RectU8::of(8, 8, 12, 12));
        assign_resize(&mut r, 3);
        assert_eq!(r, RectU8::of(9, 9, 11, 11));
        assign_resize(&mut r, 9);
        assert_eq!(r, RectU8::of(6, 6, 14, 14));
    }

    #[test]
    fn assign_resize_even() {
        let mut r = RectU8::of(5, 5, 14, 14);
        assign_resize(&mut r, 10);
        assert_eq!(r, RectU8::of(5, 5, 14, 14));
        assign_resize(&mut r, 8);
        assert_eq!(r, RectU8::of(6, 6, 13, 13));
        assign_resize(&mut r, 6);
        assert_eq!(r, RectU8::of(7, 7, 12, 12));
        assign_resize(&mut r, 4);
        assert_eq!(r, RectU8::of(8, 8, 11, 11));
        assign_resize(&mut r, 8);
        assert_eq!(r, RectU8::of(6, 6, 13, 13));
    }

    #[test]
    fn resize_odd() {
        assert_eq!(resize(&RectU8::of(5, 5, 15, 15), 11), RectU8::of(5, 5, 15, 15));
        assert_eq!(resize(&RectU8::of(5, 5, 15, 15), 9), RectU8::of(6, 6, 14, 14));
        assert_eq!(resize(&RectU8::of(6, 6, 14, 14), 7), RectU8::of(7, 7, 13, 13));
        assert_eq!(resize(&RectU8::of(7, 7, 13, 13), 5), RectU8::of(8, 8, 12, 12));
        assert_eq!(resize(&RectU8::of(8, 8, 12, 12), 3), RectU8::of(9, 9, 11, 11));
        assert_eq!(resize(&RectU8::of(9, 9, 11, 11), 9), RectU8::of(6, 6, 14, 14));
    }

    #[test]
    fn resize_even() {
        assert_eq!(resize(&RectU8::of(5, 5, 14, 14), 10), RectU8::of(5, 5, 14, 14));
        assert_eq!(resize(&RectU8::of(5, 5, 14, 14), 8), RectU8::of(6, 6, 13, 13));
        assert_eq!(resize(&RectU8::of(6, 6, 13, 13), 6), RectU8::of(7, 7, 12, 12));
        assert_eq!(resize(&RectU8::of(7, 7, 12, 12), 4), RectU8::of(8, 8, 11, 11));
        assert_eq!(resize(&RectU8::of(8, 8, 11, 11), 8), RectU8::of(6, 6, 13, 13));
    }
}
