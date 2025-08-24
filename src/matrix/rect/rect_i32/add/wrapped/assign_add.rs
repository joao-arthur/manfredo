use crate::matrix::{point::point_i32::PointI32, rect::rect_i32::RectI32};

pub fn assign_add(r: &mut RectI32, delta: &RectI32) {
    let min_row = r.min.row.wrapping_add(delta.min.row);
    let min_col = r.min.col.wrapping_add(delta.min.col);
    let max_row = r.max.row.wrapping_add(delta.max.row);
    let max_col = r.max.col.wrapping_add(delta.max.col);
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
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
        let mut min_r = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX - 2, i32::MAX - 5);
        assign_add(&mut min_r, &RectI32::of(-2, -5, 2, 5));
        assert_eq!(min_r, RectI32::largest());

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
        assert_eq!(r_min, RectI32::of(i32::MAX - 9, i32::MAX - 14, i32::MIN + 20, i32::MIN + 30));

        let mut r_max = RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX - 5, i32::MAX - 10);
        assign_add(&mut r_max, &RectI32::of(0, 0, 20, 20));
        assert_eq!(r_max, RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MIN + 14, i32::MIN + 9));
    }

    #[test]
    fn assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MAX, i32::MAX);
        assign_add(&mut r_min, &RectI32::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectI32::of(i32::MAX - 9, i32::MAX - 14, i32::MAX, i32::MAX));

        let mut r_max = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 5, i32::MAX - 10);
        assign_add(&mut r_max, &RectI32::of(0, 0, 20, 20));
        assert_eq!(r_max, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 14, i32::MIN + 9));
    }

    #[test]
    fn assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10);
        assign_add(&mut r_min, &RectI32::min());
        assert_eq!(r_min, RectI32::of(1, 1, 10, 10));

        let mut r_max = RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1);
        assign_add(&mut r_max, &RectI32::max());
        assert_eq!(r_max, RectI32::of(-12, -12, -3, -3));
    }

    #[test]
    fn assign_add_big_rect_limits_out_of_bounds() {
        let mut r1 = RectI32::largest();
        assign_add(&mut r1, &RectI32::min());
        assert_eq!(r1, RectI32::of(0, 0, -1, -1));

        let mut r2 = RectI32::largest();
        assign_add(&mut r2, &RectI32::max());
        assert_eq!(r2, RectI32::of(-1, -1, -2, -2));

        let mut r_min = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX);
        assign_add(&mut r_min, &RectI32::min());
        assert_eq!(r_min, RectI32::of(1, 1, -1, -1));

        let mut r_max = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1);
        assign_add(&mut r_max, &RectI32::max());
        assert_eq!(r_max, RectI32::of(-1, -1, -3, -3));
    }
}
