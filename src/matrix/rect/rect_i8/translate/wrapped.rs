use crate::matrix::{
    point::point_i8::PointI8,
    rect::rect_i8::{RectI8, delta_row, delta_col},
};

pub fn assign_translate(r: &mut RectI8, delta: &PointI8) {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let min_row = r.min.row.wrapping_add(delta.row);
    let min_col = r.min.col.wrapping_add(delta.col);
    let max_row = min_row.wrapping_add_unsigned(d_row);
    let max_col = min_col.wrapping_add_unsigned(d_col);
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
}

pub fn translate(r: &RectI8, delta: &PointI8) -> RectI8 {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let min_row = r.min.row.wrapping_add(delta.row);
    let min_col = r.min.col.wrapping_add(delta.col);
    let max_row = min_row.wrapping_add_unsigned(d_row);
    let max_col = min_col.wrapping_add_unsigned(d_col);
    RectI8 { min: PointI8 { row: min_row, col: min_col }, max: PointI8 { row: max_row, col: max_col } }
}

#[cfg(test)]
mod tests {
    use crate::matrix::{point::point_i8::PointI8, rect::rect_i8::RectI8};

    use super::{assign_translate, translate};

    #[test]
    fn test_assign_translate() {
        let mut r = RectI8::of(0, 0, 12, 15);
        assign_translate(&mut r, &PointI8::of(5, 4));
        assert_eq!(r, RectI8::of(5, 4, 17, 19));
        assign_translate(&mut r, &PointI8::of(-4, -2));
        assert_eq!(r, RectI8::of(1, 2, 13, 17));
    }

