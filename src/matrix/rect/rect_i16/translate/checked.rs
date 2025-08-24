use crate::matrix::{point::point_i16::PointI16, rect::rect_i16::RectI16};

pub fn try_checked_translate_assign(r: &mut RectI16, delta: &PointI16) -> Option<()> {
    let min_row = r.min.row.checked_add(delta.row)?;
    let min_col = r.min.col.checked_add(delta.col)?;
    let max_row = r.max.row.checked_add(delta.row)?;
    let max_col = r.max.col.checked_add(delta.col)?;
    r.min.row = min_row;
    r.min.col = min_col;
    r.max.row = max_row;
    r.max.col = max_col;
    Some(())
}

pub fn try_checked_translate(r: &RectI16, delta: &PointI16) -> Option<RectI16> {
    let min_row = r.min.row.checked_add(delta.row)?;
    let min_col = r.min.col.checked_add(delta.col)?;
    let max_row = r.max.row.checked_add(delta.row)?;
    let max_col = r.max.col.checked_add(delta.col)?;
    Some(RectI16 { min: PointI16 { row: min_row, col: min_col }, max: PointI16 { row: max_row, col: max_col } })
}

pub fn checked_translate_assign(r: &mut RectI16, delta: &PointI16) {
    try_checked_translate_assign(r, delta).unwrap()
}

