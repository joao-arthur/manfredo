use crate::matrix::{
    point::point_i16::PointI16,
    rect::rect_i16::{RectI16, delta_col, delta_row},
};

pub fn assign_translate(r: &mut RectI16, delta: &PointI16) {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let temp_min_row = i32::from(r.min.row) + i32::from(delta.row);
    let temp_min_col = i32::from(r.min.col) + i32::from(delta.col);
    let min_row = temp_min_row.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(d_row));
    let min_col = temp_min_col.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(d_col));
    r.min.row = min_row as i16;
    r.min.col = min_col as i16;
    r.max.row = (min_row + i32::from(d_row)) as i16;
    r.max.col = (min_col + i32::from(d_col)) as i16;
}

pub fn translate(r: &RectI16, delta: &PointI16) -> RectI16 {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let temp_min_row = i32::from(r.min.row) + i32::from(delta.row);
    let temp_min_col = i32::from(r.min.col) + i32::from(delta.col);
    let min_row = temp_min_row.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(d_row));
    let min_col = temp_min_col.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(d_col));
    let max_row = min_row + i32::from(d_row);
    let max_col = min_col + i32::from(d_col);
    RectI16 { min: PointI16 { row: min_row as i16, col: min_col as i16 }, max: PointI16 { row: max_row as i16, col: max_col as i16 } }
}

#[cfg(test)]
mod tests {
    use super::{assign_translate, translate};
    use crate::matrix::{point::point_i16::PointI16, rect::rect_i16::RectI16};

    #[test]
    fn test_assign_translate() {
        let mut r = RectI16::of(0, 0, 10, 10);
        assign_translate(&mut r, &PointI16::of(10, 20));
        assert_eq!(r, RectI16::of(10, 20, 20, 30));
        assign_translate(&mut r, &PointI16::of(-20, -15));
        assert_eq!(r, RectI16::of(-10, 5, 0, 15));
    }

    #[test]
    fn assign_translate_small_rect_to_bounds() {
        let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MIN + 12, i16::MIN + 15);
        assign_translate(&mut min_r, &PointI16::of(-2, -5));
        assert_eq!(min_r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 10));

        let mut max_r = RectI16::of(i16::MAX - 12, i16::MAX - 15, i16::MAX - 2, i16::MAX - 5);
        assign_translate(&mut max_r, &PointI16::of(2, 5));
        assert_eq!(max_r, RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX, i16::MAX));
    }

    #[test]
    fn assign_translate_big_rect_to_bounds() {
        let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX);
        assign_translate(&mut min_r, &PointI16::of(-2, -5));
        assert_eq!(min_r, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5));

        let mut max_r = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5);
        assign_translate(&mut max_r, &PointI16::of(2, 5));
        assert_eq!(max_r, RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX));
    }

    #[test]
    fn assign_translate_small_rect_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30);
        assign_translate(&mut r_min, &PointI16::of(-20, -20));
        assert_eq!(r_min, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 25));

        let mut r_max = RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10);
        assign_translate(&mut r_max, &PointI16::of(20, 20));
        assert_eq!(r_max, RectI16::of(i16::MAX - 15, i16::MAX - 20, i16::MAX, i16::MAX));
    }

    #[test]
    fn assign_translate_big_rect_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX);
        assign_translate(&mut r_min, &PointI16::of(-20, -20));
        assert_eq!(r_min, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 10, i16::MAX - 5));

        let mut r_max = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10);
        assign_translate(&mut r_max, &PointI16::of(20, 20));
        assert_eq!(r_max, RectI16::of(i16::MIN + 5, i16::MIN + 10, i16::MAX, i16::MAX));
    }

    #[test]
    fn assign_translate_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10);
        assign_translate(&mut r_min, &PointI16::min());
        assert_eq!(r_min, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 9, i16::MIN + 9));

        let mut r_max = RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1);
        assign_translate(&mut r_max, &PointI16::max());
        assert_eq!(r_max, RectI16::of(i16::MAX - 9, i16::MAX - 9, i16::MAX, i16::MAX));
    }

    #[test]
    fn assign_translate_big_rect_limits_out_of_bounds() {
        let mut r_min = RectI16::largest();
        assign_translate(&mut r_min, &PointI16::min());
        assert_eq!(r_min, RectI16::largest());
        assign_translate(&mut r_min, &PointI16::max());
        assert_eq!(r_min, RectI16::largest());

        let mut r_min = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX);
        assign_translate(&mut r_min, &PointI16::min());
        assert_eq!(r_min, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1));

        let mut r_max = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1);
        assign_translate(&mut r_max, &PointI16::max());
        assert_eq!(r_max, RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX));
    }

    #[test]
    fn test_translate() {
        assert_eq!(translate(&RectI16::of(0, 0, 10, 10), &PointI16::of(10, 20)), RectI16::of(10, 20, 20, 30));
        assert_eq!(translate(&RectI16::of(10, 20, 20, 30), &PointI16::of(-20, -15)), RectI16::of(-10, 5, 0, 15));
    }

    #[test]
    fn translate_small_rect_to_bounds() {
        assert_eq!(
            translate(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MIN + 12, i16::MIN + 15), &PointI16::of(-2, -5)),
            RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 10)
        );
        assert_eq!(
            translate(&RectI16::of(i16::MAX - 12, i16::MAX - 15, i16::MAX - 2, i16::MAX - 5), &PointI16::of(2, 5)),
            RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX, i16::MAX)
        );
    }

    #[test]
    fn translate_big_rect_to_bounds() {
        assert_eq!(
            translate(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX), &PointI16::of(-2, -5)),
            RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5)
        );
        assert_eq!(
            translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5), &PointI16::of(2, 5)),
            RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX)
        );
    }

    #[test]
    fn translate_small_rect_out_of_bounds() {
        assert_eq!(
            translate(&RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30), &PointI16::of(-20, -20)),
            RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 25)
        );
        assert_eq!(
            translate(&RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10), &PointI16::of(20, 20)),
            RectI16::of(i16::MAX - 15, i16::MAX - 20, i16::MAX, i16::MAX)
        );
    }

    #[test]
    fn translate_big_rect_out_of_bounds() {
        assert_eq!(
            translate(&RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX), &PointI16::of(-20, -20)),
            RectI16::of(i16::MIN, i16::MIN, i16::MAX - 10, i16::MAX - 5)
        );
        assert_eq!(
            translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10), &PointI16::of(20, 20)),
            RectI16::of(i16::MIN + 5, i16::MIN + 10, i16::MAX, i16::MAX)
        );
    }

    #[test]
    fn translate_small_rect_limits_out_of_bounds() {
        assert_eq!(
            translate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10), &PointI16::min()),
            RectI16::of(i16::MIN, i16::MIN, i16::MIN + 9, i16::MIN + 9)
        );
        assert_eq!(
            translate(&RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1), &PointI16::max()),
            RectI16::of(i16::MAX - 9, i16::MAX - 9, i16::MAX, i16::MAX)
        );
    }

    #[test]
    fn translate_big_rect_limits_out_of_bounds() {
        assert_eq!(translate(&RectI16::largest(), &PointI16::min()), RectI16::largest());
        assert_eq!(translate(&RectI16::largest(), &PointI16::max()), RectI16::largest());
        assert_eq!(
            translate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX), &PointI16::min()),
            RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1)
        );
        assert_eq!(
            translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1), &PointI16::max()),
            RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX)
        );
    }
}
