use crate::matrix::{point::point_i64::PointI64, rect::rect_i64::RectI64};

pub fn try_saturating_inflate_assign(r: &mut RectI64) -> Option<()> {
    let is_min_row = r.min.row == i64::MIN;
    let is_min_col = r.min.col == i64::MIN;
    let is_max_row = r.max.row == i64::MAX;
    let is_max_col = r.max.col == i64::MAX;
    if (is_min_row && is_max_row) || (is_min_col && is_max_col) {
        return None;
    }
    let min_row_modifier = 1 - i64::from(is_min_row) + i64::from(is_max_row);
    let min_col_modifier = 1 - i64::from(is_min_col) + i64::from(is_max_col);
    let max_row_modifier = 1 + i64::from(is_min_row) - i64::from(is_max_row);
    let max_col_modifier = 1 + i64::from(is_min_col) - i64::from(is_max_col);
    r.min.row = r.min.row.saturating_sub(min_row_modifier);
    r.min.col = r.min.col.saturating_sub(min_col_modifier);
    r.max.row = r.max.row.saturating_add(max_row_modifier);
    r.max.col = r.max.col.saturating_add(max_col_modifier);
    Some(())
}

pub fn try_saturating_inflate(r: &RectI64) -> Option<RectI64> {
    let is_min_row = r.min.row == i64::MIN;
    let is_min_col = r.min.col == i64::MIN;
    let is_max_row = r.max.row == i64::MAX;
    let is_max_col = r.max.col == i64::MAX;
    if (is_min_row && is_max_row) || (is_min_col && is_max_col) {
        return None;
    }
    let min_row_modifier = 1 - i64::from(is_min_row) + i64::from(is_max_row);
    let min_col_modifier = 1 - i64::from(is_min_col) + i64::from(is_max_col);
    let max_row_modifier = 1 + i64::from(is_min_row) - i64::from(is_max_row);
    let max_col_modifier = 1 + i64::from(is_min_col) - i64::from(is_max_col);
    let min_row = r.min.row.saturating_sub(min_row_modifier);
    let min_col = r.min.col.saturating_sub(min_col_modifier);
    let max_row = r.max.row.saturating_add(max_row_modifier);
    let max_col = r.max.col.saturating_add(max_col_modifier);
    Some(RectI64 { min: PointI64 { row: min_row, col: min_col }, max: PointI64 { row: max_row, col: max_col } })
}

pub fn saturating_inflate_assign(r: &mut RectI64) {
    try_saturating_inflate_assign(r).unwrap()
}

