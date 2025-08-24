use crate::matrix::{
    point::point_i32::PointI32,
    rect::rect_i32::{RectI32, delta_col, delta_row},
};

pub fn saturating_translate_assign(r: &mut RectI32, delta: &PointI32) {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let temp_min_row = i64::from(r.min.row) + i64::from(delta.row);
    let temp_min_col = i64::from(r.min.col) + i64::from(delta.col);
    let min_row = temp_min_row.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(d_row));
    let min_col = temp_min_col.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(d_col));
    r.min.row = min_row as i32;
    r.min.col = min_col as i32;
    r.max.row = (min_row + i64::from(d_row)) as i32;
    r.max.col = (min_col + i64::from(d_col)) as i32;
}

pub fn saturating_translate(r: &RectI32, delta: &PointI32) -> RectI32 {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let temp_min_row = i64::from(r.min.row) + i64::from(delta.row);
    let temp_min_col = i64::from(r.min.col) + i64::from(delta.col);
    let min_row = temp_min_row.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(d_row));
    let min_col = temp_min_col.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(d_col));
    let max_row = min_row + i64::from(d_row);
    let max_col = min_col + i64::from(d_col);
    RectI32 { min: PointI32 { row: min_row as i32, col: min_col as i32 }, max: PointI32 { row: max_row as i32, col: max_col as i32 } }
}

#[cfg(test)]
mod tests {
    use super::{saturating_translate_assign, saturating_translate};
    use crate::matrix::{point::point_i32::PointI32, rect::rect_i32::RectI32};

    #[test]
    fn test_saturating_translate_assign() {
        let mut r = RectI32::of(0, 0, 10, 10);
        saturating_translate_assign(&mut r, &PointI32::of(10, 20));
        assert_eq!(r, RectI32::of(10, 20, 20, 30));
        saturating_translate_assign(&mut r, &PointI32::of(-20, -15));
        assert_eq!(r, RectI32::of(-10, 5, 0, 15));
    }

