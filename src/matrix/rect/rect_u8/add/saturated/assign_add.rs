use crate::matrix::rect::{rect_i8::RectI8, rect_u8::RectU8};

pub fn assign_add(r: &mut RectU8, delta: &RectI8) {
    r.min.row = r.min.row.saturating_add_signed(delta.min.row);
    r.min.col = r.min.col.saturating_add_signed(delta.min.col);
    r.max.row = r.max.row.saturating_add_signed(delta.max.row);
    r.max.col = r.max.col.saturating_add_signed(delta.max.col);
}

#[cfg(test)]
mod tests {
    use super::assign_add;
    use crate::matrix::rect::{rect_i8::RectI8, rect_u8::RectU8};

    #[test]
    fn test_assign_add() {
        let mut r = RectU8::of(0, 0, 12, 10);
        assign_add(&mut r, &RectI8::of(5, 4, 3, 2));
        assert_eq!(r, RectU8::of(5, 4, 15, 12));
        assign_add(&mut r, &RectI8::of(-4, -3, -2, -1));
        assert_eq!(r, RectU8::of(1, 1, 13, 11));
    }

    #[test]
    fn assign_add_to_bounds() {
        let mut r = RectU8::of(2, 5, u8::MAX - 2, u8::MAX - 5);
        assign_add(&mut r, &RectI8::of(-2, -5, 2, 5));
        assert_eq!(r, RectU8::largest());

        let mut r_min = RectU8::of(2, 5, u8::MAX, u8::MAX);
        assign_add(&mut r_min, &RectI8::of(-2, -5, 0, 0));
        assert_eq!(r_min, RectU8::largest());

        let mut r_max = RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5);
        assign_add(&mut r_max, &RectI8::of(0, 0, 2, 5));
        assert_eq!(r_max, RectU8::largest());
    }

    #[test]
    fn assign_add_edge_out_of_bounds() {
        let mut r = RectU8::largest();
        assign_add(&mut r, &RectI8::of(-1, 0, 0, 0));
        assert_eq!(r, RectU8::largest());
        assign_add(&mut r, &RectI8::of(0, -1, 0, 0));
        assert_eq!(r, RectU8::largest());
        assign_add(&mut r, &RectI8::of(0, 0, 1, 0));
        assert_eq!(r, RectU8::largest());
        assign_add(&mut r, &RectI8::of(0, 0, 0, 1));
        assert_eq!(r, RectU8::largest());
    }

    #[test]
    fn assign_add_out_of_bounds() {
        let mut r1 = RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10);
        assign_add(&mut r1, &RectI8::of(-20, 0, 0, 0));
        assert_eq!(r1, RectU8::of(0, 10, u8::MAX - 10, u8::MAX - 10));

        let mut r2 = RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10);
        assign_add(&mut r2, &RectI8::of(0, -20, 0, 0));
        assert_eq!(r2, RectU8::of(10, 0, u8::MAX - 10, u8::MAX - 10));
    
        let mut r3 = RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10);
        assign_add(&mut r3, &RectI8::of(0, 0, 20, 0));
        assert_eq!(r3, RectU8::of(10, 10, u8::MAX, u8::MAX - 10));
        
        let mut r4 = RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10);
        assign_add(&mut r4, &RectI8::of(0, 0, 0, 20));
        assert_eq!(r4, RectU8::of(10, 10, u8::MAX - 10, u8::MAX));
    }

    #[test]
    fn assign_add_limits_out_of_bounds() {
        let mut r = RectU8::largest();
        assign_add(&mut r, &RectI8::of(i8::MIN, 0, 0, 0));
        assert_eq!(r, RectU8::largest());
        assign_add(&mut r, &RectI8::of(0, i8::MIN, 0, 0));
        assert_eq!(r, RectU8::largest());
        assign_add(&mut r, &RectI8::of(0, 0, i8::MAX, 0));
        assert_eq!(r, RectU8::largest());
        assign_add(&mut r, &RectI8::of(0, 0, 0, i8::MAX));
        assert_eq!(r, RectU8::largest());
    }
}
