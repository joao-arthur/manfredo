use crate::matrix::{point::point_u8::PointU8, rect::rect_u8::RectU8};

pub fn try_saturating_inflate_assign(r: &mut RectU8) -> Option<()> {
    let is_min_row = r.min.row == 0;
    let is_min_col = r.min.col == 0;
    let is_max_row = r.max.row == u8::MAX;
    let is_max_col = r.max.col == u8::MAX;
    if (is_min_row && is_max_row) || (is_min_col && is_max_col) {
        return None;
    }
    let min_row_modifier = 1 - u8::from(is_min_row) + u8::from(is_max_row);
    let min_col_modifier = 1 - u8::from(is_min_col) + u8::from(is_max_col);
    let max_row_modifier = 1 + u8::from(is_min_row) - u8::from(is_max_row);
    let max_col_modifier = 1 + u8::from(is_min_col) - u8::from(is_max_col);
    r.min.row = r.min.row.saturating_sub(min_row_modifier);
    r.min.col = r.min.col.saturating_sub(min_col_modifier);
    r.max.row = r.max.row.saturating_add(max_row_modifier);
    r.max.col = r.max.col.saturating_add(max_col_modifier);
    Some(())
}

pub fn try_saturating_inflate(r: &RectU8) -> Option<RectU8> {
    let is_min_row = r.min.row == 0;
    let is_min_col = r.min.col == 0;
    let is_max_row = r.max.row == u8::MAX;
    let is_max_col = r.max.col == u8::MAX;
    if (is_min_row && is_max_row) || (is_min_col && is_max_col) {
        return None;
    }
    let min_row_modifier = 1 - u8::from(is_min_row) + u8::from(is_max_row);
    let min_col_modifier = 1 - u8::from(is_min_col) + u8::from(is_max_col);
    let max_row_modifier = 1 + u8::from(is_min_row) - u8::from(is_max_row);
    let max_col_modifier = 1 + u8::from(is_min_col) - u8::from(is_max_col);
    let min_row = r.min.row.saturating_sub(min_row_modifier);
    let min_col = r.min.col.saturating_sub(min_col_modifier);
    let max_row = r.max.row.saturating_add(max_row_modifier);
    let max_col = r.max.col.saturating_add(max_col_modifier);
    Some(RectU8 { min: PointU8 { row: min_row, col: min_col }, max: PointU8 { row: max_row, col: max_col } })
}

pub fn saturating_inflate_assign(r: &mut RectU8) {
    try_saturating_inflate_assign(r).unwrap()
}

