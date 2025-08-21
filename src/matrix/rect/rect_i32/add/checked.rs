use crate::matrix::{point::point_i32::PointI32, rect::rect_i32::RectI32};

pub fn try_assign_add(r: &mut RectI32, delta: &RectI32) -> Option<()> {
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

pub fn try_add(r: &RectI32, delta: &RectI32) -> Option<RectI32> {
    let min_row = r.min.row.checked_add(delta.min.row)?;
    let min_col = r.min.col.checked_add(delta.min.col)?;
    let max_row = r.max.row.checked_add(delta.max.row)?;
    let max_col = r.max.col.checked_add(delta.max.col)?;
    Some(RectI32 { min: PointI32 { row: min_row, col: min_col }, max: PointI32 { row: max_row, col: max_col } })
}

pub fn assign_add(r: &mut RectI32, delta: &RectI32) {
    try_assign_add(r, delta).unwrap()
}

pub fn add(r: &RectI32, delta: &RectI32) -> RectI32 {
    try_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::matrix::rect::rect_i32::RectI32;

    use super::{add, assign_add, try_add, try_assign_add};

    #[test]
    fn test_try_assign_add() {
        let mut r = RectI32::of(0, 0, 12, 15);
        assert_eq!(try_assign_add(&mut r, &RectI32::of(5, 4, 3, 2)), Some(()));
        assert_eq!(r, RectI32::of(5, 4, 15, 17));
        assert_eq!(try_assign_add(&mut r, &RectI32::of(-14, -13, -12, -11)), Some(()));
        assert_eq!(r, RectI32::of(-9, -9, 3, 6));
    }

    #[test]
    fn try_assign_add_small_rect_to_bounds() {
        let mut min_r = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MIN + 12, i32::MIN + 15);
        assert_eq!(try_assign_add(&mut min_r, &RectI32::of(-2, -5, 10, 20)), Some(()));
        assert_eq!(min_r, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 22, i32::MIN + 35));

        let mut max_r = RectI32::of(i32::MAX - 12, i32::MAX - 15, i32::MAX - 2, i32::MAX - 5);
        assert_eq!(try_assign_add(&mut max_r, &RectI32::of(-1, -4, 2, 5)), Some(()));
        assert_eq!(max_r, RectI32::of(i32::MAX - 13, i32::MAX - 19, i32::MAX, i32::MAX));
    }

    #[test]
    fn try_assign_add_big_rect_to_bounds() {
        let mut min_r = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX - 2, i32::MAX - 5);
        assert_eq!(try_assign_add(&mut min_r, &RectI32::of(-2, -5, 2, 5)), Some(()));
        assert_eq!(min_r, RectI32::largest());

        let mut min_r = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX);
        assert_eq!(try_assign_add(&mut min_r, &RectI32::of(-2, -5, 0, 0)), Some(()));
        assert_eq!(min_r, RectI32::largest());

        let mut max_r = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5);
        assert_eq!(try_assign_add(&mut max_r, &RectI32::of(0, 0, 2, 5)), Some(()));
        assert_eq!(max_r, RectI32::largest());
    }

    #[test]
    fn try_assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MIN + 20, i32::MIN + 30);
        assert_eq!(try_assign_add(&mut r_min, &RectI32::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MIN + 20, i32::MIN + 30));

        let mut r_max = RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX - 5, i32::MAX - 10);
        assert_eq!(try_assign_add(&mut r_max, &RectI32::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX - 5, i32::MAX - 10));
    }

    #[test]
    fn try_assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MAX, i32::MAX);
        assert_eq!(try_assign_add(&mut r_min, &RectI32::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MAX, i32::MAX));

        let mut r_max = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 5, i32::MAX - 10);
        assert_eq!(try_assign_add(&mut r_max, &RectI32::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 5, i32::MAX - 10));
    }

    #[test]
    fn try_assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10);
        assert_eq!(try_assign_add(&mut r_min, &RectI32::min()), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI32::of(i32::MIN, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI32::of(0, i32::MIN, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI32::of(0, 0, i32::MIN, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI32::of(0, 0, 0, i32::MIN)), None);
        assert_eq!(r_min, RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10));

        let mut r_max = RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1);
        assert_eq!(try_assign_add(&mut r_max, &RectI32::max()), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI32::of(i32::MAX, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI32::of(0, i32::MAX, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI32::of(0, 0, i32::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI32::of(0, 0, 0, i32::MAX)), None);
        assert_eq!(r_max, RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1));
    }

    #[test]
    fn try_assign_add_big_rect_limits_out_of_bounds() {
        let mut r = RectI32::largest();
        assert_eq!(try_assign_add(&mut r, &RectI32::largest()), None);
        assert_eq!(try_assign_add(&mut r, &RectI32::min()), None);
        assert_eq!(try_assign_add(&mut r, &RectI32::max()), None);
        assert_eq!(r, RectI32::largest());

        let mut r_min = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1);
        assert_eq!(try_assign_add(&mut r_min, &RectI32::max()), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI32::of(0, 0, i32::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI32::of(0, 0, 0, i32::MAX)), None);
        assert_eq!(r_min, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1));

        let mut r_max = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX);
        assert_eq!(try_assign_add(&mut r_max, &RectI32::min()), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI32::of(i32::MIN, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI32::of(0, i32::MIN, 0, 0)), None);
        assert_eq!(r_max, RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX));
    }

    #[test]
    fn test_try_add() {
        assert_eq!(try_add(&RectI32::of(0, 0, 12, 15), &RectI32::of(5, 4, 3, 2)), Some(RectI32::of(5, 4, 15, 17)));
        assert_eq!(try_add(&RectI32::of(5, 4, 15, 17), &RectI32::of(-14, -13, -12, -11)), Some(RectI32::of(-9, -9, 3, 6)));
    }

    #[test]
    fn try_add_small_rect_to_bounds() {
        assert_eq!(
            try_add(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MIN + 12, i32::MIN + 15), &RectI32::of(-2, -5, 10, 20)),
            Some(RectI32::of(i32::MIN, i32::MIN, i32::MIN + 22, i32::MIN + 35))
        );
        assert_eq!(
            try_add(&RectI32::of(i32::MAX - 12, i32::MAX - 15, i32::MAX - 2, i32::MAX - 5), &RectI32::of(-1, -4, 2, 5)),
            Some(RectI32::of(i32::MAX - 13, i32::MAX - 19, i32::MAX, i32::MAX))
        );
    }

    #[test]
    fn try_add_big_rect_to_bounds() {
        assert_eq!(
            try_add(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX - 2, i32::MAX - 5), &RectI32::of(-2, -5, 2, 5)),
            Some(RectI32::largest())
        );
        assert_eq!(try_add(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX), &RectI32::of(-2, -5, 0, 0)), Some(RectI32::largest()));
        assert_eq!(try_add(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5), &RectI32::of(0, 0, 2, 5)), Some(RectI32::largest()));
    }

    #[test]
    fn try_add_small_rect_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MIN + 20, i32::MIN + 30);
        assert_eq!(try_add(&mut r_min, &RectI32::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MIN + 20, i32::MIN + 30));

        let mut r_max = RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX - 5, i32::MAX - 10);
        assert_eq!(try_add(&mut r_max, &RectI32::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX - 5, i32::MAX - 10));
    }

    #[test]
    fn try_add_big_rect_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MAX, i32::MAX);
        assert_eq!(try_add(&mut r_min, &RectI32::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MAX, i32::MAX));

        let mut r_max = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 5, i32::MAX - 10);
        assert_eq!(try_add(&mut r_max, &RectI32::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 5, i32::MAX - 10));
    }

    #[test]
    fn try_add_small_rect_limits_out_of_bounds() {
        let r_min = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10);
        assert_eq!(try_add(&r_min, &RectI32::min()), None);
        assert_eq!(try_add(&r_min, &RectI32::of(i32::MIN, 0, 0, 0)), None);
        assert_eq!(try_add(&r_min, &RectI32::of(0, i32::MIN, 0, 0)), None);
        assert_eq!(try_add(&r_min, &RectI32::of(0, 0, i32::MIN, 0)), None);
        assert_eq!(try_add(&r_min, &RectI32::of(0, 0, 0, i32::MIN)), None);

        let r_max = RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1);
        assert_eq!(try_add(&r_max, &RectI32::max()), None);
        assert_eq!(try_add(&r_max, &RectI32::of(i32::MAX, 0, 0, 0)), None);
        assert_eq!(try_add(&r_max, &RectI32::of(0, i32::MAX, 0, 0)), None);
        assert_eq!(try_add(&r_max, &RectI32::of(0, 0, i32::MAX, 0)), None);
        assert_eq!(try_add(&r_max, &RectI32::of(0, 0, 0, i32::MAX)), None);
    }

    #[test]
    fn try_add_big_rect_limits_out_of_bounds() {
        let r = RectI32::largest();
        assert_eq!(try_add(&r, &RectI32::largest()), None);
        assert_eq!(try_add(&r, &RectI32::min()), None);
        assert_eq!(try_add(&r, &RectI32::max()), None);

        let r_min = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1);
        assert_eq!(try_add(&r_min, &RectI32::max()), None);
        assert_eq!(try_add(&r_min, &RectI32::of(0, 0, i32::MAX, 0)), None);
        assert_eq!(try_add(&r_min, &RectI32::of(0, 0, 0, i32::MAX)), None);

        let r_max = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX);
        assert_eq!(try_add(&r_max, &RectI32::min()), None);
        assert_eq!(try_add(&r_max, &RectI32::of(i32::MIN, 0, 0, 0)), None);
        assert_eq!(try_add(&r_max, &RectI32::of(0, i32::MIN, 0, 0)), None);
    }

    #[test]
    fn test_assign_add() {
        let mut r = RectI32::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI32::of(5, 4, 3, 2));
        assert_eq!(r, RectI32::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI32::of(-14, -13, -12, -11));
        assert_eq!(r, RectI32::of(-9, -9, 3, 6));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&RectI32::of(0, 0, 12, 15), &RectI32::of(5, 4, 3, 2)), RectI32::of(5, 4, 15, 17));
        assert_eq!(add(&RectI32::of(5, 4, 15, 17), &RectI32::of(-14, -13, -12, -11)), RectI32::of(-9, -9, 3, 6));
    }
}