pub fn saturating_inflate(r: &RectI64) -> RectI64 {
    try_saturating_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{saturating_inflate_assign, saturating_inflate, try_saturating_inflate_assign, try_saturating_inflate};
    use crate::matrix::rect::rect_i64::RectI64;

    #[test]
    fn try_saturating_inflate_assign_min_bounds() {
        let mut r = RectI64::of(i64::MIN + 7, i64::MIN + 2, i64::MIN + 17, i64::MIN + 13);
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN + 6, i64::MIN + 1, i64::MIN + 18, i64::MIN + 14));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN + 5, i64::MIN, i64::MIN + 19, i64::MIN + 15));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 20, i64::MIN + 17));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN + 3, i64::MIN, i64::MIN + 21, i64::MIN + 19));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 22, i64::MIN + 21));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN + 1, i64::MIN, i64::MIN + 23, i64::MIN + 23));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 24, i64::MIN + 25));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 26, i64::MIN + 27));
    }

    #[test]
    fn try_saturating_inflate_assign_max_bounds() {
        let mut r = RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3);
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 37, i64::MAX - 22, i64::MAX - 1, i64::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 38, i64::MAX - 24, i64::MAX, i64::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 40, i64::MAX - 26, i64::MAX, i64::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 42, i64::MAX - 28, i64::MAX, i64::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 44, i64::MAX - 30, i64::MAX, i64::MAX));
    }

    #[test]
    fn try_saturating_inflate_assign_to_bounds() {
        let mut r_odd = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_odd), Some(()));
        assert_eq!(r_odd, RectI64::largest());

        let mut r_even_1 = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_even_1), Some(()));
        assert_eq!(r_even_1, RectI64::largest());

        let mut r_even_2 = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX);
        assert_eq!(try_saturating_inflate_assign(&mut r_even_2), Some(()));
        assert_eq!(r_even_2, RectI64::largest());
    }

    #[test]
    fn try_saturating_inflate_assign_width_to_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 1, i64::MIN + 10, i64::MIN + 20, i64::MIN + 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN + 9, i64::MIN + 21, i64::MIN + 21));

        let mut r_max = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 1, i64::MIN + 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX, i64::MIN + 21));
    }

    #[test]
    fn try_saturating_inflate_assign_height_to_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 1, i64::MIN + 20, i64::MIN + 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectI64::of(i64::MIN + 9, i64::MIN, i64::MIN + 21, i64::MIN + 21));

        let mut r_max = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MIN + 20, i64::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MIN + 21, i64::MAX));
    }

    #[test]
    fn try_saturating_inflate_assign_out_of_bounds() {
        let mut r = RectI64::largest();
        assert_eq!(try_saturating_inflate_assign(&mut r), None);
        assert_eq!(r, RectI64::largest());

        let mut r_row = RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX, i64::MIN + 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_row), None);
        assert_eq!(r_row, RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX, i64::MIN + 20));

        let mut r_col = RectI64::of(i64::MIN + 10, i64::MIN, i64::MIN + 20, i64::MAX);
        assert_eq!(try_saturating_inflate_assign(&mut r_col), None);
        assert_eq!(r_col, RectI64::of(i64::MIN + 10, i64::MIN, i64::MIN + 20, i64::MAX));
    }

    #[test]
    fn try_saturating_inflate_min_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectI64::of(i64::MIN + 7, i64::MIN + 2, i64::MIN + 17, i64::MIN + 13)),
            Some(RectI64::of(i64::MIN + 6, i64::MIN + 1, i64::MIN + 18, i64::MIN + 14))
        );
        assert_eq!(
            try_saturating_inflate(&RectI64::of(i64::MIN + 6, i64::MIN + 1, i64::MIN + 18, i64::MIN + 14)),
            Some(RectI64::of(i64::MIN + 5, i64::MIN, i64::MIN + 19, i64::MIN + 15))
        );
        assert_eq!(
            try_saturating_inflate(&RectI64::of(i64::MIN + 5, i64::MIN, i64::MIN + 19, i64::MIN + 15)),
            Some(RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 20, i64::MIN + 17))
        );
        assert_eq!(
            try_saturating_inflate(&RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 20, i64::MIN + 17)),
            Some(RectI64::of(i64::MIN + 3, i64::MIN, i64::MIN + 21, i64::MIN + 19))
        );
        assert_eq!(
            try_saturating_inflate(&RectI64::of(i64::MIN + 3, i64::MIN, i64::MIN + 21, i64::MIN + 19)),
            Some(RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 22, i64::MIN + 21))
        );
        assert_eq!(
            try_saturating_inflate(&RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 22, i64::MIN + 21)),
            Some(RectI64::of(i64::MIN + 1, i64::MIN, i64::MIN + 23, i64::MIN + 23))
        );
        assert_eq!(
            try_saturating_inflate(&RectI64::of(i64::MIN + 1, i64::MIN, i64::MIN + 23, i64::MIN + 23)),
            Some(RectI64::of(i64::MIN, i64::MIN, i64::MIN + 24, i64::MIN + 25))
        );
        assert_eq!(
            try_saturating_inflate(&RectI64::of(i64::MIN, i64::MIN, i64::MIN + 24, i64::MIN + 25)),
            Some(RectI64::of(i64::MIN, i64::MIN, i64::MIN + 26, i64::MIN + 27))
        );
    }

    #[test]
    fn try_saturating_inflate_max_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3)),
            Some(RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2))
        );
        assert_eq!(
            try_saturating_inflate(&RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2)),
            Some(RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1))
        );
        assert_eq!(
            try_saturating_inflate(&RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1)),
            Some(RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX)),
            Some(RectI64::of(i64::MAX - 37, i64::MAX - 22, i64::MAX - 1, i64::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI64::of(i64::MAX - 37, i64::MAX - 22, i64::MAX - 1, i64::MAX)),
            Some(RectI64::of(i64::MAX - 38, i64::MAX - 24, i64::MAX, i64::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI64::of(i64::MAX - 38, i64::MAX - 24, i64::MAX, i64::MAX)),
            Some(RectI64::of(i64::MAX - 40, i64::MAX - 26, i64::MAX, i64::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI64::of(i64::MAX - 40, i64::MAX - 26, i64::MAX, i64::MAX)),
            Some(RectI64::of(i64::MAX - 42, i64::MAX - 28, i64::MAX, i64::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI64::of(i64::MAX - 42, i64::MAX - 28, i64::MAX, i64::MAX)),
            Some(RectI64::of(i64::MAX - 44, i64::MAX - 30, i64::MAX, i64::MAX))
        );
    }

    #[test]
    fn try_saturating_inflate_to_bounds() {
        assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1)), Some(RectI64::largest()));
        assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1)), Some(RectI64::largest()));
        assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX)), Some(RectI64::largest()));
    }

    #[test]
    fn try_saturating_inflate_width_to_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectI64::of(i64::MIN + 1, i64::MIN + 10, i64::MIN + 20, i64::MIN + 20)),
            Some(RectI64::of(i64::MIN, i64::MIN + 9, i64::MIN + 21, i64::MIN + 21))
        );
        assert_eq!(
            try_saturating_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 1, i64::MIN + 20)),
            Some(RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX, i64::MIN + 21))
        );
    }

    #[test]
    fn try_saturating_inflate_height_to_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 1, i64::MIN + 20, i64::MIN + 20)),
            Some(RectI64::of(i64::MIN + 9, i64::MIN, i64::MIN + 21, i64::MIN + 21))
        );
        assert_eq!(
            try_saturating_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MIN + 20, i64::MAX - 1)),
            Some(RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MIN + 21, i64::MAX))
        );
    }

    #[test]
    fn try_saturating_inflate_out_of_bounds() {
        assert_eq!(try_saturating_inflate(&RectI64::largest()), None);
        assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX, i64::MIN + 20)), None);
        assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN + 10, i64::MIN, i64::MIN + 20, i64::MAX)), None);
    }

    #[test]
    fn saturating_inflate_assign_min_bounds() {
        let mut r = RectI64::of(i64::MIN + 7, i64::MIN + 2, i64::MIN + 17, i64::MIN + 13);
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 6, i64::MIN + 1, i64::MIN + 18, i64::MIN + 14));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 5, i64::MIN, i64::MIN + 19, i64::MIN + 15));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 20, i64::MIN + 17));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 3, i64::MIN, i64::MIN + 21, i64::MIN + 19));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 22, i64::MIN + 21));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 1, i64::MIN, i64::MIN + 23, i64::MIN + 23));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 24, i64::MIN + 25));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 26, i64::MIN + 27));
    }

    #[test]
    fn saturating_inflate_assign_max_bounds() {
        let mut r = RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3);
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 37, i64::MAX - 22, i64::MAX - 1, i64::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 38, i64::MAX - 24, i64::MAX, i64::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 40, i64::MAX - 26, i64::MAX, i64::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 42, i64::MAX - 28, i64::MAX, i64::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 44, i64::MAX - 30, i64::MAX, i64::MAX));
    }

    #[test]
    fn saturating_inflate_min_bounds() {
        assert_eq!(
            saturating_inflate(&RectI64::of(i64::MIN + 7, i64::MIN + 2, i64::MIN + 17, i64::MIN + 13)),
            RectI64::of(i64::MIN + 6, i64::MIN + 1, i64::MIN + 18, i64::MIN + 14)
        );
        assert_eq!(
            saturating_inflate(&RectI64::of(i64::MIN + 6, i64::MIN + 1, i64::MIN + 18, i64::MIN + 14)),
            RectI64::of(i64::MIN + 5, i64::MIN, i64::MIN + 19, i64::MIN + 15)
        );
        assert_eq!(
            saturating_inflate(&RectI64::of(i64::MIN + 5, i64::MIN, i64::MIN + 19, i64::MIN + 15)),
            RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 20, i64::MIN + 17)
        );
        assert_eq!(
            saturating_inflate(&RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 20, i64::MIN + 17)),
            RectI64::of(i64::MIN + 3, i64::MIN, i64::MIN + 21, i64::MIN + 19)
        );
        assert_eq!(
            saturating_inflate(&RectI64::of(i64::MIN + 3, i64::MIN, i64::MIN + 21, i64::MIN + 19)),
            RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 22, i64::MIN + 21)
        );
        assert_eq!(
            saturating_inflate(&RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 22, i64::MIN + 21)),
            RectI64::of(i64::MIN + 1, i64::MIN, i64::MIN + 23, i64::MIN + 23)
        );
        assert_eq!(
            saturating_inflate(&RectI64::of(i64::MIN + 1, i64::MIN, i64::MIN + 23, i64::MIN + 23)),
            RectI64::of(i64::MIN, i64::MIN, i64::MIN + 24, i64::MIN + 25)
        );
        assert_eq!(
            saturating_inflate(&RectI64::of(i64::MIN, i64::MIN, i64::MIN + 24, i64::MIN + 25)),
            RectI64::of(i64::MIN, i64::MIN, i64::MIN + 26, i64::MIN + 27)
        );
    }

    #[test]
    fn saturating_inflate_max_bounds() {
        assert_eq!(
            saturating_inflate(&RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3)),
            RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2)
        );
        assert_eq!(
            saturating_inflate(&RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2)),
            RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1)
        );
        assert_eq!(
            saturating_inflate(&RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1)),
            RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX)),
            RectI64::of(i64::MAX - 37, i64::MAX - 22, i64::MAX - 1, i64::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI64::of(i64::MAX - 37, i64::MAX - 22, i64::MAX - 1, i64::MAX)),
            RectI64::of(i64::MAX - 38, i64::MAX - 24, i64::MAX, i64::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI64::of(i64::MAX - 38, i64::MAX - 24, i64::MAX, i64::MAX)),
            RectI64::of(i64::MAX - 40, i64::MAX - 26, i64::MAX, i64::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI64::of(i64::MAX - 40, i64::MAX - 26, i64::MAX, i64::MAX)),
            RectI64::of(i64::MAX - 42, i64::MAX - 28, i64::MAX, i64::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI64::of(i64::MAX - 42, i64::MAX - 28, i64::MAX, i64::MAX)),
            RectI64::of(i64::MAX - 44, i64::MAX - 30, i64::MAX, i64::MAX)
        );
    }
}
