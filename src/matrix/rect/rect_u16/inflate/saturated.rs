use crate::matrix::{point::point_u16::PointU16, rect::rect_u16::RectU16};

pub fn try_saturating_inflate_assign(r: &mut RectU16) -> Option<()> {
    let is_min_row = r.min.row == 0;
    let is_min_col = r.min.col == 0;
    let is_max_row = r.max.row == u16::MAX;
    let is_max_col = r.max.col == u16::MAX;
    if (is_min_row && is_max_row) || (is_min_col && is_max_col) {
        return None;
    }
    let min_row_modifier = 1 - u16::from(is_min_row) + u16::from(is_max_row);
    let min_col_modifier = 1 - u16::from(is_min_col) + u16::from(is_max_col);
    let max_row_modifier = 1 + u16::from(is_min_row) - u16::from(is_max_row);
    let max_col_modifier = 1 + u16::from(is_min_col) - u16::from(is_max_col);
    r.min.row = r.min.row.saturating_sub(min_row_modifier);
    r.min.col = r.min.col.saturating_sub(min_col_modifier);
    r.max.row = r.max.row.saturating_add(max_row_modifier);
    r.max.col = r.max.col.saturating_add(max_col_modifier);
    Some(())
}

pub fn try_saturating_inflate(r: &RectU16) -> Option<RectU16> {
    let is_min_row = r.min.row == 0;
    let is_min_col = r.min.col == 0;
    let is_max_row = r.max.row == u16::MAX;
    let is_max_col = r.max.col == u16::MAX;
    if (is_min_row && is_max_row) || (is_min_col && is_max_col) {
        return None;
    }
    let min_row_modifier = 1 - u16::from(is_min_row) + u16::from(is_max_row);
    let min_col_modifier = 1 - u16::from(is_min_col) + u16::from(is_max_col);
    let max_row_modifier = 1 + u16::from(is_min_row) - u16::from(is_max_row);
    let max_col_modifier = 1 + u16::from(is_min_col) - u16::from(is_max_col);
    let min_row = r.min.row.saturating_sub(min_row_modifier);
    let min_col = r.min.col.saturating_sub(min_col_modifier);
    let max_row = r.max.row.saturating_add(max_row_modifier);
    let max_col = r.max.col.saturating_add(max_col_modifier);
    Some(RectU16 { min: PointU16 { row: min_row, col: min_col }, max: PointU16 { row: max_row, col: max_col } })
}

pub fn saturating_inflate_assign(r: &mut RectU16) {
    try_saturating_inflate_assign(r).unwrap()
}

