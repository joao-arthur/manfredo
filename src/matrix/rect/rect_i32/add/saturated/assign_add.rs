use crate::matrix::rect::rect_i32::RectI32;

pub fn assign_add(r: &mut RectI32, delta: &RectI32) {
    r.min.row = r.min.row.saturating_add(delta.min.row);
    r.min.col = r.min.col.saturating_add(delta.min.col);
    r.max.row = r.max.row.saturating_add(delta.max.row);
    r.max.col = r.max.col.saturating_add(delta.max.col);
}

#[cfg(test)]
mod tests {
    use super::assign_add;
    use crate::matrix::rect::rect_i32::RectI32;

    #[test]
    fn test_assign_add() {
        let mut r = RectI32::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI32::of(5, 4, 3, 2));
        assert_eq!(r, RectI32::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI32::of(-14, -13, -12, -11));
        assert_eq!(r, RectI32::of(-9, -9, 3, 6));
    }

    #[test]
    fn assign_add_to_bounds() {
        let mut r = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX - 2, i32::MAX - 5);
        assign_add(&mut r, &RectI32::of(-2, -5, 2, 5));
        assert_eq!(r, RectI32::largest());

        let mut r_min = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX);
        assign_add(&mut r_min, &RectI32::of(-2, -5, 0, 0));
        assert_eq!(r_min, RectI32::largest());

        let mut r_max = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5);
        assign_add(&mut r_max, &RectI32::of(0, 0, 2, 5));
        assert_eq!(r_max, RectI32::largest());
    }

    #[test]
    fn assign_add_edge_out_of_bounds() {
        let mut r = RectI32::largest();
        assign_add(&mut r, &RectI32::of(-1, 0, 0, 0));
        assert_eq!(r, RectI32::largest());
        assign_add(&mut r, &RectI32::of(0, -1, 0, 0));
        assert_eq!(r, RectI32::largest());
        assign_add(&mut r, &RectI32::of(0, 0, 1, 0));
        assert_eq!(r, RectI32::largest());
        assign_add(&mut r, &RectI32::of(0, 0, 0, 1));
        assert_eq!(r, RectI32::largest());
    }

    #[test]
    fn assign_add_out_of_bounds() {
        let mut r1 = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 10, i32::MAX - 10);
        assign_add(&mut r1, &RectI32::of(-20, 0, 0, 0));
        assert_eq!(r1, RectI32::of(i32::MIN, i32::MIN + 10, i32::MAX - 10, i32::MAX - 10));
      
        let mut r2 = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 10, i32::MAX - 10);
        assign_add(&mut r2, &RectI32::of(0, -20, 0, 0));
        assert_eq!(r2, RectI32::of(i32::MIN + 10, i32::MIN, i32::MAX - 10, i32::MAX - 10));
     
        let mut r3 = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 10, i32::MAX - 10);
        assign_add(&mut r3, &RectI32::of(0, 0, 20, 0));
        assert_eq!(r3, RectI32::of(i32::MIN + 10, i32::MIN +10, i32::MAX, i32::MAX - 10));
    
        let mut r4 = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 10, i32::MAX - 10);
        assign_add(&mut r4, &RectI32::of(0, 0, 0, 20));
        assert_eq!(r4, RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 10, i32::MAX));
    }

    #[test]
    fn assign_add_limits_out_of_bounds() {
        let mut r = RectI32::largest();
        assign_add(&mut r, &RectI32::of(i32::MIN, 0, 0, 0));
        assert_eq!(r, RectI32::largest());
        assign_add(&mut r, &RectI32::of(0, i32::MIN, 0, 0));
        assert_eq!(r, RectI32::largest());
        assign_add(&mut r, &RectI32::of(0, 0, i32::MAX, 0));
        assert_eq!(r, RectI32::largest());
        assign_add(&mut r, &RectI32::of(0, 0, 0, i32::MAX));
        assert_eq!(r, RectI32::largest());
    }
}
