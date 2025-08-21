use crate::matrix::{point::point_i16::PointI16, rect::rect_i16::RectI16};

pub fn try_assign_inflate(r: &mut RectI16) -> Option<()> {
    let is_min_row = r.min.row == i16::MIN;
    let is_min_col = r.min.col == i16::MIN;
    let is_max_row = r.max.row == i16::MAX;
    let is_max_col = r.max.col == i16::MAX;
    if (is_min_row && is_max_row) || (is_min_col && is_max_col) {
        return None;
    }
    let min_row_modifier = 1 - i16::from(is_min_row) + i16::from(is_max_row);
    let min_col_modifier = 1 - i16::from(is_min_col) + i16::from(is_max_col);
    let max_row_modifier = 1 + i16::from(is_min_row) - i16::from(is_max_row);
    let max_col_modifier = 1 + i16::from(is_min_col) - i16::from(is_max_col);
    r.min.row = r.min.row.saturating_sub(min_row_modifier);
    r.min.col = r.min.col.saturating_sub(min_col_modifier);
    r.max.row = r.max.row.saturating_add(max_row_modifier);
    r.max.col = r.max.col.saturating_add(max_col_modifier);
    Some(())
}

pub fn try_inflate(r: &RectI16) -> Option<RectI16> {
    let is_min_row = r.min.row == i16::MIN;
    let is_min_col = r.min.col == i16::MIN;
    let is_max_row = r.max.row == i16::MAX;
    let is_max_col = r.max.col == i16::MAX;
    if (is_min_row && is_max_row) || (is_min_col && is_max_col) {
        return None;
    }
    let min_row_modifier = 1 - i16::from(is_min_row) + i16::from(is_max_row);
    let min_col_modifier = 1 - i16::from(is_min_col) + i16::from(is_max_col);
    let max_row_modifier = 1 + i16::from(is_min_row) - i16::from(is_max_row);
    let max_col_modifier = 1 + i16::from(is_min_col) - i16::from(is_max_col);
    let min_row = r.min.row.saturating_sub(min_row_modifier);
    let min_col = r.min.col.saturating_sub(min_col_modifier);
    let max_row = r.max.row.saturating_add(max_row_modifier);
    let max_col = r.max.col.saturating_add(max_col_modifier);
    Some(RectI16 { min: PointI16 { row: min_row, col: min_col }, max: PointI16 { row: max_row, col: max_col } })
}

pub fn assign_inflate(r: &mut RectI16) {
    try_assign_inflate(r).unwrap()
}

