use crate::matrix::rect::rect_i16::RectI16;

pub fn assign_add(r: &mut RectI16, delta: &RectI16) {
    let min_row = r.min.row.saturating_add(delta.min.row);
    let min_col = r.min.col.saturating_add(delta.min.col);
    let max_row = r.max.row.saturating_add(delta.max.row);
    let max_col = r.max.col.saturating_add(delta.max.col);
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
}

#[cfg(test)]
mod tests {
    use super::assign_add;
    use crate::matrix::rect::rect_i16::RectI16;

    #[test]
    fn test_assign_add() {
        let mut r = RectI16::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI16::of(5, 4, 3, 2));
        assert_eq!(r, RectI16::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI16::of(-14, -13, -12, -11));
        assert_eq!(r, RectI16::of(-9, -9, 3, 6));
    }

    #[test]
    fn assign_add_small_rect_to_bounds() {
        let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MIN + 12, i16::MIN + 15);
        assign_add(&mut min_r, &RectI16::of(-2, -5, 9, 7));
        assert_eq!(min_r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 21, i16::MIN + 22));

        let mut max_r = RectI16::of(i16::MAX - 12, i16::MAX - 15, i16::MAX - 2, i16::MAX - 5);
        assign_add(&mut max_r, &RectI16::of(-9, -7, 2, 5));
        assert_eq!(max_r, RectI16::of(i16::MAX - 21, i16::MAX - 22, i16::MAX, i16::MAX));
    }

    #[test]
    fn assign_add_big_rect_to_bounds() {
        let mut r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX - 2, i16::MAX - 5);
        assign_add(&mut r, &RectI16::of(-2, -5, 2, 5));
        assert_eq!(r, RectI16::largest());

        let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX);
        assign_add(&mut min_r, &RectI16::of(-2, -5, 0, 0));
        assert_eq!(min_r, RectI16::largest());

        let mut max_r = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5);
        assign_add(&mut max_r, &RectI16::of(0, 0, 2, 5));
        assert_eq!(max_r, RectI16::largest());
    }

    #[test]
    fn assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30);
        assign_add(&mut r_min, &RectI16::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 20, i16::MIN + 30));

        let mut r_max = RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10);
        assign_add(&mut r_max, &RectI16::of(0, 0, 20, 20));
        assert_eq!(r_max, RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX, i16::MAX));
    }

    #[test]
    fn assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX);
        assign_add(&mut r_min, &RectI16::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectI16::largest());

        let mut r_max = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10);
        assign_add(&mut r_max, &RectI16::of(0, 0, 20, 20));
        assert_eq!(r_max, RectI16::largest());
    }

    #[test]
    fn assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10);
        assign_add(&mut r_min, &RectI16::min());
        assert_eq!(r_min, RectI16::min());

        let mut r_max = RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1);
        assign_add(&mut r_max, &RectI16::max());
        assert_eq!(r_max, RectI16::max());
    }

    #[test]
    fn assign_add_big_rect_limits_out_of_bounds() {
        let mut r = RectI16::largest();
        assign_add(&mut r, &RectI16::largest());
        assert_eq!(r, RectI16::largest());

        let mut r_large = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assign_add(&mut r_large, &RectI16::largest());
        assert_eq!(r_large, RectI16::largest());

        let mut r_min_row = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assign_add(&mut r_min_row, &RectI16::of(i16::MIN, 0, 0, 0));
        assert_eq!(r_min_row, RectI16::of(i16::MIN, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1));

        let mut r_min_col = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assign_add(&mut r_min_col, &RectI16::of(0, i16::MIN, 0, 0));
        assert_eq!(r_min_col, RectI16::of(i16::MIN + 1, i16::MIN, i16::MAX - 1, i16::MAX - 1));

        let mut r_max_row = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assign_add(&mut r_max_row, &RectI16::of(0, 0, i16::MAX, 0));
        assert_eq!(r_max_row, RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX - 1));

        let mut r_max_col = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assign_add(&mut r_max_col, &RectI16::of(0, 0, 0, i16::MAX));
        assert_eq!(r_max_col, RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX));
    }
}
