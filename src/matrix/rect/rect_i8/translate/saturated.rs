use crate::matrix::{
    point::point_i8::PointI8,
    rect::rect_i8::{RectI8, delta_col, delta_row},
};

pub fn saturating_translate_assign(r: &mut RectI8, delta: &PointI8) {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let temp_min_row = i16::from(r.min.row) + i16::from(delta.row);
    let temp_min_col = i16::from(r.min.col) + i16::from(delta.col);
    let min_row = temp_min_row.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(d_row));
    let min_col = temp_min_col.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(d_col));
    r.min.row = min_row as i8;
    r.min.col = min_col as i8;
    r.max.row = (min_row + i16::from(d_row)) as i8;
    r.max.col = (min_col + i16::from(d_col)) as i8;
}

pub fn saturating_translate(r: &RectI8, delta: &PointI8) -> RectI8 {
    let d_row = delta_row(r);
    let d_col = delta_col(r);
    let temp_min_row = i16::from(r.min.row) + i16::from(delta.row);
    let temp_min_col = i16::from(r.min.col) + i16::from(delta.col);
    let min_row = temp_min_row.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(d_row));
    let min_col = temp_min_col.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(d_col));
    let max_row = min_row + i16::from(d_row);
    let max_col = min_col + i16::from(d_col);
    RectI8 { min: PointI8 { row: min_row as i8, col: min_col as i8 }, max: PointI8 { row: max_row as i8, col: max_col as i8 } }
}

#[cfg(test)]
mod tests {
    use super::{saturating_translate_assign, saturating_translate};
    use crate::matrix::{point::point_i8::PointI8, rect::rect_i8::RectI8};

    #[test]
    fn test_saturating_translate_assign() {
        let mut r = RectI8::of(0, 0, 10, 10);
        saturating_translate_assign(&mut r, &PointI8::of(10, 20));
        assert_eq!(r, RectI8::of(10, 20, 20, 30));
        saturating_translate_assign(&mut r, &PointI8::of(-20, -15));
        assert_eq!(r, RectI8::of(-10, 5, 0, 15));
    }

