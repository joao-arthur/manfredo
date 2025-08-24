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
    fn assign_add_small_rect_to_bounds() {
        let mut min_r = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MIN + 12, i32::MIN + 15);
        assign_add(&mut min_r, &RectI32::of(-2, -5, 9, 7));
        assert_eq!(min_r, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 21, i32::MIN + 22));

        let mut max_r = RectI32::of(i32::MAX - 12, i32::MAX - 15, i32::MAX - 2, i32::MAX - 5);
        assign_add(&mut max_r, &RectI32::of(-9, -7, 2, 5));
        assert_eq!(max_r, RectI32::of(i32::MAX - 21, i32::MAX - 22, i32::MAX, i32::MAX));
    }

    #[test]
    fn assign_add_big_rect_to_bounds() {
        let mut r = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX - 2, i32::MAX - 5);
        assign_add(&mut r, &RectI32::of(-2, -5, 2, 5));
        assert_eq!(r, RectI32::largest());

        let mut min_r = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX);
        assign_add(&mut min_r, &RectI32::of(-2, -5, 0, 0));
        assert_eq!(min_r, RectI32::largest());

        let mut max_r = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5);
        assign_add(&mut max_r, &RectI32::of(0, 0, 2, 5));
        assert_eq!(max_r, RectI32::largest());
    }

    #[test]
    fn assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MIN + 20, i32::MIN + 30);
        assign_add(&mut r_min, &RectI32::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 20, i32::MIN + 30));

        let mut r_max = RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX - 5, i32::MAX - 10);
        assign_add(&mut r_max, &RectI32::of(0, 0, 20, 20));
        assert_eq!(r_max, RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX, i32::MAX));
    }

    #[test]
    fn assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MAX, i32::MAX);
        assign_add(&mut r_min, &RectI32::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectI32::largest());

        let mut r_max = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 5, i32::MAX - 10);
        assign_add(&mut r_max, &RectI32::of(0, 0, 20, 20));
        assert_eq!(r_max, RectI32::largest());
    }

    #[test]
    fn assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10);
        assign_add(&mut r_min, &RectI32::min());
        assert_eq!(r_min, RectI32::min());

        let mut r_max = RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1);
        assign_add(&mut r_max, &RectI32::max());
        assert_eq!(r_max, RectI32::max());
    }

    #[test]
    fn assign_add_big_rect_limits_out_of_bounds() {
        let mut r = RectI32::largest();
        assign_add(&mut r, &RectI32::largest());
        assert_eq!(r, RectI32::largest());

        let mut r_large = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1);
        assign_add(&mut r_large, &RectI32::largest());
        assert_eq!(r_large, RectI32::largest());

        let mut r_min_row = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1);
        assign_add(&mut r_min_row, &RectI32::of(i32::MIN, 0, 0, 0));
        assert_eq!(r_min_row, RectI32::of(i32::MIN, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1));

        let mut r_min_col = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1);
        assign_add(&mut r_min_col, &RectI32::of(0, i32::MIN, 0, 0));
        assert_eq!(r_min_col, RectI32::of(i32::MIN + 1, i32::MIN, i32::MAX - 1, i32::MAX - 1));

        let mut r_max_row = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1);
        assign_add(&mut r_max_row, &RectI32::of(0, 0, i32::MAX, 0));
        assert_eq!(r_max_row, RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX - 1));

        let mut r_max_col = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1);
        assign_add(&mut r_max_col, &RectI32::of(0, 0, 0, i32::MAX));
        assert_eq!(r_max_col, RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX));
    }
}
