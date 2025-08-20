use crate::matrix::{
    point::{point_i32::PointI32, point_u32::PointU32},
    rect::rect_u32::{RectU32, delta_col, delta_row},
};

pub fn assign_translate(r: &mut RectU32, delta: &PointI32) {
    let dx = delta_row(r);
    let dy = delta_col(r);
    let temp_min_row = i64::from(r.min.row) + i64::from(delta.row);
    let temp_min_col = i64::from(r.min.col) + i64::from(delta.col);
    let clamped_row = temp_min_row.clamp(0, i64::from(u32::MAX) - i64::from(dx));
    let clamped_col = temp_min_col.clamp(0, i64::from(u32::MAX) - i64::from(dy));
    let min_row = clamped_row as u32;
    let min_col = clamped_col as u32;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = min_row + dx;
    r.max.col = min_col + dy;
}

pub fn translate(r: &RectU32, delta: &PointI32) -> RectU32 {
    let dx = delta_row(r);
    let dy = delta_col(r);
    let temp_min_row = i64::from(r.min.row) + i64::from(delta.row);
    let temp_min_col = i64::from(r.min.col) + i64::from(delta.col);
    let clamped_row = temp_min_row.clamp(0, i64::from(u32::MAX) - i64::from(dx));
    let clamped_col = temp_min_col.clamp(0, i64::from(u32::MAX) - i64::from(dy));
    let min_row = clamped_row as u32;
    let min_col = clamped_col as u32;
    let max_row = min_row + dx;
    let max_col = min_col + dy;
    RectU32 { min: PointU32 { row: min_row, col: min_col }, max: PointU32 { row: max_row, col: max_col } }
}

#[cfg(test)]
mod tests {
    use crate::matrix::{point::point_i32::PointI32, rect::rect_u32::RectU32};

    use super::{assign_translate, translate};

    #[test]
    fn test_assign_translate() {
        let mut r = RectU32::of(0, 0, 12, 15);
        assign_translate(&mut r, &PointI32::of(5, 4));
        assert_eq!(r, RectU32::of(5, 4, 17, 19));
        assign_translate(&mut r, &PointI32::of(-4, -2));
        assert_eq!(r, RectU32::of(1, 2, 13, 17));
    }

