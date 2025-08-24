use crate::matrix::{
    point::{point_i16::PointI16, point_u16::PointU16},
    rect::rect_u16::RectU16,
};

pub fn try_checked_translate_assign(r: &mut RectU16, delta: &PointI16) -> Option<()> {
    let min_row = r.min.row.checked_add_signed(delta.row)?;
    let min_col = r.min.col.checked_add_signed(delta.col)?;
    let max_row = r.max.row.checked_add_signed(delta.row)?;
    let max_col = r.max.col.checked_add_signed(delta.col)?;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Some(())
}

pub fn try_checked_translate(r: &RectU16, delta: &PointI16) -> Option<RectU16> {
    let min_row = r.min.row.checked_add_signed(delta.row)?;
    let min_col = r.min.col.checked_add_signed(delta.col)?;
    let max_row = r.max.row.checked_add_signed(delta.row)?;
    let max_col = r.max.col.checked_add_signed(delta.col)?;
    Some(RectU16 { min: PointU16 { row: min_row, col: min_col }, max: PointU16 { row: max_row, col: max_col } })
}

pub fn checked_translate_assign(r: &mut RectU16, delta: &PointI16) {
    try_checked_translate_assign(r, delta).unwrap()
}

pub fn checked_translate(r: &RectU16, delta: &PointI16) -> RectU16 {
    try_checked_translate(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{checked_translate_assign, checked_translate, try_checked_translate_assign, try_checked_translate};
    use crate::matrix::{point::point_i16::PointI16, rect::rect_u16::RectU16};

    #[test]
    fn test_try_checked_translate_assign() {
        let mut r = RectU16::of(0, 0, 12, 15);
        assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(5, 4)), Some(()));
        assert_eq!(r, RectU16::of(5, 4, 17, 19));
        assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(-4, -2)), Some(()));
        assert_eq!(r, RectU16::of(1, 2, 13, 17));
    }

    #[test]
    fn try_checked_translate_assign_small_rect_to_bounds() {
        let mut min_r = RectU16::of(2, 5, 12, 15);
        assert_eq!(try_checked_translate_assign(&mut min_r, &PointI16::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectU16::of(0, 0, 10, 10));

        let mut max_r = RectU16::of(u16::MAX - 12, u16::MAX - 15, u16::MAX - 2, u16::MAX - 5);
        assert_eq!(try_checked_translate_assign(&mut max_r, &PointI16::of(2, 5)), Some(()));
        assert_eq!(max_r, RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX, u16::MAX));
    }

    #[test]
    fn try_checked_translate_assign_big_rect_to_bounds() {
        let mut min_r = RectU16::of(2, 5, u16::MAX, u16::MAX);
        assert_eq!(try_checked_translate_assign(&mut min_r, &PointI16::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5));

        let mut max_r = RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5);
        assert_eq!(try_checked_translate_assign(&mut max_r, &PointI16::of(2, 5)), Some(()));
        assert_eq!(max_r, RectU16::of(2, 5, u16::MAX, u16::MAX));
    }

    #[test]
    fn try_checked_translate_assign_small_rect_out_of_bounds() {
        let mut r_min = RectU16::of(10, 5, 20, 30);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI16::of(-20, -20)), None);
        assert_eq!(r_min, RectU16::of(10, 5, 20, 30));

        let mut r_max = RectU16::of(u16::MAX - 20, u16::MAX - 30, u16::MAX - 5, u16::MAX - 10);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI16::of(20, 20)), None);
        assert_eq!(r_max, RectU16::of(u16::MAX - 20, u16::MAX - 30, u16::MAX - 5, u16::MAX - 10));
    }

    #[test]
    fn try_checked_translate_assign_big_rect_out_of_bounds() {
        let mut r_min = RectU16::of(10, 5, u16::MAX, u16::MAX);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI16::of(-20, -20)), None);
        assert_eq!(r_min, RectU16::of(10, 5, u16::MAX, u16::MAX));

        let mut r_max = RectU16::of(0, 0, u16::MAX - 5, u16::MAX - 10);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI16::of(20, 20)), None);
        assert_eq!(r_max, RectU16::of(0, 0, u16::MAX - 5, u16::MAX - 10));
    }

    #[test]
    fn try_checked_translate_assign_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU16::of(1, 1, 10, 10);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI16::min()), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI16::of(i16::MIN, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI16::of(0, i16::MIN)), None);
        assert_eq!(r_min, RectU16::of(1, 1, 10, 10));

        let mut r_max = RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX - 1, u16::MAX - 1);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI16::max()), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI16::of(i16::MAX, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI16::of(0, i16::MAX)), None);
        assert_eq!(r_max, RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX - 1, u16::MAX - 1));
    }

    #[test]
    fn try_checked_translate_assign_big_rect_limits_out_of_bounds() {
        let mut r = RectU16::largest();
        assert_eq!(try_checked_translate_assign(&mut r, &PointI16::min()), None);
        assert_eq!(try_checked_translate_assign(&mut r, &PointI16::max()), None);
        assert_eq!(r, RectU16::largest());

        let mut r_min = RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI16::max()), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI16::of(i16::MAX, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI16::of(0, i16::MAX)), None);
        assert_eq!(r_min, RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1));

        let mut r_max = RectU16::of(1, 1, u16::MAX, u16::MAX);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI16::min()), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI16::of(i16::MIN, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI16::of(0, i16::MIN)), None);
        assert_eq!(r_max, RectU16::of(1, 1, u16::MAX, u16::MAX));
    }

    #[test]
    fn test_try_checked_translate() {
        assert_eq!(try_checked_translate(&RectU16::of(0, 0, 12, 15), &PointI16::of(5, 4)), Some(RectU16::of(5, 4, 17, 19)));
        assert_eq!(try_checked_translate(&RectU16::of(5, 4, 17, 19), &PointI16::of(-4, -2)), Some(RectU16::of(1, 2, 13, 17)));
    }

    #[test]
    fn try_checked_translate_small_rect_to_bounds() {
        assert_eq!(try_checked_translate(&RectU16::of(2, 5, 12, 15), &PointI16::of(-2, -5)), Some(RectU16::of(0, 0, 10, 10)));
        assert_eq!(
            try_checked_translate(&RectU16::of(u16::MAX - 12, u16::MAX - 15, u16::MAX - 2, u16::MAX - 5), &PointI16::of(2, 5)),
            Some(RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX, u16::MAX))
        );
    }

    #[test]
    fn try_checked_translate_big_rect_to_bounds() {
        assert_eq!(try_checked_translate(&RectU16::of(2, 5, u16::MAX, u16::MAX), &PointI16::of(-2, -5)), Some(RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5)));
        assert_eq!(try_checked_translate(&RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5), &PointI16::of(2, 5)), Some(RectU16::of(2, 5, u16::MAX, u16::MAX)));
    }

    #[test]
    fn try_checked_translate_small_rect_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectU16::of(10, 5, 20, 30), &PointI16::of(-20, -20)), None);
        assert_eq!(try_checked_translate(&RectU16::of(u16::MAX - 20, u16::MAX - 30, u16::MAX - 5, u16::MAX - 10), &PointI16::of(20, 20)), None);
    }

    #[test]
    fn try_checked_translate_big_rect_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectU16::of(10, 5, u16::MAX, u16::MAX), &PointI16::of(-20, -20)), None);
        assert_eq!(try_checked_translate(&RectU16::of(0, 0, u16::MAX - 5, u16::MAX - 10), &PointI16::of(20, 20)), None);
    }

    #[test]
    fn try_checked_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectU16::of(1, 1, 10, 10), &PointI16::min()), None);
        assert_eq!(try_checked_translate(&RectU16::of(1, 1, 10, 10), &PointI16::of(i16::MIN, 0)), None);
        assert_eq!(try_checked_translate(&RectU16::of(1, 1, 10, 10), &PointI16::of(0, i16::MIN)), None);
        assert_eq!(try_checked_translate(&RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX - 1, u16::MAX - 1), &PointI16::max()), None);
        assert_eq!(try_checked_translate(&RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX - 1, u16::MAX - 1), &PointI16::of(i16::MAX, 0)), None);
        assert_eq!(try_checked_translate(&RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX - 1, u16::MAX - 1), &PointI16::of(0, i16::MAX)), None);
    }

    #[test]
    fn try_checked_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectU16::largest(), &PointI16::min()), None);
        assert_eq!(try_checked_translate(&RectU16::largest(), &PointI16::max()), None);
        assert_eq!(try_checked_translate(&RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1), &PointI16::max()), None);
        assert_eq!(try_checked_translate(&RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1), &PointI16::of(i16::MAX, 0)), None);
        assert_eq!(try_checked_translate(&RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1), &PointI16::of(0, i16::MAX)), None);
        assert_eq!(try_checked_translate(&RectU16::of(1, 1, u16::MAX, u16::MAX), &PointI16::min()), None);
        assert_eq!(try_checked_translate(&RectU16::of(1, 1, u16::MAX, u16::MAX), &PointI16::of(i16::MIN, 0)), None);
        assert_eq!(try_checked_translate(&RectU16::of(1, 1, u16::MAX, u16::MAX), &PointI16::of(0, i16::MIN)), None);
    }

    #[test]
    fn test_checked_translate_assign() {
        let mut r = RectU16::of(0, 0, 12, 15);
        checked_translate_assign(&mut r, &PointI16::of(5, 4));
        assert_eq!(r, RectU16::of(5, 4, 17, 19));
        checked_translate_assign(&mut r, &PointI16::of(-4, -2));
        assert_eq!(r, RectU16::of(1, 2, 13, 17));
    }

    #[test]
    fn test_checked_translate() {
        assert_eq!(checked_translate(&RectU16::of(0, 0, 12, 15), &PointI16::of(5, 4)), RectU16::of(5, 4, 17, 19));
        assert_eq!(checked_translate(&RectU16::of(5, 4, 17, 19), &PointI16::of(-4, -2)), RectU16::of(1, 2, 13, 17));
    }
}
