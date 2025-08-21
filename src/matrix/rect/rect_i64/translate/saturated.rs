use crate::matrix::{
    point::point_i64::PointI64,
    rect::rect_i64::{RectI64, delta_col, delta_row},
};

pub fn assign_translate(r: &mut RectI64, delta: &PointI64) {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let temp_min_row = i128::from(r.min.row) + i128::from(delta.row);
    let temp_min_col = i128::from(r.min.col) + i128::from(delta.col);
    let min_row = temp_min_row.clamp(i128::from(i64::MIN), i128::from(i64::MAX) - i128::from(d_row));
    let min_col = temp_min_col.clamp(i128::from(i64::MIN), i128::from(i64::MAX) - i128::from(d_col));
    r.min.row = min_row as i64;
    r.min.col = min_col as i64;
    r.max.row = (min_row + i128::from(d_row)) as i64;
    r.max.col = (min_col + i128::from(d_col)) as i64;
}

pub fn translate(r: &RectI64, delta: &PointI64) -> RectI64 {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let temp_min_row = i128::from(r.min.row) + i128::from(delta.row);
    let temp_min_col = i128::from(r.min.col) + i128::from(delta.col);
    let min_row = temp_min_row.clamp(i128::from(i64::MIN), i128::from(i64::MAX) - i128::from(d_row));
    let min_col = temp_min_col.clamp(i128::from(i64::MIN), i128::from(i64::MAX) - i128::from(d_col));
    let max_row = min_row + i128::from(d_row);
    let max_col = min_col + i128::from(d_col);
    RectI64 { min: PointI64 { row: min_row as i64, col: min_col as i64 }, max: PointI64 { row: max_row as i64, col: max_col as i64 } }
}

#[cfg(test)]
mod tests {
    use crate::matrix::{point::point_i64::PointI64, rect::rect_i64::RectI64};

    use super::{assign_translate, translate};

    #[test]
    fn test_assign_translate() {
        let mut r = RectI64::of(0, 0, 10, 10);
        assign_translate(&mut r, &PointI64::of(10, 20));
        assert_eq!(r, RectI64::of(10, 20, 20, 30));
        assign_translate(&mut r, &PointI64::of(-20, -15));
        assert_eq!(r, RectI64::of(-10, 5, 0, 15));
    }