pub fn saturating_inflate(r: &RectU16) -> RectU16 {
    try_saturating_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{saturating_inflate, saturating_inflate_assign, try_saturating_inflate, try_saturating_inflate_assign};
    use crate::matrix::rect::rect_u16::RectU16;

    #[test]
    fn try_saturating_inflate_assign_min_bounds() {
        let mut r = RectU16::of(7, 2, 17, 13);
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(6, 1, 18, 14));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(5, 0, 19, 15));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(4, 0, 20, 17));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(3, 0, 21, 19));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(2, 0, 22, 21));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(1, 0, 23, 23));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(0, 0, 24, 25));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(0, 0, 26, 27));
    }

    #[test]
    fn try_saturating_inflate_assign_max_bounds() {
        let mut r = RectU16::of(u16::MAX - 33, u16::MAX - 17, u16::MAX - 5, u16::MAX - 3);
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 37, u16::MAX - 22, u16::MAX - 1, u16::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 38, u16::MAX - 24, u16::MAX, u16::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 40, u16::MAX - 26, u16::MAX, u16::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 42, u16::MAX - 28, u16::MAX, u16::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 44, u16::MAX - 30, u16::MAX, u16::MAX));
    }

    #[test]
    fn try_saturating_inflate_assign_to_bounds() {
        let mut r_odd = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_odd), Some(()));
        assert_eq!(r_odd, RectU16::largest());

        let mut r_even_1 = RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_even_1), Some(()));
        assert_eq!(r_even_1, RectU16::largest());

        let mut r_even_2 = RectU16::of(1, 1, u16::MAX, u16::MAX);
        assert_eq!(try_saturating_inflate_assign(&mut r_even_2), Some(()));
        assert_eq!(r_even_2, RectU16::largest());
    }

    #[test]
    fn try_saturating_inflate_assign_width_to_bounds() {
        let mut r_min = RectU16::of(1, 10, 20, 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectU16::of(0, 9, 21, 21));

        let mut r_max = RectU16::of(10, 10, u16::MAX - 1, 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectU16::of(9, 9, u16::MAX, 21));
    }

    #[test]
    fn try_saturating_inflate_assign_height_to_bounds() {
        let mut r_min = RectU16::of(10, 1, 20, 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectU16::of(9, 0, 21, 21));

        let mut r_max = RectU16::of(10, 10, 20, u16::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectU16::of(9, 9, 21, u16::MAX));
    }

    #[test]
    fn try_saturating_inflate_assign_out_of_bounds() {
        let mut r = RectU16::largest();
        assert_eq!(try_saturating_inflate_assign(&mut r), None);
        assert_eq!(r, RectU16::largest());

        let mut r_row = RectU16::of(0, 10, u16::MAX, 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_row), None);
        assert_eq!(r_row, RectU16::of(0, 10, u16::MAX, 20));

        let mut r_col = RectU16::of(10, 0, 20, u16::MAX);
        assert_eq!(try_saturating_inflate_assign(&mut r_col), None);
        assert_eq!(r_col, RectU16::of(10, 0, 20, u16::MAX));
    }

    #[test]
    fn try_saturating_inflate_min_bounds() {
        assert_eq!(try_saturating_inflate(&RectU16::of(7, 2, 17, 13)), Some(RectU16::of(6, 1, 18, 14)));
        assert_eq!(try_saturating_inflate(&RectU16::of(6, 1, 18, 14)), Some(RectU16::of(5, 0, 19, 15)));
        assert_eq!(try_saturating_inflate(&RectU16::of(5, 0, 19, 15)), Some(RectU16::of(4, 0, 20, 17)));
        assert_eq!(try_saturating_inflate(&RectU16::of(4, 0, 20, 17)), Some(RectU16::of(3, 0, 21, 19)));
        assert_eq!(try_saturating_inflate(&RectU16::of(3, 0, 21, 19)), Some(RectU16::of(2, 0, 22, 21)));
        assert_eq!(try_saturating_inflate(&RectU16::of(2, 0, 22, 21)), Some(RectU16::of(1, 0, 23, 23)));
        assert_eq!(try_saturating_inflate(&RectU16::of(1, 0, 23, 23)), Some(RectU16::of(0, 0, 24, 25)));
        assert_eq!(try_saturating_inflate(&RectU16::of(0, 0, 24, 25)), Some(RectU16::of(0, 0, 26, 27)));
    }

    #[test]
    fn try_saturating_inflate_max_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectU16::of(u16::MAX - 33, u16::MAX - 17, u16::MAX - 5, u16::MAX - 3)),
            Some(RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2))
        );
        assert_eq!(
            try_saturating_inflate(&RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2)),
            Some(RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1))
        );
        assert_eq!(
            try_saturating_inflate(&RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1)),
            Some(RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX)),
            Some(RectU16::of(u16::MAX - 37, u16::MAX - 22, u16::MAX - 1, u16::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectU16::of(u16::MAX - 37, u16::MAX - 22, u16::MAX - 1, u16::MAX)),
            Some(RectU16::of(u16::MAX - 38, u16::MAX - 24, u16::MAX, u16::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectU16::of(u16::MAX - 38, u16::MAX - 24, u16::MAX, u16::MAX)),
            Some(RectU16::of(u16::MAX - 40, u16::MAX - 26, u16::MAX, u16::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectU16::of(u16::MAX - 40, u16::MAX - 26, u16::MAX, u16::MAX)),
            Some(RectU16::of(u16::MAX - 42, u16::MAX - 28, u16::MAX, u16::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectU16::of(u16::MAX - 42, u16::MAX - 28, u16::MAX, u16::MAX)),
            Some(RectU16::of(u16::MAX - 44, u16::MAX - 30, u16::MAX, u16::MAX))
        );
    }

    #[test]
    fn try_saturating_inflate_to_bounds() {
        assert_eq!(try_saturating_inflate(&RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1)), Some(RectU16::largest()));
        assert_eq!(try_saturating_inflate(&RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1)), Some(RectU16::largest()));
        assert_eq!(try_saturating_inflate(&RectU16::of(1, 1, u16::MAX, u16::MAX)), Some(RectU16::largest()));
    }

    #[test]
    fn try_saturating_inflate_width_to_bounds() {
        assert_eq!(try_saturating_inflate(&RectU16::of(1, 10, 20, 20)), Some(RectU16::of(0, 9, 21, 21)));
        assert_eq!(try_saturating_inflate(&RectU16::of(10, 10, u16::MAX - 1, 20)), Some(RectU16::of(9, 9, u16::MAX, 21)));
    }

    #[test]
    fn try_saturating_inflate_height_to_bounds() {
        assert_eq!(try_saturating_inflate(&RectU16::of(10, 1, 20, 20)), Some(RectU16::of(9, 0, 21, 21)));
        assert_eq!(try_saturating_inflate(&RectU16::of(10, 10, 20, u16::MAX - 1)), Some(RectU16::of(9, 9, 21, u16::MAX)));
    }

    #[test]
    fn try_saturating_inflate_out_of_bounds() {
        assert_eq!(try_saturating_inflate(&RectU16::largest()), None);
        assert_eq!(try_saturating_inflate(&RectU16::of(0, 10, u16::MAX, 20)), None);
        assert_eq!(try_saturating_inflate(&RectU16::of(10, 0, 20, u16::MAX)), None);
    }

    #[test]
    fn saturating_inflate_assign_min_bounds() {
        let mut r = RectU16::of(7, 2, 17, 13);
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(6, 1, 18, 14));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(5, 0, 19, 15));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(4, 0, 20, 17));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(3, 0, 21, 19));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(2, 0, 22, 21));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(1, 0, 23, 23));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(0, 0, 24, 25));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(0, 0, 26, 27));
    }

    #[test]
    fn saturating_inflate_assign_max_bounds() {
        let mut r = RectU16::of(u16::MAX - 33, u16::MAX - 17, u16::MAX - 5, u16::MAX - 3);
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(u16::MAX - 37, u16::MAX - 22, u16::MAX - 1, u16::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(u16::MAX - 38, u16::MAX - 24, u16::MAX, u16::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(u16::MAX - 40, u16::MAX - 26, u16::MAX, u16::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(u16::MAX - 42, u16::MAX - 28, u16::MAX, u16::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(u16::MAX - 44, u16::MAX - 30, u16::MAX, u16::MAX));
    }

    #[test]
    fn saturating_inflate_min_bounds() {
        assert_eq!(saturating_inflate(&RectU16::of(7, 2, 17, 13)), RectU16::of(6, 1, 18, 14));
        assert_eq!(saturating_inflate(&RectU16::of(6, 1, 18, 14)), RectU16::of(5, 0, 19, 15));
        assert_eq!(saturating_inflate(&RectU16::of(5, 0, 19, 15)), RectU16::of(4, 0, 20, 17));
        assert_eq!(saturating_inflate(&RectU16::of(4, 0, 20, 17)), RectU16::of(3, 0, 21, 19));
        assert_eq!(saturating_inflate(&RectU16::of(3, 0, 21, 19)), RectU16::of(2, 0, 22, 21));
        assert_eq!(saturating_inflate(&RectU16::of(2, 0, 22, 21)), RectU16::of(1, 0, 23, 23));
        assert_eq!(saturating_inflate(&RectU16::of(1, 0, 23, 23)), RectU16::of(0, 0, 24, 25));
        assert_eq!(saturating_inflate(&RectU16::of(0, 0, 24, 25)), RectU16::of(0, 0, 26, 27));
    }

    #[test]
    fn saturating_inflate_max_bounds() {
        assert_eq!(
            saturating_inflate(&RectU16::of(u16::MAX - 33, u16::MAX - 17, u16::MAX - 5, u16::MAX - 3)),
            RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2)
        );
        assert_eq!(
            saturating_inflate(&RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2)),
            RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1)
        );
        assert_eq!(
            saturating_inflate(&RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1)),
            RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX)),
            RectU16::of(u16::MAX - 37, u16::MAX - 22, u16::MAX - 1, u16::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectU16::of(u16::MAX - 37, u16::MAX - 22, u16::MAX - 1, u16::MAX)),
            RectU16::of(u16::MAX - 38, u16::MAX - 24, u16::MAX, u16::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectU16::of(u16::MAX - 38, u16::MAX - 24, u16::MAX, u16::MAX)),
            RectU16::of(u16::MAX - 40, u16::MAX - 26, u16::MAX, u16::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectU16::of(u16::MAX - 40, u16::MAX - 26, u16::MAX, u16::MAX)),
            RectU16::of(u16::MAX - 42, u16::MAX - 28, u16::MAX, u16::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectU16::of(u16::MAX - 42, u16::MAX - 28, u16::MAX, u16::MAX)),
            RectU16::of(u16::MAX - 44, u16::MAX - 30, u16::MAX, u16::MAX)
        );
    }
}
