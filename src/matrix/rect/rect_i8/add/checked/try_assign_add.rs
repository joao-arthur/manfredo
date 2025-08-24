use crate::matrix::rect::rect_i8::RectI8;

pub fn try_checked_add_assign(r: &mut RectI8, delta: &RectI8) -> Option<()> {
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
    use crate::matrix::rect::rect_i8::RectI8;

    #[test]
    fn test_try_checked_add_assign() {
        let mut r = RectI8::of(0, 0, 12, 15);
        assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(5, 4, 3, 2)), Some(()));
        assert_eq!(r, RectI8::of(5, 4, 15, 17));
        assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(-14, -13, -12, -11)), Some(()));
        assert_eq!(r, RectI8::of(-9, -9, 3, 6));
    }

    #[test]
    fn try_checked_add_assign_to_bounds() {
        let mut r = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX - 2, i8::MAX - 5);
        assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(-2, -5, 2, 5)), Some(()));
        assert_eq!(r, RectI8::largest());

        let mut r_min = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX);
        assert_eq!(try_checked_add_assign(&mut r_min, &RectI8::of(-2, -5, 0, 0)), Some(()));
        assert_eq!(r_min, RectI8::largest());

        let mut r_max = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5);
        assert_eq!(try_checked_add_assign(&mut r_max, &RectI8::of(0, 0, 2, 5)), Some(()));
        assert_eq!(r_max, RectI8::largest());
    }

    #[test]
    fn try_checked_add_assign_edge_out_of_bounds() {
        let mut r = RectI8::largest();
        assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(-1, 0, 0, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(0, -1, 0, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(0, 0, 1, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(0, 0, 0, 1)), None);
        assert_eq!(r, RectI8::largest());
    }

    #[test]
    fn try_checked_add_assign_out_of_bounds() {
        let mut r = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10);
        assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(-20, 0, 0, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(0, -20, 0, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(0, 0, 20, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(0, 0, 0, 20)), None);
        assert_eq!(r, RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10));
    }

    #[test]
    fn try_checked_add_assign_limits_out_of_bounds() {
        let mut r = RectI8::largest();
        assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(i8::MIN, 0, 0, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(0, i8::MIN, 0, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(0, 0, i8::MAX, 0)), None);
        assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(0, 0, 0, i8::MAX)), None);
        assert_eq!(r, RectI8::largest());
    }
}
