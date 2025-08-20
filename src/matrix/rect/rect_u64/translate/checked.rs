use crate::matrix::{
    point::{point_i64::PointI64, point_u64::PointU64},
    rect::rect_u64::RectU64,
};

pub fn try_assign_translate(r: &mut RectU64, delta: &PointI64) -> Option<()> {
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

pub fn try_translate(r: &RectU64, delta: &PointI64) -> Option<RectU64> {
    let min_row = r.min.row.checked_add_signed(delta.row)?;
    let min_col = r.min.col.checked_add_signed(delta.col)?;
    let max_row = r.max.row.checked_add_signed(delta.row)?;
    let max_col = r.max.col.checked_add_signed(delta.col)?;
    Some(RectU64 { min: PointU64 { row: min_row, col: min_col }, max: PointU64 { row: max_row, col: max_col } })
}

pub fn assign_translate(r: &mut RectU64, delta: &PointI64) {
    try_assign_translate(r, delta).unwrap()
}

pub fn translate(r: &RectU64, delta: &PointI64) -> RectU64 {
    try_translate(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::matrix::{point::point_i64::PointI64, rect::rect_u64::RectU64};

    use super::{assign_translate, translate, try_assign_translate, try_translate};

    #[test]
    fn test_try_assign_translate() {
        let mut r = RectU64::of(0, 0, 12, 15);
        assert_eq!(try_assign_translate(&mut r, &PointI64::of(5, 4)), Some(()));
        assert_eq!(r, RectU64::of(5, 4, 17, 19));
        assert_eq!(try_assign_translate(&mut r, &PointI64::of(-4, -2)), Some(()));
        assert_eq!(r, RectU64::of(1, 2, 13, 17));
    }

    #[test]
    fn try_assign_translate_small_rect_to_bounds() {
        let mut min_r = RectU64::of(2, 5, 12, 15);
        assert_eq!(try_assign_translate(&mut min_r, &PointI64::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectU64::of(0, 0, 10, 10));

        let mut max_r = RectU64::of(u64::MAX - 12, u64::MAX - 15, u64::MAX - 2, u64::MAX - 5);
        assert_eq!(try_assign_translate(&mut max_r, &PointI64::of(2, 5)), Some(()));
        assert_eq!(max_r, RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX));
    }

    #[test]
    fn try_assign_translate_big_rect_to_bounds() {
        let mut min_r = RectU64::of(2, 5, u64::MAX, u64::MAX);
        assert_eq!(try_assign_translate(&mut min_r, &PointI64::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5));

        let mut max_r = RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5);
        assert_eq!(try_assign_translate(&mut max_r, &PointI64::of(2, 5)), Some(()));
        assert_eq!(max_r, RectU64::of(2, 5, u64::MAX, u64::MAX));
    }

    #[test]
    fn try_assign_translate_small_rect_out_of_bounds() {
        let mut r_min = RectU64::of(10, 5, 20, 30);
        assert_eq!(try_assign_translate(&mut r_min, &PointI64::of(-20, -20)), None);
        assert_eq!(r_min, RectU64::of(10, 5, 20, 30));

        let mut r_max = RectU64::of(u64::MAX - 20, u64::MAX - 30, u64::MAX - 5, u64::MAX - 10);
        assert_eq!(try_assign_translate(&mut r_max, &PointI64::of(20, 20)), None);
        assert_eq!(r_max, RectU64::of(u64::MAX - 20, u64::MAX - 30, u64::MAX - 5, u64::MAX - 10));
    }

    #[test]
    fn try_assign_translate_big_rect_out_of_bounds() {
        let mut r_min = RectU64::of(10, 5, u64::MAX, u64::MAX);
        assert_eq!(try_assign_translate(&mut r_min, &PointI64::of(-20, -20)), None);
        assert_eq!(r_min, RectU64::of(10, 5, u64::MAX, u64::MAX));

        let mut r_max = RectU64::of(0, 0, u64::MAX - 5, u64::MAX - 10);
        assert_eq!(try_assign_translate(&mut r_max, &PointI64::of(20, 20)), None);
        assert_eq!(r_max, RectU64::of(0, 0, u64::MAX - 5, u64::MAX - 10));
    }

    #[test]
    fn try_assign_translate_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU64::of(1, 1, 10, 10);
        assert_eq!(try_assign_translate(&mut r_min, &PointI64::min()), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointI64::of(i64::MIN, 0)), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointI64::of(0, i64::MIN)), None);
        assert_eq!(r_min, RectU64::of(1, 1, 10, 10));

        let mut r_max = RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1);
        assert_eq!(try_assign_translate(&mut r_max, &PointI64::max()), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointI64::of(i64::MAX, 0)), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointI64::of(0, i64::MAX)), None);
        assert_eq!(r_max, RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1));
    }

    #[test]
    fn try_assign_translate_big_rect_limits_out_of_bounds() {
        let mut r = RectU64::largest();
        assert_eq!(try_assign_translate(&mut r, &PointI64::min()), None);
        assert_eq!(try_assign_translate(&mut r, &PointI64::max()), None);
        assert_eq!(r, RectU64::largest());

        let mut r_min = RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1);
        assert_eq!(try_assign_translate(&mut r_min, &PointI64::max()), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointI64::of(i64::MAX, 0)), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointI64::of(0, i64::MAX)), None);
        assert_eq!(r_min, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));

        let mut r_max = RectU64::of(1, 1, u64::MAX, u64::MAX);
        assert_eq!(try_assign_translate(&mut r_max, &PointI64::min()), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointI64::of(i64::MIN, 0)), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointI64::of(0, i64::MIN)), None);
        assert_eq!(r_max, RectU64::of(1, 1, u64::MAX, u64::MAX));
    }

    #[test]
    fn test_try_translate() {
        assert_eq!(try_translate(&RectU64::of(0, 0, 12, 15), &PointI64::of(5, 4)), Some(RectU64::of(5, 4, 17, 19)));
        assert_eq!(try_translate(&RectU64::of(5, 4, 17, 19), &PointI64::of(-4, -2)), Some(RectU64::of(1, 2, 13, 17)));
    }

    #[test]
    fn try_translate_small_rect_to_bounds() {
        assert_eq!(try_translate(&RectU64::of(2, 5, 12, 15), &PointI64::of(-2, -5)), Some(RectU64::of(0, 0, 10, 10)));
        assert_eq!(
            try_translate(&RectU64::of(u64::MAX - 12, u64::MAX - 15, u64::MAX - 2, u64::MAX - 5), &PointI64::of(2, 5)),
            Some(RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX))
        );
    }

    #[test]
    fn try_translate_big_rect_to_bounds() {
        assert_eq!(try_translate(&RectU64::of(2, 5, u64::MAX, u64::MAX), &PointI64::of(-2, -5)), Some(RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5)));
        assert_eq!(try_translate(&RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5), &PointI64::of(2, 5)), Some(RectU64::of(2, 5, u64::MAX, u64::MAX)));
    }

    #[test]
    fn try_translate_small_rect_out_of_bounds() {
        assert_eq!(try_translate(&RectU64::of(10, 5, 20, 30), &PointI64::of(-20, -20)), None);
        assert_eq!(try_translate(&RectU64::of(u64::MAX - 20, u64::MAX - 30, u64::MAX - 5, u64::MAX - 10), &PointI64::of(20, 20)), None);
    }

    #[test]
    fn try_translate_big_rect_out_of_bounds() {
        assert_eq!(try_translate(&RectU64::of(10, 5, u64::MAX, u64::MAX), &PointI64::of(-20, -20)), None);
        assert_eq!(try_translate(&RectU64::of(0, 0, u64::MAX - 5, u64::MAX - 10), &PointI64::of(20, 20)), None);
    }

    #[test]
    fn try_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(try_translate(&RectU64::of(1, 1, 10, 10), &PointI64::min()), None);
        assert_eq!(try_translate(&RectU64::of(1, 1, 10, 10), &PointI64::of(i64::MIN, 0)), None);
        assert_eq!(try_translate(&RectU64::of(1, 1, 10, 10), &PointI64::of(0, i64::MIN)), None);
        assert_eq!(try_translate(&RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1), &PointI64::max()), None);
        assert_eq!(try_translate(&RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1), &PointI64::of(i64::MAX, 0)), None);
        assert_eq!(try_translate(&RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1), &PointI64::of(0, i64::MAX)), None);
    }

    #[test]
    fn try_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(try_translate(&RectU64::largest(), &PointI64::min()), None);
        assert_eq!(try_translate(&RectU64::largest(), &PointI64::max()), None);
        assert_eq!(try_translate(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1), &PointI64::max()), None);
        assert_eq!(try_translate(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1), &PointI64::of(i64::MAX, 0)), None);
        assert_eq!(try_translate(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1), &PointI64::of(0, i64::MAX)), None);
        assert_eq!(try_translate(&RectU64::of(1, 1, u64::MAX, u64::MAX), &PointI64::min()), None);
        assert_eq!(try_translate(&RectU64::of(1, 1, u64::MAX, u64::MAX), &PointI64::of(i64::MIN, 0)), None);
        assert_eq!(try_translate(&RectU64::of(1, 1, u64::MAX, u64::MAX), &PointI64::of(0, i64::MIN)), None);
    }

    #[test]
    fn test_assign_translate() {
        let mut r = RectU64::of(0, 0, 12, 15);
        assign_translate(&mut r, &PointI64::of(5, 4));
        assert_eq!(r, RectU64::of(5, 4, 17, 19));
        assign_translate(&mut r, &PointI64::of(-4, -2));
        assert_eq!(r, RectU64::of(1, 2, 13, 17));
    }

    #[test]
    fn test_translate() {
        assert_eq!(translate(&RectU64::of(0, 0, 12, 15), &PointI64::of(5, 4)), RectU64::of(5, 4, 17, 19));
        assert_eq!(translate(&RectU64::of(5, 4, 17, 19), &PointI64::of(-4, -2)), RectU64::of(1, 2, 13, 17));
    }
}