pub fn saturating_inflate(r: &RectU8) -> RectU8 {
    try_saturating_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{saturating_inflate, saturating_inflate_assign, try_saturating_inflate, try_saturating_inflate_assign};
    use crate::matrix::rect::rect_u8::RectU8;

    #[test]
    fn try_saturating_inflate_assign_min_bounds() {
        let mut r = RectU8::of(7, 2, 17, 13);
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU8::of(6, 1, 18, 14));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU8::of(5, 0, 19, 15));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU8::of(4, 0, 20, 17));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU8::of(3, 0, 21, 19));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU8::of(2, 0, 22, 21));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU8::of(1, 0, 23, 23));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU8::of(0, 0, 24, 25));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU8::of(0, 0, 26, 27));
    }

    #[test]
    fn try_saturating_inflate_assign_max_bounds() {
        let mut r = RectU8::of(u8::MAX - 33, u8::MAX - 17, u8::MAX - 5, u8::MAX - 3);
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU8::of(u8::MAX - 34, u8::MAX - 18, u8::MAX - 4, u8::MAX - 2));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU8::of(u8::MAX - 35, u8::MAX - 19, u8::MAX - 3, u8::MAX - 1));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU8::of(u8::MAX - 36, u8::MAX - 20, u8::MAX - 2, u8::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU8::of(u8::MAX - 37, u8::MAX - 22, u8::MAX - 1, u8::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU8::of(u8::MAX - 38, u8::MAX - 24, u8::MAX, u8::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU8::of(u8::MAX - 40, u8::MAX - 26, u8::MAX, u8::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU8::of(u8::MAX - 42, u8::MAX - 28, u8::MAX, u8::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU8::of(u8::MAX - 44, u8::MAX - 30, u8::MAX, u8::MAX));
    }

    #[test]
    fn try_saturating_inflate_assign_to_bounds() {
        let mut r_odd = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_odd), Some(()));
        assert_eq!(r_odd, RectU8::largest());

        let mut r_even_1 = RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_even_1), Some(()));
        assert_eq!(r_even_1, RectU8::largest());

        let mut r_even_2 = RectU8::of(1, 1, u8::MAX, u8::MAX);
        assert_eq!(try_saturating_inflate_assign(&mut r_even_2), Some(()));
        assert_eq!(r_even_2, RectU8::largest());
    }

    #[test]
    fn try_saturating_inflate_assign_width_to_bounds() {
        let mut r_min = RectU8::of(1, 10, 20, 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectU8::of(0, 9, 21, 21));

        let mut r_max = RectU8::of(10, 10, u8::MAX - 1, 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectU8::of(9, 9, u8::MAX, 21));
    }

    #[test]
    fn try_saturating_inflate_assign_height_to_bounds() {
        let mut r_min = RectU8::of(10, 1, 20, 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectU8::of(9, 0, 21, 21));

        let mut r_max = RectU8::of(10, 10, 20, u8::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectU8::of(9, 9, 21, u8::MAX));
    }

    #[test]
    fn try_saturating_inflate_assign_out_of_bounds() {
        let mut r = RectU8::largest();
        assert_eq!(try_saturating_inflate_assign(&mut r), None);
        assert_eq!(r, RectU8::largest());

        let mut r_row = RectU8::of(0, 10, u8::MAX, 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_row), None);
        assert_eq!(r_row, RectU8::of(0, 10, u8::MAX, 20));

        let mut r_col = RectU8::of(10, 0, 20, u8::MAX);
        assert_eq!(try_saturating_inflate_assign(&mut r_col), None);
        assert_eq!(r_col, RectU8::of(10, 0, 20, u8::MAX));
    }

    #[test]
    fn try_saturating_inflate_min_bounds() {
        assert_eq!(try_saturating_inflate(&RectU8::of(7, 2, 17, 13)), Some(RectU8::of(6, 1, 18, 14)));
        assert_eq!(try_saturating_inflate(&RectU8::of(6, 1, 18, 14)), Some(RectU8::of(5, 0, 19, 15)));
        assert_eq!(try_saturating_inflate(&RectU8::of(5, 0, 19, 15)), Some(RectU8::of(4, 0, 20, 17)));
        assert_eq!(try_saturating_inflate(&RectU8::of(4, 0, 20, 17)), Some(RectU8::of(3, 0, 21, 19)));
        assert_eq!(try_saturating_inflate(&RectU8::of(3, 0, 21, 19)), Some(RectU8::of(2, 0, 22, 21)));
        assert_eq!(try_saturating_inflate(&RectU8::of(2, 0, 22, 21)), Some(RectU8::of(1, 0, 23, 23)));
        assert_eq!(try_saturating_inflate(&RectU8::of(1, 0, 23, 23)), Some(RectU8::of(0, 0, 24, 25)));
        assert_eq!(try_saturating_inflate(&RectU8::of(0, 0, 24, 25)), Some(RectU8::of(0, 0, 26, 27)));
    }

    #[test]
    fn try_saturating_inflate_max_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectU8::of(u8::MAX - 33, u8::MAX - 17, u8::MAX - 5, u8::MAX - 3)),
            Some(RectU8::of(u8::MAX - 34, u8::MAX - 18, u8::MAX - 4, u8::MAX - 2))
        );
        assert_eq!(
            try_saturating_inflate(&RectU8::of(u8::MAX - 34, u8::MAX - 18, u8::MAX - 4, u8::MAX - 2)),
            Some(RectU8::of(u8::MAX - 35, u8::MAX - 19, u8::MAX - 3, u8::MAX - 1))
        );
        assert_eq!(
            try_saturating_inflate(&RectU8::of(u8::MAX - 35, u8::MAX - 19, u8::MAX - 3, u8::MAX - 1)),
            Some(RectU8::of(u8::MAX - 36, u8::MAX - 20, u8::MAX - 2, u8::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectU8::of(u8::MAX - 36, u8::MAX - 20, u8::MAX - 2, u8::MAX)),
            Some(RectU8::of(u8::MAX - 37, u8::MAX - 22, u8::MAX - 1, u8::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectU8::of(u8::MAX - 37, u8::MAX - 22, u8::MAX - 1, u8::MAX)),
            Some(RectU8::of(u8::MAX - 38, u8::MAX - 24, u8::MAX, u8::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectU8::of(u8::MAX - 38, u8::MAX - 24, u8::MAX, u8::MAX)),
            Some(RectU8::of(u8::MAX - 40, u8::MAX - 26, u8::MAX, u8::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectU8::of(u8::MAX - 40, u8::MAX - 26, u8::MAX, u8::MAX)),
            Some(RectU8::of(u8::MAX - 42, u8::MAX - 28, u8::MAX, u8::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectU8::of(u8::MAX - 42, u8::MAX - 28, u8::MAX, u8::MAX)),
            Some(RectU8::of(u8::MAX - 44, u8::MAX - 30, u8::MAX, u8::MAX))
        );
    }

    #[test]
    fn try_saturating_inflate_to_bounds() {
        assert_eq!(try_saturating_inflate(&RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1)), Some(RectU8::largest()));
        assert_eq!(try_saturating_inflate(&RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1)), Some(RectU8::largest()));
        assert_eq!(try_saturating_inflate(&RectU8::of(1, 1, u8::MAX, u8::MAX)), Some(RectU8::largest()));
    }

    #[test]
    fn try_saturating_inflate_width_to_bounds() {
        assert_eq!(try_saturating_inflate(&RectU8::of(1, 10, 20, 20)), Some(RectU8::of(0, 9, 21, 21)));
        assert_eq!(try_saturating_inflate(&RectU8::of(10, 10, u8::MAX - 1, 20)), Some(RectU8::of(9, 9, u8::MAX, 21)));
    }

    #[test]
    fn try_saturating_inflate_height_to_bounds() {
        assert_eq!(try_saturating_inflate(&RectU8::of(10, 1, 20, 20)), Some(RectU8::of(9, 0, 21, 21)));
        assert_eq!(try_saturating_inflate(&RectU8::of(10, 10, 20, u8::MAX - 1)), Some(RectU8::of(9, 9, 21, u8::MAX)));
    }

    #[test]
    fn try_saturating_inflate_out_of_bounds() {
        assert_eq!(try_saturating_inflate(&RectU8::largest()), None);
        assert_eq!(try_saturating_inflate(&RectU8::of(0, 10, u8::MAX, 20)), None);
        assert_eq!(try_saturating_inflate(&RectU8::of(10, 0, 20, u8::MAX)), None);
    }

    #[test]
    fn saturating_inflate_assign_min_bounds() {
        let mut r = RectU8::of(7, 2, 17, 13);
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU8::of(6, 1, 18, 14));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU8::of(5, 0, 19, 15));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU8::of(4, 0, 20, 17));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU8::of(3, 0, 21, 19));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU8::of(2, 0, 22, 21));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU8::of(1, 0, 23, 23));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU8::of(0, 0, 24, 25));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU8::of(0, 0, 26, 27));
    }

    #[test]
    fn saturating_inflate_assign_max_bounds() {
        let mut r = RectU8::of(u8::MAX - 33, u8::MAX - 17, u8::MAX - 5, u8::MAX - 3);
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU8::of(u8::MAX - 34, u8::MAX - 18, u8::MAX - 4, u8::MAX - 2));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU8::of(u8::MAX - 35, u8::MAX - 19, u8::MAX - 3, u8::MAX - 1));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU8::of(u8::MAX - 36, u8::MAX - 20, u8::MAX - 2, u8::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU8::of(u8::MAX - 37, u8::MAX - 22, u8::MAX - 1, u8::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU8::of(u8::MAX - 38, u8::MAX - 24, u8::MAX, u8::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU8::of(u8::MAX - 40, u8::MAX - 26, u8::MAX, u8::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU8::of(u8::MAX - 42, u8::MAX - 28, u8::MAX, u8::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU8::of(u8::MAX - 44, u8::MAX - 30, u8::MAX, u8::MAX));
    }

    #[test]
    fn saturating_inflate_min_bounds() {
        assert_eq!(saturating_inflate(&RectU8::of(7, 2, 17, 13)), RectU8::of(6, 1, 18, 14));
        assert_eq!(saturating_inflate(&RectU8::of(6, 1, 18, 14)), RectU8::of(5, 0, 19, 15));
        assert_eq!(saturating_inflate(&RectU8::of(5, 0, 19, 15)), RectU8::of(4, 0, 20, 17));
        assert_eq!(saturating_inflate(&RectU8::of(4, 0, 20, 17)), RectU8::of(3, 0, 21, 19));
        assert_eq!(saturating_inflate(&RectU8::of(3, 0, 21, 19)), RectU8::of(2, 0, 22, 21));
        assert_eq!(saturating_inflate(&RectU8::of(2, 0, 22, 21)), RectU8::of(1, 0, 23, 23));
        assert_eq!(saturating_inflate(&RectU8::of(1, 0, 23, 23)), RectU8::of(0, 0, 24, 25));
        assert_eq!(saturating_inflate(&RectU8::of(0, 0, 24, 25)), RectU8::of(0, 0, 26, 27));
    }

    #[test]
    fn saturating_inflate_max_bounds() {
        assert_eq!(
            saturating_inflate(&RectU8::of(u8::MAX - 33, u8::MAX - 17, u8::MAX - 5, u8::MAX - 3)),
            RectU8::of(u8::MAX - 34, u8::MAX - 18, u8::MAX - 4, u8::MAX - 2)
        );
        assert_eq!(
            saturating_inflate(&RectU8::of(u8::MAX - 34, u8::MAX - 18, u8::MAX - 4, u8::MAX - 2)),
            RectU8::of(u8::MAX - 35, u8::MAX - 19, u8::MAX - 3, u8::MAX - 1)
        );
        assert_eq!(
            saturating_inflate(&RectU8::of(u8::MAX - 35, u8::MAX - 19, u8::MAX - 3, u8::MAX - 1)),
            RectU8::of(u8::MAX - 36, u8::MAX - 20, u8::MAX - 2, u8::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectU8::of(u8::MAX - 36, u8::MAX - 20, u8::MAX - 2, u8::MAX)),
            RectU8::of(u8::MAX - 37, u8::MAX - 22, u8::MAX - 1, u8::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectU8::of(u8::MAX - 37, u8::MAX - 22, u8::MAX - 1, u8::MAX)),
            RectU8::of(u8::MAX - 38, u8::MAX - 24, u8::MAX, u8::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectU8::of(u8::MAX - 38, u8::MAX - 24, u8::MAX, u8::MAX)),
            RectU8::of(u8::MAX - 40, u8::MAX - 26, u8::MAX, u8::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectU8::of(u8::MAX - 40, u8::MAX - 26, u8::MAX, u8::MAX)),
            RectU8::of(u8::MAX - 42, u8::MAX - 28, u8::MAX, u8::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectU8::of(u8::MAX - 42, u8::MAX - 28, u8::MAX, u8::MAX)),
            RectU8::of(u8::MAX - 44, u8::MAX - 30, u8::MAX, u8::MAX)
        );
    }
}