    #[test]
    fn assign_translate_small_rect_to_bounds() {
        let mut min_r = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MIN + 12, i8::MIN + 15);
        assign_translate(&mut min_r, &PointI8::of(-2, -5));
        assert_eq!(min_r, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10));

        let mut max_r = RectI8::of(i8::MAX - 12, i8::MAX - 15, i8::MAX - 2, i8::MAX - 5);
        assign_translate(&mut max_r, &PointI8::of(2, 5));
        assert_eq!(max_r, RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX, i8::MAX));
    }

    #[test]
    fn assign_translate_big_rect_to_bounds() {
        let mut min_r = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX);
        assign_translate(&mut min_r, &PointI8::of(-2, -5));
        assert_eq!(min_r, RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5));

        let mut max_r = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5);
        assign_translate(&mut max_r, &PointI8::of(2, 5));
        assert_eq!(max_r, RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX));
    }

    #[test]
    fn assign_translate_small_rect_out_of_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MIN + 20, i8::MIN + 30);
        assign_translate(&mut r_min, &PointI8::of(-20, -20));
        assert_eq!(r_min, RectI8::of(i8::MAX - 9, i8::MAX - 14, i8::MIN, i8::MIN + 10));

        let mut r_max = RectI8::of(i8::MAX - 20, i8::MAX - 30, i8::MAX - 5, i8::MAX - 10);
        assign_translate(&mut r_max, &PointI8::of(20, 20));
        assert_eq!(r_max, RectI8::of(i8::MAX, i8::MAX - 10, i8::MIN + 14, i8::MIN + 9));
    }

    #[test]
    fn assign_translate_big_rect_out_of_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MAX, i8::MAX);
        assign_translate(&mut r_min, &PointI8::of(-20, -20));
        assert_eq!(r_min, RectI8::of(i8::MAX - 9, i8::MAX - 14, i8::MAX - 20, i8::MAX - 20));

        let mut r_max = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 5, i8::MAX - 10);
        assign_translate(&mut r_max, &PointI8::of(20, 20));
        assert_eq!(r_max, RectI8::of(i8::MIN + 20, i8::MIN + 20, i8::MIN + 14, i8::MIN + 9));
    }

    #[test]
    fn assign_translate_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MIN + 10, i8::MIN + 10);
        assign_translate(&mut r_min, &PointI8::min());
        assert_eq!(r_min, RectI8::of(1, 1, 10, 10));

        let mut r_max = RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX - 1, i8::MAX - 1);
        assign_translate(&mut r_max, &PointI8::max());
        assert_eq!(r_max, RectI8::of(-12, -12, -3, -3));
    }

    #[test]
    fn assign_translate_big_rect_limits_out_of_bounds() {
        let mut r1 = RectI8::largest();
        assign_translate(&mut r1, &PointI8::min());
        assert_eq!(r1, RectI8::of(0, 0, -1, -1));

        let mut r2 = RectI8::largest();
        assign_translate(&mut r2, &PointI8::max());
        assert_eq!(r2, RectI8::of(-1, -1, -2, -2));

        let mut r_min = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX);
        assign_translate(&mut r_min, &PointI8::min());
        assert_eq!(r_min, RectI8::of(1, 1, -1, -1));

        let mut r_max = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1);
        assign_translate(&mut r_max, &PointI8::max());
        assert_eq!(r_max, RectI8::of(-1, -1, -3, -3));
    }

    #[test]
    fn test_translate() {
        assert_eq!(translate(&RectI8::of(0, 0, 12, 15), &PointI8::of(5, 4)), RectI8::of(5, 4, 17, 19));
        assert_eq!(translate(&RectI8::of(5, 4, 17, 19), &PointI8::of(-4, -2)), RectI8::of(1, 2, 13, 17));
    }

    #[test]
    fn translate_small_rect_to_bounds() {
        assert_eq!(
            translate(&RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MIN + 12, i8::MIN + 15), &PointI8::of(-2, -5)),
            RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10)
        );
        assert_eq!(
            translate(&RectI8::of(i8::MAX - 12, i8::MAX - 15, i8::MAX - 2, i8::MAX - 5), &PointI8::of(2, 5)),
            RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX, i8::MAX)
        );
    }

    #[test]
    fn translate_big_rect_to_bounds() {
        assert_eq!(
            translate(&RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX), &PointI8::of(-2, -5)),
            RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5)
        );
        assert_eq!(
            translate(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5), &PointI8::of(2, 5)),
            RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX)
        );
    }

    #[test]
    fn translate_small_rect_out_of_bounds() {
        assert_eq!(
            translate(&RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MIN + 20, i8::MIN + 30), &PointI8::of(-20, -20)),
            RectI8::of(i8::MAX - 9, i8::MAX - 14, i8::MIN, i8::MIN + 10)
        );
        assert_eq!(
            translate(&RectI8::of(i8::MAX - 20, i8::MAX - 30, i8::MAX - 5, i8::MAX - 10), &PointI8::of(20, 20)),
            RectI8::of(i8::MAX, i8::MAX - 10, i8::MIN + 14, i8::MIN + 9)
        );
    }

    #[test]
    fn translate_big_rect_out_of_bounds() {
        assert_eq!(
            translate(&RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MAX, i8::MAX), &PointI8::of(-20, -20)),
            RectI8::of(i8::MAX - 9, i8::MAX - 14, i8::MAX - 20, i8::MAX - 20)
        );
        assert_eq!(
            translate(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 5, i8::MAX - 10), &PointI8::of(20, 20)),
            RectI8::of(i8::MIN + 20, i8::MIN + 20, i8::MIN + 14, i8::MIN + 9)
        );
    }

    #[test]
    fn translate_small_rect_limits_out_of_bounds() {
        assert_eq!(translate(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MIN + 10, i8::MIN + 10), &PointI8::min()), RectI8::of(1, 1, 10, 10));
        assert_eq!(translate(&RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX - 1, i8::MAX - 1), &PointI8::max()), RectI8::of(-12, -12, -3, -3));
    }

    #[test]
    fn translate_big_rect_limits_out_of_bounds() {
        assert_eq!(translate(&RectI8::largest(), &PointI8::min()), RectI8::of(0, 0, -1, -1));
        assert_eq!(translate(&RectI8::largest(), &PointI8::max()), RectI8::of(-1, -1, -2, -2));
        assert_eq!(translate(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX), &PointI8::min()), RectI8::of(1, 1, -1, -1));
        assert_eq!(translate(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1), &PointI8::max()), RectI8::of(-1, -1, -3, -3));
    }
}
