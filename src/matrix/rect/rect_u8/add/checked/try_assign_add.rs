use crate::matrix::rect::{rect_i8::RectI8, rect_u8::RectU8};

pub fn try_assign_add(r: &mut RectU8, delta: &RectI8) -> Option<()> {
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
    use super::try_assign_add;
    use crate::matrix::rect::{rect_i8::RectI8, rect_u8::RectU8};

    #[test]
    fn test_try_assign_add() {
        let mut r = RectU8::of(0, 0, 12, 12);
        assert_eq!(try_assign_add(&mut r, &RectI8::of(5, 4, 3, 2)), Some(()));
        assert_eq!(r, RectU8::of(5, 4, 15, 14));
        assert_eq!(try_assign_add(&mut r, &RectI8::of(-4, -3, -2, -1)), Some(()));
        assert_eq!(r, RectU8::of(1, 1, 13, 13));
    }

    #[test]
    fn try_assign_add_to_bounds() {
        let mut r = RectU8::of(2, 5, u8::MAX - 2, u8::MAX - 5);
        assert_eq!(try_assign_add(&mut r, &RectI8::of(-2, -5, 2, 5)), Some(()));
        assert_eq!(r, RectU8::largest());

        let mut min_r = RectU8::of(2, 5, u8::MAX, u8::MAX);
        assert_eq!(try_assign_add(&mut min_r, &RectI8::of(-2, -5, 0, 0)), Some(()));
        assert_eq!(min_r, RectU8::largest());

        let mut max_r = RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5);
        assert_eq!(try_assign_add(&mut max_r, &RectI8::of(0, 0, 2, 5)), Some(()));
        assert_eq!(max_r, RectU8::largest());
    }

    #[test]
    fn try_assign_add_edge_out_of_bounds() {
        let mut r = RectU8::largest();
        assert_eq!(try_assign_add(&mut r, &RectI8::of(-1, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r, &RectI8::of(0, -1, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r, &RectI8::of(0, 0, 1, 0)), None);
        assert_eq!(try_assign_add(&mut r, &RectI8::of(0, 0, 0, 1)), None);
        assert_eq!(r, RectU8::largest());
    }

    #[test]
    fn try_assign_add_out_of_bounds() {
        let mut r = RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10);
        assert_eq!(try_assign_add(&mut r, &RectI8::of(-20, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r, &RectI8::of(0, -20, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r, &RectI8::of(0, 0, 20, 0)), None);
        assert_eq!(try_assign_add(&mut r, &RectI8::of(0, 0, 0, 20)), None);
        assert_eq!(r, RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10));
    }

    #[test]
    fn try_assign_add_limits_out_of_bounds() {
        let mut r = RectU8::largest();
        assert_eq!(try_assign_add(&mut r, &RectI8::of(i8::MIN, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r, &RectI8::of(0, i8::MIN, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r, &RectI8::of(0, 0, i8::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut r, &RectI8::of(0, 0, 0, i8::MAX)), None);
        assert_eq!(r, RectU8::largest());
    }
}