    #[test]
    fn assign_translate_small_rect_to_bounds() {
        let mut min_r = RectU32::of(2, 5, 12, 15);
        assign_translate(&mut min_r, &PointI32::of(-2, -5));
        assert_eq!(min_r, RectU32::of(0, 0, 10, 10));

        let mut max_r = RectU32::of(u32::MAX - 12, u32::MAX - 15, u32::MAX - 2, u32::MAX - 5);
        assign_translate(&mut max_r, &PointI32::of(2, 5));
        assert_eq!(max_r, RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX, u32::MAX));
    }

    #[test]
    fn assign_translate_big_rect_to_bounds() {
        let mut min_r = RectU32::of(2, 5, u32::MAX, u32::MAX);
        assign_translate(&mut min_r, &PointI32::of(-2, -5));
        assert_eq!(min_r, RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5));

        let mut max_r = RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5);
        assign_translate(&mut max_r, &PointI32::of(2, 5));
        assert_eq!(max_r, RectU32::of(2, 5, u32::MAX, u32::MAX));
    }

    #[test]
    fn assign_translate_small_rect_out_of_bounds() {
        let mut r_min = RectU32::of(10, 5, 20, 30);
        assign_translate(&mut r_min, &PointI32::of(-20, -20));
        assert_eq!(r_min, RectU32::of(0, 0, 10, 25));

        let mut r_max = RectU32::of(u32::MAX - 20, u32::MAX - 30, u32::MAX - 5, u32::MAX - 10);
        assign_translate(&mut r_max, &PointI32::of(20, 20));
        assert_eq!(r_max, RectU32::of(u32::MAX - 15, u32::MAX - 20, u32::MAX, u32::MAX));
    }

    #[test]
    fn assign_translate_big_rect_out_of_bounds() {
        let mut r_min = RectU32::of(10, 5, u32::MAX, u32::MAX);
        assign_translate(&mut r_min, &PointI32::of(-20, -20));
        assert_eq!(r_min, RectU32::of(0, 0, u32::MAX - 10, u32::MAX - 5));

        let mut r_max = RectU32::of(0, 0, u32::MAX - 5, u32::MAX - 10);
        assign_translate(&mut r_max, &PointI32::of(20, 20));
        assert_eq!(r_max, RectU32::of(5, 10, u32::MAX, u32::MAX));
    }

    #[test]
    fn assign_translate_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU32::of(1, 1, 10, 10);
        assign_translate(&mut r_min, &PointI32::min());
        assert_eq!(r_min, RectU32::of(0, 0, 9, 9));

        let mut r_max = RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1);
        assign_translate(&mut r_max, &PointI32::max());
        assert_eq!(r_max, RectU32::of(u32::MAX - 9, u32::MAX - 9, u32::MAX, u32::MAX));
    }

    #[test]
    fn assign_translate_big_rect_limits_out_of_bounds() {
        let mut r = RectU32::largest();
        assign_translate(&mut r, &PointI32::min());
        assert_eq!(r, RectU32::largest());
        assign_translate(&mut r, &PointI32::max());
        assert_eq!(r, RectU32::largest());

        let mut r_min = RectU32::of(1, 1, u32::MAX, u32::MAX);
        assign_translate(&mut r_min, &PointI32::min());
        assert_eq!(r_min, RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1));

        let mut r_max = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
        assign_translate(&mut r_max, &PointI32::max());
        assert_eq!(r_max, RectU32::of(1, 1, u32::MAX, u32::MAX));
    }

    #[test]
    fn test_translate() {
        assert_eq!(translate(&RectU32::of(0, 0, 12, 15), &PointI32::of(5, 4)), RectU32::of(5, 4, 17, 19));
        assert_eq!(translate(&RectU32::of(5, 4, 17, 19), &PointI32::of(-4, -2)), RectU32::of(1, 2, 13, 17));
    }

    #[test]
    fn translate_small_rect_to_bounds() {
        assert_eq!(translate(&RectU32::of(2, 5, 12, 15), &PointI32::of(-2, -5)), RectU32::of(0, 0, 10, 10));
        assert_eq!(
            translate(&RectU32::of(u32::MAX - 12, u32::MAX - 15, u32::MAX - 2, u32::MAX - 5), &PointI32::of(2, 5)),
            RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX, u32::MAX)
        );
    }

    #[test]
    fn translate_big_rect_to_bounds() {
        assert_eq!(translate(&RectU32::of(2, 5, u32::MAX, u32::MAX), &PointI32::of(-2, -5)), RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5));
        assert_eq!(translate(&RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5), &PointI32::of(2, 5)), RectU32::of(2, 5, u32::MAX, u32::MAX));
    }

    #[test]
    fn translate_small_rect_out_of_bounds() {
        assert_eq!(translate(&RectU32::of(10, 5, 20, 30), &PointI32::of(-20, -20)), RectU32::of(0, 0, 10, 25));
        assert_eq!(
            translate(&RectU32::of(u32::MAX - 20, u32::MAX - 30, u32::MAX - 5, u32::MAX - 10), &PointI32::of(20, 20)),
            RectU32::of(u32::MAX - 15, u32::MAX - 20, u32::MAX, u32::MAX)
        );
    }

    #[test]
    fn translate_big_rect_out_of_bounds() {
        assert_eq!(translate(&RectU32::of(10, 5, u32::MAX, u32::MAX), &PointI32::of(-20, -20)), RectU32::of(0, 0, u32::MAX - 10, u32::MAX - 5));
        assert_eq!(translate(&RectU32::of(0, 0, u32::MAX - 5, u32::MAX - 10), &PointI32::of(20, 20)), RectU32::of(5, 10, u32::MAX, u32::MAX));
    }

    #[test]
    fn translate_small_rect_limits_out_of_bounds() {
        assert_eq!(translate(&RectU32::of(1, 1, 10, 10), &PointI32::min()), RectU32::of(0, 0, 9, 9));
        assert_eq!(
            translate(&RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1), &PointI32::max()),
            RectU32::of(u32::MAX - 9, u32::MAX - 9, u32::MAX, u32::MAX)
        );
    }

    #[test]
    fn translate_big_rect_limits_out_of_bounds() {
        assert_eq!(translate(&RectU32::of(1, 1, u32::MAX, u32::MAX), &PointI32::min()), RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1));
        assert_eq!(translate(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1), &PointI32::max()), RectU32::of(1, 1, u32::MAX, u32::MAX));
    }
}
