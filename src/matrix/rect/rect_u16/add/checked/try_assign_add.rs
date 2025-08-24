use crate::matrix::rect::{rect_i16::RectI16, rect_u16::RectU16};

pub fn try_checked_add_assign(r: &mut RectU16, delta: &RectI16) -> Option<()> {
    let min_row = r.min.row.checked_add_signed(delta.min.row)?;
    let min_col = r.min.col.checked_add_signed(delta.min.col)?;
    let max_row = r.max.row.checked_add_signed(delta.max.row)?;
    let max_col = r.max.col.checked_add_signed(delta.max.col)?;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Some(())
}

#[cfg(test)]
mod tests {
    use super::try_checked_add_assign;
    use crate::matrix::rect::{rect_i16::RectI16, rect_u16::RectU16};

    #[test]
    fn test_try_checked_add_assign() {
        let mut r = RectU16::of(0, 0, 12, 12);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(5, 4, 3, 2)), Some(()));
        assert_eq!(r, RectU16::of(5, 4, 15, 14));
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(-4, -3, -2, -1)), Some(()));
        assert_eq!(r, RectU16::of(1, 1, 13, 13));
    }

    #[test]
    fn try_checked_add_assign_to_bounds() {
        let mut r = RectU16::of(2, 5, u16::MAX - 2, u16::MAX - 5);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(-2, -5, 2, 5)), Some(()));
        assert_eq!(r, RectU16::largest());

        let mut min_r = RectU16::of(2, 5, u16::MAX, u16::MAX);
        assert_eq!(try_checked_add_assign(&mut min_r, &RectI16::of(-2, -5, 0, 0)), Some(()));
        assert_eq!(min_r, RectU16::largest());

        let mut max_r = RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5);
        assert_eq!(try_checked_add_assign(&mut max_r, &RectI16::of(0, 0, 2, 5)), Some(()));
        assert_eq!(max_r, RectU16::largest());
    }

    #[test]
    fn try_checked_add_assign_edge_out_of_bounds() {
        let mut r = RectU16::largest();
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(-1, 0, 0, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(0, -1, 0, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(0, 0, 1, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(0, 0, 0, 1)), None);
        assert_eq!(r, RectU16::largest());
    }

    #[test]
    fn try_checked_add_assign_out_of_bounds() {
        let mut r = RectU16::of(10, 10, u16::MAX - 10, u16::MAX - 10);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(-20, 0, 0, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(0, -20, 0, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(0, 0, 20, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(0, 0, 0, 20)), None);
        assert_eq!(r, RectU16::of(10, 10, u16::MAX - 10, u16::MAX - 10));
    }

    #[test]
    fn try_checked_add_assign_limits_out_of_bounds() {
        let mut r = RectU16::largest();
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(i16::MIN, 0, 0, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(0, i16::MIN, 0, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(0, 0, i16::MAX, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(0, 0, 0, i16::MAX)), None);
        assert_eq!(r, RectU16::largest());
    }
}
