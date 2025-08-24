use crate::matrix::{
    point::{point_i64::PointI64, point_u64::PointU64},
    rect::rect_u64::{RectU64, delta_col, delta_row},
};

pub fn assign_translate(r: &mut RectU64, delta: &PointI64) {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let min_row = r.min.row.wrapping_add_signed(delta.row);
    let min_col = r.min.col.wrapping_add_signed(delta.col);
    let max_row = min_row.wrapping_add(d_row);
    let max_col = min_col.wrapping_add(d_col);
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
}

pub fn translate(r: &RectU64, delta: &PointI64) -> RectU64 {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let min_row = r.min.row.wrapping_add_signed(delta.row);
    let min_col = r.min.col.wrapping_add_signed(delta.col);
    let max_row = min_row.wrapping_add(d_row);
    let max_col = min_col.wrapping_add(d_col);
    RectU64 { min: PointU64 { row: min_row, col: min_col }, max: PointU64 { row: max_row, col: max_col } }
}

#[cfg(test)]
mod tests {
    use super::{assign_translate, translate};
    use crate::matrix::{point::point_i64::PointI64, rect::rect_u64::RectU64};

    #[test]
    fn test_assign_translate() {
        let mut r = RectU64::of(0, 0, 12, 15);
        assign_translate(&mut r, &PointI64::of(5, 4));
        assert_eq!(r, RectU64::of(5, 4, 17, 19));
        assign_translate(&mut r, &PointI64::of(-4, -2));
        assert_eq!(r, RectU64::of(1, 2, 13, 17));
    }

