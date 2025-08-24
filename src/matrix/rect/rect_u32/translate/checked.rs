use crate::matrix::{
    point::{point_i32::PointI32, point_u32::PointU32},
    rect::rect_u32::RectU32,
};

pub fn try_assign_translate(r: &mut RectU32, delta: &PointI32) -> Option<()> {
    let min_row = r.min.row.checked_add_signed(delta.row)?;
    let min_col = r.min.col.checked_add_signed(delta.col)?;
    let max_row = r.max.row.checked_add_signed(delta.row)?;
    let max_col = r.max.col.checked_add_signed(delta.col)?;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Some(())
}

pub fn try_translate(r: &RectU32, delta: &PointI32) -> Option<RectU32> {
    let min_row = r.min.row.checked_add_signed(delta.row)?;
    let min_col = r.min.col.checked_add_signed(delta.col)?;
    let max_row = r.max.row.checked_add_signed(delta.row)?;
    let max_col = r.max.col.checked_add_signed(delta.col)?;
    Some(RectU32 { min: PointU32 { row: min_row, col: min_col }, max: PointU32 { row: max_row, col: max_col } })
}

pub fn assign_translate(r: &mut RectU32, delta: &PointI32) {
    try_assign_translate(r, delta).unwrap()
}

pub fn translate(r: &RectU32, delta: &PointI32) -> RectU32 {
    try_translate(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{assign_translate, translate, try_assign_translate, try_translate};
    use crate::matrix::{point::point_i32::PointI32, rect::rect_u32::RectU32};

    #[test]
    fn test_try_assign_translate() {
        let mut r = RectU32::of(0, 0, 12, 15);
        assert_eq!(try_assign_translate(&mut r, &PointI32::of(5, 4)), Some(()));
        assert_eq!(r, RectU32::of(5, 4, 17, 19));
        assert_eq!(try_assign_translate(&mut r, &PointI32::of(-4, -2)), Some(()));
        assert_eq!(r, RectU32::of(1, 2, 13, 17));
    }

    #[test]
    fn try_assign_translate_small_rect_to_bounds() {
        let mut min_r = RectU32::of(2, 5, 12, 15);
        assert_eq!(try_assign_translate(&mut min_r, &PointI32::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectU32::of(0, 0, 10, 10));

        let mut max_r = RectU32::of(u32::MAX - 12, u32::MAX - 15, u32::MAX - 2, u32::MAX - 5);
        assert_eq!(try_assign_translate(&mut max_r, &PointI32::of(2, 5)), Some(()));
        assert_eq!(max_r, RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX, u32::MAX));
    }

    #[test]
    fn try_assign_translate_big_rect_to_bounds() {
        let mut min_r = RectU32::of(2, 5, u32::MAX, u32::MAX);
        assert_eq!(try_assign_translate(&mut min_r, &PointI32::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5));

        let mut max_r = RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5);
        assert_eq!(try_assign_translate(&mut max_r, &PointI32::of(2, 5)), Some(()));
        assert_eq!(max_r, RectU32::of(2, 5, u32::MAX, u32::MAX));
    }

    #[test]
    fn try_assign_translate_small_rect_out_of_bounds() {
        let mut r_min = RectU32::of(10, 5, 20, 30);
        assert_eq!(try_assign_translate(&mut r_min, &PointI32::of(-20, -20)), None);
        assert_eq!(r_min, RectU32::of(10, 5, 20, 30));

        let mut r_max = RectU32::of(u32::MAX - 20, u32::MAX - 30, u32::MAX - 5, u32::MAX - 10);
        assert_eq!(try_assign_translate(&mut r_max, &PointI32::of(20, 20)), None);
        assert_eq!(r_max, RectU32::of(u32::MAX - 20, u32::MAX - 30, u32::MAX - 5, u32::MAX - 10));
    }

    #[test]
    fn try_assign_translate_big_rect_out_of_bounds() {
        let mut r_min = RectU32::of(10, 5, u32::MAX, u32::MAX);
        assert_eq!(try_assign_translate(&mut r_min, &PointI32::of(-20, -20)), None);
        assert_eq!(r_min, RectU32::of(10, 5, u32::MAX, u32::MAX));

        let mut r_max = RectU32::of(0, 0, u32::MAX - 5, u32::MAX - 10);
        assert_eq!(try_assign_translate(&mut r_max, &PointI32::of(20, 20)), None);
        assert_eq!(r_max, RectU32::of(0, 0, u32::MAX - 5, u32::MAX - 10));
    }

    #[test]
    fn try_assign_translate_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU32::of(1, 1, 10, 10);
        assert_eq!(try_assign_translate(&mut r_min, &PointI32::min()), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointI32::of(0, i32::MIN)), None);
        assert_eq!(r_min, RectU32::of(1, 1, 10, 10));

        let mut r_max = RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_assign_translate(&mut r_max, &PointI32::max()), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointI32::of(0, i32::MAX)), None);
        assert_eq!(r_max, RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1));
    }

    #[test]
    fn try_assign_translate_big_rect_limits_out_of_bounds() {
        let mut r = RectU32::largest();
        assert_eq!(try_assign_translate(&mut r, &PointI32::min()), None);
        assert_eq!(try_assign_translate(&mut r, &PointI32::max()), None);
        assert_eq!(r, RectU32::largest());

        let mut r_min = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_assign_translate(&mut r_min, &PointI32::max()), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointI32::of(0, i32::MAX)), None);
        assert_eq!(r_min, RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1));

        let mut r_max = RectU32::of(1, 1, u32::MAX, u32::MAX);
        assert_eq!(try_assign_translate(&mut r_max, &PointI32::min()), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointI32::of(0, i32::MIN)), None);
        assert_eq!(r_max, RectU32::of(1, 1, u32::MAX, u32::MAX));
    }

    #[test]
    fn test_try_translate() {
        assert_eq!(try_translate(&RectU32::of(0, 0, 12, 15), &PointI32::of(5, 4)), Some(RectU32::of(5, 4, 17, 19)));
        assert_eq!(try_translate(&RectU32::of(5, 4, 17, 19), &PointI32::of(-4, -2)), Some(RectU32::of(1, 2, 13, 17)));
    }

    #[test]
    fn try_translate_small_rect_to_bounds() {
        assert_eq!(try_translate(&RectU32::of(2, 5, 12, 15), &PointI32::of(-2, -5)), Some(RectU32::of(0, 0, 10, 10)));
        assert_eq!(
            try_translate(&RectU32::of(u32::MAX - 12, u32::MAX - 15, u32::MAX - 2, u32::MAX - 5), &PointI32::of(2, 5)),
            Some(RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX, u32::MAX))
        );
    }

    #[test]
    fn try_translate_big_rect_to_bounds() {
        assert_eq!(try_translate(&RectU32::of(2, 5, u32::MAX, u32::MAX), &PointI32::of(-2, -5)), Some(RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5)));
        assert_eq!(try_translate(&RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5), &PointI32::of(2, 5)), Some(RectU32::of(2, 5, u32::MAX, u32::MAX)));
    }

    #[test]
    fn try_translate_small_rect_out_of_bounds() {
        assert_eq!(try_translate(&RectU32::of(10, 5, 20, 30), &PointI32::of(-20, -20)), None);
        assert_eq!(try_translate(&RectU32::of(u32::MAX - 20, u32::MAX - 30, u32::MAX - 5, u32::MAX - 10), &PointI32::of(20, 20)), None);
    }

    #[test]
    fn try_translate_big_rect_out_of_bounds() {
        assert_eq!(try_translate(&RectU32::of(10, 5, u32::MAX, u32::MAX), &PointI32::of(-20, -20)), None);
        assert_eq!(try_translate(&RectU32::of(0, 0, u32::MAX - 5, u32::MAX - 10), &PointI32::of(20, 20)), None);
    }

    #[test]
    fn try_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(try_translate(&RectU32::of(1, 1, 10, 10), &PointI32::min()), None);
        assert_eq!(try_translate(&RectU32::of(1, 1, 10, 10), &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(try_translate(&RectU32::of(1, 1, 10, 10), &PointI32::of(0, i32::MIN)), None);
        assert_eq!(try_translate(&RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1), &PointI32::max()), None);
        assert_eq!(try_translate(&RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1), &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(try_translate(&RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1), &PointI32::of(0, i32::MAX)), None);
    }

    #[test]
    fn try_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(try_translate(&RectU32::largest(), &PointI32::min()), None);
        assert_eq!(try_translate(&RectU32::largest(), &PointI32::min()), None);
        assert_eq!(try_translate(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1), &PointI32::max()), None);
        assert_eq!(try_translate(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1), &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(try_translate(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1), &PointI32::of(0, i32::MAX)), None);
        assert_eq!(try_translate(&RectU32::of(1, 1, u32::MAX, u32::MAX), &PointI32::min()), None);
        assert_eq!(try_translate(&RectU32::of(1, 1, u32::MAX, u32::MAX), &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(try_translate(&RectU32::of(1, 1, u32::MAX, u32::MAX), &PointI32::of(0, i32::MIN)), None);
    }

    #[test]
    fn test_assign_translate() {
        let mut r = RectU32::of(0, 0, 12, 15);
        assign_translate(&mut r, &PointI32::of(5, 4));
        assert_eq!(r, RectU32::of(5, 4, 17, 19));
        assign_translate(&mut r, &PointI32::of(-4, -2));
        assert_eq!(r, RectU32::of(1, 2, 13, 17));
    }

    #[test]
    fn test_translate() {
        assert_eq!(translate(&RectU32::of(0, 0, 12, 15), &PointI32::of(5, 4)), RectU32::of(5, 4, 17, 19));
        assert_eq!(translate(&RectU32::of(5, 4, 17, 19), &PointI32::of(-4, -2)), RectU32::of(1, 2, 13, 17));
    }
}