    #[test]
    fn saturating_translate_assign_small_rect_to_bounds() {
        let mut min_r = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MIN + 12, i32::MIN + 15);
        saturating_translate_assign(&mut min_r, &PointI32::of(-2, -5));
        assert_eq!(min_r, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 10, i32::MIN + 10));

        let mut max_r = RectI32::of(i32::MAX - 12, i32::MAX - 15, i32::MAX - 2, i32::MAX - 5);
        saturating_translate_assign(&mut max_r, &PointI32::of(2, 5));
        assert_eq!(max_r, RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX, i32::MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_to_bounds() {
        let mut min_r = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX);
        saturating_translate_assign(&mut min_r, &PointI32::of(-2, -5));
        assert_eq!(min_r, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5));

        let mut max_r = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5);
        saturating_translate_assign(&mut max_r, &PointI32::of(2, 5));
        assert_eq!(max_r, RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX));
    }

    #[test]
    fn saturating_translate_assign_small_rect_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MIN + 20, i32::MIN + 30);
        saturating_translate_assign(&mut r_min, &PointI32::of(-20, -20));
        assert_eq!(r_min, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 10, i32::MIN + 25));

        let mut r_max = RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX - 5, i32::MAX - 10);
        saturating_translate_assign(&mut r_max, &PointI32::of(20, 20));
        assert_eq!(r_max, RectI32::of(i32::MAX - 15, i32::MAX - 20, i32::MAX, i32::MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MAX, i32::MAX);
        saturating_translate_assign(&mut r_min, &PointI32::of(-20, -20));
        assert_eq!(r_min, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 10, i32::MAX - 5));

        let mut r_max = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 5, i32::MAX - 10);
        saturating_translate_assign(&mut r_max, &PointI32::of(20, 20));
        assert_eq!(r_max, RectI32::of(i32::MIN + 5, i32::MIN + 10, i32::MAX, i32::MAX));
    }

    #[test]
    fn saturating_translate_assign_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10);
        saturating_translate_assign(&mut r_min, &PointI32::min());
        assert_eq!(r_min, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 9, i32::MIN + 9));

        let mut r_max = RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1);
        saturating_translate_assign(&mut r_max, &PointI32::max());
        assert_eq!(r_max, RectI32::of(i32::MAX - 9, i32::MAX - 9, i32::MAX, i32::MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_limits_out_of_bounds() {
        let mut r_min = RectI32::largest();
        saturating_translate_assign(&mut r_min, &PointI32::min());
        assert_eq!(r_min, RectI32::largest());
        saturating_translate_assign(&mut r_min, &PointI32::max());
        assert_eq!(r_min, RectI32::largest());

        let mut r_min = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX);
        saturating_translate_assign(&mut r_min, &PointI32::min());
        assert_eq!(r_min, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1));

        let mut r_max = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1);
        saturating_translate_assign(&mut r_max, &PointI32::max());
        assert_eq!(r_max, RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX));
    }

    #[test]
    fn test_saturating_translate() {
        assert_eq!(saturating_translate(&RectI32::of(0, 0, 10, 10), &PointI32::of(10, 20)), RectI32::of(10, 20, 20, 30));
        assert_eq!(saturating_translate(&RectI32::of(10, 20, 20, 30), &PointI32::of(-20, -15)), RectI32::of(-10, 5, 0, 15));
    }

    #[test]
    fn saturating_translate_small_rect_to_bounds() {
        assert_eq!(
            saturating_translate(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MIN + 12, i32::MIN + 15), &PointI32::of(-2, -5)),
            RectI32::of(i32::MIN, i32::MIN, i32::MIN + 10, i32::MIN + 10)
        );
        assert_eq!(
            saturating_translate(&RectI32::of(i32::MAX - 12, i32::MAX - 15, i32::MAX - 2, i32::MAX - 5), &PointI32::of(2, 5)),
            RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX, i32::MAX)
        );
    }

    #[test]
    fn saturating_translate_big_rect_to_bounds() {
        assert_eq!(
            saturating_translate(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX), &PointI32::of(-2, -5)),
            RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5)
        );
        assert_eq!(
            saturating_translate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5), &PointI32::of(2, 5)),
            RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX)
        );
    }

    #[test]
    fn saturating_translate_small_rect_out_of_bounds() {
        assert_eq!(
            saturating_translate(&RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MIN + 20, i32::MIN + 30), &PointI32::of(-20, -20)),
            RectI32::of(i32::MIN, i32::MIN, i32::MIN + 10, i32::MIN + 25)
        );
        assert_eq!(
            saturating_translate(&RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX - 5, i32::MAX - 10), &PointI32::of(20, 20)),
            RectI32::of(i32::MAX - 15, i32::MAX - 20, i32::MAX, i32::MAX)
        );
    }

    #[test]
    fn saturating_translate_big_rect_out_of_bounds() {
        assert_eq!(
            saturating_translate(&RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MAX, i32::MAX), &PointI32::of(-20, -20)),
            RectI32::of(i32::MIN, i32::MIN, i32::MAX - 10, i32::MAX - 5)
        );
        assert_eq!(
            saturating_translate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 5, i32::MAX - 10), &PointI32::of(20, 20)),
            RectI32::of(i32::MIN + 5, i32::MIN + 10, i32::MAX, i32::MAX)
        );
    }

    #[test]
    fn saturating_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(
            saturating_translate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10), &PointI32::min()),
            RectI32::of(i32::MIN, i32::MIN, i32::MIN + 9, i32::MIN + 9)
        );
        assert_eq!(
            saturating_translate(&RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1), &PointI32::max()),
            RectI32::of(i32::MAX - 9, i32::MAX - 9, i32::MAX, i32::MAX)
        );
    }

    #[test]
    fn saturating_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(saturating_translate(&RectI32::largest(), &PointI32::min()), RectI32::largest());
        assert_eq!(saturating_translate(&RectI32::largest(), &PointI32::max()), RectI32::largest());
        assert_eq!(
            saturating_translate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX), &PointI32::min()),
            RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1)
        );
        assert_eq!(
            saturating_translate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1), &PointI32::max()),
            RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX)
        );
    }
}
