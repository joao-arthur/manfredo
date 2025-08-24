use crate::matrix::{point::point_u32::PointU32, rect::rect_u32::RectU32};

pub fn try_assign_inflate(r: &mut RectU32) -> Option<()> {
    let is_min_row = r.min.row == 0;
    let is_min_col = r.min.col == 0;
    let is_max_row = r.max.row == u32::MAX;
    let is_max_col = r.max.col == u32::MAX;
    if (is_min_row && is_max_row) || (is_min_col && is_max_col) {
        return None;
    }
    let min_row_modifier = 1 - u32::from(is_min_row) + u32::from(is_max_row);
    let min_col_modifier = 1 - u32::from(is_min_col) + u32::from(is_max_col);
    let max_row_modifier = 1 + u32::from(is_min_row) - u32::from(is_max_row);
    let max_col_modifier = 1 + u32::from(is_min_col) - u32::from(is_max_col);
    r.min.row = r.min.row.saturating_sub(min_row_modifier);
    r.min.col = r.min.col.saturating_sub(min_col_modifier);
    r.max.row = r.max.row.saturating_add(max_row_modifier);
    r.max.col = r.max.col.saturating_add(max_col_modifier);
    Some(())
}

pub fn try_inflate(r: &RectU32) -> Option<RectU32> {
    let is_min_row = r.min.row == 0;
    let is_min_col = r.min.col == 0;
    let is_max_row = r.max.row == u32::MAX;
    let is_max_col = r.max.col == u32::MAX;
    if (is_min_row && is_max_row) || (is_min_col && is_max_col) {
        return None;
    }
    let min_row_modifier = 1 - u32::from(is_min_row) + u32::from(is_max_row);
    let min_col_modifier = 1 - u32::from(is_min_col) + u32::from(is_max_col);
    let max_row_modifier = 1 + u32::from(is_min_row) - u32::from(is_max_row);
    let max_col_modifier = 1 + u32::from(is_min_col) - u32::from(is_max_col);
    let min_row = r.min.row.saturating_sub(min_row_modifier);
    let min_col = r.min.col.saturating_sub(min_col_modifier);
    let max_row = r.max.row.saturating_add(max_row_modifier);
    let max_col = r.max.col.saturating_add(max_col_modifier);
    Some(RectU32 { min: PointU32 { row: min_row, col: min_col }, max: PointU32 { row: max_row, col: max_col } })
}

pub fn assign_inflate(r: &mut RectU32) {
    try_assign_inflate(r).unwrap()
}

