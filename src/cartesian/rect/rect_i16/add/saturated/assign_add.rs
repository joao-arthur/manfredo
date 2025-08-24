use crate::cartesian::rect::rect_i16::RectI16;

pub fn assign_add(r: &mut RectI16, delta: &RectI16) {
    r.min.x = r.min.x.saturating_add(delta.min.x);
    r.min.y = r.min.y.saturating_add(delta.min.y);
    r.max.x = r.max.x.saturating_add(delta.max.x);
    r.max.y = r.max.y.saturating_add(delta.max.y);
}

#[cfg(test)]
mod tests {
    use super::assign_add;
    use crate::cartesian::rect::rect_i16::RectI16;

    #[test]
    fn test_assign_add() {
        let mut r = RectI16::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI16::of(5, 4, 3, 2));
        assert_eq!(r, RectI16::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI16::of(-14, -13, -12, -11));
        assert_eq!(r, RectI16::of(-9, -9, 3, 6));
    }

    #[test]
    fn assign_add_to_bounds() {
        let mut r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX - 2, i16::MAX - 5);
        assign_add(&mut r, &RectI16::of(-2, -5, 2, 5));
        assert_eq!(r, RectI16::largest());

        let mut r_min = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX);
        assign_add(&mut r_min, &RectI16::of(-2, -5, 0, 0));
        assert_eq!(r_min, RectI16::largest());

        let mut r_max = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5);
        assign_add(&mut r_max, &RectI16::of(0, 0, 2, 5));
        assert_eq!(r_max, RectI16::largest());
    }

    #[test]
    fn assign_add_edge_out_of_bounds() {
        let mut r = RectI16::largest();
        assign_add(&mut r, &RectI16::of(-1, 0, 0, 0));
        assert_eq!(r, RectI16::largest());
        assign_add(&mut r, &RectI16::of(0, -1, 0, 0));
        assert_eq!(r, RectI16::largest());
        assign_add(&mut r, &RectI16::of(0, 0, 1, 0));
        assert_eq!(r, RectI16::largest());
        assign_add(&mut r, &RectI16::of(0, 0, 0, 1));
        assert_eq!(r, RectI16::largest());
    }

    #[test]
    fn assign_add_out_of_bounds() {
        let mut r1 = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10);
        assign_add(&mut r1, &RectI16::of(-20, 0, 0, 0));
        assert_eq!(r1, RectI16::of(i16::MIN, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10));
      
        let mut r2 = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10);
        assign_add(&mut r2, &RectI16::of(0, -20, 0, 0));
        assert_eq!(r2, RectI16::of(i16::MIN + 10, i16::MIN, i16::MAX - 10, i16::MAX - 10));
     
        let mut r3 = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10);
        assign_add(&mut r3, &RectI16::of(0, 0, 20, 0));
        assert_eq!(r3, RectI16::of(i16::MIN + 10, i16::MIN +10, i16::MAX, i16::MAX - 10));
    
        let mut r4 = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10);
        assign_add(&mut r4, &RectI16::of(0, 0, 0, 20));
        assert_eq!(r4, RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX));
    }

    #[test]
    fn assign_add_limits_out_of_bounds() {
        let mut r = RectI16::largest();
        assign_add(&mut r, &RectI16::of(i16::MIN, 0, 0, 0));
        assert_eq!(r, RectI16::largest());
        assign_add(&mut r, &RectI16::of(0, i16::MIN, 0, 0));
        assert_eq!(r, RectI16::largest());
        assign_add(&mut r, &RectI16::of(0, 0, i16::MAX, 0));
        assert_eq!(r, RectI16::largest());
        assign_add(&mut r, &RectI16::of(0, 0, 0, i16::MAX));
        assert_eq!(r, RectI16::largest());
    }
}
