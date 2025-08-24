use crate::matrix::{
    point::point_u32::PointU32,
    rect::{rect_i32::RectI32, rect_u32::RectU32},
};

pub fn try_assign_add(r: &mut RectU32, delta: &RectI32) -> Option<()> {
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

pub fn try_add(r: &RectU32, delta: &RectI32) -> Option<RectU32> {
    let min_row = r.min.row.checked_add_signed(delta.min.row)?;
    let min_col = r.min.col.checked_add_signed(delta.min.col)?;
    let max_row = r.max.row.checked_add_signed(delta.max.row)?;
    let max_col = r.max.col.checked_add_signed(delta.max.col)?;
    Some(RectU32 { min: PointU32 { row: min_row, col: min_col }, max: PointU32 { row: max_row, col: max_col } })
}

pub fn assign_add(r: &mut RectU32, delta: &RectI32) {
    try_assign_add(r, delta).unwrap()
}

pub fn add(r: &RectU32, delta: &RectI32) -> RectU32 {
    try_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::matrix::rect::{rect_i32::RectI32, rect_u32::RectU32};

    use super::{add, assign_add, try_add, try_assign_add};

    #[test]
    fn test_try_assign_add() {
        let mut r = RectU32::of(0, 0, 12, 15);
        assert_eq!(try_assign_add(&mut r, &RectI32::of(5, 4, 3, 2)), Some(()));
        assert_eq!(r, RectU32::of(5, 4, 15, 17));
        assert_eq!(try_assign_add(&mut r, &RectI32::of(-4, -3, -2, -1)), Some(()));
        assert_eq!(r, RectU32::of(1, 1, 13, 16));
    }

    #[test]
    fn try_assign_add_small_rect_to_bounds() {
        let mut min_r = RectU32::of(2, 5, 12, 15);
        assert_eq!(try_assign_add(&mut min_r, &RectI32::of(-2, -5, 10, 20)), Some(()));
        assert_eq!(min_r, RectU32::of(0, 0, 22, 35));

        let mut max_r = RectU32::of(u32::MAX - 12, u32::MAX - 15, u32::MAX - 2, u32::MAX - 5);
        assert_eq!(try_assign_add(&mut max_r, &RectI32::of(-1, -4, 2, 5)), Some(()));
        assert_eq!(max_r, RectU32::of(u32::MAX - 13, u32::MAX - 19, u32::MAX, u32::MAX));
    }

    #[test]
    fn try_assign_add_big_rect_to_bounds() {
        let mut min_r = RectU32::of(2, 5, u32::MAX - 2, u32::MAX - 5);
        assert_eq!(try_assign_add(&mut min_r, &RectI32::of(-2, -5, 2, 5)), Some(()));
        assert_eq!(min_r, RectU32::largest());

        let mut min_r = RectU32::of(2, 5, u32::MAX, u32::MAX);
        assert_eq!(try_assign_add(&mut min_r, &RectI32::of(-2, -5, 0, 0)), Some(()));
        assert_eq!(min_r, RectU32::largest());

        let mut max_r = RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5);
        assert_eq!(try_assign_add(&mut max_r, &RectI32::of(0, 0, 2, 5)), Some(()));
        assert_eq!(max_r, RectU32::largest());
    }

    #[test]
    fn try_assign_add_out_of_bounds() {
        let mut r_min = RectU32::largest();
        assert_eq!(try_assign_add(&mut r_min, &RectI32::of(-1, -1, 1, 1)), None);
        assert_eq!(r_min, RectU32::largest());
    }

    #[test]
    fn try_assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectU32::of(10, 5, 20, 30);
        assert_eq!(try_assign_add(&mut r_min, &RectI32::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectU32::of(10, 5, 20, 30));

        let mut r_max = RectU32::of(u32::MAX - 20, u32::MAX - 30, u32::MAX - 5, u32::MAX - 10);
        assert_eq!(try_assign_add(&mut r_max, &RectI32::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectU32::of(u32::MAX - 20, u32::MAX - 30, u32::MAX - 5, u32::MAX - 10));
    }

    #[test]
    fn try_assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectU32::of(10, 5, u32::MAX, u32::MAX);
        assert_eq!(try_assign_add(&mut r_min, &RectI32::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectU32::of(10, 5, u32::MAX, u32::MAX));

        let mut r_max = RectU32::of(0, 0, u32::MAX - 5, u32::MAX - 10);
        assert_eq!(try_assign_add(&mut r_max, &RectI32::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectU32::of(0, 0, u32::MAX - 5, u32::MAX - 10));
    }

    #[test]
    fn try_assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU32::of(1, 1, 10, 10);
        assert_eq!(try_assign_add(&mut r_min, &RectI32::min()), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI32::of(i32::MIN, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI32::of(0, i32::MIN, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI32::of(0, 0, i32::MIN, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI32::of(0, 0, 0, i32::MIN)), None);
        assert_eq!(r_min, RectU32::of(1, 1, 10, 10));

        let mut r_max = RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_assign_add(&mut r_max, &RectI32::max()), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI32::of(i32::MAX, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI32::of(0, i32::MAX, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI32::of(0, 0, i32::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI32::of(0, 0, 0, i32::MAX)), None);
        assert_eq!(r_max, RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1));
    }

    #[test]
    fn try_assign_add_big_rect_limits_out_of_bounds() {
        let mut r = RectU32::largest();
        assert_eq!(try_assign_add(&mut r, &RectI32::largest()), None);
        assert_eq!(try_assign_add(&mut r, &RectI32::min()), None);
        assert_eq!(try_assign_add(&mut r, &RectI32::max()), None);
        assert_eq!(r, RectU32::largest());

        let mut r_min = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_assign_add(&mut r_min, &RectI32::max()), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI32::of(0, 0, i32::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI32::of(0, 0, 0, i32::MAX)), None);
        assert_eq!(r_min, RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1));

        let mut r_max = RectU32::of(1, 1, u32::MAX, u32::MAX);
        assert_eq!(try_assign_add(&mut r_max, &RectI32::min()), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI32::of(i32::MIN, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI32::of(0, i32::MIN, 0, 0)), None);
        assert_eq!(r_max, RectU32::of(1, 1, u32::MAX, u32::MAX));
    }

    #[test]
    fn test_try_add() {
        assert_eq!(try_add(&RectU32::of(0, 0, 12, 15), &RectI32::of(5, 4, 3, 2)), Some(RectU32::of(5, 4, 15, 17)));
        assert_eq!(try_add(&RectU32::of(5, 4, 15, 17), &RectI32::of(-4, -3, -2, -1)), Some(RectU32::of(1, 1, 13, 16)));
    }

    #[test]
    fn try_add_small_rect_to_bounds() {
        assert_eq!(try_add(&RectU32::of(2, 5, 12, 15), &RectI32::of(-2, -5, 10, 20)), Some(RectU32::of(0, 0, 22, 35)));
        assert_eq!(
            try_add(&RectU32::of(u32::MAX - 12, u32::MAX - 15, u32::MAX - 2, u32::MAX - 5), &RectI32::of(-1, -4, 2, 5)),
            Some(RectU32::of(u32::MAX - 13, u32::MAX - 19, u32::MAX, u32::MAX))
        );
    }

    #[test]
    fn try_add_big_rect_to_bounds() {
        assert_eq!(try_add(&RectU32::of(2, 5, u32::MAX - 2, u32::MAX - 5), &RectI32::of(-2, -5, 2, 5)), Some(RectU32::largest()));
        assert_eq!(try_add(&RectU32::of(2, 5, u32::MAX, u32::MAX), &RectI32::of(-2, -5, 0, 0)), Some(RectU32::largest()));
        assert_eq!(try_add(&RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5), &RectI32::of(0, 0, 2, 5)), Some(RectU32::largest()));
    }

    #[test]
    fn try_add_small_rect_out_of_bounds() {
        let mut r_min = RectU32::of(10, 5, 20, 30);
        assert_eq!(try_add(&mut r_min, &RectI32::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectU32::of(10, 5, 20, 30));

        let mut r_max = RectU32::of(u32::MAX - 20, u32::MAX - 30, u32::MAX - 5, u32::MAX - 10);
        assert_eq!(try_add(&mut r_max, &RectI32::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectU32::of(u32::MAX - 20, u32::MAX - 30, u32::MAX - 5, u32::MAX - 10));
    }

    #[test]
    fn try_add_big_rect_out_of_bounds() {
        let mut r_min = RectU32::of(10, 5, u32::MAX, u32::MAX);
        assert_eq!(try_add(&mut r_min, &RectI32::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectU32::of(10, 5, u32::MAX, u32::MAX));

        let mut r_max = RectU32::of(0, 0, u32::MAX - 5, u32::MAX - 10);
        assert_eq!(try_add(&mut r_max, &RectI32::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectU32::of(0, 0, u32::MAX - 5, u32::MAX - 10));
    }

    #[test]
    fn try_add_small_rect_limits_out_of_bounds() {
        let r_min = RectU32::of(1, 1, 10, 10);
        assert_eq!(try_add(&r_min, &RectI32::min()), None);
        assert_eq!(try_add(&r_min, &RectI32::of(i32::MIN, 0, 0, 0)), None);
        assert_eq!(try_add(&r_min, &RectI32::of(0, i32::MIN, 0, 0)), None);
        assert_eq!(try_add(&r_min, &RectI32::of(0, 0, i32::MIN, 0)), None);
        assert_eq!(try_add(&r_min, &RectI32::of(0, 0, 0, i32::MIN)), None);

        let r_max = RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_add(&r_max, &RectI32::max()), None);
        assert_eq!(try_add(&r_max, &RectI32::of(i32::MAX, 0, 0, 0)), None);
        assert_eq!(try_add(&r_max, &RectI32::of(0, i32::MAX, 0, 0)), None);
        assert_eq!(try_add(&r_max, &RectI32::of(0, 0, i32::MAX, 0)), None);
        assert_eq!(try_add(&r_max, &RectI32::of(0, 0, 0, i32::MAX)), None);
    }

    #[test]
    fn try_add_big_rect_limits_out_of_bounds() {
        let r = RectU32::largest();
        assert_eq!(try_add(&r, &RectI32::largest()), None);
        assert_eq!(try_add(&r, &RectI32::min()), None);
        assert_eq!(try_add(&r, &RectI32::max()), None);

        let r_min = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_add(&r_min, &RectI32::max()), None);
        assert_eq!(try_add(&r_min, &RectI32::of(0, 0, i32::MAX, 0)), None);
        assert_eq!(try_add(&r_min, &RectI32::of(0, 0, 0, i32::MAX)), None);

        let r_max = RectU32::of(1, 1, u32::MAX, u32::MAX);
        assert_eq!(try_add(&r_max, &RectI32::min()), None);
        assert_eq!(try_add(&r_max, &RectI32::of(i32::MIN, 0, 0, 0)), None);
        assert_eq!(try_add(&r_max, &RectI32::of(0, i32::MIN, 0, 0)), None);
    }

    #[test]
    fn test_assign_add() {
        let mut r = RectU32::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI32::of(5, 4, 3, 2));
        assert_eq!(r, RectU32::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI32::of(-4, -3, -2, -1));
        assert_eq!(r, RectU32::of(1, 1, 13, 16));
    }

    #[test]
    fn test_add() {
        let mut r = RectU32::of(0, 0, 12, 15);
        assert_eq!(add(&mut r, &RectI32::of(5, 4, 3, 2)), RectU32::of(5, 4, 15, 17));
        assert_eq!(add(&RectU32::of(5, 4, 15, 17), &RectI32::of(-4, -3, -2, -1)), RectU32::of(1, 1, 13, 16));
    }
}
