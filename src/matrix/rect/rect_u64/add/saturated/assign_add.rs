use crate::matrix::rect::{rect_i64::RectI64, rect_u64::RectU64};

pub fn assign_add(r: &mut RectU64, delta: &RectI64) {
    r.min.row = r.min.row.saturating_add_signed(delta.min.row);
    r.min.col = r.min.col.saturating_add_signed(delta.min.col);
    r.max.row = r.max.row.saturating_add_signed(delta.max.row);
    r.max.col = r.max.col.saturating_add_signed(delta.max.col);
}

#[cfg(test)]
mod tests {
    use super::assign_add;
    use crate::matrix::rect::{rect_i64::RectI64, rect_u64::RectU64};

    #[test]
    fn test_assign_add() {
        let mut r = RectU64::of(0, 0, 12, 10);
        assign_add(&mut r, &RectI64::of(5, 4, 3, 2));
        assert_eq!(r, RectU64::of(5, 4, 15, 12));
        assign_add(&mut r, &RectI64::of(-4, -3, -2, -1));
        assert_eq!(r, RectU64::of(1, 1, 13, 11));
    }

    #[test]
    fn assign_add_to_bounds() {
        let mut r = RectU64::of(2, 5, u64::MAX - 2, u64::MAX - 5);
        assign_add(&mut r, &RectI64::of(-2, -5, 2, 5));
        assert_eq!(r, RectU64::largest());

        let mut r_min = RectU64::of(2, 5, u64::MAX, u64::MAX);
        assign_add(&mut r_min, &RectI64::of(-2, -5, 0, 0));
        assert_eq!(r_min, RectU64::largest());

        let mut r_max = RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5);
        assign_add(&mut r_max, &RectI64::of(0, 0, 2, 5));
        assert_eq!(r_max, RectU64::largest());
    }

    #[test]
    fn assign_add_edge_out_of_bounds() {
        let mut r = RectU64::largest();
        assign_add(&mut r, &RectI64::of(-1, 0, 0, 0));
        assert_eq!(r, RectU64::largest());
        assign_add(&mut r, &RectI64::of(0, -1, 0, 0));
        assert_eq!(r, RectU64::largest());
        assign_add(&mut r, &RectI64::of(0, 0, 1, 0));
        assert_eq!(r, RectU64::largest());
        assign_add(&mut r, &RectI64::of(0, 0, 0, 1));
        assert_eq!(r, RectU64::largest());
    }

    #[test]
    fn assign_add_out_of_bounds() {
        let mut r1 = RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10);
        assign_add(&mut r1, &RectI64::of(-20, 0, 0, 0));
        assert_eq!(r1, RectU64::of(0, 10, u64::MAX - 10, u64::MAX - 10));
        
        let mut r2 = RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10);
        assign_add(&mut r2, &RectI64::of(0, -20, 0, 0));
        assert_eq!(r2, RectU64::of(10, 0, u64::MAX - 10, u64::MAX - 10));

        let mut r3 = RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10);
        assign_add(&mut r3, &RectI64::of(0, 0, 20, 0));
        assert_eq!(r3, RectU64::of(10, 10, u64::MAX, u64::MAX - 10));

        let mut r4 = RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10);
        assign_add(&mut r4, &RectI64::of(0, 0, 0, 20));
        assert_eq!(r4, RectU64::of(10, 10, u64::MAX - 10, u64::MAX));
    }

    #[test]
    fn assign_add_limits_out_of_bounds() {
        let mut r = RectU64::largest();
        assign_add(&mut r, &RectI64::of(i64::MIN, 0, 0, 0));
        assert_eq!(r, RectU64::largest());
        assign_add(&mut r, &RectI64::of(0, i64::MIN, 0, 0));
        assert_eq!(r, RectU64::largest());
        assign_add(&mut r, &RectI64::of(0, 0, i64::MAX, 0));
        assert_eq!(r, RectU64::largest());
        assign_add(&mut r, &RectI64::of(0, 0, 0, i64::MAX));
        assert_eq!(r, RectU64::largest());
    }
}
