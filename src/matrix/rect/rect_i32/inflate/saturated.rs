use crate::matrix::{point::point_i32::PointI32, rect::rect_i32::RectI32};

pub fn try_saturating_inflate_assign(r: &mut RectI32) -> Option<()> {
    let is_min_row = r.min.row == i32::MIN;
    let is_min_col = r.min.col == i32::MIN;
    let is_max_row = r.max.row == i32::MAX;
    let is_max_col = r.max.col == i32::MAX;
    if (is_min_row && is_max_row) || (is_min_col && is_max_col) {
        return None;
    }
    let min_row_modifier = 1 - i32::from(is_min_row) + i32::from(is_max_row);
    let min_col_modifier = 1 - i32::from(is_min_col) + i32::from(is_max_col);
    let max_row_modifier = 1 + i32::from(is_min_row) - i32::from(is_max_row);
    let max_col_modifier = 1 + i32::from(is_min_col) - i32::from(is_max_col);
    r.min.row = r.min.row.saturating_sub(min_row_modifier);
    r.min.col = r.min.col.saturating_sub(min_col_modifier);
    r.max.row = r.max.row.saturating_add(max_row_modifier);
    r.max.col = r.max.col.saturating_add(max_col_modifier);
    Some(())
}

pub fn try_saturating_inflate(r: &RectI32) -> Option<RectI32> {
    let is_min_row = r.min.row == i32::MIN;
    let is_min_col = r.min.col == i32::MIN;
    let is_max_row = r.max.row == i32::MAX;
    let is_max_col = r.max.col == i32::MAX;
    if (is_min_row && is_max_row) || (is_min_col && is_max_col) {
        return None;
    }
    let min_row_modifier = 1 - i32::from(is_min_row) + i32::from(is_max_row);
    let min_col_modifier = 1 - i32::from(is_min_col) + i32::from(is_max_col);
    let max_row_modifier = 1 + i32::from(is_min_row) - i32::from(is_max_row);
    let max_col_modifier = 1 + i32::from(is_min_col) - i32::from(is_max_col);
    let min_row = r.min.row.saturating_sub(min_row_modifier);
    let min_col = r.min.col.saturating_sub(min_col_modifier);
    let max_row = r.max.row.saturating_add(max_row_modifier);
    let max_col = r.max.col.saturating_add(max_col_modifier);
    Some(RectI32 { min: PointI32 { row: min_row, col: min_col }, max: PointI32 { row: max_row, col: max_col } })
}

pub fn saturating_inflate_assign(r: &mut RectI32) {
    try_saturating_inflate_assign(r).unwrap()
}