    #[test]
    fn assign_translate_small_rect_to_bounds() {
        let mut min_r = RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MIN + 12, i64::MIN + 15);
        assign_translate(&mut min_r, &PointI64::of(-2, -5));
        assert_eq!(min_r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10));

        let mut max_r = RectI64::of(i64::MAX - 12, i64::MAX - 15, i64::MAX - 2, i64::MAX - 5);
        assign_translate(&mut max_r, &PointI64::of(2, 5));
        assert_eq!(max_r, RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX, i64::MAX));
    }

    #[test]
    fn assign_translate_big_rect_to_bounds() {
        let mut min_r = RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX);
        assign_translate(&mut min_r, &PointI64::of(-2, -5));
        assert_eq!(min_r, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5));

        let mut max_r = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5);
        assign_translate(&mut max_r, &PointI64::of(2, 5));
        assert_eq!(max_r, RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX));
    }

    #[test]
    fn assign_translate_small_rect_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MIN + 20, i64::MIN + 30);
        assign_translate(&mut r_min, &PointI64::of(-20, -20));
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 25));

        let mut r_max = RectI64::of(i64::MAX - 20, i64::MAX - 30, i64::MAX - 5, i64::MAX - 10);
        assign_translate(&mut r_max, &PointI64::of(20, 20));
        assert_eq!(r_max, RectI64::of(i64::MAX - 15, i64::MAX - 20, i64::MAX, i64::MAX));
    }

    #[test]
    fn assign_translate_big_rect_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MAX, i64::MAX);
        assign_translate(&mut r_min, &PointI64::of(-20, -20));
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 10, i64::MAX - 5));

        let mut r_max = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 5, i64::MAX - 10);
        assign_translate(&mut r_max, &PointI64::of(20, 20));
        assert_eq!(r_max, RectI64::of(i64::MIN + 5, i64::MIN + 10, i64::MAX, i64::MAX));
    }

    #[test]
    fn assign_translate_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MIN + 10, i64::MIN + 10);
        assign_translate(&mut r_min, &PointI64::min());
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 9, i64::MIN + 9));

        let mut r_max = RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX - 1, i64::MAX - 1);
        assign_translate(&mut r_max, &PointI64::max());
        assert_eq!(r_max, RectI64::of(i64::MAX - 9, i64::MAX - 9, i64::MAX, i64::MAX));
    }

    #[test]
    fn assign_translate_big_rect_limits_out_of_bounds() {
        let mut r_min = RectI64::largest();
        assign_translate(&mut r_min, &PointI64::min());
        assert_eq!(r_min, RectI64::largest());
        assign_translate(&mut r_min, &PointI64::max());
        assert_eq!(r_min, RectI64::largest());

        let mut r_min = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX);
        assign_translate(&mut r_min, &PointI64::min());
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1));

        let mut r_max = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1);
        assign_translate(&mut r_max, &PointI64::max());
        assert_eq!(r_max, RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX));
    }

    #[test]
    fn test_translate() {
        assert_eq!(translate(&RectI64::of(0, 0, 10, 10), &PointI64::of(10, 20)), RectI64::of(10, 20, 20, 30));
        assert_eq!(translate(&RectI64::of(10, 20, 20, 30), &PointI64::of(-20, -15)), RectI64::of(-10, 5, 0, 15));
    }

    #[test]
    fn translate_small_rect_to_bounds() {
        assert_eq!(
            translate(&RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MIN + 12, i64::MIN + 15), &PointI64::of(-2, -5)),
            RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10)
        );
        assert_eq!(
            translate(&RectI64::of(i64::MAX - 12, i64::MAX - 15, i64::MAX - 2, i64::MAX - 5), &PointI64::of(2, 5)),
            RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX, i64::MAX)
        );
    }

    #[test]
    fn translate_big_rect_to_bounds() {
        assert_eq!(
            translate(&RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX), &PointI64::of(-2, -5)),
            RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5)
        );
        assert_eq!(
            translate(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5), &PointI64::of(2, 5)),
            RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX)
        );
    }

    #[test]
    fn translate_small_rect_out_of_bounds() {
        assert_eq!(
            translate(&RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MIN + 20, i64::MIN + 30), &PointI64::of(-20, -20)),
            RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 25)
        );
        assert_eq!(
            translate(&RectI64::of(i64::MAX - 20, i64::MAX - 30, i64::MAX - 5, i64::MAX - 10), &PointI64::of(20, 20)),
            RectI64::of(i64::MAX - 15, i64::MAX - 20, i64::MAX, i64::MAX)
        );
    }

    #[test]
    fn translate_big_rect_out_of_bounds() {
        assert_eq!(
            translate(&RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MAX, i64::MAX), &PointI64::of(-20, -20)),
            RectI64::of(i64::MIN, i64::MIN, i64::MAX - 10, i64::MAX - 5)
        );
        assert_eq!(
            translate(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 5, i64::MAX - 10), &PointI64::of(20, 20)),
            RectI64::of(i64::MIN + 5, i64::MIN + 10, i64::MAX, i64::MAX)
        );
    }

    #[test]
    fn translate_small_rect_limits_out_of_bounds() {
        assert_eq!(
            translate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MIN + 10, i64::MIN + 10), &PointI64::min()),
            RectI64::of(i64::MIN, i64::MIN, i64::MIN + 9, i64::MIN + 9)
        );
        assert_eq!(
            translate(&RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX - 1, i64::MAX - 1), &PointI64::max()),
            RectI64::of(i64::MAX - 9, i64::MAX - 9, i64::MAX, i64::MAX)
        );
    }

    #[test]
    fn translate_big_rect_limits_out_of_bounds() {
        assert_eq!(translate(&RectI64::largest(), &PointI64::min()), RectI64::largest());
        assert_eq!(translate(&RectI64::largest(), &PointI64::max()), RectI64::largest());
        assert_eq!(
            translate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX), &PointI64::min()),
            RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1)
        );
        assert_eq!(
            translate(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1), &PointI64::max()),
            RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX)
        );
    }
}
