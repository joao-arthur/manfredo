use crate::matrix::rect::{rect_i64::RectI64, rect_u64::RectU64};

pub fn wrapping_add_assign(r: &mut RectU64, delta: &RectI64) {
    r.min.row = r.min.row.wrapping_add_signed(delta.min.row);
    r.min.col = r.min.col.wrapping_add_signed(delta.min.col);
    r.max.row = r.max.row.wrapping_add_signed(delta.max.row);
    r.max.col = r.max.col.wrapping_add_signed(delta.max.col);
}

#[cfg(test)]
mod tests {
    use super::wrapping_add_assign;
    use crate::matrix::rect::{rect_i64::RectI64, rect_u64::RectU64};

    #[test]
    fn test_wrapping_add_assign() {
        let mut r = RectU64::of(0, 0, 12, 10);
        wrapping_add_assign(&mut r, &RectI64::of(5, 4, 3, 2));
        assert_eq!(r, RectU64::of(5, 4, 15, 12));
        wrapping_add_assign(&mut r, &RectI64::of(-4, -3, -2, -1));
        assert_eq!(r, RectU64::of(1, 1, 13, 11));
    }

    #[test]
    fn wrapping_add_assign_small_rect_to_bounds() {
        let mut min_r = RectU64::of(2, 5, 12, 15);
        wrapping_add_assign(&mut min_r, &RectI64::of(-2, -5, 9, 7));
        assert_eq!(min_r, RectU64::of(0, 0, 21, 22));

        let mut max_r = RectU64::of(u64::MAX - 12, u64::MAX - 15, u64::MAX - 2, u64::MAX - 5);
        wrapping_add_assign(&mut max_r, &RectI64::of(-9, -7, 2, 5));
        assert_eq!(max_r, RectU64::of(u64::MAX - 21, u64::MAX - 22, u64::MAX, u64::MAX));
    }

    #[test]
    fn wrapping_add_assign_big_rect_to_bounds() {
        let mut r = RectU64::of(2, 5, u64::MAX - 2, u64::MAX - 5);
        wrapping_add_assign(&mut r, &RectI64::of(-2, -5, 2, 5));
        assert_eq!(r, RectU64::largest());

        let mut min_r = RectU64::of(2, 5, u64::MAX, u64::MAX);
        wrapping_add_assign(&mut min_r, &RectI64::of(-2, -5, 0, 0));
        assert_eq!(min_r, RectU64::largest());

        let mut max_r = RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5);
        wrapping_add_assign(&mut max_r, &RectI64::of(0, 0, 2, 5));
        assert_eq!(max_r, RectU64::largest());
    }

    #[test]
    fn wrapping_add_assign_small_rect_out_of_bounds() {
        let mut r_min = RectU64::of(10, 5, 20, 30);
        wrapping_add_assign(&mut r_min, &RectI64::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectU64::of(u64::MAX - 9, u64::MAX - 14, 20, 30));

        let mut r_max = RectU64::of(u64::MAX - 20, u64::MAX - 30, u64::MAX - 5, u64::MAX - 10);
        wrapping_add_assign(&mut r_max, &RectI64::of(0, 0, 20, 20));
        assert_eq!(r_max, RectU64::of(u64::MAX - 20, u64::MAX - 30, 14, 9));
    }

    #[test]
    fn wrapping_add_assign_big_rect_out_of_bounds() {
        let mut r_min = RectU64::of(10, 5, u64::MAX, u64::MAX);
        wrapping_add_assign(&mut r_min, &RectI64::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectU64::of(u64::MAX - 9, u64::MAX - 14, u64::MAX, u64::MAX));

        let mut r_max = RectU64::of(0, 0, u64::MAX - 5, u64::MAX - 10);
        wrapping_add_assign(&mut r_max, &RectI64::of(0, 0, 20, 20));
        assert_eq!(r_max, RectU64::of(0, 0, 14, 9));
    }

    #[test]
    fn wrapping_add_assign_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU64::of(1, 1, 10, 10);
        wrapping_add_assign(&mut r_min, &RectI64::min());
        assert_eq!(r_min, RectU64::of((i64::MAX as u64) + 2, (i64::MAX as u64) + 2, (i64::MAX as u64) + 11, (i64::MAX as u64) + 11));

        let mut r_max = RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1);
        wrapping_add_assign(&mut r_max, &RectI64::max());
        assert_eq!(r_max, RectU64::of((i64::MAX as u64) - 11, (i64::MAX as u64) - 11, (i64::MAX as u64) - 2, (i64::MAX as u64) - 2));
    }

    #[test]
    fn wrapping_add_assign_big_rect_limits_out_of_bounds() {
        let mut r1 = RectU64::largest();
        wrapping_add_assign(&mut r1, &RectI64::min());
        assert_eq!(r1, RectU64::of((i64::MAX as u64) + 1, (i64::MAX as u64) + 1, i64::MAX as u64, i64::MAX as u64));

        let mut r2 = RectU64::largest();
        wrapping_add_assign(&mut r2, &RectI64::max());
        assert_eq!(r2, RectU64::of(i64::MAX as u64, i64::MAX as u64, (i64::MAX as u64) - 1, (i64::MAX as u64) - 1));

        let mut r_min = RectU64::of(1, 1, u64::MAX, u64::MAX);
        wrapping_add_assign(&mut r_min, &RectI64::min());
        assert_eq!(r_min, RectU64::of((i64::MAX as u64) + 2, (i64::MAX as u64) + 2, i64::MAX as u64, i64::MAX as u64));

        let mut r_max = RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1);
        wrapping_add_assign(&mut r_max, &RectI64::max());
        assert_eq!(r_max, RectU64::of(i64::MAX as u64, i64::MAX as u64, (i64::MAX as u64) - 2, (i64::MAX as u64) - 2));
    }
}
