use crate::matrix::{
    point::point_i16::PointI16,
    rect::rect_i16::{RectI16, delta_row, delta_col},
};

pub fn try_assign_resize(r: &mut RectI16, size: u16) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_row = i32::from(delta_row(r)) + 1 - i32::from(size);
    let diff_col = i32::from(delta_col(r)) + 1 - i32::from(size);
    let temp_min_row = i32::from(r.min.row) + diff_row / 2;
    let temp_min_col = i32::from(r.min.col) + diff_col / 2;
    let min_row = temp_min_row.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(size) + 1);
    let min_col = temp_min_col.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(size) + 1);
    r.min.row = min_row as i16;
    r.min.col = min_col as i16;
    r.max.row = (min_row + i32::from(size) - 1) as i16;
    r.max.col = (min_col + i32::from(size) - 1) as i16;
    Some(())
}

pub fn try_resize(r: &RectI16, size: u16) -> Option<RectI16> {
    if size < 3 {
        return None;
    }
    let diff_row = i32::from(delta_row(r)) + 1 - i32::from(size);
    let diff_col = i32::from(delta_col(r)) + 1 - i32::from(size);
    let temp_min_row = i32::from(r.min.row) + diff_row / 2;
    let temp_min_col = i32::from(r.min.col) + diff_col / 2;
    let clamped_min_row = temp_min_row.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(size) + 1);
    let clamped_min_col = temp_min_col.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(size) + 1);
    let min_row = clamped_min_row as i16;
    let min_col = clamped_min_col as i16;
    let max_row = (clamped_min_row + i32::from(size) - 1) as i16;
    let max_col = (clamped_min_col + i32::from(size) - 1) as i16;
    Some(RectI16 { min: PointI16 { row: min_row, col: min_col }, max: PointI16 { row: max_row, col: max_col } })
}

pub fn assign_resize(r: &mut RectI16, size: u16) {
    try_assign_resize(r, size).unwrap()
}