pub fn inflate(r: &RectU32) -> RectU32 {
    try_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{assign_inflate, inflate, try_assign_inflate, try_inflate};
    use crate::matrix::rect::rect_u32::RectU32;

    #[test]
    fn try_assign_inflate_min_bounds() {
        let mut r = RectU32::of(7, 2, 17, 13);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(6, 1, 18, 14));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(5, 0, 19, 15));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(4, 0, 20, 17));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(3, 0, 21, 19));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(2, 0, 22, 21));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(1, 0, 23, 23));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(0, 0, 24, 25));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(0, 0, 26, 27));
    }

    #[test]
    fn try_assign_inflate_max_bounds() {
        let mut r = RectU32::of(u32::MAX - 33, u32::MAX - 17, u32::MAX - 5, u32::MAX - 3);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 37, u32::MAX - 22, u32::MAX - 1, u32::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 38, u32::MAX - 24, u32::MAX, u32::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 40, u32::MAX - 26, u32::MAX, u32::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 42, u32::MAX - 28, u32::MAX, u32::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 44, u32::MAX - 30, u32::MAX, u32::MAX));
    }

    #[test]
    fn try_assign_inflate_to_bounds() {
        let mut r_odd = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_odd), Some(()));
        assert_eq!(r_odd, RectU32::largest());

        let mut r_even_1 = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_even_1), Some(()));
        assert_eq!(r_even_1, RectU32::largest());

        let mut r_even_2 = RectU32::of(1, 1, u32::MAX, u32::MAX);
        assert_eq!(try_assign_inflate(&mut r_even_2), Some(()));
        assert_eq!(r_even_2, RectU32::largest());
    }

    #[test]
    fn try_assign_inflate_width_to_bounds() {
        let mut r_min = RectU32::of(1, 10, 20, 20);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectU32::of(0, 9, 21, 21));

        let mut r_max = RectU32::of(10, 10, u32::MAX - 1, 20);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectU32::of(9, 9, u32::MAX, 21));
    }

    #[test]
    fn try_assign_inflate_height_to_bounds() {
        let mut r_min = RectU32::of(10, 1, 20, 20);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectU32::of(9, 0, 21, 21));

        let mut r_max = RectU32::of(10, 10, 20, u32::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectU32::of(9, 9, 21, u32::MAX));
    }

    #[test]
    fn try_assign_inflate_out_of_bounds() {
        let mut r = RectU32::largest();
        assert_eq!(try_assign_inflate(&mut r), None);
        assert_eq!(r, RectU32::largest());

        let mut r_row = RectU32::of(0, 10, u32::MAX, 20);
        assert_eq!(try_assign_inflate(&mut r_row), None);
        assert_eq!(r_row, RectU32::of(0, 10, u32::MAX, 20));

        let mut r_col = RectU32::of(10, 0, 20, u32::MAX);
        assert_eq!(try_assign_inflate(&mut r_col), None);
        assert_eq!(r_col, RectU32::of(10, 0, 20, u32::MAX));
    }

    #[test]
    fn try_inflate_min_bounds() {
        assert_eq!(try_inflate(&RectU32::of(7, 2, 17, 13)), Some(RectU32::of(6, 1, 18, 14)));
        assert_eq!(try_inflate(&RectU32::of(6, 1, 18, 14)), Some(RectU32::of(5, 0, 19, 15)));
        assert_eq!(try_inflate(&RectU32::of(5, 0, 19, 15)), Some(RectU32::of(4, 0, 20, 17)));
        assert_eq!(try_inflate(&RectU32::of(4, 0, 20, 17)), Some(RectU32::of(3, 0, 21, 19)));
        assert_eq!(try_inflate(&RectU32::of(3, 0, 21, 19)), Some(RectU32::of(2, 0, 22, 21)));
        assert_eq!(try_inflate(&RectU32::of(2, 0, 22, 21)), Some(RectU32::of(1, 0, 23, 23)));
        assert_eq!(try_inflate(&RectU32::of(1, 0, 23, 23)), Some(RectU32::of(0, 0, 24, 25)));
        assert_eq!(try_inflate(&RectU32::of(0, 0, 24, 25)), Some(RectU32::of(0, 0, 26, 27)));
    }

    #[test]
    fn try_inflate_max_bounds() {
        assert_eq!(
            try_inflate(&RectU32::of(u32::MAX - 33, u32::MAX - 17, u32::MAX - 5, u32::MAX - 3)),
            Some(RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2))
        );
        assert_eq!(
            try_inflate(&RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2)),
            Some(RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1))
        );
        assert_eq!(
            try_inflate(&RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1)),
            Some(RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX))
        );
        assert_eq!(
            try_inflate(&RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX)),
            Some(RectU32::of(u32::MAX - 37, u32::MAX - 22, u32::MAX - 1, u32::MAX))
        );
        assert_eq!(
            try_inflate(&RectU32::of(u32::MAX - 37, u32::MAX - 22, u32::MAX - 1, u32::MAX)),
            Some(RectU32::of(u32::MAX - 38, u32::MAX - 24, u32::MAX, u32::MAX))
        );
        assert_eq!(
            try_inflate(&RectU32::of(u32::MAX - 38, u32::MAX - 24, u32::MAX, u32::MAX)),
            Some(RectU32::of(u32::MAX - 40, u32::MAX - 26, u32::MAX, u32::MAX))
        );
        assert_eq!(
            try_inflate(&RectU32::of(u32::MAX - 40, u32::MAX - 26, u32::MAX, u32::MAX)),
            Some(RectU32::of(u32::MAX - 42, u32::MAX - 28, u32::MAX, u32::MAX))
        );
        assert_eq!(
            try_inflate(&RectU32::of(u32::MAX - 42, u32::MAX - 28, u32::MAX, u32::MAX)),
            Some(RectU32::of(u32::MAX - 44, u32::MAX - 30, u32::MAX, u32::MAX))
        );
    }

    #[test]
    fn try_inflate_to_bounds() {
        assert_eq!(try_inflate(&RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1)), Some(RectU32::largest()));
        assert_eq!(try_inflate(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1)), Some(RectU32::largest()));
        assert_eq!(try_inflate(&RectU32::of(1, 1, u32::MAX, u32::MAX)), Some(RectU32::largest()));
    }

    #[test]
    fn try_inflate_width_to_bounds() {
        assert_eq!(try_inflate(&RectU32::of(1, 10, 20, 20)), Some(RectU32::of(0, 9, 21, 21)));
        assert_eq!(try_inflate(&RectU32::of(10, 10, u32::MAX - 1, 20)), Some(RectU32::of(9, 9, u32::MAX, 21)));
    }

    #[test]
    fn try_inflate_height_to_bounds() {
        assert_eq!(try_inflate(&RectU32::of(10, 1, 20, 20)), Some(RectU32::of(9, 0, 21, 21)));
        assert_eq!(try_inflate(&RectU32::of(10, 10, 20, u32::MAX - 1)), Some(RectU32::of(9, 9, 21, u32::MAX)));
    }

    #[test]
    fn try_inflate_out_of_bounds() {
        assert_eq!(try_inflate(&RectU32::largest()), None);
        assert_eq!(try_inflate(&RectU32::of(0, 10, u32::MAX, 20)), None);
        assert_eq!(try_inflate(&RectU32::of(10, 0, 20, u32::MAX)), None);
    }

    #[test]
    fn assign_inflate_min_bounds() {
        let mut r = RectU32::of(7, 2, 17, 13);
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(6, 1, 18, 14));
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(5, 0, 19, 15));
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(4, 0, 20, 17));
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(3, 0, 21, 19));
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(2, 0, 22, 21));
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(1, 0, 23, 23));
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(0, 0, 24, 25));
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(0, 0, 26, 27));
    }

    #[test]
    fn assign_inflate_max_bounds() {
        let mut r = RectU32::of(u32::MAX - 33, u32::MAX - 17, u32::MAX - 5, u32::MAX - 3);
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2));
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1));
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(u32::MAX - 37, u32::MAX - 22, u32::MAX - 1, u32::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(u32::MAX - 38, u32::MAX - 24, u32::MAX, u32::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(u32::MAX - 40, u32::MAX - 26, u32::MAX, u32::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(u32::MAX - 42, u32::MAX - 28, u32::MAX, u32::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectU32::of(u32::MAX - 44, u32::MAX - 30, u32::MAX, u32::MAX));
    }

    #[test]
    fn inflate_min_bounds() {
        assert_eq!(inflate(&RectU32::of(7, 2, 17, 13)), RectU32::of(6, 1, 18, 14));
        assert_eq!(inflate(&RectU32::of(6, 1, 18, 14)), RectU32::of(5, 0, 19, 15));
        assert_eq!(inflate(&RectU32::of(5, 0, 19, 15)), RectU32::of(4, 0, 20, 17));
        assert_eq!(inflate(&RectU32::of(4, 0, 20, 17)), RectU32::of(3, 0, 21, 19));
        assert_eq!(inflate(&RectU32::of(3, 0, 21, 19)), RectU32::of(2, 0, 22, 21));
        assert_eq!(inflate(&RectU32::of(2, 0, 22, 21)), RectU32::of(1, 0, 23, 23));
        assert_eq!(inflate(&RectU32::of(1, 0, 23, 23)), RectU32::of(0, 0, 24, 25));
        assert_eq!(inflate(&RectU32::of(0, 0, 24, 25)), RectU32::of(0, 0, 26, 27));
    }

    #[test]
    fn inflate_max_bounds() {
        assert_eq!(
            inflate(&RectU32::of(u32::MAX - 33, u32::MAX - 17, u32::MAX - 5, u32::MAX - 3)),
            RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2)
        );
        assert_eq!(
            inflate(&RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2)),
            RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1)
        );
        assert_eq!(
            inflate(&RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1)),
            RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX)
        );
        assert_eq!(
            inflate(&RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX)),
            RectU32::of(u32::MAX - 37, u32::MAX - 22, u32::MAX - 1, u32::MAX)
        );
        assert_eq!(
            inflate(&RectU32::of(u32::MAX - 37, u32::MAX - 22, u32::MAX - 1, u32::MAX)),
            RectU32::of(u32::MAX - 38, u32::MAX - 24, u32::MAX, u32::MAX)
        );
        assert_eq!(
            inflate(&RectU32::of(u32::MAX - 38, u32::MAX - 24, u32::MAX, u32::MAX)),
            RectU32::of(u32::MAX - 40, u32::MAX - 26, u32::MAX, u32::MAX)
        );
        assert_eq!(
            inflate(&RectU32::of(u32::MAX - 40, u32::MAX - 26, u32::MAX, u32::MAX)),
            RectU32::of(u32::MAX - 42, u32::MAX - 28, u32::MAX, u32::MAX)
        );
        assert_eq!(
            inflate(&RectU32::of(u32::MAX - 42, u32::MAX - 28, u32::MAX, u32::MAX)),
            RectU32::of(u32::MAX - 44, u32::MAX - 30, u32::MAX, u32::MAX)
        );
    }
}