pub fn saturating_inflate(r: &RectI32) -> RectI32 {
    try_saturating_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{saturating_inflate, saturating_inflate_assign, try_saturating_inflate, try_saturating_inflate_assign};
    use crate::matrix::rect::rect_i32::RectI32;

    #[test]
    fn try_saturating_inflate_assign_min_bounds() {
        let mut r = RectI32::of(i32::MIN + 7, i32::MIN + 2, i32::MIN + 17, i32::MIN + 13);
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MIN + 6, i32::MIN + 1, i32::MIN + 18, i32::MIN + 14));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MIN + 5, i32::MIN, i32::MIN + 19, i32::MIN + 15));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MIN + 4, i32::MIN, i32::MIN + 20, i32::MIN + 17));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MIN + 3, i32::MIN, i32::MIN + 21, i32::MIN + 19));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 22, i32::MIN + 21));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MIN + 1, i32::MIN, i32::MIN + 23, i32::MIN + 23));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 24, i32::MIN + 25));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 26, i32::MIN + 27));
    }

    #[test]
    fn try_saturating_inflate_assign_max_bounds() {
        let mut r = RectI32::of(i32::MAX - 33, i32::MAX - 17, i32::MAX - 5, i32::MAX - 3);
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MAX - 34, i32::MAX - 18, i32::MAX - 4, i32::MAX - 2));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MAX - 35, i32::MAX - 19, i32::MAX - 3, i32::MAX - 1));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MAX - 36, i32::MAX - 20, i32::MAX - 2, i32::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MAX - 37, i32::MAX - 22, i32::MAX - 1, i32::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MAX - 38, i32::MAX - 24, i32::MAX, i32::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MAX - 40, i32::MAX - 26, i32::MAX, i32::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MAX - 42, i32::MAX - 28, i32::MAX, i32::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MAX - 44, i32::MAX - 30, i32::MAX, i32::MAX));
    }

    #[test]
    fn try_saturating_inflate_assign_to_bounds() {
        let mut r_odd = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_odd), Some(()));
        assert_eq!(r_odd, RectI32::largest());

        let mut r_even_1 = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_even_1), Some(()));
        assert_eq!(r_even_1, RectI32::largest());

        let mut r_even_2 = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX);
        assert_eq!(try_saturating_inflate_assign(&mut r_even_2), Some(()));
        assert_eq!(r_even_2, RectI32::largest());
    }

    #[test]
    fn try_saturating_inflate_assign_width_to_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 1, i32::MIN + 10, i32::MIN + 20, i32::MIN + 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectI32::of(i32::MIN, i32::MIN + 9, i32::MIN + 21, i32::MIN + 21));

        let mut r_max = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 1, i32::MIN + 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectI32::of(i32::MIN + 9, i32::MIN + 9, i32::MAX, i32::MIN + 21));
    }

    #[test]
    fn try_saturating_inflate_assign_height_to_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 10, i32::MIN + 1, i32::MIN + 20, i32::MIN + 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectI32::of(i32::MIN + 9, i32::MIN, i32::MIN + 21, i32::MIN + 21));

        let mut r_max = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MIN + 20, i32::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectI32::of(i32::MIN + 9, i32::MIN + 9, i32::MIN + 21, i32::MAX));
    }

    #[test]
    fn try_saturating_inflate_assign_out_of_bounds() {
        let mut r = RectI32::largest();
        assert_eq!(try_saturating_inflate_assign(&mut r), None);
        assert_eq!(r, RectI32::largest());

        let mut r_row = RectI32::of(i32::MIN, i32::MIN + 10, i32::MAX, i32::MIN + 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_row), None);
        assert_eq!(r_row, RectI32::of(i32::MIN, i32::MIN + 10, i32::MAX, i32::MIN + 20));

        let mut r_col = RectI32::of(i32::MIN + 10, i32::MIN, i32::MIN + 20, i32::MAX);
        assert_eq!(try_saturating_inflate_assign(&mut r_col), None);
        assert_eq!(r_col, RectI32::of(i32::MIN + 10, i32::MIN, i32::MIN + 20, i32::MAX));
    }

    #[test]
    fn try_saturating_inflate_min_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectI32::of(i32::MIN + 7, i32::MIN + 2, i32::MIN + 17, i32::MIN + 13)),
            Some(RectI32::of(i32::MIN + 6, i32::MIN + 1, i32::MIN + 18, i32::MIN + 14))
        );
        assert_eq!(
            try_saturating_inflate(&RectI32::of(i32::MIN + 6, i32::MIN + 1, i32::MIN + 18, i32::MIN + 14)),
            Some(RectI32::of(i32::MIN + 5, i32::MIN, i32::MIN + 19, i32::MIN + 15))
        );
        assert_eq!(
            try_saturating_inflate(&RectI32::of(i32::MIN + 5, i32::MIN, i32::MIN + 19, i32::MIN + 15)),
            Some(RectI32::of(i32::MIN + 4, i32::MIN, i32::MIN + 20, i32::MIN + 17))
        );
        assert_eq!(
            try_saturating_inflate(&RectI32::of(i32::MIN + 4, i32::MIN, i32::MIN + 20, i32::MIN + 17)),
            Some(RectI32::of(i32::MIN + 3, i32::MIN, i32::MIN + 21, i32::MIN + 19))
        );
        assert_eq!(
            try_saturating_inflate(&RectI32::of(i32::MIN + 3, i32::MIN, i32::MIN + 21, i32::MIN + 19)),
            Some(RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 22, i32::MIN + 21))
        );
        assert_eq!(
            try_saturating_inflate(&RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 22, i32::MIN + 21)),
            Some(RectI32::of(i32::MIN + 1, i32::MIN, i32::MIN + 23, i32::MIN + 23))
        );
        assert_eq!(
            try_saturating_inflate(&RectI32::of(i32::MIN + 1, i32::MIN, i32::MIN + 23, i32::MIN + 23)),
            Some(RectI32::of(i32::MIN, i32::MIN, i32::MIN + 24, i32::MIN + 25))
        );
        assert_eq!(
            try_saturating_inflate(&RectI32::of(i32::MIN, i32::MIN, i32::MIN + 24, i32::MIN + 25)),
            Some(RectI32::of(i32::MIN, i32::MIN, i32::MIN + 26, i32::MIN + 27))
        );
    }

    #[test]
    fn try_saturating_inflate_max_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectI32::of(i32::MAX - 33, i32::MAX - 17, i32::MAX - 5, i32::MAX - 3)),
            Some(RectI32::of(i32::MAX - 34, i32::MAX - 18, i32::MAX - 4, i32::MAX - 2))
        );
        assert_eq!(
            try_saturating_inflate(&RectI32::of(i32::MAX - 34, i32::MAX - 18, i32::MAX - 4, i32::MAX - 2)),
            Some(RectI32::of(i32::MAX - 35, i32::MAX - 19, i32::MAX - 3, i32::MAX - 1))
        );
        assert_eq!(
            try_saturating_inflate(&RectI32::of(i32::MAX - 35, i32::MAX - 19, i32::MAX - 3, i32::MAX - 1)),
            Some(RectI32::of(i32::MAX - 36, i32::MAX - 20, i32::MAX - 2, i32::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI32::of(i32::MAX - 36, i32::MAX - 20, i32::MAX - 2, i32::MAX)),
            Some(RectI32::of(i32::MAX - 37, i32::MAX - 22, i32::MAX - 1, i32::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI32::of(i32::MAX - 37, i32::MAX - 22, i32::MAX - 1, i32::MAX)),
            Some(RectI32::of(i32::MAX - 38, i32::MAX - 24, i32::MAX, i32::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI32::of(i32::MAX - 38, i32::MAX - 24, i32::MAX, i32::MAX)),
            Some(RectI32::of(i32::MAX - 40, i32::MAX - 26, i32::MAX, i32::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI32::of(i32::MAX - 40, i32::MAX - 26, i32::MAX, i32::MAX)),
            Some(RectI32::of(i32::MAX - 42, i32::MAX - 28, i32::MAX, i32::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI32::of(i32::MAX - 42, i32::MAX - 28, i32::MAX, i32::MAX)),
            Some(RectI32::of(i32::MAX - 44, i32::MAX - 30, i32::MAX, i32::MAX))
        );
    }

    #[test]
    fn try_saturating_inflate_to_bounds() {
        assert_eq!(try_saturating_inflate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1)), Some(RectI32::largest()));
        assert_eq!(try_saturating_inflate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1)), Some(RectI32::largest()));
        assert_eq!(try_saturating_inflate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX)), Some(RectI32::largest()));
    }

    #[test]
    fn try_saturating_inflate_width_to_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectI32::of(i32::MIN + 1, i32::MIN + 10, i32::MIN + 20, i32::MIN + 20)),
            Some(RectI32::of(i32::MIN, i32::MIN + 9, i32::MIN + 21, i32::MIN + 21))
        );
        assert_eq!(
            try_saturating_inflate(&RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 1, i32::MIN + 20)),
            Some(RectI32::of(i32::MIN + 9, i32::MIN + 9, i32::MAX, i32::MIN + 21))
        );
    }

    #[test]
    fn try_saturating_inflate_height_to_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectI32::of(i32::MIN + 10, i32::MIN + 1, i32::MIN + 20, i32::MIN + 20)),
            Some(RectI32::of(i32::MIN + 9, i32::MIN, i32::MIN + 21, i32::MIN + 21))
        );
        assert_eq!(
            try_saturating_inflate(&RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MIN + 20, i32::MAX - 1)),
            Some(RectI32::of(i32::MIN + 9, i32::MIN + 9, i32::MIN + 21, i32::MAX))
        );
    }

    #[test]
    fn try_saturating_inflate_out_of_bounds() {
        assert_eq!(try_saturating_inflate(&RectI32::largest()), None);
        assert_eq!(try_saturating_inflate(&RectI32::of(i32::MIN, i32::MIN + 10, i32::MAX, i32::MIN + 20)), None);
        assert_eq!(try_saturating_inflate(&RectI32::of(i32::MIN + 10, i32::MIN, i32::MIN + 20, i32::MAX)), None);
    }

    #[test]
    fn saturating_inflate_assign_min_bounds() {
        let mut r = RectI32::of(i32::MIN + 7, i32::MIN + 2, i32::MIN + 17, i32::MIN + 13);
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN + 6, i32::MIN + 1, i32::MIN + 18, i32::MIN + 14));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN + 5, i32::MIN, i32::MIN + 19, i32::MIN + 15));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN + 4, i32::MIN, i32::MIN + 20, i32::MIN + 17));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN + 3, i32::MIN, i32::MIN + 21, i32::MIN + 19));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 22, i32::MIN + 21));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN + 1, i32::MIN, i32::MIN + 23, i32::MIN + 23));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 24, i32::MIN + 25));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 26, i32::MIN + 27));
    }

    #[test]
    fn saturating_inflate_assign_max_bounds() {
        let mut r = RectI32::of(i32::MAX - 33, i32::MAX - 17, i32::MAX - 5, i32::MAX - 3);
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI32::of(i32::MAX - 34, i32::MAX - 18, i32::MAX - 4, i32::MAX - 2));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI32::of(i32::MAX - 35, i32::MAX - 19, i32::MAX - 3, i32::MAX - 1));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI32::of(i32::MAX - 36, i32::MAX - 20, i32::MAX - 2, i32::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI32::of(i32::MAX - 37, i32::MAX - 22, i32::MAX - 1, i32::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI32::of(i32::MAX - 38, i32::MAX - 24, i32::MAX, i32::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI32::of(i32::MAX - 40, i32::MAX - 26, i32::MAX, i32::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI32::of(i32::MAX - 42, i32::MAX - 28, i32::MAX, i32::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI32::of(i32::MAX - 44, i32::MAX - 30, i32::MAX, i32::MAX));
    }

    #[test]
    fn saturating_inflate_min_bounds() {
        assert_eq!(
            saturating_inflate(&RectI32::of(i32::MIN + 7, i32::MIN + 2, i32::MIN + 17, i32::MIN + 13)),
            RectI32::of(i32::MIN + 6, i32::MIN + 1, i32::MIN + 18, i32::MIN + 14)
        );
        assert_eq!(
            saturating_inflate(&RectI32::of(i32::MIN + 6, i32::MIN + 1, i32::MIN + 18, i32::MIN + 14)),
            RectI32::of(i32::MIN + 5, i32::MIN, i32::MIN + 19, i32::MIN + 15)
        );
        assert_eq!(
            saturating_inflate(&RectI32::of(i32::MIN + 5, i32::MIN, i32::MIN + 19, i32::MIN + 15)),
            RectI32::of(i32::MIN + 4, i32::MIN, i32::MIN + 20, i32::MIN + 17)
        );
        assert_eq!(
            saturating_inflate(&RectI32::of(i32::MIN + 4, i32::MIN, i32::MIN + 20, i32::MIN + 17)),
            RectI32::of(i32::MIN + 3, i32::MIN, i32::MIN + 21, i32::MIN + 19)
        );
        assert_eq!(
            saturating_inflate(&RectI32::of(i32::MIN + 3, i32::MIN, i32::MIN + 21, i32::MIN + 19)),
            RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 22, i32::MIN + 21)
        );
        assert_eq!(
            saturating_inflate(&RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 22, i32::MIN + 21)),
            RectI32::of(i32::MIN + 1, i32::MIN, i32::MIN + 23, i32::MIN + 23)
        );
        assert_eq!(
            saturating_inflate(&RectI32::of(i32::MIN + 1, i32::MIN, i32::MIN + 23, i32::MIN + 23)),
            RectI32::of(i32::MIN, i32::MIN, i32::MIN + 24, i32::MIN + 25)
        );
        assert_eq!(
            saturating_inflate(&RectI32::of(i32::MIN, i32::MIN, i32::MIN + 24, i32::MIN + 25)),
            RectI32::of(i32::MIN, i32::MIN, i32::MIN + 26, i32::MIN + 27)
        );
    }

    #[test]
    fn saturating_inflate_max_bounds() {
        assert_eq!(
            saturating_inflate(&RectI32::of(i32::MAX - 33, i32::MAX - 17, i32::MAX - 5, i32::MAX - 3)),
            RectI32::of(i32::MAX - 34, i32::MAX - 18, i32::MAX - 4, i32::MAX - 2)
        );
        assert_eq!(
            saturating_inflate(&RectI32::of(i32::MAX - 34, i32::MAX - 18, i32::MAX - 4, i32::MAX - 2)),
            RectI32::of(i32::MAX - 35, i32::MAX - 19, i32::MAX - 3, i32::MAX - 1)
        );
        assert_eq!(
            saturating_inflate(&RectI32::of(i32::MAX - 35, i32::MAX - 19, i32::MAX - 3, i32::MAX - 1)),
            RectI32::of(i32::MAX - 36, i32::MAX - 20, i32::MAX - 2, i32::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI32::of(i32::MAX - 36, i32::MAX - 20, i32::MAX - 2, i32::MAX)),
            RectI32::of(i32::MAX - 37, i32::MAX - 22, i32::MAX - 1, i32::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI32::of(i32::MAX - 37, i32::MAX - 22, i32::MAX - 1, i32::MAX)),
            RectI32::of(i32::MAX - 38, i32::MAX - 24, i32::MAX, i32::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI32::of(i32::MAX - 38, i32::MAX - 24, i32::MAX, i32::MAX)),
            RectI32::of(i32::MAX - 40, i32::MAX - 26, i32::MAX, i32::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI32::of(i32::MAX - 40, i32::MAX - 26, i32::MAX, i32::MAX)),
            RectI32::of(i32::MAX - 42, i32::MAX - 28, i32::MAX, i32::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI32::of(i32::MAX - 42, i32::MAX - 28, i32::MAX, i32::MAX)),
            RectI32::of(i32::MAX - 44, i32::MAX - 30, i32::MAX, i32::MAX)
        );
    }
}
