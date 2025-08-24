use crate::matrix::{
    point::point_u64::PointU64,
    rect::{rect_i64::RectI64, rect_u64::RectU64},
};

pub fn try_assign_add(r: &mut RectU64, delta: &RectI64) -> Option<()> {
    let min_row = r.min.row.checked_add_signed(delta.min.row)?;
    let min_col = r.min.col.checked_add_signed(delta.min.col)?;
    let max_row = r.max.row.checked_add_signed(delta.max.row)?;
    let max_col = r.max.col.checked_add_signed(delta.max.col)?;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Some(())
}

pub fn try_add(r: &RectU64, delta: &RectI64) -> Option<RectU64> {
    let min_row = r.min.row.checked_add_signed(delta.min.row)?;
    let min_col = r.min.col.checked_add_signed(delta.min.col)?;
    let max_row = r.max.row.checked_add_signed(delta.max.row)?;
    let max_col = r.max.col.checked_add_signed(delta.max.col)?;
    Some(RectU64 { min: PointU64 { row: min_row, col: min_col }, max: PointU64 { row: max_row, col: max_col } })
}

pub fn assign_add(r: &mut RectU64, delta: &RectI64) {
    try_assign_add(r, delta).unwrap()
}

pub fn add(r: &RectU64, delta: &RectI64) -> RectU64 {
    try_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::matrix::rect::{rect_i64::RectI64, rect_u64::RectU64};

    use super::{add, assign_add, try_add, try_assign_add};

    #[test]
    fn test_try_assign_add() {
        let mut r = RectU64::of(0, 0, 12, 15);
        assert_eq!(try_assign_add(&mut r, &RectI64::of(5, 4, 3, 2)), Some(()));
        assert_eq!(r, RectU64::of(5, 4, 15, 17));
        assert_eq!(try_assign_add(&mut r, &RectI64::of(-4, -3, -2, -1)), Some(()));
        assert_eq!(r, RectU64::of(1, 1, 13, 16));
    }

    #[test]
    fn try_assign_add_small_rect_to_bounds() {
        let mut min_r = RectU64::of(2, 5, 12, 15);
        assert_eq!(try_assign_add(&mut min_r, &RectI64::of(-2, -5, 10, 20)), Some(()));
        assert_eq!(min_r, RectU64::of(0, 0, 22, 35));

        let mut max_r = RectU64::of(u64::MAX - 12, u64::MAX - 15, u64::MAX - 2, u64::MAX - 5);
        assert_eq!(try_assign_add(&mut max_r, &RectI64::of(-1, -4, 2, 5)), Some(()));
        assert_eq!(max_r, RectU64::of(u64::MAX - 13, u64::MAX - 19, u64::MAX, u64::MAX));
    }

    #[test]
    fn try_assign_add_big_rect_to_bounds() {
        let mut min_r = RectU64::of(2, 5, u64::MAX - 2, u64::MAX - 5);
        assert_eq!(try_assign_add(&mut min_r, &RectI64::of(-2, -5, 2, 5)), Some(()));
        assert_eq!(min_r, RectU64::largest());

        let mut min_r = RectU64::of(2, 5, u64::MAX, u64::MAX);
        assert_eq!(try_assign_add(&mut min_r, &RectI64::of(-2, -5, 0, 0)), Some(()));
        assert_eq!(min_r, RectU64::largest());

        let mut max_r = RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5);
        assert_eq!(try_assign_add(&mut max_r, &RectI64::of(0, 0, 2, 5)), Some(()));
        assert_eq!(max_r, RectU64::largest());
    }

    #[test]
    fn try_assign_add_out_of_bounds() {
        let mut r_min = RectU64::largest();
        assert_eq!(try_assign_add(&mut r_min, &RectI64::of(-1, -1, 1, 1)), None);
        assert_eq!(r_min, RectU64::largest());
    }

    #[test]
    fn try_assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectU64::of(10, 5, 20, 30);
        assert_eq!(try_assign_add(&mut r_min, &RectI64::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectU64::of(10, 5, 20, 30));

        let mut r_max = RectU64::of(u64::MAX - 20, u64::MAX - 30, u64::MAX - 5, u64::MAX - 10);
        assert_eq!(try_assign_add(&mut r_max, &RectI64::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectU64::of(u64::MAX - 20, u64::MAX - 30, u64::MAX - 5, u64::MAX - 10));
    }

    #[test]
    fn try_assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectU64::of(10, 5, u64::MAX, u64::MAX);
        assert_eq!(try_assign_add(&mut r_min, &RectI64::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectU64::of(10, 5, u64::MAX, u64::MAX));

        let mut r_max = RectU64::of(0, 0, u64::MAX - 5, u64::MAX - 10);
        assert_eq!(try_assign_add(&mut r_max, &RectI64::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectU64::of(0, 0, u64::MAX - 5, u64::MAX - 10));
    }

    #[test]
    fn try_assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU64::of(1, 1, 10, 10);
        assert_eq!(try_assign_add(&mut r_min, &RectI64::min()), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI64::of(i64::MIN, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI64::of(0, i64::MIN, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI64::of(0, 0, i64::MIN, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI64::of(0, 0, 0, i64::MIN)), None);
        assert_eq!(r_min, RectU64::of(1, 1, 10, 10));

        let mut r_max = RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1);
        assert_eq!(try_assign_add(&mut r_max, &RectI64::max()), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI64::of(i64::MAX, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI64::of(0, i64::MAX, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI64::of(0, 0, i64::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI64::of(0, 0, 0, i64::MAX)), None);
        assert_eq!(r_max, RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1));
    }

    #[test]
    fn try_assign_add_big_rect_limits_out_of_bounds() {
        let mut r = RectU64::largest();
        assert_eq!(try_assign_add(&mut r, &RectI64::largest()), None);
        assert_eq!(try_assign_add(&mut r, &RectI64::min()), None);
        assert_eq!(try_assign_add(&mut r, &RectI64::max()), None);
        assert_eq!(r, RectU64::largest());

        let mut r_min = RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1);
        assert_eq!(try_assign_add(&mut r_min, &RectI64::max()), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI64::of(0, 0, i64::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI64::of(0, 0, 0, i64::MAX)), None);
        assert_eq!(r_min, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));

        let mut r_max = RectU64::of(1, 1, u64::MAX, u64::MAX);
        assert_eq!(try_assign_add(&mut r_max, &RectI64::min()), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI64::of(i64::MIN, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI64::of(0, i64::MIN, 0, 0)), None);
        assert_eq!(r_max, RectU64::of(1, 1, u64::MAX, u64::MAX));
    }

    #[test]
    fn test_try_add() {
        assert_eq!(try_add(&RectU64::of(0, 0, 12, 15), &RectI64::of(5, 4, 3, 2)), Some(RectU64::of(5, 4, 15, 17)));
        assert_eq!(try_add(&RectU64::of(5, 4, 15, 17), &RectI64::of(-4, -3, -2, -1)), Some(RectU64::of(1, 1, 13, 16)));
    }

    #[test]
    fn try_add_small_rect_to_bounds() {
        assert_eq!(try_add(&RectU64::of(2, 5, 12, 15), &RectI64::of(-2, -5, 10, 20)), Some(RectU64::of(0, 0, 22, 35)));
        assert_eq!(
            try_add(&RectU64::of(u64::MAX - 12, u64::MAX - 15, u64::MAX - 2, u64::MAX - 5), &RectI64::of(-1, -4, 2, 5)),
            Some(RectU64::of(u64::MAX - 13, u64::MAX - 19, u64::MAX, u64::MAX))
        );
    }

    #[test]
    fn try_add_big_rect_to_bounds() {
        assert_eq!(try_add(&RectU64::of(2, 5, u64::MAX - 2, u64::MAX - 5), &RectI64::of(-2, -5, 2, 5)), Some(RectU64::largest()));
        assert_eq!(try_add(&RectU64::of(2, 5, u64::MAX, u64::MAX), &RectI64::of(-2, -5, 0, 0)), Some(RectU64::largest()));
        assert_eq!(try_add(&RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5), &RectI64::of(0, 0, 2, 5)), Some(RectU64::largest()));
    }

    #[test]
    fn try_add_small_rect_out_of_bounds() {
        let mut r_min = RectU64::of(10, 5, 20, 30);
        assert_eq!(try_add(&mut r_min, &RectI64::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectU64::of(10, 5, 20, 30));

        let mut r_max = RectU64::of(u64::MAX - 20, u64::MAX - 30, u64::MAX - 5, u64::MAX - 10);
        assert_eq!(try_add(&mut r_max, &RectI64::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectU64::of(u64::MAX - 20, u64::MAX - 30, u64::MAX - 5, u64::MAX - 10));
    }

    #[test]
    fn try_add_big_rect_out_of_bounds() {
        let mut r_min = RectU64::of(10, 5, u64::MAX, u64::MAX);
        assert_eq!(try_add(&mut r_min, &RectI64::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectU64::of(10, 5, u64::MAX, u64::MAX));

        let mut r_max = RectU64::of(0, 0, u64::MAX - 5, u64::MAX - 10);
        assert_eq!(try_add(&mut r_max, &RectI64::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectU64::of(0, 0, u64::MAX - 5, u64::MAX - 10));
    }

    #[test]
    fn try_add_small_rect_limits_out_of_bounds() {
        let r_min = RectU64::of(1, 1, 10, 10);
        assert_eq!(try_add(&r_min, &RectI64::min()), None);
        assert_eq!(try_add(&r_min, &RectI64::of(i64::MIN, 0, 0, 0)), None);
        assert_eq!(try_add(&r_min, &RectI64::of(0, i64::MIN, 0, 0)), None);
        assert_eq!(try_add(&r_min, &RectI64::of(0, 0, i64::MIN, 0)), None);
        assert_eq!(try_add(&r_min, &RectI64::of(0, 0, 0, i64::MIN)), None);

        let r_max = RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1);
        assert_eq!(try_add(&r_max, &RectI64::max()), None);
        assert_eq!(try_add(&r_max, &RectI64::of(i64::MAX, 0, 0, 0)), None);
        assert_eq!(try_add(&r_max, &RectI64::of(0, i64::MAX, 0, 0)), None);
        assert_eq!(try_add(&r_max, &RectI64::of(0, 0, i64::MAX, 0)), None);
        assert_eq!(try_add(&r_max, &RectI64::of(0, 0, 0, i64::MAX)), None);
    }

    #[test]
    fn try_add_big_rect_limits_out_of_bounds() {
        let r = RectU64::largest();
        assert_eq!(try_add(&r, &RectI64::largest()), None);
        assert_eq!(try_add(&r, &RectI64::min()), None);
        assert_eq!(try_add(&r, &RectI64::max()), None);

        let r_min = RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1);
        assert_eq!(try_add(&r_min, &RectI64::max()), None);
        assert_eq!(try_add(&r_min, &RectI64::of(0, 0, i64::MAX, 0)), None);
        assert_eq!(try_add(&r_min, &RectI64::of(0, 0, 0, i64::MAX)), None);

        let r_max = RectU64::of(1, 1, u64::MAX, u64::MAX);
        assert_eq!(try_add(&r_max, &RectI64::min()), None);
        assert_eq!(try_add(&r_max, &RectI64::of(i64::MIN, 0, 0, 0)), None);
        assert_eq!(try_add(&r_max, &RectI64::of(0, i64::MIN, 0, 0)), None);
    }

    #[test]
    fn test_assign_add() {
        let mut r = RectU64::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI64::of(5, 4, 3, 2));
        assert_eq!(r, RectU64::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI64::of(-4, -3, -2, -1));
        assert_eq!(r, RectU64::of(1, 1, 13, 16));
    }

    #[test]
    fn test_add() {
        let mut r = RectU64::of(0, 0, 12, 15);
        assert_eq!(add(&mut r, &RectI64::of(5, 4, 3, 2)), RectU64::of(5, 4, 15, 17));
        assert_eq!(add(&RectU64::of(5, 4, 15, 17), &RectI64::of(-4, -3, -2, -1)), RectU64::of(1, 1, 13, 16));
    }
}
