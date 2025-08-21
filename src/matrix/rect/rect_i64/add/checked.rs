use crate::matrix::{point::point_i64::PointI64, rect::rect_i64::RectI64};

pub fn try_assign_add(r: &mut RectI64, delta: &RectI64) -> Option<()> {
    let min_row = r.min.row.checked_add(delta.min.row)?;
    let min_col = r.min.col.checked_add(delta.min.col)?;
    let max_row = r.max.row.checked_add(delta.max.row)?;
    let max_col = r.max.col.checked_add(delta.max.col)?;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Some(())
}

pub fn try_add(r: &RectI64, delta: &RectI64) -> Option<RectI64> {
    let min_row = r.min.row.checked_add(delta.min.row)?;
    let min_col = r.min.col.checked_add(delta.min.col)?;
    let max_row = r.max.row.checked_add(delta.max.row)?;
    let max_col = r.max.col.checked_add(delta.max.col)?;
    Some(RectI64 { min: PointI64 { row: min_row, col: min_col }, max: PointI64 { row: max_row, col: max_col } })
}

pub fn assign_add(r: &mut RectI64, delta: &RectI64) {
    try_assign_add(r, delta).unwrap()
}

pub fn add(r: &RectI64, delta: &RectI64) -> RectI64 {
    try_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::matrix::rect::rect_i64::RectI64;

    use super::{add, assign_add, try_add, try_assign_add};

    #[test]
    fn test_try_assign_add() {
        let mut r = RectI64::of(0, 0, 12, 15);
        assert_eq!(try_assign_add(&mut r, &RectI64::of(5, 4, 3, 2)), Some(()));
        assert_eq!(r, RectI64::of(5, 4, 15, 17));
        assert_eq!(try_assign_add(&mut r, &RectI64::of(-14, -13, -12, -11)), Some(()));
        assert_eq!(r, RectI64::of(-9, -9, 3, 6));
    }

    #[test]
    fn try_assign_add_small_rect_to_bounds() {
        let mut min_r = RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MIN + 12, i64::MIN + 15);
        assert_eq!(try_assign_add(&mut min_r, &RectI64::of(-2, -5, 10, 20)), Some(()));
        assert_eq!(min_r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 22, i64::MIN + 35));

        let mut max_r = RectI64::of(i64::MAX - 12, i64::MAX - 15, i64::MAX - 2, i64::MAX - 5);
        assert_eq!(try_assign_add(&mut max_r, &RectI64::of(-1, -4, 2, 5)), Some(()));
        assert_eq!(max_r, RectI64::of(i64::MAX - 13, i64::MAX - 19, i64::MAX, i64::MAX));
    }

    #[test]
    fn try_assign_add_big_rect_to_bounds() {
        let mut min_r = RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX - 2, i64::MAX - 5);
        assert_eq!(try_assign_add(&mut min_r, &RectI64::of(-2, -5, 2, 5)), Some(()));
        assert_eq!(min_r, RectI64::largest());

        let mut min_r = RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX);
        assert_eq!(try_assign_add(&mut min_r, &RectI64::of(-2, -5, 0, 0)), Some(()));
        assert_eq!(min_r, RectI64::largest());

        let mut max_r = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5);
        assert_eq!(try_assign_add(&mut max_r, &RectI64::of(0, 0, 2, 5)), Some(()));
        assert_eq!(max_r, RectI64::largest());
    }

    #[test]
    fn try_assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MIN + 20, i64::MIN + 30);
        assert_eq!(try_assign_add(&mut r_min, &RectI64::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MIN + 20, i64::MIN + 30));

        let mut r_max = RectI64::of(i64::MAX - 20, i64::MAX - 30, i64::MAX - 5, i64::MAX - 10);
        assert_eq!(try_assign_add(&mut r_max, &RectI64::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectI64::of(i64::MAX - 20, i64::MAX - 30, i64::MAX - 5, i64::MAX - 10));
    }

    #[test]
    fn try_assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MAX, i64::MAX);
        assert_eq!(try_assign_add(&mut r_min, &RectI64::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MAX, i64::MAX));

        let mut r_max = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 5, i64::MAX - 10);
        assert_eq!(try_assign_add(&mut r_max, &RectI64::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 5, i64::MAX - 10));
    }

    #[test]
    fn try_assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MIN + 10, i64::MIN + 10);
        assert_eq!(try_assign_add(&mut r_min, &RectI64::min()), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI64::of(i64::MIN, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI64::of(0, i64::MIN, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI64::of(0, 0, i64::MIN, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI64::of(0, 0, 0, i64::MIN)), None);
        assert_eq!(r_min, RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MIN + 10, i64::MIN + 10));

        let mut r_max = RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX - 1, i64::MAX - 1);
        assert_eq!(try_assign_add(&mut r_max, &RectI64::max()), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI64::of(i64::MAX, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI64::of(0, i64::MAX, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI64::of(0, 0, i64::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI64::of(0, 0, 0, i64::MAX)), None);
        assert_eq!(r_max, RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX - 1, i64::MAX - 1));
    }

    #[test]
    fn try_assign_add_big_rect_limits_out_of_bounds() {
        let mut r = RectI64::largest();
        assert_eq!(try_assign_add(&mut r, &RectI64::largest()), None);
        assert_eq!(try_assign_add(&mut r, &RectI64::min()), None);
        assert_eq!(try_assign_add(&mut r, &RectI64::max()), None);
        assert_eq!(r, RectI64::largest());

        let mut r_min = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1);
        assert_eq!(try_assign_add(&mut r_min, &RectI64::max()), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI64::of(0, 0, i64::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI64::of(0, 0, 0, i64::MAX)), None);
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1));

        let mut r_max = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX);
        assert_eq!(try_assign_add(&mut r_max, &RectI64::min()), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI64::of(i64::MIN, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI64::of(0, i64::MIN, 0, 0)), None);
        assert_eq!(r_max, RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX));
    }

    #[test]
    fn test_try_add() {
        assert_eq!(try_add(&RectI64::of(0, 0, 12, 15), &RectI64::of(5, 4, 3, 2)), Some(RectI64::of(5, 4, 15, 17)));
        assert_eq!(try_add(&RectI64::of(5, 4, 15, 17), &RectI64::of(-14, -13, -12, -11)), Some(RectI64::of(-9, -9, 3, 6)));
    }

    #[test]
    fn try_add_small_rect_to_bounds() {
        assert_eq!(
            try_add(&RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MIN + 12, i64::MIN + 15), &RectI64::of(-2, -5, 10, 20)),
            Some(RectI64::of(i64::MIN, i64::MIN, i64::MIN + 22, i64::MIN + 35))
        );
        assert_eq!(
            try_add(&RectI64::of(i64::MAX - 12, i64::MAX - 15, i64::MAX - 2, i64::MAX - 5), &RectI64::of(-1, -4, 2, 5)),
            Some(RectI64::of(i64::MAX - 13, i64::MAX - 19, i64::MAX, i64::MAX))
        );
    }

    #[test]
    fn try_add_big_rect_to_bounds() {
        assert_eq!(
            try_add(&RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX - 2, i64::MAX - 5), &RectI64::of(-2, -5, 2, 5)),
            Some(RectI64::largest())
        );
        assert_eq!(try_add(&RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX), &RectI64::of(-2, -5, 0, 0)), Some(RectI64::largest()));
        assert_eq!(try_add(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5), &RectI64::of(0, 0, 2, 5)), Some(RectI64::largest()));
    }

    #[test]
    fn try_add_small_rect_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MIN + 20, i64::MIN + 30);
        assert_eq!(try_add(&mut r_min, &RectI64::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MIN + 20, i64::MIN + 30));

        let mut r_max = RectI64::of(i64::MAX - 20, i64::MAX - 30, i64::MAX - 5, i64::MAX - 10);
        assert_eq!(try_add(&mut r_max, &RectI64::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectI64::of(i64::MAX - 20, i64::MAX - 30, i64::MAX - 5, i64::MAX - 10));
    }

    #[test]
    fn try_add_big_rect_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MAX, i64::MAX);
        assert_eq!(try_add(&mut r_min, &RectI64::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MAX, i64::MAX));

        let mut r_max = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 5, i64::MAX - 10);
        assert_eq!(try_add(&mut r_max, &RectI64::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 5, i64::MAX - 10));
    }

    #[test]
    fn try_add_small_rect_limits_out_of_bounds() {
        let r_min = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MIN + 10, i64::MIN + 10);
        assert_eq!(try_add(&r_min, &RectI64::min()), None);
        assert_eq!(try_add(&r_min, &RectI64::of(i64::MIN, 0, 0, 0)), None);
        assert_eq!(try_add(&r_min, &RectI64::of(0, i64::MIN, 0, 0)), None);
        assert_eq!(try_add(&r_min, &RectI64::of(0, 0, i64::MIN, 0)), None);
        assert_eq!(try_add(&r_min, &RectI64::of(0, 0, 0, i64::MIN)), None);

        let r_max = RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX - 1, i64::MAX - 1);
        assert_eq!(try_add(&r_max, &RectI64::max()), None);
        assert_eq!(try_add(&r_max, &RectI64::of(i64::MAX, 0, 0, 0)), None);
        assert_eq!(try_add(&r_max, &RectI64::of(0, i64::MAX, 0, 0)), None);
        assert_eq!(try_add(&r_max, &RectI64::of(0, 0, i64::MAX, 0)), None);
        assert_eq!(try_add(&r_max, &RectI64::of(0, 0, 0, i64::MAX)), None);
    }

    #[test]
    fn try_add_big_rect_limits_out_of_bounds() {
        let r = RectI64::largest();
        assert_eq!(try_add(&r, &RectI64::largest()), None);
        assert_eq!(try_add(&r, &RectI64::min()), None);
        assert_eq!(try_add(&r, &RectI64::max()), None);

        let r_min = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1);
        assert_eq!(try_add(&r_min, &RectI64::max()), None);
        assert_eq!(try_add(&r_min, &RectI64::of(0, 0, i64::MAX, 0)), None);
        assert_eq!(try_add(&r_min, &RectI64::of(0, 0, 0, i64::MAX)), None);

        let r_max = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX);
        assert_eq!(try_add(&r_max, &RectI64::min()), None);
        assert_eq!(try_add(&r_max, &RectI64::of(i64::MIN, 0, 0, 0)), None);
        assert_eq!(try_add(&r_max, &RectI64::of(0, i64::MIN, 0, 0)), None);
    }

    #[test]
    fn test_assign_add() {
        let mut r = RectI64::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI64::of(5, 4, 3, 2));
        assert_eq!(r, RectI64::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI64::of(-14, -13, -12, -11));
        assert_eq!(r, RectI64::of(-9, -9, 3, 6));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&RectI64::of(0, 0, 12, 15), &RectI64::of(5, 4, 3, 2)), RectI64::of(5, 4, 15, 17));
        assert_eq!(add(&RectI64::of(5, 4, 15, 17), &RectI64::of(-14, -13, -12, -11)), RectI64::of(-9, -9, 3, 6));
    }
}