pub fn checked_translate(r: &RectI16, delta: &PointI16) -> RectI16 {
    try_checked_translate(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{checked_translate_assign, checked_translate, try_checked_translate_assign, try_checked_translate};
    use crate::matrix::{point::point_i16::PointI16, rect::rect_i16::RectI16};

    #[test]
    fn test_try_checked_translate_assign() {
        let mut r = RectI16::of(0, 0, 10, 10);
        assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(10, 20)), Some(()));
        assert_eq!(r, RectI16::of(10, 20, 20, 30));
        assert_eq!(try_checked_translate_assign(&mut r, &PointI16::of(-20, -15)), Some(()));
        assert_eq!(r, RectI16::of(-10, 5, 0, 15));
    }

    #[test]
    fn try_checked_translate_assign_small_rect_to_bounds() {
        let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MIN + 12, i16::MIN + 15);
        assert_eq!(try_checked_translate_assign(&mut min_r, &PointI16::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 10));

        let mut max_r = RectI16::of(i16::MAX - 12, i16::MAX - 15, i16::MAX - 2, i16::MAX - 5);
        assert_eq!(try_checked_translate_assign(&mut max_r, &PointI16::of(2, 5)), Some(()));
        assert_eq!(max_r, RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX, i16::MAX));
    }

    #[test]
    fn try_checked_translate_assign_big_rect_to_bounds() {
        let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX);
        assert_eq!(try_checked_translate_assign(&mut min_r, &PointI16::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5));

        let mut max_r = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5);
        assert_eq!(try_checked_translate_assign(&mut max_r, &PointI16::of(2, 5)), Some(()));
        assert_eq!(max_r, RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX));
    }

    #[test]
    fn try_checked_translate_assign_small_rect_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI16::of(-20, -20)), None);
        assert_eq!(r_min, RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30));

        let mut r_max = RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI16::of(20, 20)), None);
        assert_eq!(r_max, RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10));
    }

    #[test]
    fn try_checked_translate_assign_big_rect_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI16::of(-20, -20)), None);
        assert_eq!(r_min, RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX));

        let mut r_max = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI16::of(20, 20)), None);
        assert_eq!(r_max, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10));
    }

    #[test]
    fn try_checked_translate_assign_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI16::min()), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI16::of(i16::MIN, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI16::of(0, i16::MIN)), None);
        assert_eq!(r_min, RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10));

        let mut r_max = RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI16::max()), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI16::of(i16::MAX, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI16::of(0, i16::MAX)), None);
        assert_eq!(r_max, RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1));
    }

    #[test]
    fn try_checked_translate_assign_big_rect_limits_out_of_bounds() {
        let mut r = RectI16::largest();
        assert_eq!(try_checked_translate_assign(&mut r, &PointI16::min()), None);
        assert_eq!(try_checked_translate_assign(&mut r, &PointI16::max()), None);
        assert_eq!(r, RectI16::largest());

        let mut r_min = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI16::max()), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI16::of(i16::MAX, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_min, &PointI16::of(0, i16::MAX)), None);
        assert_eq!(r_min, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1));

        let mut r_max = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI16::min()), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI16::of(i16::MIN, 0)), None);
        assert_eq!(try_checked_translate_assign(&mut r_max, &PointI16::of(0, i16::MIN)), None);
        assert_eq!(r_max, RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX));
    }

    #[test]
    fn test_try_checked_translate() {
        assert_eq!(try_checked_translate(&RectI16::of(0, 0, 10, 10), &PointI16::of(10, 20)), Some(RectI16::of(10, 20, 20, 30)));
        assert_eq!(try_checked_translate(&RectI16::of(10, 20, 20, 30), &PointI16::of(-20, -15)), Some(RectI16::of(-10, 5, 0, 15)));
    }

    #[test]
    fn try_checked_translate_small_rect_to_bounds() {
        assert_eq!(
            try_checked_translate(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MIN + 12, i16::MIN + 15), &PointI16::of(-2, -5)),
            Some(RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 10))
        );
        assert_eq!(
            try_checked_translate(&RectI16::of(i16::MAX - 12, i16::MAX - 15, i16::MAX - 2, i16::MAX - 5), &PointI16::of(2, 5)),
            Some(RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX, i16::MAX))
        );
    }

    #[test]
    fn try_checked_translate_big_rect_to_bounds() {
        assert_eq!(
            try_checked_translate(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX), &PointI16::of(-2, -5)),
            Some(RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5))
        );
        assert_eq!(
            try_checked_translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5), &PointI16::of(2, 5)),
            Some(RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX))
        );
    }

    #[test]
    fn try_checked_translate_small_rect_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30), &PointI16::of(-20, -20)), None);
        assert_eq!(try_checked_translate(&RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10), &PointI16::of(20, 20)), None);
    }

    #[test]
    fn try_checked_translate_big_rect_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX), &PointI16::of(-20, -20)), None);
        assert_eq!(try_checked_translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10), &PointI16::of(20, 20)), None);
    }

    #[test]
    fn try_checked_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10), &PointI16::min()), None);
        assert_eq!(try_checked_translate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10), &PointI16::of(i16::MIN, 0)), None);
        assert_eq!(try_checked_translate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10), &PointI16::of(0, i16::MIN)), None);
        assert_eq!(try_checked_translate(&RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1), &PointI16::max()), None);
        assert_eq!(try_checked_translate(&RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1), &PointI16::of(i16::MAX, 0)), None);
        assert_eq!(try_checked_translate(&RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1), &PointI16::of(0, i16::MAX)), None);
    }

    #[test]
    fn try_checked_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(try_checked_translate(&RectI16::largest(), &PointI16::min()), None);
        assert_eq!(try_checked_translate(&RectI16::largest(), &PointI16::max()), None);
        assert_eq!(try_checked_translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1), &PointI16::max()), None);
        assert_eq!(try_checked_translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1), &PointI16::of(i16::MAX, 0)), None);
        assert_eq!(try_checked_translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1), &PointI16::of(0, i16::MAX)), None);
        assert_eq!(try_checked_translate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX), &PointI16::min()), None);
        assert_eq!(try_checked_translate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX), &PointI16::of(i16::MIN, 0)), None);
        assert_eq!(try_checked_translate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX), &PointI16::of(0, i16::MIN)), None);
    }

    #[test]
    fn test_checked_translate_assign() {
        let mut r = RectI16::of(0, 0, 10, 10);
        checked_translate_assign(&mut r, &PointI16::of(10, 20));
        assert_eq!(r, RectI16::of(10, 20, 20, 30));
        checked_translate_assign(&mut r, &PointI16::of(-20, -15));
        assert_eq!(r, RectI16::of(-10, 5, 0, 15));
    }

    #[test]
    fn test_checked_translate() {
        assert_eq!(checked_translate(&RectI16::of(0, 0, 10, 10), &PointI16::of(10, 20)), RectI16::of(10, 20, 20, 30));
        assert_eq!(checked_translate(&RectI16::of(10, 20, 20, 30), &PointI16::of(-20, -15)), RectI16::of(-10, 5, 0, 15));
    }
}
