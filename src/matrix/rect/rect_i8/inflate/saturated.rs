use crate::matrix::{point::point_i8::PointI8, rect::rect_i8::RectI8};

pub fn try_assign_inflate(r: &mut RectI8) -> Option<()> {
    let is_min_row = r.min.row == i8::MIN;
    let is_min_col = r.min.col == i8::MIN;
    let is_max_row = r.max.row == i8::MAX;
    let is_max_col = r.max.col == i8::MAX;
    if (is_min_row && is_max_row) || (is_min_col && is_max_col) {
        return None;
    }
    let min_row_modifier = 1 - i8::from(is_min_row) + i8::from(is_max_row);
    let min_col_modifier = 1 - i8::from(is_min_col) + i8::from(is_max_col);
    let max_row_modifier = 1 + i8::from(is_min_row) - i8::from(is_max_row);
    let max_col_modifier = 1 + i8::from(is_min_col) - i8::from(is_max_col);
    r.min.row = r.min.row.saturating_sub(min_row_modifier);
    r.min.col = r.min.col.saturating_sub(min_col_modifier);
    r.max.row = r.max.row.saturating_add(max_row_modifier);
    r.max.col = r.max.col.saturating_add(max_col_modifier);
    Some(())
}

pub fn try_inflate(r: &RectI8) -> Option<RectI8> {
    let is_min_row = r.min.row == i8::MIN;
    let is_min_col = r.min.col == i8::MIN;
    let is_max_row = r.max.row == i8::MAX;
    let is_max_col = r.max.col == i8::MAX;
    if (is_min_row && is_max_row) || (is_min_col && is_max_col) {
        return None;
    }
    let min_row_modifier = 1 - i8::from(is_min_row) + i8::from(is_max_row);
    let min_col_modifier = 1 - i8::from(is_min_col) + i8::from(is_max_col);
    let max_row_modifier = 1 + i8::from(is_min_row) - i8::from(is_max_row);
    let max_col_modifier = 1 + i8::from(is_min_col) - i8::from(is_max_col);
    let min_row = r.min.row.saturating_sub(min_row_modifier);
    let min_col = r.min.col.saturating_sub(min_col_modifier);
    let max_row = r.max.row.saturating_add(max_row_modifier);
    let max_col = r.max.col.saturating_add(max_col_modifier);
    Some(RectI8 { min: PointI8 { row: min_row, col: min_col }, max: PointI8 { row: max_row, col: max_col } })
}

pub fn assign_inflate(r: &mut RectI8) {
    try_assign_inflate(r).unwrap()
}

