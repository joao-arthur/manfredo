use crate::matrix::{point::point_i64::PointI64, rect::rect_i64::RectI64};

pub fn try_assign_inflate(r: &mut RectI64) -> Option<()> {
    let min_row = r.min.row.checked_sub(1)?;
    let min_col = r.min.col.checked_sub(1)?;
    let max_row = r.max.row.checked_add(1)?;
    let max_col = r.max.col.checked_add(1)?;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Some(())
}

pub fn try_inflate(r: &RectI64) -> Option<RectI64> {
    let min_row = r.min.row.checked_sub(1)?;
    let min_col = r.min.col.checked_sub(1)?;
    let max_row = r.max.row.checked_add(1)?;
    let max_col = r.max.col.checked_add(1)?;
    Some(RectI64 { min: PointI64 { row: min_row, col: min_col }, max: PointI64 { row: max_row, col: max_col } })
}

pub fn assign_inflate(r: &mut RectI64) {
    try_assign_inflate(r).unwrap()
}

pub fn inflate(r: &RectI64) -> RectI64 {
    try_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{assign_inflate, inflate, try_assign_inflate, try_inflate};
    use crate::matrix::rect::rect_i64::RectI64;

    #[test]
    fn try_assign_inflate_min_bounds() {
        let mut r = RectI64::of(i64::MIN + 7, i64::MIN + 3, i64::MIN + 9, i64::MIN + 13);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN + 6, i64::MIN + 2, i64::MIN + 10, i64::MIN + 14));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN + 5, i64::MIN + 1, i64::MIN + 11, i64::MIN + 15));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 12, i64::MIN + 16));
    }

    #[test]
    fn try_assign_inflate_max_bounds() {
        let mut r = RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX));
    }

    #[test]
    fn try_assign_inflate_to_bounds() {
        let mut r = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::largest());
    }

    #[test]
    fn try_assign_inflate_width_to_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 1, i64::MIN + 10, i64::MIN + 20, i64::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN + 9, i64::MIN + 21, i64::MIN + 21));

        let mut r_max = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 1, i64::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX, i64::MIN + 21));
    }

    #[test]
    fn try_assign_inflate_height_to_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 1, i64::MIN + 20, i64::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectI64::of(i64::MIN + 9, i64::MIN, i64::MIN + 21, i64::MIN + 21));

        let mut r_max = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MIN + 20, i64::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MIN + 21, i64::MAX));
    }

    #[test]
    fn try_assign_inflate_out_of_bounds() {
        let mut r_min_row = RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX, i64::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_min_row), None);
        assert_eq!(r_min_row, RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX, i64::MIN + 20));

        let mut r_max_row = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX, i64::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_max_row), None);
        assert_eq!(r_max_row, RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX, i64::MIN + 20));

        let mut r_min_col = RectI64::of(i64::MIN + 10, i64::MIN, i64::MIN + 20, i64::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_min_col), None);
        assert_eq!(r_min_col, RectI64::of(i64::MIN + 10, i64::MIN, i64::MIN + 20, i64::MIN + 20));

        let mut r_max_col = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MIN + 20, i64::MAX);
        assert_eq!(try_assign_inflate(&mut r_max_col), None);
        assert_eq!(r_max_col, RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MIN + 20, i64::MAX));

        let mut r_min = RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10);
        assert_eq!(try_assign_inflate(&mut r_min), None);
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10));

        let mut r_max = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX, i64::MAX);
        assert_eq!(try_assign_inflate(&mut r_max), None);
        assert_eq!(r_max, RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX, i64::MAX));
    }

    #[test]
    fn try_assign_inflate_limits_out_of_bounds() {
        let mut r = RectI64::largest();
        assert_eq!(try_assign_inflate(&mut r), None);
        assert_eq!(r, RectI64::largest());
    }

    #[test]
    fn try_inflate_min_bounds() {
        assert_eq!(
            try_inflate(&RectI64::of(i64::MIN + 7, i64::MIN + 3, i64::MIN + 9, i64::MIN + 13)),
            Some(RectI64::of(i64::MIN + 6, i64::MIN + 2, i64::MIN + 10, i64::MIN + 14))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MIN + 6, i64::MIN + 2, i64::MIN + 10, i64::MIN + 14)),
            Some(RectI64::of(i64::MIN + 5, i64::MIN + 1, i64::MIN + 11, i64::MIN + 15))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MIN + 5, i64::MIN + 1, i64::MIN + 11, i64::MIN + 15)),
            Some(RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 12, i64::MIN + 16))
        );
    }

    #[test]
    fn try_inflate_max_bounds() {
        assert_eq!(
            try_inflate(&RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3)),
            Some(RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2)),
            Some(RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1)),
            Some(RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX))
        );
    }

    #[test]
    fn try_inflate_to_bounds() {
        assert_eq!(try_inflate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1)), Some(RectI64::largest()));
    }

    #[test]
    fn try_inflate_width_to_bounds() {
        assert_eq!(
            try_inflate(&RectI64::of(i64::MIN + 1, i64::MIN + 10, i64::MIN + 20, i64::MIN + 20)),
            Some(RectI64::of(i64::MIN, i64::MIN + 9, i64::MIN + 21, i64::MIN + 21))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 1, i64::MIN + 20)),
            Some(RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX, i64::MIN + 21))
        );
    }

    #[test]
    fn try_inflate_height_to_bounds() {
        assert_eq!(
            try_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 1, i64::MIN + 20, i64::MIN + 20)),
            Some(RectI64::of(i64::MIN + 9, i64::MIN, i64::MIN + 21, i64::MIN + 21))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MIN + 20, i64::MAX - 1)),
            Some(RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MIN + 21, i64::MAX))
        );
    }

    #[test]
    fn try_inflate_out_of_bounds() {
        assert_eq!(try_inflate(&RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX, i64::MIN + 20)), None);
        assert_eq!(try_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX, i64::MIN + 20)), None);
        assert_eq!(try_inflate(&RectI64::of(i64::MIN + 10, i64::MIN, i64::MIN + 20, i64::MIN + 20)), None);
        assert_eq!(try_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MIN + 20, i64::MAX)), None);
        assert_eq!(try_inflate(&RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10)), None);
        assert_eq!(try_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX, i64::MAX)), None);
    }

    #[test]
    fn try_inflate_limits_out_of_bounds() {
        assert_eq!(try_inflate(&RectI64::largest()), None);
    }

    #[test]
    fn assign_inflate_min_bounds() {
        let mut r = RectI64::of(i64::MIN + 7, i64::MIN + 3, i64::MIN + 9, i64::MIN + 13);
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 6, i64::MIN + 2, i64::MIN + 10, i64::MIN + 14));
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 5, i64::MIN + 1, i64::MIN + 11, i64::MIN + 15));
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 12, i64::MIN + 16));
    }

    #[test]
    fn assign_inflate_max_bounds() {
        let mut r = RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3);
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2));
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1));
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX));
    }

    #[test]
    fn inflate_min_bounds() {
        assert_eq!(
            inflate(&RectI64::of(i64::MIN + 7, i64::MIN + 3, i64::MIN + 9, i64::MIN + 13)),
            RectI64::of(i64::MIN + 6, i64::MIN + 2, i64::MIN + 10, i64::MIN + 14)
        );
        assert_eq!(
            inflate(&RectI64::of(i64::MIN + 6, i64::MIN + 2, i64::MIN + 10, i64::MIN + 14)),
            RectI64::of(i64::MIN + 5, i64::MIN + 1, i64::MIN + 11, i64::MIN + 15)
        );
        assert_eq!(
            inflate(&RectI64::of(i64::MIN + 5, i64::MIN + 1, i64::MIN + 11, i64::MIN + 15)),
            RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 12, i64::MIN + 16)
        );
    }

    #[test]
    fn inflate_max_bounds() {
        assert_eq!(
            inflate(&RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3)),
            RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2)
        );
        assert_eq!(
            inflate(&RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2)),
            RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1)
        );
        assert_eq!(
            inflate(&RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1)),
            RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX)
        );
    }
}