    #[test]
    fn assign_translate_small_rect_to_bounds() {
        let mut min_r = RectU64::of(2, 5, 12, 15);
        assign_translate(&mut min_r, &PointI64::of(-2, -5));
        assert_eq!(min_r, RectU64::of(0, 0, 10, 10));

        let mut max_r = RectU64::of(u64::MAX - 12, u64::MAX - 15, u64::MAX - 2, u64::MAX - 5);
        assign_translate(&mut max_r, &PointI64::of(2, 5));
        assert_eq!(max_r, RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX));
    }

    #[test]
    fn assign_translate_big_rect_to_bounds() {
        let mut min_r = RectU64::of(2, 5, u64::MAX, u64::MAX);
        assign_translate(&mut min_r, &PointI64::of(-2, -5));
        assert_eq!(min_r, RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5));

        let mut max_r = RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5);
        assign_translate(&mut max_r, &PointI64::of(2, 5));
        assert_eq!(max_r, RectU64::of(2, 5, u64::MAX, u64::MAX));
    }

    #[test]
    fn assign_translate_small_rect_out_of_bounds() {
        let mut r_min = RectU64::of(10, 5, 20, 30);
        assign_translate(&mut r_min, &PointI64::of(-20, -20));
        assert_eq!(r_min, RectU64::of(u64::MAX - 9, u64::MAX - 14, 0, 10));

        let mut r_max = RectU64::of(u64::MAX - 20, u64::MAX - 30, u64::MAX - 5, u64::MAX - 10);
        assign_translate(&mut r_max, &PointI64::of(20, 20));
        assert_eq!(r_max, RectU64::of(u64::MAX, u64::MAX - 10, 14, 9));
    }

    #[test]
    fn assign_translate_big_rect_out_of_bounds() {
        let mut r_min = RectU64::of(10, 5, u64::MAX, u64::MAX);
        assign_translate(&mut r_min, &PointI64::of(-20, -20));
        assert_eq!(r_min, RectU64::of(u64::MAX - 9, u64::MAX - 14, u64::MAX - 20, u64::MAX - 20));

        let mut r_max = RectU64::of(0, 0, u64::MAX - 5, u64::MAX - 10);
        assign_translate(&mut r_max, &PointI64::of(20, 20));
        assert_eq!(r_max, RectU64::of(20, 20, 14, 9));
    }

    #[test]
    fn assign_translate_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU64::of(1, 1, 10, 10);
        assign_translate(&mut r_min, &PointI64::min());
        assert_eq!(r_min, RectU64::of((i64::MAX as u64) + 2, (i64::MAX as u64) + 2, (i64::MAX as u64) + 11, (i64::MAX as u64) + 11));

        let mut r_max = RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1);
        assign_translate(&mut r_max, &PointI64::max());
        assert_eq!(r_max, RectU64::of((i64::MAX as u64) - 11, (i64::MAX as u64) - 11, (i64::MAX as u64) - 2, (i64::MAX as u64) - 2));
    }

    #[test]
    fn assign_translate_big_rect_limits_out_of_bounds() {
        let mut r1 = RectU64::largest();
        assign_translate(&mut r1, &PointI64::min());
        assert_eq!(r1, RectU64::of((i64::MAX as u64) + 1, (i64::MAX as u64) + 1, i64::MAX as u64, i64::MAX as u64));

        let mut r2 = RectU64::largest();
        assign_translate(&mut r2, &PointI64::max());
        assert_eq!(r2, RectU64::of(i64::MAX as u64, i64::MAX as u64, (i64::MAX as u64) - 1, (i64::MAX as u64) - 1));

        let mut r_min = RectU64::of(1, 1, u64::MAX, u64::MAX);
        assign_translate(&mut r_min, &PointI64::min());
        assert_eq!(r_min, RectU64::of((i64::MAX as u64) + 2, (i64::MAX as u64) + 2, i64::MAX as u64, i64::MAX as u64));

        let mut r_max = RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1);
        assign_translate(&mut r_max, &PointI64::max());
        assert_eq!(r_max, RectU64::of(i64::MAX as u64, i64::MAX as u64, (i64::MAX as u64) - 2, (i64::MAX as u64) - 2));
    }

    #[test]
    fn test_translate() {
        assert_eq!(translate(&RectU64::of(0, 0, 12, 15), &PointI64::of(5, 4)), RectU64::of(5, 4, 17, 19));
        assert_eq!(translate(&RectU64::of(5, 4, 17, 19), &PointI64::of(-4, -2)), RectU64::of(1, 2, 13, 17));
    }

    #[test]
    fn translate_small_rect_to_bounds() {
        assert_eq!(translate(&RectU64::of(2, 5, 12, 15), &PointI64::of(-2, -5)), RectU64::of(0, 0, 10, 10));
        assert_eq!(
            translate(&RectU64::of(u64::MAX - 12, u64::MAX - 15, u64::MAX - 2, u64::MAX - 5), &PointI64::of(2, 5)),
            RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX)
        );
    }

    #[test]
    fn translate_big_rect_to_bounds() {
        assert_eq!(translate(&RectU64::of(2, 5, u64::MAX, u64::MAX), &PointI64::of(-2, -5)), RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5));
        assert_eq!(translate(&RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5), &PointI64::of(2, 5)), RectU64::of(2, 5, u64::MAX, u64::MAX));
    }

    #[test]
    fn translate_small_rect_out_of_bounds() {
        assert_eq!(translate(&RectU64::of(10, 5, 20, 30), &PointI64::of(-20, -20)), RectU64::of(u64::MAX - 9, u64::MAX - 14, 0, 10));
        assert_eq!(
            translate(&RectU64::of(u64::MAX - 20, u64::MAX - 30, u64::MAX - 5, u64::MAX - 10), &PointI64::of(20, 20)),
            RectU64::of(u64::MAX, u64::MAX - 10, 14, 9)
        );
    }

    #[test]
    fn translate_big_rect_out_of_bounds() {
        assert_eq!(
            translate(&RectU64::of(10, 5, u64::MAX, u64::MAX), &PointI64::of(-20, -20)),
            RectU64::of(u64::MAX - 9, u64::MAX - 14, u64::MAX - 20, u64::MAX - 20)
        );
        assert_eq!(translate(&RectU64::of(0, 0, u64::MAX - 5, u64::MAX - 10), &PointI64::of(20, 20)), RectU64::of(20, 20, 14, 9));
    }

    #[test]
    fn translate_small_rect_limits_out_of_bounds() {
        assert_eq!(
            translate(&RectU64::of(1, 1, 10, 10), &PointI64::min()),
            RectU64::of((i64::MAX as u64) + 2, (i64::MAX as u64) + 2, (i64::MAX as u64) + 11, (i64::MAX as u64) + 11)
        );
        assert_eq!(
            translate(&RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1), &PointI64::max()),
            RectU64::of((i64::MAX as u64) - 11, (i64::MAX as u64) - 11, (i64::MAX as u64) - 2, (i64::MAX as u64) - 2)
        );
    }

    #[test]
    fn translate_big_rect_limits_out_of_bounds() {
        assert_eq!(
            translate(&RectU64::largest(), &PointI64::min()),
            RectU64::of((i64::MAX as u64) + 1, (i64::MAX as u64) + 1, i64::MAX as u64, i64::MAX as u64)
        );
        assert_eq!(
            translate(&RectU64::largest(), &PointI64::max()),
            RectU64::of(i64::MAX as u64, i64::MAX as u64, (i64::MAX as u64) - 1, (i64::MAX as u64) - 1)
        );
        assert_eq!(
            translate(&RectU64::of(1, 1, u64::MAX, u64::MAX), &PointI64::min()),
            RectU64::of((i64::MAX as u64) + 2, (i64::MAX as u64) + 2, i64::MAX as u64, i64::MAX as u64)
        );
        assert_eq!(
            translate(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1), &PointI64::max()),
            RectU64::of(i64::MAX as u64, i64::MAX as u64, (i64::MAX as u64) - 2, (i64::MAX as u64) - 2)
        );
    }
}
