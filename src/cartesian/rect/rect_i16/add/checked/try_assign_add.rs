use crate::cartesian::rect::rect_i16::RectI16;

pub fn try_assign_add(r: &mut RectI16, delta: &RectI16) -> Option<()> {
    let min_x = r.min.x.checked_add(delta.min.x)?;
    let min_y = r.min.y.checked_add(delta.min.y)?;
    let max_x = r.max.x.checked_add(delta.max.x)?;
    let max_y = r.max.y.checked_add(delta.max.y)?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

#[cfg(test)]
mod tests {
    use super::try_assign_add;
    use crate::cartesian::rect::rect_i16::RectI16;

    #[test]
    fn test_try_assign_add() {
        let mut r = RectI16::of(0, 0, 12, 15);
        assert_eq!(try_assign_add(&mut r, &RectI16::of(5, 4, 3, 2)), Some(()));
        assert_eq!(r, RectI16::of(5, 4, 15, 17));
        assert_eq!(try_assign_add(&mut r, &RectI16::of(-14, -13, -12, -11)), Some(()));
        assert_eq!(r, RectI16::of(-9, -9, 3, 6));
    }

    #[test]
    fn try_assign_add_to_bounds() {
        let mut r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX - 2, i16::MAX - 5);
        assert_eq!(try_assign_add(&mut r, &RectI16::of(-2, -5, 2, 5)), Some(()));
        assert_eq!(r, RectI16::largest());

        let mut r_min = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX);
        assert_eq!(try_assign_add(&mut r_min, &RectI16::of(-2, -5, 0, 0)), Some(()));
        assert_eq!(r_min, RectI16::largest());

        let mut r_max = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5);
        assert_eq!(try_assign_add(&mut r_max, &RectI16::of(0, 0, 2, 5)), Some(()));
        assert_eq!(r_max, RectI16::largest());
    }

    #[test]
    fn try_assign_add_edge_out_of_bounds() {
        let mut r = RectI16::largest();
        assert_eq!(try_assign_add(&mut r, &RectI16::of(-1, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r, &RectI16::of(0, -1, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r, &RectI16::of(0, 0, 1, 0)), None);
        assert_eq!(try_assign_add(&mut r, &RectI16::of(0, 0, 0, 1)), None);
        assert_eq!(r, RectI16::largest());
    }

    #[test]
    fn try_assign_add_out_of_bounds() {
        let mut r = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10);
        assert_eq!(try_assign_add(&mut r, &RectI16::of(-20, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r, &RectI16::of(0, -20, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r, &RectI16::of(0, 0, 20, 0)), None);
        assert_eq!(try_assign_add(&mut r, &RectI16::of(0, 0, 0, 20)), None);
        assert_eq!(r, RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10));
    }

    #[test]
    fn try_assign_add_limits_out_of_bounds() {
        let mut r = RectI16::largest();
        assert_eq!(try_assign_add(&mut r, &RectI16::of(i16::MIN, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r, &RectI16::of(0, i16::MIN, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r, &RectI16::of(0, 0, i16::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut r, &RectI16::of(0, 0, 0, i16::MAX)), None);
        assert_eq!(r, RectI16::largest());
    }
}
