use crate::cartesian::rect::rect_i64::RectI64;

pub fn saturating_add_assign(r: &mut RectI64, delta: &RectI64) {
    r.min.x = r.min.x.saturating_add(delta.min.x);
    r.min.y = r.min.y.saturating_add(delta.min.y);
    r.max.x = r.max.x.saturating_add(delta.max.x);
    r.max.y = r.max.y.saturating_add(delta.max.y);
}

#[cfg(test)]
mod tests {
    use super::saturating_add_assign;
    use crate::cartesian::rect::rect_i64::RectI64;

    #[test]
    fn test_saturating_add_assign() {
        let mut r = RectI64::of(0, 0, 12, 15);
        saturating_add_assign(&mut r, &RectI64::of(5, 4, 3, 2));
        assert_eq!(r, RectI64::of(5, 4, 15, 17));
        saturating_add_assign(&mut r, &RectI64::of(-14, -13, -12, -11));
        assert_eq!(r, RectI64::of(-9, -9, 3, 6));
    }

    #[test]
    fn saturating_add_assign_to_bounds() {
        let mut r = RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX - 2, i64::MAX - 5);
        saturating_add_assign(&mut r, &RectI64::of(-2, -5, 2, 5));
        assert_eq!(r, RectI64::largest());

        let mut r_min = RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX);
        saturating_add_assign(&mut r_min, &RectI64::of(-2, -5, 0, 0));
        assert_eq!(r_min, RectI64::largest());

        let mut r_max = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5);
        saturating_add_assign(&mut r_max, &RectI64::of(0, 0, 2, 5));
        assert_eq!(r_max, RectI64::largest());
    }

    #[test]
    fn saturating_add_assign_edge_out_of_bounds() {
        let mut r = RectI64::largest();
        saturating_add_assign(&mut r, &RectI64::of(-1, 0, 0, 0));
        assert_eq!(r, RectI64::largest());
        saturating_add_assign(&mut r, &RectI64::of(0, -1, 0, 0));
        assert_eq!(r, RectI64::largest());
        saturating_add_assign(&mut r, &RectI64::of(0, 0, 1, 0));
        assert_eq!(r, RectI64::largest());
        saturating_add_assign(&mut r, &RectI64::of(0, 0, 0, 1));
        assert_eq!(r, RectI64::largest());
    }

    #[test]
    fn saturating_add_assign_out_of_bounds() {
        let mut r1 = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10);
        saturating_add_assign(&mut r1, &RectI64::of(-20, 0, 0, 0));
        assert_eq!(r1, RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10));

        let mut r2 = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10);
        saturating_add_assign(&mut r2, &RectI64::of(0, -20, 0, 0));
        assert_eq!(r2, RectI64::of(i64::MIN + 10, i64::MIN, i64::MAX - 10, i64::MAX - 10));

        let mut r3 = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10);
        saturating_add_assign(&mut r3, &RectI64::of(0, 0, 20, 0));
        assert_eq!(r3, RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX, i64::MAX - 10));

        let mut r4 = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10);
        saturating_add_assign(&mut r4, &RectI64::of(0, 0, 0, 20));
        assert_eq!(r4, RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX));
    }

    #[test]
    fn saturating_add_assign_limits_out_of_bounds() {
        let mut r = RectI64::largest();
        saturating_add_assign(&mut r, &RectI64::of(i64::MIN, 0, 0, 0));
        assert_eq!(r, RectI64::largest());
        saturating_add_assign(&mut r, &RectI64::of(0, i64::MIN, 0, 0));
        assert_eq!(r, RectI64::largest());
        saturating_add_assign(&mut r, &RectI64::of(0, 0, i64::MAX, 0));
        assert_eq!(r, RectI64::largest());
        saturating_add_assign(&mut r, &RectI64::of(0, 0, 0, i64::MAX));
        assert_eq!(r, RectI64::largest());
    }
}
