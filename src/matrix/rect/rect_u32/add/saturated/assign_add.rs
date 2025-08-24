use crate::matrix::{
    point::point_u32::PointU32,
    rect::{rect_i32::RectI32, rect_u32::RectU32},
};

pub fn assign_add(r: &mut RectU32, delta: &RectI32) {
    let min_row = r.min.row.saturating_add_signed(delta.min.row);
    let min_col = r.min.col.saturating_add_signed(delta.min.col);
    let max_row = r.max.row.saturating_add_signed(delta.max.row);
    let max_col = r.max.col.saturating_add_signed(delta.max.col);
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
}

#[cfg(test)]
mod tests {
    use super::assign_add;
    use crate::matrix::rect::{rect_i32::RectI32, rect_u32::RectU32};

    #[test]
    fn test_assign_add() {
        let mut r = RectU32::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI32::of(5, 4, 3, 2));
        assert_eq!(r, RectU32::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI32::of(-4, -3, -2, -1));
        assert_eq!(r, RectU32::of(1, 1, 13, 16));
    }

    #[test]
    fn assign_add_small_rect_to_bounds() {
        let mut min_r = RectU32::of(2, 5, 12, 15);
        assign_add(&mut min_r, &RectI32::of(-2, -5, 9, 7));
        assert_eq!(min_r, RectU32::of(0, 0, 21, 22));

        let mut max_r = RectU32::of(u32::MAX - 12, u32::MAX - 15, u32::MAX - 2, u32::MAX - 5);
        assign_add(&mut max_r, &RectI32::of(-9, -7, 2, 5));
        assert_eq!(max_r, RectU32::of(u32::MAX - 21, u32::MAX - 22, u32::MAX, u32::MAX));
    }

    #[test]
    fn assign_add_big_rect_to_bounds() {
        let mut r = RectU32::of(2, 5, u32::MAX - 2, u32::MAX - 5);
        assign_add(&mut r, &RectI32::of(-2, -5, 2, 5));
        assert_eq!(r, RectU32::largest());

        let mut min_r = RectU32::of(2, 5, u32::MAX, u32::MAX);
        assign_add(&mut min_r, &RectI32::of(-2, -5, 0, 0));
        assert_eq!(min_r, RectU32::largest());

        let mut max_r = RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5);
        assign_add(&mut max_r, &RectI32::of(0, 0, 2, 5));
        assert_eq!(max_r, RectU32::largest());
    }

    #[test]
    fn assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectU32::of(10, 5, 20, 30);
        assign_add(&mut r_min, &RectI32::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectU32::of(0, 0, 20, 30));

        let mut r_max = RectU32::of(u32::MAX - 20, u32::MAX - 30, u32::MAX - 5, u32::MAX - 10);
        assign_add(&mut r_max, &RectI32::of(0, 0, 20, 20));
        assert_eq!(r_max, RectU32::of(u32::MAX - 20, u32::MAX - 30, u32::MAX, u32::MAX));
    }

    #[test]
    fn assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectU32::of(10, 5, u32::MAX, u32::MAX);
        assign_add(&mut r_min, &RectI32::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectU32::largest());

        let mut r_max = RectU32::of(0, 0, u32::MAX - 5, u32::MAX - 10);
        assign_add(&mut r_max, &RectI32::of(0, 0, 20, 20));
        assert_eq!(r_max, RectU32::largest());
    }

    #[test]
    fn assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU32::of(1, 1, 10, 10);
        assign_add(&mut r_min, &RectI32::min());
        assert_eq!(r_min, RectU32::min());

        let mut r_max = RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1);
        assign_add(&mut r_max, &RectI32::max());
        assert_eq!(r_max, RectU32::max());
    }

    #[test]
    fn assign_add_big_rect_limits_out_of_bounds() {
        let mut r = RectU32::largest();
        assign_add(&mut r, &RectI32::largest());
        assert_eq!(r, RectU32::largest());

        let mut r_large = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
        assign_add(&mut r_large, &RectI32::largest());
        assert_eq!(r_large, RectU32::largest());

        let mut r_min_row = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
        assign_add(&mut r_min_row, &RectI32::of(i32::MIN, 0, 0, 0));
        assert_eq!(r_min_row, RectU32::of(0, 1, u32::MAX - 1, u32::MAX - 1));

        let mut r_min_col = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
        assign_add(&mut r_min_col, &RectI32::of(0, i32::MIN, 0, 0));
        assert_eq!(r_min_col, RectU32::of(1, 0, u32::MAX - 1, u32::MAX - 1));

        let mut r_max_row = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
        assign_add(&mut r_max_row, &RectI32::of(0, 0, i32::MAX, 0));
        assert_eq!(r_max_row, RectU32::of(1, 1, u32::MAX, u32::MAX - 1));

        let mut r_max_col = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
        assign_add(&mut r_max_col, &RectI32::of(0, 0, 0, i32::MAX));
        assert_eq!(r_max_col, RectU32::of(1, 1, u32::MAX - 1, u32::MAX));
    }
}