pub fn inflate(r: &RectI8) -> RectI8 {
    try_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::matrix::rect::rect_i8::RectI8;

    use super::{assign_inflate, inflate, try_assign_inflate, try_inflate};

    #[test]
    fn try_assign_inflate_min_bounds() {
        let mut r = RectI8::of(i8::MIN + 7, i8::MIN + 2, i8::MIN + 17, i8::MIN + 13);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN + 6, i8::MIN + 1, i8::MIN + 18, i8::MIN + 14));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN + 5, i8::MIN, i8::MIN + 19, i8::MIN + 15));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 20, i8::MIN + 17));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN + 3, i8::MIN, i8::MIN + 21, i8::MIN + 19));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN + 2, i8::MIN, i8::MIN + 22, i8::MIN + 21));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN + 1, i8::MIN, i8::MIN + 23, i8::MIN + 23));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 24, i8::MIN + 25));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 26, i8::MIN + 27));
    }

    #[test]
    fn try_assign_inflate_max_bounds() {
        let mut r = RectI8::of(i8::MAX - 33, i8::MAX - 17, i8::MAX - 5, i8::MAX - 3);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 37, i8::MAX - 22, i8::MAX - 1, i8::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 38, i8::MAX - 24, i8::MAX, i8::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 40, i8::MAX - 26, i8::MAX, i8::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 42, i8::MAX - 28, i8::MAX, i8::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 44, i8::MAX - 30, i8::MAX, i8::MAX));
    }

    #[test]
    fn try_assign_inflate_to_bounds() {
        let mut r_odd = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_odd), Some(()));
        assert_eq!(r_odd, RectI8::largest());

        let mut r_even_1 = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_even_1), Some(()));
        assert_eq!(r_even_1, RectI8::largest());

        let mut r_even_2 = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX);
        assert_eq!(try_assign_inflate(&mut r_even_2), Some(()));
        assert_eq!(r_even_2, RectI8::largest());
    }

    #[test]
    fn try_assign_inflate_width_to_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 1, i8::MIN + 10, i8::MIN + 20, i8::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectI8::of(i8::MIN, i8::MIN + 9, i8::MIN + 21, i8::MIN + 21));

        let mut r_max = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 1, i8::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MAX, i8::MIN + 21));
    }

    #[test]
    fn try_assign_inflate_height_to_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 10, i8::MIN + 1, i8::MIN + 20, i8::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectI8::of(i8::MIN + 9, i8::MIN, i8::MIN + 21, i8::MIN + 21));

        let mut r_max = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MIN + 20, i8::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MIN + 21, i8::MAX));
    }

    #[test]
    fn try_assign_inflate_out_of_bounds() {
        let mut r = RectI8::largest();
        assert_eq!(try_assign_inflate(&mut r), None);
        assert_eq!(r, RectI8::largest());

        let mut r_row = RectI8::of(i8::MIN, i8::MIN + 10, i8::MAX, i8::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_row), None);
        assert_eq!(r_row, RectI8::of(i8::MIN, i8::MIN + 10, i8::MAX, i8::MIN + 20));

        let mut r_col = RectI8::of(i8::MIN + 10, i8::MIN, i8::MIN + 20, i8::MAX);
        assert_eq!(try_assign_inflate(&mut r_col), None);
        assert_eq!(r_col, RectI8::of(i8::MIN + 10, i8::MIN, i8::MIN + 20, i8::MAX));
    }

    #[test]
    fn try_inflate_min_bounds() {
        assert_eq!(
            try_inflate(&RectI8::of(i8::MIN + 7, i8::MIN + 2, i8::MIN + 17, i8::MIN + 13)),
            Some(RectI8::of(i8::MIN + 6, i8::MIN + 1, i8::MIN + 18, i8::MIN + 14))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MIN + 6, i8::MIN + 1, i8::MIN + 18, i8::MIN + 14)),
            Some(RectI8::of(i8::MIN + 5, i8::MIN, i8::MIN + 19, i8::MIN + 15))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MIN + 5, i8::MIN, i8::MIN + 19, i8::MIN + 15)),
            Some(RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 20, i8::MIN + 17))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 20, i8::MIN + 17)),
            Some(RectI8::of(i8::MIN + 3, i8::MIN, i8::MIN + 21, i8::MIN + 19))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MIN + 3, i8::MIN, i8::MIN + 21, i8::MIN + 19)),
            Some(RectI8::of(i8::MIN + 2, i8::MIN, i8::MIN + 22, i8::MIN + 21))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MIN + 2, i8::MIN, i8::MIN + 22, i8::MIN + 21)),
            Some(RectI8::of(i8::MIN + 1, i8::MIN, i8::MIN + 23, i8::MIN + 23))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MIN + 1, i8::MIN, i8::MIN + 23, i8::MIN + 23)),
            Some(RectI8::of(i8::MIN, i8::MIN, i8::MIN + 24, i8::MIN + 25))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MIN, i8::MIN, i8::MIN + 24, i8::MIN + 25)),
            Some(RectI8::of(i8::MIN, i8::MIN, i8::MIN + 26, i8::MIN + 27))
        );
    }

    #[test]
    fn try_inflate_max_bounds() {
        assert_eq!(
            try_inflate(&RectI8::of(i8::MAX - 33, i8::MAX - 17, i8::MAX - 5, i8::MAX - 3)),
            Some(RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2)),
            Some(RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1)),
            Some(RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX)),
            Some(RectI8::of(i8::MAX - 37, i8::MAX - 22, i8::MAX - 1, i8::MAX))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MAX - 37, i8::MAX - 22, i8::MAX - 1, i8::MAX)),
            Some(RectI8::of(i8::MAX - 38, i8::MAX - 24, i8::MAX, i8::MAX))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MAX - 38, i8::MAX - 24, i8::MAX, i8::MAX)),
            Some(RectI8::of(i8::MAX - 40, i8::MAX - 26, i8::MAX, i8::MAX))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MAX - 40, i8::MAX - 26, i8::MAX, i8::MAX)),
            Some(RectI8::of(i8::MAX - 42, i8::MAX - 28, i8::MAX, i8::MAX))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MAX - 42, i8::MAX - 28, i8::MAX, i8::MAX)),
            Some(RectI8::of(i8::MAX - 44, i8::MAX - 30, i8::MAX, i8::MAX))
        );
    }

    #[test]
    fn try_inflate_to_bounds() {
        assert_eq!(try_inflate(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1)), Some(RectI8::largest()));
        assert_eq!(try_inflate(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1)), Some(RectI8::largest()));
        assert_eq!(try_inflate(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX)), Some(RectI8::largest()));
    }

    #[test]
    fn try_inflate_width_to_bounds() {
        assert_eq!(
            try_inflate(&RectI8::of(i8::MIN + 1, i8::MIN + 10, i8::MIN + 20, i8::MIN + 20)),
            Some(RectI8::of(i8::MIN, i8::MIN + 9, i8::MIN + 21, i8::MIN + 21))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 1, i8::MIN + 20)),
            Some(RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MAX, i8::MIN + 21))
        );
    }

    #[test]
    fn try_inflate_height_to_bounds() {
        assert_eq!(
            try_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 1, i8::MIN + 20, i8::MIN + 20)),
            Some(RectI8::of(i8::MIN + 9, i8::MIN, i8::MIN + 21, i8::MIN + 21))
        );
        assert_eq!(
            try_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MIN + 20, i8::MAX - 1)),
            Some(RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MIN + 21, i8::MAX))
        );
    }

    #[test]
    fn try_inflate_out_of_bounds() {
        assert_eq!(try_inflate(&RectI8::largest()), None);
        assert_eq!(try_inflate(&RectI8::of(i8::MIN, i8::MIN + 10, i8::MAX, i8::MIN + 20)), None);
        assert_eq!(try_inflate(&RectI8::of(i8::MIN + 10, i8::MIN, i8::MIN + 20, i8::MAX)), None);
    }

    #[test]
    fn assign_inflate_min_bounds() {
        let mut r = RectI8::of(i8::MIN + 7, i8::MIN + 2, i8::MIN + 17, i8::MIN + 13);
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 6, i8::MIN + 1, i8::MIN + 18, i8::MIN + 14));
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 5, i8::MIN, i8::MIN + 19, i8::MIN + 15));
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 20, i8::MIN + 17));
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 3, i8::MIN, i8::MIN + 21, i8::MIN + 19));
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 2, i8::MIN, i8::MIN + 22, i8::MIN + 21));
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 1, i8::MIN, i8::MIN + 23, i8::MIN + 23));
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 24, i8::MIN + 25));
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 26, i8::MIN + 27));
    }

    #[test]
    fn assign_inflate_max_bounds() {
        let mut r = RectI8::of(i8::MAX - 33, i8::MAX - 17, i8::MAX - 5, i8::MAX - 3);
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2));
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1));
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 37, i8::MAX - 22, i8::MAX - 1, i8::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 38, i8::MAX - 24, i8::MAX, i8::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 40, i8::MAX - 26, i8::MAX, i8::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 42, i8::MAX - 28, i8::MAX, i8::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 44, i8::MAX - 30, i8::MAX, i8::MAX));
    }

    #[test]
    fn inflate_min_bounds() {
        assert_eq!(
            inflate(&RectI8::of(i8::MIN + 7, i8::MIN + 2, i8::MIN + 17, i8::MIN + 13)),
            RectI8::of(i8::MIN + 6, i8::MIN + 1, i8::MIN + 18, i8::MIN + 14)
        );
        assert_eq!(
            inflate(&RectI8::of(i8::MIN + 6, i8::MIN + 1, i8::MIN + 18, i8::MIN + 14)),
            RectI8::of(i8::MIN + 5, i8::MIN, i8::MIN + 19, i8::MIN + 15)
        );
        assert_eq!(
            inflate(&RectI8::of(i8::MIN + 5, i8::MIN, i8::MIN + 19, i8::MIN + 15)),
            RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 20, i8::MIN + 17)
        );
        assert_eq!(
            inflate(&RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 20, i8::MIN + 17)),
            RectI8::of(i8::MIN + 3, i8::MIN, i8::MIN + 21, i8::MIN + 19)
        );
        assert_eq!(
            inflate(&RectI8::of(i8::MIN + 3, i8::MIN, i8::MIN + 21, i8::MIN + 19)),
            RectI8::of(i8::MIN + 2, i8::MIN, i8::MIN + 22, i8::MIN + 21)
        );
        assert_eq!(
            inflate(&RectI8::of(i8::MIN + 2, i8::MIN, i8::MIN + 22, i8::MIN + 21)),
            RectI8::of(i8::MIN + 1, i8::MIN, i8::MIN + 23, i8::MIN + 23)
        );
        assert_eq!(inflate(&RectI8::of(i8::MIN + 1, i8::MIN, i8::MIN + 23, i8::MIN + 23)), RectI8::of(i8::MIN, i8::MIN, i8::MIN + 24, i8::MIN + 25));
        assert_eq!(inflate(&RectI8::of(i8::MIN, i8::MIN, i8::MIN + 24, i8::MIN + 25)), RectI8::of(i8::MIN, i8::MIN, i8::MIN + 26, i8::MIN + 27));
    }

    #[test]
    fn inflate_max_bounds() {
        assert_eq!(
            inflate(&RectI8::of(i8::MAX - 33, i8::MAX - 17, i8::MAX - 5, i8::MAX - 3)),
            RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2)
        );
        assert_eq!(
            inflate(&RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2)),
            RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1)
        );
        assert_eq!(
            inflate(&RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1)),
            RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX)
        );
        assert_eq!(
            inflate(&RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX)),
            RectI8::of(i8::MAX - 37, i8::MAX - 22, i8::MAX - 1, i8::MAX)
        );
        assert_eq!(inflate(&RectI8::of(i8::MAX - 37, i8::MAX - 22, i8::MAX - 1, i8::MAX)), RectI8::of(i8::MAX - 38, i8::MAX - 24, i8::MAX, i8::MAX));
        assert_eq!(inflate(&RectI8::of(i8::MAX - 38, i8::MAX - 24, i8::MAX, i8::MAX)), RectI8::of(i8::MAX - 40, i8::MAX - 26, i8::MAX, i8::MAX));
        assert_eq!(inflate(&RectI8::of(i8::MAX - 40, i8::MAX - 26, i8::MAX, i8::MAX)), RectI8::of(i8::MAX - 42, i8::MAX - 28, i8::MAX, i8::MAX));
        assert_eq!(inflate(&RectI8::of(i8::MAX - 42, i8::MAX - 28, i8::MAX, i8::MAX)), RectI8::of(i8::MAX - 44, i8::MAX - 30, i8::MAX, i8::MAX));
    }
}