pub fn inflate(r: &RectI16) -> RectI16 {
    try_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::matrix::rect::rect_i16::RectI16;

    use super::{assign_inflate, inflate, try_assign_inflate, try_inflate};

    #[test]
    fn try_assign_inflate_min_bounds() {
        let mut r = RectI16::of(i16::MIN + 7, i16::MIN + 2, i16::MIN + 17, i16::MIN + 13);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 26, i16::MIN + 27));
    }

    #[test]
    fn try_assign_inflate_max_bounds() {
        let mut r = RectI16::of(i16::MAX - 33, i16::MAX - 17, i16::MAX - 5, i16::MAX - 3);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 44, i16::MAX - 30, i16::MAX, i16::MAX));
    }

    #[test]
    fn try_assign_inflate_to_bounds() {
        let mut r_odd = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_odd), Some(()));
        assert_eq!(r_odd, RectI16::largest());

        let mut r_even_1 = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_even_1), Some(()));
        assert_eq!(r_even_1, RectI16::largest());

        let mut r_even_2 = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX);
        assert_eq!(try_assign_inflate(&mut r_even_2), Some(()));
        assert_eq!(r_even_2, RectI16::largest());
    }

    #[test]
    fn try_assign_inflate_width_to_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 1, i16::MIN + 10, i16::MIN + 20, i16::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectI16::of(i16::MIN, i16::MIN + 9, i16::MIN + 21, i16::MIN + 21));

        let mut r_max = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 1, i16::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MAX, i16::MIN + 21));
    }

    #[test]
    fn try_assign_inflate_height_to_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 1, i16::MIN + 20, i16::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectI16::of(i16::MIN + 9, i16::MIN, i16::MIN + 21, i16::MIN + 21));

        let mut r_max = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MIN + 20, i16::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MIN + 21, i16::MAX));
    }

    #[test]
    fn try_assign_inflate_out_of_bounds() {
        let mut r = RectI16::largest();
        assert_eq!(try_assign_inflate(&mut r), None);
        assert_eq!(r, RectI16::largest());

        let mut r_row = RectI16::of(i16::MIN, i16::MIN + 10, i16::MAX, i16::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_row), None);
        assert_eq!(r_row, RectI16::of(i16::MIN, i16::MIN + 10, i16::MAX, i16::MIN + 20));

        let mut r_col = RectI16::of(i16::MIN + 10, i16::MIN, i16::MIN + 20, i16::MAX);
        assert_eq!(try_assign_inflate(&mut r_col), None);
        assert_eq!(r_col, RectI16::of(i16::MIN + 10, i16::MIN, i16::MIN + 20, i16::MAX));
    }

    #[test]
    fn try_inflate_min_bounds() {
        assert_eq!(
            try_inflate(&RectI16::of(i16::MIN + 7, i16::MIN + 2, i16::MIN + 17, i16::MIN + 13)),
            Some(RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14))
        );
        assert_eq!(
            try_inflate(&RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14)),
            Some(RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15))
        );
        assert_eq!(
            try_inflate(&RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15)),
            Some(RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17))
        );
        assert_eq!(
            try_inflate(&RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17)),
            Some(RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19))
        );
        assert_eq!(
            try_inflate(&RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19)),
            Some(RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21))
        );
        assert_eq!(
            try_inflate(&RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21)),
            Some(RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23))
        );
        assert_eq!(
            try_inflate(&RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23)),
            Some(RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25))
        );
        assert_eq!(
            try_inflate(&RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25)),
            Some(RectI16::of(i16::MIN, i16::MIN, i16::MIN + 26, i16::MIN + 27))
        );
    }

    #[test]
    fn try_inflate_max_bounds() {
        assert_eq!(
            try_inflate(&RectI16::of(i16::MAX - 33, i16::MAX - 17, i16::MAX - 5, i16::MAX - 3)),
            Some(RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2))
        );
        assert_eq!(
            try_inflate(&RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2)),
            Some(RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1))
        );
        assert_eq!(
            try_inflate(&RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1)),
            Some(RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX))
        );
        assert_eq!(
            try_inflate(&RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX)),
            Some(RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX))
        );
        assert_eq!(
            try_inflate(&RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX)),
            Some(RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX))
        );
        assert_eq!(
            try_inflate(&RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX)),
            Some(RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX))
        );
        assert_eq!(
            try_inflate(&RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX)),
            Some(RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX))
        );
        assert_eq!(
            try_inflate(&RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX)),
            Some(RectI16::of(i16::MAX - 44, i16::MAX - 30, i16::MAX, i16::MAX))
        );
    }

    #[test]
    fn try_inflate_to_bounds() {
        assert_eq!(try_inflate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1)), Some(RectI16::largest()));
        assert_eq!(try_inflate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1)), Some(RectI16::largest()));
        assert_eq!(try_inflate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX)), Some(RectI16::largest()));
    }

    #[test]
    fn try_inflate_width_to_bounds() {
        assert_eq!(
            try_inflate(&RectI16::of(i16::MIN + 1, i16::MIN + 10, i16::MIN + 20, i16::MIN + 20)),
            Some(RectI16::of(i16::MIN, i16::MIN + 9, i16::MIN + 21, i16::MIN + 21))
        );
        assert_eq!(
            try_inflate(&RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 1, i16::MIN + 20)),
            Some(RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MAX, i16::MIN + 21))
        );
    }

    #[test]
    fn try_inflate_height_to_bounds() {
        assert_eq!(
            try_inflate(&RectI16::of(i16::MIN + 10, i16::MIN + 1, i16::MIN + 20, i16::MIN + 20)),
            Some(RectI16::of(i16::MIN + 9, i16::MIN, i16::MIN + 21, i16::MIN + 21))
        );
        assert_eq!(
            try_inflate(&RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MIN + 20, i16::MAX - 1)),
            Some(RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MIN + 21, i16::MAX))
        );
    }

    #[test]
    fn try_inflate_out_of_bounds() {
        assert_eq!(try_inflate(&RectI16::largest()), None);
        assert_eq!(try_inflate(&RectI16::of(i16::MIN, i16::MIN + 10, i16::MAX, i16::MIN + 20)), None);
        assert_eq!(try_inflate(&RectI16::of(i16::MIN + 10, i16::MIN, i16::MIN + 20, i16::MAX)), None);
    }

    #[test]
    fn assign_inflate_min_bounds() {
        let mut r = RectI16::of(i16::MIN + 7, i16::MIN + 2, i16::MIN + 17, i16::MIN + 13);
        assign_inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14));
        assign_inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15));
        assign_inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17));
        assign_inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19));
        assign_inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21));
        assign_inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23));
        assign_inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25));
        assign_inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 26, i16::MIN + 27));
    }

    #[test]
    fn assign_inflate_max_bounds() {
        let mut r = RectI16::of(i16::MAX - 33, i16::MAX - 17, i16::MAX - 5, i16::MAX - 3);
        assign_inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2));
        assign_inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1));
        assign_inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 44, i16::MAX - 30, i16::MAX, i16::MAX));
    }

    #[test]
    fn inflate_min_bounds() {
        assert_eq!(
            inflate(&RectI16::of(i16::MIN + 7, i16::MIN + 2, i16::MIN + 17, i16::MIN + 13)),
            RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14)
        );
        assert_eq!(
            inflate(&RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14)),
            RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15)
        );
        assert_eq!(
            inflate(&RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15)),
            RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17)
        );
        assert_eq!(
            inflate(&RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17)),
            RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19)
        );
        assert_eq!(
            inflate(&RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19)),
            RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21)
        );
        assert_eq!(
            inflate(&RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21)),
            RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23)
        );
        assert_eq!(
            inflate(&RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23)),
            RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25)
        );
        assert_eq!(
            inflate(&RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25)),
            RectI16::of(i16::MIN, i16::MIN, i16::MIN + 26, i16::MIN + 27)
        );
    }

    #[test]
    fn inflate_max_bounds() {
        assert_eq!(
            inflate(&RectI16::of(i16::MAX - 33, i16::MAX - 17, i16::MAX - 5, i16::MAX - 3)),
            RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2)
        );
        assert_eq!(
            inflate(&RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2)),
            RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1)
        );
        assert_eq!(
            inflate(&RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1)),
            RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX)
        );
        assert_eq!(
            inflate(&RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX)),
            RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX)
        );
        assert_eq!(
            inflate(&RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX)),
            RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX)
        );
        assert_eq!(
            inflate(&RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX)),
            RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX)
        );
        assert_eq!(
            inflate(&RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX)),
            RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX)
        );
        assert_eq!(
            inflate(&RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX)),
            RectI16::of(i16::MAX - 44, i16::MAX - 30, i16::MAX, i16::MAX)
        );
    }
}