    #[test]
    fn saturating_translate_assign_small_rect_to_bounds() {
        let mut min_r = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MIN + 12, i8::MIN + 15);
        saturating_translate_assign(&mut min_r, &PointI8::of(-2, -5));
        assert_eq!(min_r, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10));

        let mut max_r = RectI8::of(i8::MAX - 12, i8::MAX - 15, i8::MAX - 2, i8::MAX - 5);
        saturating_translate_assign(&mut max_r, &PointI8::of(2, 5));
        assert_eq!(max_r, RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX, i8::MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_to_bounds() {
        let mut min_r = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX);
        saturating_translate_assign(&mut min_r, &PointI8::of(-2, -5));
        assert_eq!(min_r, RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5));

        let mut max_r = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5);
        saturating_translate_assign(&mut max_r, &PointI8::of(2, 5));
        assert_eq!(max_r, RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX));
    }

    #[test]
    fn saturating_translate_assign_small_rect_out_of_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MIN + 20, i8::MIN + 30);
        saturating_translate_assign(&mut r_min, &PointI8::of(-20, -20));
        assert_eq!(r_min, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 25));

        let mut r_max = RectI8::of(i8::MAX - 20, i8::MAX - 30, i8::MAX - 5, i8::MAX - 10);
        saturating_translate_assign(&mut r_max, &PointI8::of(20, 20));
        assert_eq!(r_max, RectI8::of(i8::MAX - 15, i8::MAX - 20, i8::MAX, i8::MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_out_of_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MAX, i8::MAX);
        saturating_translate_assign(&mut r_min, &PointI8::of(-20, -20));
        assert_eq!(r_min, RectI8::of(i8::MIN, i8::MIN, i8::MAX - 10, i8::MAX - 5));

        let mut r_max = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 5, i8::MAX - 10);
        saturating_translate_assign(&mut r_max, &PointI8::of(20, 20));
        assert_eq!(r_max, RectI8::of(i8::MIN + 5, i8::MIN + 10, i8::MAX, i8::MAX));
    }

    #[test]
    fn saturating_translate_assign_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MIN + 10, i8::MIN + 10);
        saturating_translate_assign(&mut r_min, &PointI8::min());
        assert_eq!(r_min, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 9, i8::MIN + 9));

        let mut r_max = RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX - 1, i8::MAX - 1);
        saturating_translate_assign(&mut r_max, &PointI8::max());
        assert_eq!(r_max, RectI8::of(i8::MAX - 9, i8::MAX - 9, i8::MAX, i8::MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_limits_out_of_bounds() {
        let mut r_min = RectI8::largest();
        saturating_translate_assign(&mut r_min, &PointI8::min());
        assert_eq!(r_min, RectI8::largest());
        saturating_translate_assign(&mut r_min, &PointI8::max());
        assert_eq!(r_min, RectI8::largest());

        let mut r_min = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX);
        saturating_translate_assign(&mut r_min, &PointI8::min());
        assert_eq!(r_min, RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1));

        let mut r_max = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1);
        saturating_translate_assign(&mut r_max, &PointI8::max());
        assert_eq!(r_max, RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX));
    }

    #[test]
    fn test_saturating_translate() {
        assert_eq!(saturating_translate(&RectI8::of(0, 0, 10, 10), &PointI8::of(10, 20)), RectI8::of(10, 20, 20, 30));
        assert_eq!(saturating_translate(&RectI8::of(10, 20, 20, 30), &PointI8::of(-20, -15)), RectI8::of(-10, 5, 0, 15));
    }

    #[test]
    fn saturating_translate_small_rect_to_bounds() {
        assert_eq!(
            saturating_translate(&RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MIN + 12, i8::MIN + 15), &PointI8::of(-2, -5)),
            RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10)
        );
        assert_eq!(
            saturating_translate(&RectI8::of(i8::MAX - 12, i8::MAX - 15, i8::MAX - 2, i8::MAX - 5), &PointI8::of(2, 5)),
            RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX, i8::MAX)
        );
    }

    #[test]
    fn saturating_translate_big_rect_to_bounds() {
        assert_eq!(
            saturating_translate(&RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX), &PointI8::of(-2, -5)),
            RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5)
        );
        assert_eq!(
            saturating_translate(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5), &PointI8::of(2, 5)),
            RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX)
        );
    }

    #[test]
    fn saturating_translate_small_rect_out_of_bounds() {
        assert_eq!(
            saturating_translate(&RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MIN + 20, i8::MIN + 30), &PointI8::of(-20, -20)),
            RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 25)
        );
        assert_eq!(
            saturating_translate(&RectI8::of(i8::MAX - 20, i8::MAX - 30, i8::MAX - 5, i8::MAX - 10), &PointI8::of(20, 20)),
            RectI8::of(i8::MAX - 15, i8::MAX - 20, i8::MAX, i8::MAX)
        );
    }

    #[test]
    fn saturating_translate_big_rect_out_of_bounds() {
        assert_eq!(
            saturating_translate(&RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MAX, i8::MAX), &PointI8::of(-20, -20)),
            RectI8::of(i8::MIN, i8::MIN, i8::MAX - 10, i8::MAX - 5)
        );
        assert_eq!(
            saturating_translate(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 5, i8::MAX - 10), &PointI8::of(20, 20)),
            RectI8::of(i8::MIN + 5, i8::MIN + 10, i8::MAX, i8::MAX)
        );
    }

    #[test]
    fn saturating_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(
            saturating_translate(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MIN + 10, i8::MIN + 10), &PointI8::min()),
            RectI8::of(i8::MIN, i8::MIN, i8::MIN + 9, i8::MIN + 9)
        );
        assert_eq!(
            saturating_translate(&RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX - 1, i8::MAX - 1), &PointI8::max()),
            RectI8::of(i8::MAX - 9, i8::MAX - 9, i8::MAX, i8::MAX)
        );
    }

    #[test]
    fn saturating_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(saturating_translate(&RectI8::largest(), &PointI8::min()), RectI8::largest());
        assert_eq!(saturating_translate(&RectI8::largest(), &PointI8::max()), RectI8::largest());
        assert_eq!(
            saturating_translate(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX), &PointI8::min()),
            RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1)
        );
        assert_eq!(
            saturating_translate(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1), &PointI8::max()),
            RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX)
        );
    }
}