pub fn resize(r: &RectI16, size: u16) -> RectI16 {
    try_resize(r, size).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::matrix::rect::rect_i16::RectI16;

    use super::{assign_resize, resize, try_assign_resize, try_resize};

    #[test]
    fn try_assign_resize_odd() {
        let mut r = RectI16::of(-5, -5, 5, 5);
        assert_eq!(try_assign_resize(&mut r, 11), Some(()));
        assert_eq!(r, RectI16::of(-5, -5, 5, 5));
        assert_eq!(try_assign_resize(&mut r, 9), Some(()));
        assert_eq!(r, RectI16::of(-4, -4, 4, 4));
        assert_eq!(try_assign_resize(&mut r, 7), Some(()));
        assert_eq!(r, RectI16::of(-3, -3, 3, 3));
        assert_eq!(try_assign_resize(&mut r, 5), Some(()));
        assert_eq!(r, RectI16::of(-2, -2, 2, 2));
        assert_eq!(try_assign_resize(&mut r, 3), Some(()));
        assert_eq!(r, RectI16::of(-1, -1, 1, 1));
        assert_eq!(try_assign_resize(&mut r, 9), Some(()));
        assert_eq!(r, RectI16::of(-4, -4, 4, 4));
    }

    #[test]
    fn try_assign_resize_even() {
        let mut r = RectI16::of(-5, -5, 4, 4);
        assert_eq!(try_assign_resize(&mut r, 10), Some(()));
        assert_eq!(r, RectI16::of(-5, -5, 4, 4));
        assert_eq!(try_assign_resize(&mut r, 8), Some(()));
        assert_eq!(r, RectI16::of(-4, -4, 3, 3));
        assert_eq!(try_assign_resize(&mut r, 6), Some(()));
        assert_eq!(r, RectI16::of(-3, -3, 2, 2));
        assert_eq!(try_assign_resize(&mut r, 4), Some(()));
        assert_eq!(r, RectI16::of(-2, -2, 1, 1));
        assert_eq!(try_assign_resize(&mut r, 8), Some(()));
        assert_eq!(r, RectI16::of(-4, -4, 3, 3));
    }

    #[test]
    fn try_assign_resize_small_size() {
        let mut r = RectI16::of(10, 10, 20, 20);
        assert_eq!(try_assign_resize(&mut r, 0), None);
        assert_eq!(try_assign_resize(&mut r, 1), None);
        assert_eq!(try_assign_resize(&mut r, 2), None);
        assert_eq!(r, RectI16::of(10, 10, 20, 20));
    }

    #[test]
    fn try_assign_resize_same_size() {
        let mut r_11 = RectI16::of(10, 10, 20, 20);
        assert_eq!(try_assign_resize(&mut r_11, 11), Some(()));
        assert_eq!(r_11, RectI16::of(10, 10, 20, 20));

        let mut r_12 = RectI16::of(10, 10, 21, 21);
        assert_eq!(try_assign_resize(&mut r_12, 12), Some(()));
        assert_eq!(r_12, RectI16::of(10, 10, 21, 21));

        let mut r_13 = RectI16::of(9, 9, 21, 21);
        assert_eq!(try_assign_resize(&mut r_13, 13), Some(()));
        assert_eq!(r_13, RectI16::of(9, 9, 21, 21));
    }

    #[test]
    fn try_assign_resize_odd_small_rect_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN, i16::MIN, i16::MIN + 2, i16::MIN + 2);
        assert_eq!(try_assign_resize(&mut r_min, 11), Some(()));
        assert_eq!(r_min, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 10));

        let mut r_max = RectI16::of(i16::MAX - 2, i16::MAX - 2, i16::MAX, i16::MAX);
        assert_eq!(try_assign_resize(&mut r_max, 11), Some(()));
        assert_eq!(r_max, RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX, i16::MAX));
    }

    #[test]
    fn try_assign_resize_even_small_rect_out_of_bounds() {
        let mut r = RectI16::of(i16::MIN, i16::MIN, i16::MIN + 3, i16::MIN + 3);
        assert_eq!(try_assign_resize(&mut r, 11), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 10));

        let mut r = RectI16::of(i16::MAX - 3, i16::MAX - 3, i16::MAX, i16::MAX);
        assert_eq!(try_assign_resize(&mut r, 11), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX, i16::MAX));
    }

    #[test]
    fn try_assign_resize_odd_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN, i16::MIN, i16::MIN + 2, i16::MIN + 2);
        assert_eq!(try_assign_resize(&mut r_min, u16::MAX), Some(()));
        assert_eq!(r_min, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1));

        let mut r_max = RectI16::of(i16::MAX - 2, i16::MAX - 2, i16::MAX, i16::MAX);
        assert_eq!(try_assign_resize(&mut r_max, u16::MAX), Some(()));
        assert_eq!(r_max, RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX));
    }

    #[test]
    fn try_assign_resize_even_small_rect_limits_out_of_bounds() {
        let mut r = RectI16::of(i16::MIN, i16::MIN, i16::MIN + 3, i16::MIN + 3);
        assert_eq!(try_assign_resize(&mut r, u16::MAX - 1), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 2));

        let mut r = RectI16::of(i16::MAX - 3, i16::MAX - 3, i16::MAX, i16::MAX);
        assert_eq!(try_assign_resize(&mut r, u16::MAX - 1), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN + 2, i16::MIN + 2, i16::MAX, i16::MAX));
    }

    #[test]
    fn try_assign_resize_big_rect_limits_out_of_bounds() {
        let mut r_odd_1 = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1);
        assert_eq!(try_assign_resize(&mut r_odd_1, u16::MAX), Some(()));
        assert_eq!(r_odd_1, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1));

        let mut r_odd_1 = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX);
        assert_eq!(try_assign_resize(&mut r_odd_1, u16::MAX), Some(()));
        assert_eq!(r_odd_1, RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX));

        let mut r_even = RectI16::largest();
        assert_eq!(try_assign_resize(&mut r_even, u16::MAX), Some(()));
        assert_eq!(r_even, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1));
    }

    #[test]
    fn try_resize_odd() {
        assert_eq!(try_resize(&RectI16::of(-5, -5, 5, 5), 11), Some(RectI16::of(-5, -5, 5, 5)));
        assert_eq!(try_resize(&RectI16::of(-5, -5, 5, 5), 9), Some(RectI16::of(-4, -4, 4, 4)));
        assert_eq!(try_resize(&RectI16::of(-4, -4, 4, 4), 7), Some(RectI16::of(-3, -3, 3, 3)));
        assert_eq!(try_resize(&RectI16::of(-3, -3, 3, 3), 5), Some(RectI16::of(-2, -2, 2, 2)));
        assert_eq!(try_resize(&RectI16::of(-2, -2, 2, 2), 3), Some(RectI16::of(-1, -1, 1, 1)));
        assert_eq!(try_resize(&RectI16::of(-1, -1, 1, 1), 9), Some(RectI16::of(-4, -4, 4, 4)));
    }

    #[test]
    fn try_resize_even() {
        assert_eq!(try_resize(&RectI16::of(-5, -5, 4, 4), 10), Some(RectI16::of(-5, -5, 4, 4)));
        assert_eq!(try_resize(&RectI16::of(-5, -5, 4, 4), 8), Some(RectI16::of(-4, -4, 3, 3)));
        assert_eq!(try_resize(&RectI16::of(-4, -4, 3, 3), 6), Some(RectI16::of(-3, -3, 2, 2)));
        assert_eq!(try_resize(&RectI16::of(-3, -3, 2, 2), 4), Some(RectI16::of(-2, -2, 1, 1)));
        assert_eq!(try_resize(&RectI16::of(-2, -2, 1, 1), 8), Some(RectI16::of(-4, -4, 3, 3)));
    }

    #[test]
    fn try_resize_small_size() {
        let r = RectI16::of(10, 10, 20, 20);
        assert_eq!(try_resize(&r, 0), None);
        assert_eq!(try_resize(&r, 1), None);
        assert_eq!(try_resize(&r, 2), None);
    }

    #[test]
    fn try_resize_same_size() {
        assert_eq!(try_resize(&RectI16::of(10, 10, 20, 20), 11), Some(RectI16::of(10, 10, 20, 20)));
        assert_eq!(try_resize(&RectI16::of(10, 10, 21, 21), 12), Some(RectI16::of(10, 10, 21, 21)));
        assert_eq!(try_resize(&RectI16::of(9, 9, 21, 21), 13), Some(RectI16::of(9, 9, 21, 21)));
    }

    #[test]
    fn try_resize_odd_small_rect_out_of_bounds() {
        assert_eq!(
            try_resize(&RectI16::of(i16::MIN, i16::MIN, i16::MIN + 2, i16::MIN + 2), 11),
            Some(RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 10))
        );
        assert_eq!(
            try_resize(&RectI16::of(i16::MAX - 2, i16::MAX - 2, i16::MAX, i16::MAX), 11),
            Some(RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX, i16::MAX))
        );
    }

    #[test]
    fn try_resize_even_small_rect_out_of_bounds() {
        assert_eq!(
            try_resize(&RectI16::of(i16::MIN, i16::MIN, i16::MIN + 3, i16::MIN + 3), 11),
            Some(RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 10))
        );
        assert_eq!(
            try_resize(&RectI16::of(i16::MAX - 3, i16::MAX - 3, i16::MAX, i16::MAX), 11),
            Some(RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX, i16::MAX))
        );
    }

    #[test]
    fn try_resize_odd_small_rect_limits_out_of_bounds() {
        assert_eq!(
            try_resize(&RectI16::of(i16::MIN, i16::MIN, i16::MIN + 2, i16::MIN + 2), u16::MAX),
            Some(RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1))
        );
        assert_eq!(
            try_resize(&RectI16::of(i16::MAX - 2, i16::MAX - 2, i16::MAX, i16::MAX), u16::MAX),
            Some(RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX))
        );
    }

    #[test]
    fn try_resize_even_small_rect_limits_out_of_bounds() {
        assert_eq!(
            try_resize(&RectI16::of(i16::MIN, i16::MIN, i16::MIN + 3, i16::MIN + 3), u16::MAX - 1),
            Some(RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 2))
        );
        assert_eq!(
            try_resize(&RectI16::of(i16::MAX - 3, i16::MAX - 3, i16::MAX, i16::MAX), u16::MAX - 1),
            Some(RectI16::of(i16::MIN + 2, i16::MIN + 2, i16::MAX, i16::MAX))
        );
    }

    #[test]
    fn try_resize_big_rect_limits_out_of_bounds() {
        assert_eq!(
            try_resize(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1), u16::MAX),
            Some(RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1))
        );
        assert_eq!(
            try_resize(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX), u16::MAX),
            Some(RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX))
        );
        assert_eq!(try_resize(&RectI16::largest(), u16::MAX), Some(RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1)));
    }

    #[test]
    fn assign_resize_odd() {
        let mut r = RectI16::of(-5, -5, 5, 5);
        assign_resize(&mut r, 11);
        assert_eq!(r, RectI16::of(-5, -5, 5, 5));
        assign_resize(&mut r, 9);
        assert_eq!(r, RectI16::of(-4, -4, 4, 4));
        assign_resize(&mut r, 7);
        assert_eq!(r, RectI16::of(-3, -3, 3, 3));
        assign_resize(&mut r, 5);
        assert_eq!(r, RectI16::of(-2, -2, 2, 2));
        assign_resize(&mut r, 3);
        assert_eq!(r, RectI16::of(-1, -1, 1, 1));
        assign_resize(&mut r, 9);
        assert_eq!(r, RectI16::of(-4, -4, 4, 4));
    }

    #[test]
    fn assign_resize_even() {
        let mut r = RectI16::of(-5, -5, 4, 4);
        assign_resize(&mut r, 10);
        assert_eq!(r, RectI16::of(-5, -5, 4, 4));
        assign_resize(&mut r, 8);
        assert_eq!(r, RectI16::of(-4, -4, 3, 3));
        assign_resize(&mut r, 6);
        assert_eq!(r, RectI16::of(-3, -3, 2, 2));
        assign_resize(&mut r, 4);
        assert_eq!(r, RectI16::of(-2, -2, 1, 1));
        assign_resize(&mut r, 8);
        assert_eq!(r, RectI16::of(-4, -4, 3, 3));
    }

    #[test]
    fn resize_odd() {
        assert_eq!(resize(&RectI16::of(-5, -5, 5, 5), 11), RectI16::of(-5, -5, 5, 5));
        assert_eq!(resize(&RectI16::of(-5, -5, 5, 5), 9), RectI16::of(-4, -4, 4, 4));
        assert_eq!(resize(&RectI16::of(-4, -4, 4, 4), 7), RectI16::of(-3, -3, 3, 3));
        assert_eq!(resize(&RectI16::of(-3, -3, 3, 3), 5), RectI16::of(-2, -2, 2, 2));
        assert_eq!(resize(&RectI16::of(-2, -2, 2, 2), 3), RectI16::of(-1, -1, 1, 1));
        assert_eq!(resize(&RectI16::of(-1, -1, 1, 1), 9), RectI16::of(-4, -4, 4, 4));
    }

    #[test]
    fn resize_even() {
        assert_eq!(resize(&RectI16::of(-5, -5, 4, 4), 10), RectI16::of(-5, -5, 4, 4));
        assert_eq!(resize(&RectI16::of(-5, -5, 4, 4), 8), RectI16::of(-4, -4, 3, 3));
        assert_eq!(resize(&RectI16::of(-4, -4, 3, 3), 6), RectI16::of(-3, -3, 2, 2));
        assert_eq!(resize(&RectI16::of(-3, -3, 2, 2), 4), RectI16::of(-2, -2, 1, 1));
        assert_eq!(resize(&RectI16::of(-2, -2, 1, 1), 8), RectI16::of(-4, -4, 3, 3));
    }
}
