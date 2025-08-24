use crate::matrix::rect::rect_i8::RectI8;

pub fn assign_add(r: &mut RectI8, delta: &RectI8) {
    r.min.row = r.min.row.saturating_add(delta.min.row);
    r.min.col = r.min.col.saturating_add(delta.min.col);
    r.max.row = r.max.row.saturating_add(delta.max.row);
    r.max.col = r.max.col.saturating_add(delta.max.col);
}

#[cfg(test)]
mod tests {
    use super::assign_add;
    use crate::matrix::rect::rect_i8::RectI8;

    #[test]
    fn test_assign_add() {
        let mut r = RectI8::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI8::of(5, 4, 3, 2));
        assert_eq!(r, RectI8::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI8::of(-14, -13, -12, -11));
        assert_eq!(r, RectI8::of(-9, -9, 3, 6));
    }

    #[test]
    fn assign_add_to_bounds() {
        let mut r = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX - 2, i8::MAX - 5);
        assign_add(&mut r, &RectI8::of(-2, -5, 2, 5));
        assert_eq!(r, RectI8::largest());

        let mut r_min = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX);
        assign_add(&mut r_min, &RectI8::of(-2, -5, 0, 0));
        assert_eq!(r_min, RectI8::largest());

        let mut r_max = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5);
        assign_add(&mut r_max, &RectI8::of(0, 0, 2, 5));
        assert_eq!(r_max, RectI8::largest());
    }

    #[test]
    fn assign_add_edge_out_of_bounds() {
        let mut r = RectI8::largest();
        assign_add(&mut r, &RectI8::of(-1, 0, 0, 0));
        assert_eq!(r, RectI8::largest());
        assign_add(&mut r, &RectI8::of(0, -1, 0, 0));
        assert_eq!(r, RectI8::largest());
        assign_add(&mut r, &RectI8::of(0, 0, 1, 0));
        assert_eq!(r, RectI8::largest());
        assign_add(&mut r, &RectI8::of(0, 0, 0, 1));
        assert_eq!(r, RectI8::largest());
    }

    #[test]
    fn assign_add_out_of_bounds() {
        let mut r1 = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10);
        assign_add(&mut r1, &RectI8::of(-20, 0, 0, 0));
        assert_eq!(r1, RectI8::of(i8::MIN, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10));
      
        let mut r2 = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10);
        assign_add(&mut r2, &RectI8::of(0, -20, 0, 0));
        assert_eq!(r2, RectI8::of(i8::MIN + 10, i8::MIN, i8::MAX - 10, i8::MAX - 10));
     
        let mut r3 = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10);
        assign_add(&mut r3, &RectI8::of(0, 0, 20, 0));
        assert_eq!(r3, RectI8::of(i8::MIN + 10, i8::MIN +10, i8::MAX, i8::MAX - 10));
    
        let mut r4 = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10);
        assign_add(&mut r4, &RectI8::of(0, 0, 0, 20));
        assert_eq!(r4, RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX));
    }

    #[test]
    fn assign_add_limits_out_of_bounds() {
        let mut r = RectI8::largest();
        assign_add(&mut r, &RectI8::of(i8::MIN, 0, 0, 0));
        assert_eq!(r, RectI8::largest());
        assign_add(&mut r, &RectI8::of(0, i8::MIN, 0, 0));
        assert_eq!(r, RectI8::largest());
        assign_add(&mut r, &RectI8::of(0, 0, i8::MAX, 0));
        assert_eq!(r, RectI8::largest());
        assign_add(&mut r, &RectI8::of(0, 0, 0, i8::MAX));
        assert_eq!(r, RectI8::largest());
    }
}
