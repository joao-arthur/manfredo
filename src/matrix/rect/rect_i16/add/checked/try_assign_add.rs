use crate::matrix::rect::rect_i16::RectI16;

pub fn try_checked_add_assign(r: &mut RectI16, delta: &RectI16) -> Option<()> {
    let min_row = r.min.row.checked_add(delta.min.row)?;
    let min_col = r.min.col.checked_add(delta.min.col)?;
    let max_row = r.max.row.checked_add(delta.max.row)?;
    let max_col = r.max.col.checked_add(delta.max.col)?;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Some(())
}

#[cfg(test)]
mod tests {
    use super::try_checked_add_assign;
    use crate::matrix::rect::rect_i16::RectI16;

    #[test]
    fn test_try_checked_add_assign() {
        let mut r = RectI16::of(0, 0, 12, 15);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(5, 4, 3, 2)), Some(()));
        assert_eq!(r, RectI16::of(5, 4, 15, 17));
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(-14, -13, -12, -11)), Some(()));
        assert_eq!(r, RectI16::of(-9, -9, 3, 6));
    }

    #[test]
    fn try_checked_add_assign_to_bounds() {
        let mut r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX - 2, i16::MAX - 5);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(-2, -5, 2, 5)), Some(()));
        assert_eq!(r, RectI16::largest());

        let mut r_min = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX);
        assert_eq!(try_checked_add_assign(&mut r_min, &RectI16::of(-2, -5, 0, 0)), Some(()));
        assert_eq!(r_min, RectI16::largest());

        let mut r_max = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5);
        assert_eq!(try_checked_add_assign(&mut r_max, &RectI16::of(0, 0, 2, 5)), Some(()));
        assert_eq!(r_max, RectI16::largest());
    }

    #[test]
    fn try_checked_add_assign_edge_out_of_bounds() {
        let mut r = RectI16::largest();
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(-1, 0, 0, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(0, -1, 0, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(0, 0, 1, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(0, 0, 0, 1)), None);
        assert_eq!(r, RectI16::largest());
    }

    #[test]
    fn try_checked_add_assign_out_of_bounds() {
        let mut r = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(-20, 0, 0, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(0, -20, 0, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(0, 0, 20, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(0, 0, 0, 20)), None);
        assert_eq!(r, RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10));
    }

    #[test]
    fn try_checked_add_assign_limits_out_of_bounds() {
        let mut r = RectI16::largest();
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(i16::MIN, 0, 0, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(0, i16::MIN, 0, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(0, 0, i16::MAX, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI16::of(0, 0, 0, i16::MAX)), None);
        assert_eq!(r, RectI16::largest());
    }
}
