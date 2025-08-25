use crate::matrix::rect::{rect_i8::RectI8, rect_u8::RectU8};

pub fn wrapping_add_assign(r: &mut RectU8, delta: &RectI8) {
    r.min.row = r.min.row.wrapping_add_signed(delta.min.row);
    r.min.col = r.min.col.wrapping_add_signed(delta.min.col);
    r.max.row = r.max.row.wrapping_add_signed(delta.max.row);
    r.max.col = r.max.col.wrapping_add_signed(delta.max.col);
}

#[cfg(test)]
mod tests {
    use super::wrapping_add_assign;
    use crate::matrix::rect::{rect_i8::RectI8, rect_u8::RectU8};

    #[test]
    fn test_wrapping_add_assign() {
        let mut r = RectU8::of(0, 0, 12, 10);
        wrapping_add_assign(&mut r, &RectI8::of(5, 4, 3, 2));
        assert_eq!(r, RectU8::of(5, 4, 15, 12));
        wrapping_add_assign(&mut r, &RectI8::of(-4, -3, -2, -1));
        assert_eq!(r, RectU8::of(1, 1, 13, 11));
    }

    #[test]
    fn wrapping_add_assign_small_rect_to_bounds() {
        let mut min_r = RectU8::of(2, 5, 12, 15);
        wrapping_add_assign(&mut min_r, &RectI8::of(-2, -5, 9, 7));
        assert_eq!(min_r, RectU8::of(0, 0, 21, 22));

        let mut max_r = RectU8::of(u8::MAX - 12, u8::MAX - 15, u8::MAX - 2, u8::MAX - 5);
        wrapping_add_assign(&mut max_r, &RectI8::of(-9, -7, 2, 5));
        assert_eq!(max_r, RectU8::of(u8::MAX - 21, u8::MAX - 22, u8::MAX, u8::MAX));
    }

    #[test]
    fn wrapping_add_assign_big_rect_to_bounds() {
        let mut r = RectU8::of(2, 5, u8::MAX - 2, u8::MAX - 5);
        wrapping_add_assign(&mut r, &RectI8::of(-2, -5, 2, 5));
        assert_eq!(r, RectU8::largest());

        let mut min_r = RectU8::of(2, 5, u8::MAX, u8::MAX);
        wrapping_add_assign(&mut min_r, &RectI8::of(-2, -5, 0, 0));
        assert_eq!(min_r, RectU8::largest());

        let mut max_r = RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5);
        wrapping_add_assign(&mut max_r, &RectI8::of(0, 0, 2, 5));
        assert_eq!(max_r, RectU8::largest());
    }

    #[test]
    fn wrapping_add_assign_small_rect_out_of_bounds() {
        let mut r_min = RectU8::of(10, 5, 20, 30);
        wrapping_add_assign(&mut r_min, &RectI8::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectU8::of(u8::MAX - 9, u8::MAX - 14, 20, 30));

        let mut r_max = RectU8::of(u8::MAX - 20, u8::MAX - 30, u8::MAX - 5, u8::MAX - 10);
        wrapping_add_assign(&mut r_max, &RectI8::of(0, 0, 20, 20));
        assert_eq!(r_max, RectU8::of(u8::MAX - 20, u8::MAX - 30, 14, 9));
    }

    #[test]
    fn wrapping_add_assign_big_rect_out_of_bounds() {
        let mut r_min = RectU8::of(10, 5, u8::MAX, u8::MAX);
        wrapping_add_assign(&mut r_min, &RectI8::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectU8::of(u8::MAX - 9, u8::MAX - 14, u8::MAX, u8::MAX));

        let mut r_max = RectU8::of(0, 0, u8::MAX - 5, u8::MAX - 10);
        wrapping_add_assign(&mut r_max, &RectI8::of(0, 0, 20, 20));
        assert_eq!(r_max, RectU8::of(0, 0, 14, 9));
    }

    #[test]
    fn wrapping_add_assign_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU8::of(1, 1, 10, 10);
        wrapping_add_assign(&mut r_min, &RectI8::min());
        assert_eq!(r_min, RectU8::of((i8::MAX as u8) + 2, (i8::MAX as u8) + 2, (i8::MAX as u8) + 11, (i8::MAX as u8) + 11));

        let mut r_max = RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX - 1, u8::MAX - 1);
        wrapping_add_assign(&mut r_max, &RectI8::max());
        assert_eq!(r_max, RectU8::of((i8::MAX as u8) - 11, (i8::MAX as u8) - 11, (i8::MAX as u8) - 2, (i8::MAX as u8) - 2));
    }

    #[test]
    fn wrapping_add_assign_big_rect_limits_out_of_bounds() {
        let mut r1 = RectU8::largest();
        wrapping_add_assign(&mut r1, &RectI8::min());
        assert_eq!(r1, RectU8::of((i8::MAX as u8) + 1, (i8::MAX as u8) + 1, i8::MAX as u8, i8::MAX as u8));

        let mut r2 = RectU8::largest();
        wrapping_add_assign(&mut r2, &RectI8::max());
        assert_eq!(r2, RectU8::of(i8::MAX as u8, i8::MAX as u8, (i8::MAX as u8) - 1, (i8::MAX as u8) - 1));

        let mut r_min = RectU8::of(1, 1, u8::MAX, u8::MAX);
        wrapping_add_assign(&mut r_min, &RectI8::min());
        assert_eq!(r_min, RectU8::of((i8::MAX as u8) + 2, (i8::MAX as u8) + 2, i8::MAX as u8, i8::MAX as u8));

        let mut r_max = RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1);
        wrapping_add_assign(&mut r_max, &RectI8::max());
        assert_eq!(r_max, RectU8::of(i8::MAX as u8, i8::MAX as u8, (i8::MAX as u8) - 2, (i8::MAX as u8) - 2));
    }
}
