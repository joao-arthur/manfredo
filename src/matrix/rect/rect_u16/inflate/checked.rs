use crate::matrix::{point::point_u16::PointU16, rect::rect_u16::RectU16};

pub fn try_checked_inflate_assign(r: &mut RectU16) -> Option<()> {
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

pub fn try_checked_inflate(r: &RectU16) -> Option<RectU16> {
    let min_row = r.min.row.checked_sub(1)?;
    let min_col = r.min.col.checked_sub(1)?;
    let max_row = r.max.row.checked_add(1)?;
    let max_col = r.max.col.checked_add(1)?;
    Some(RectU16 { min: PointU16 { row: min_row, col: min_col }, max: PointU16 { row: max_row, col: max_col } })
}

pub fn checked_inflate_assign(r: &mut RectU16) {
    try_checked_inflate_assign(r).unwrap()
}

pub fn checked_inflate(r: &RectU16) -> RectU16 {
    try_checked_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{checked_inflate_assign, checked_inflate, try_checked_inflate_assign, try_checked_inflate};
    use crate::matrix::rect::rect_u16::RectU16;

    #[test]
    fn try_checked_inflate_assign_min_bounds() {
        let mut r = RectU16::of(7, 3, 9, 13);
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(6, 2, 10, 14));
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(5, 1, 11, 15));
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(4, 0, 12, 16));
    }

    #[test]
    fn try_checked_inflate_assign_max_bounds() {
        let mut r = RectU16::of(u16::MAX - 33, u16::MAX - 17, u16::MAX - 5, u16::MAX - 3);
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2));
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1));
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX));
    }

    #[test]
    fn try_checked_inflate_assign_to_bounds() {
        let mut r = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU16::largest());
    }

    #[test]
    fn try_checked_inflate_assign_width_to_bounds() {
        let mut r_min = RectU16::of(1, 10, 20, 20);
        assert_eq!(try_checked_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectU16::of(0, 9, 21, 21));

        let mut r_max = RectU16::of(10, 10, u16::MAX - 1, 20);
        assert_eq!(try_checked_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectU16::of(9, 9, u16::MAX, 21));
    }

    #[test]
    fn try_checked_inflate_assign_height_to_bounds() {
        let mut r_min = RectU16::of(10, 1, 20, 20);
        assert_eq!(try_checked_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectU16::of(9, 0, 21, 21));

        let mut r_max = RectU16::of(10, 10, 20, u16::MAX - 1);
        assert_eq!(try_checked_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectU16::of(9, 9, 21, u16::MAX));
    }

    #[test]
    fn try_checked_inflate_assign_out_of_bounds() {
        let mut r_min_row = RectU16::of(0, 10, u16::MAX, 20);
        assert_eq!(try_checked_inflate_assign(&mut r_min_row), None);
        assert_eq!(r_min_row, RectU16::of(0, 10, u16::MAX, 20));

        let mut r_max_row = RectU16::of(10, 10, u16::MAX, 20);
        assert_eq!(try_checked_inflate_assign(&mut r_max_row), None);
        assert_eq!(r_max_row, RectU16::of(10, 10, u16::MAX, 20));

        let mut r_min_col = RectU16::of(10, 0, 20, 20);
        assert_eq!(try_checked_inflate_assign(&mut r_min_col), None);
        assert_eq!(r_min_col, RectU16::of(10, 0, 20, 20));

        let mut r_max_col = RectU16::of(10, 10, 20, u16::MAX);
        assert_eq!(try_checked_inflate_assign(&mut r_max_col), None);
        assert_eq!(r_max_col, RectU16::of(10, 10, 20, u16::MAX));

        let mut r_min = RectU16::of(0, 0, 10, 10);
        assert_eq!(try_checked_inflate_assign(&mut r_min), None);
        assert_eq!(r_min, RectU16::of(0, 0, 10, 10));

        let mut r_max = RectU16::of(10, 10, u16::MAX, u16::MAX);
        assert_eq!(try_checked_inflate_assign(&mut r_max), None);
        assert_eq!(r_max, RectU16::of(10, 10, u16::MAX, u16::MAX));
    }

    #[test]
    fn try_checked_inflate_assign_limits_out_of_bounds() {
        let mut r = RectU16::largest();
        assert_eq!(try_checked_inflate_assign(&mut r), None);
        assert_eq!(r, RectU16::largest());
    }

    #[test]
    fn try_checked_inflate_min_bounds() {
        assert_eq!(try_checked_inflate(&RectU16::of(7, 3, 9, 13)), Some(RectU16::of(6, 2, 10, 14)));
        assert_eq!(try_checked_inflate(&RectU16::of(6, 2, 10, 14)), Some(RectU16::of(5, 1, 11, 15)));
        assert_eq!(try_checked_inflate(&RectU16::of(5, 1, 11, 15)), Some(RectU16::of(4, 0, 12, 16)));
    }

    #[test]
    fn try_checked_inflate_max_bounds() {
        assert_eq!(
            try_checked_inflate(&RectU16::of(u16::MAX - 33, u16::MAX - 17, u16::MAX - 5, u16::MAX - 3)),
            Some(RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2))
        );
        assert_eq!(
            try_checked_inflate(&RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2)),
            Some(RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1))
        );
        assert_eq!(
            try_checked_inflate(&RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1)),
            Some(RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX))
        );
    }

    #[test]
    fn try_checked_inflate_to_bounds() {
        assert_eq!(try_checked_inflate(&RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1)), Some(RectU16::largest()));
    }

    #[test]
    fn try_checked_inflate_width_to_bounds() {
        assert_eq!(try_checked_inflate(&RectU16::of(1, 10, 20, 20)), Some(RectU16::of(0, 9, 21, 21)));
        assert_eq!(try_checked_inflate(&RectU16::of(10, 10, u16::MAX - 1, 20)), Some(RectU16::of(9, 9, u16::MAX, 21)));
    }

    #[test]
    fn try_checked_inflate_height_to_bounds() {
        assert_eq!(try_checked_inflate(&RectU16::of(10, 1, 20, 20)), Some(RectU16::of(9, 0, 21, 21)));
        assert_eq!(try_checked_inflate(&RectU16::of(10, 10, 20, u16::MAX - 1)), Some(RectU16::of(9, 9, 21, u16::MAX)));
    }

    #[test]
    fn try_checked_inflate_out_of_bounds() {
        assert_eq!(try_checked_inflate(&RectU16::of(0, 10, u16::MAX, 20)), None);
        assert_eq!(try_checked_inflate(&RectU16::of(10, 10, u16::MAX, 20)), None);
        assert_eq!(try_checked_inflate(&RectU16::of(10, 0, 20, 20)), None);
        assert_eq!(try_checked_inflate(&RectU16::of(10, 10, 20, u16::MAX)), None);
        assert_eq!(try_checked_inflate(&RectU16::of(0, 0, 10, 10)), None);
        assert_eq!(try_checked_inflate(&RectU16::of(10, 10, u16::MAX, u16::MAX)), None);
    }

    #[test]
    fn try_checked_inflate_limits_out_of_bounds() {
        assert_eq!(try_checked_inflate(&RectU16::largest()), None);
    }

    #[test]
    fn checked_inflate_assign_min_bounds() {
        let mut r = RectU16::of(7, 3, 9, 13);
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(6, 2, 10, 14));
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(5, 1, 11, 15));
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(4, 0, 12, 16));
    }

    #[test]
    fn checked_inflate_assign_max_bounds() {
        let mut r = RectU16::of(u16::MAX - 33, u16::MAX - 17, u16::MAX - 5, u16::MAX - 3);
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2));
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1));
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX));
    }

    #[test]
    fn checked_inflate_min_bounds() {
        assert_eq!(checked_inflate(&RectU16::of(7, 3, 9, 13)), RectU16::of(6, 2, 10, 14));
        assert_eq!(checked_inflate(&RectU16::of(6, 2, 10, 14)), RectU16::of(5, 1, 11, 15));
        assert_eq!(checked_inflate(&RectU16::of(5, 1, 11, 15)), RectU16::of(4, 0, 12, 16));
    }

    #[test]
    fn checked_inflate_max_bounds() {
        assert_eq!(
            checked_inflate(&RectU16::of(u16::MAX - 33, u16::MAX - 17, u16::MAX - 5, u16::MAX - 3)),
            RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2)
        );
        assert_eq!(
            checked_inflate(&RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2)),
            RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1)
        );
        assert_eq!(
            checked_inflate(&RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1)),
            RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX)
        );
    }
}
