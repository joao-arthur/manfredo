use crate::cartesian::rect::{rect_i32::RectI32, rect_u32::RectU32};

pub fn wrapping_add_assign(r: &mut RectU32, delta: &RectI32) {
    r.min.x = r.min.x.wrapping_add_signed(delta.min.x);
    r.min.y = r.min.y.wrapping_add_signed(delta.min.y);
    r.max.x = r.max.x.wrapping_add_signed(delta.max.x);
    r.max.y = r.max.y.wrapping_add_signed(delta.max.y);
}

#[cfg(test)]
mod tests {
    use super::wrapping_add_assign;
    use crate::cartesian::rect::{rect_i32::RectI32, rect_u32::RectU32};

    #[test]
    fn test_wrapping_add_assign() {
        let mut r = RectU32::of(0, 0, 12, 10);
        wrapping_add_assign(&mut r, &RectI32::of(5, 4, 3, 2));
        assert_eq!(r, RectU32::of(5, 4, 15, 12));
        wrapping_add_assign(&mut r, &RectI32::of(-4, -3, -2, -1));
        assert_eq!(r, RectU32::of(1, 1, 13, 11));
    }

    #[test]
    fn wrapping_add_assign_small_rect_to_bounds() {
        let mut min_r = RectU32::of(2, 5, 12, 15);
        wrapping_add_assign(&mut min_r, &RectI32::of(-2, -5, 9, 7));
        assert_eq!(min_r, RectU32::of(0, 0, 21, 22));

        let mut max_r = RectU32::of(u32::MAX - 12, u32::MAX - 15, u32::MAX - 2, u32::MAX - 5);
        wrapping_add_assign(&mut max_r, &RectI32::of(-9, -7, 2, 5));
        assert_eq!(max_r, RectU32::of(u32::MAX - 21, u32::MAX - 22, u32::MAX, u32::MAX));
    }

    #[test]
    fn wrapping_add_assign_big_rect_to_bounds() {
        let mut r = RectU32::of(2, 5, u32::MAX - 2, u32::MAX - 5);
        wrapping_add_assign(&mut r, &RectI32::of(-2, -5, 2, 5));
        assert_eq!(r, RectU32::largest());

        let mut min_r = RectU32::of(2, 5, u32::MAX, u32::MAX);
        wrapping_add_assign(&mut min_r, &RectI32::of(-2, -5, 0, 0));
        assert_eq!(min_r, RectU32::largest());

        let mut max_r = RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5);
        wrapping_add_assign(&mut max_r, &RectI32::of(0, 0, 2, 5));
        assert_eq!(max_r, RectU32::largest());
    }

    #[test]
    fn wrapping_add_assign_small_rect_out_of_bounds() {
        let mut r_min = RectU32::of(10, 5, 20, 30);
        wrapping_add_assign(&mut r_min, &RectI32::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectU32::of(u32::MAX - 9, u32::MAX - 14, 20, 30));

        let mut r_max = RectU32::of(u32::MAX - 20, u32::MAX - 30, u32::MAX - 5, u32::MAX - 10);
        wrapping_add_assign(&mut r_max, &RectI32::of(0, 0, 20, 20));
        assert_eq!(r_max, RectU32::of(u32::MAX - 20, u32::MAX - 30, 14, 9));
    }

    #[test]
    fn wrapping_add_assign_big_rect_out_of_bounds() {
        let mut r_min = RectU32::of(10, 5, u32::MAX, u32::MAX);
        wrapping_add_assign(&mut r_min, &RectI32::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectU32::of(u32::MAX - 9, u32::MAX - 14, u32::MAX, u32::MAX));

        let mut r_max = RectU32::of(0, 0, u32::MAX - 5, u32::MAX - 10);
        wrapping_add_assign(&mut r_max, &RectI32::of(0, 0, 20, 20));
        assert_eq!(r_max, RectU32::of(0, 0, 14, 9));
    }

    #[test]
    fn wrapping_add_assign_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU32::of(1, 1, 10, 10);
        wrapping_add_assign(&mut r_min, &RectI32::min());
        assert_eq!(r_min, RectU32::of((i32::MAX as u32) + 2, (i32::MAX as u32) + 2, (i32::MAX as u32) + 11, (i32::MAX as u32) + 11));

        let mut r_max = RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1);
        wrapping_add_assign(&mut r_max, &RectI32::max());
        assert_eq!(r_max, RectU32::of((i32::MAX as u32) - 11, (i32::MAX as u32) - 11, (i32::MAX as u32) - 2, (i32::MAX as u32) - 2));
    }

    #[test]
    fn wrapping_add_assign_big_rect_limits_out_of_bounds() {
        let mut r1 = RectU32::largest();
        wrapping_add_assign(&mut r1, &RectI32::min());
        assert_eq!(r1, RectU32::of((i32::MAX as u32) + 1, (i32::MAX as u32) + 1, i32::MAX as u32, i32::MAX as u32));

        let mut r2 = RectU32::largest();
        wrapping_add_assign(&mut r2, &RectI32::max());
        assert_eq!(r2, RectU32::of(i32::MAX as u32, i32::MAX as u32, (i32::MAX as u32) - 1, (i32::MAX as u32) - 1));

        let mut r_min = RectU32::of(1, 1, u32::MAX, u32::MAX);
        wrapping_add_assign(&mut r_min, &RectI32::min());
        assert_eq!(r_min, RectU32::of((i32::MAX as u32) + 2, (i32::MAX as u32) + 2, i32::MAX as u32, i32::MAX as u32));

        let mut r_max = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
        wrapping_add_assign(&mut r_max, &RectI32::max());
        assert_eq!(r_max, RectU32::of(i32::MAX as u32, i32::MAX as u32, (i32::MAX as u32) - 2, (i32::MAX as u32) - 2));
    }
}
