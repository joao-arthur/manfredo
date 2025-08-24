use crate::cartesian::{point::point_i64::PointI64, rect::rect_i64::RectI64};

pub fn try_assign_translate(r: &mut RectI64, delta: &PointI64) -> Option<()> {
    let min_x = r.min.x.checked_add(delta.x)?;
    let min_y = r.min.y.checked_add(delta.y)?;
    let max_x = r.max.x.checked_add(delta.x)?;
    let max_y = r.max.y.checked_add(delta.y)?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_translate(r: &RectI64, delta: &PointI64) -> Option<RectI64> {
    let min_x = r.min.x.checked_add(delta.x)?;
    let min_y = r.min.y.checked_add(delta.y)?;
    let max_x = r.max.x.checked_add(delta.x)?;
    let max_y = r.max.y.checked_add(delta.y)?;
    Some(RectI64 { min: PointI64 { x: min_x, y: min_y }, max: PointI64 { x: max_x, y: max_y } })
}

pub fn assign_translate(r: &mut RectI64, delta: &PointI64) {
    try_assign_translate(r, delta).unwrap()
}

pub fn translate(r: &RectI64, delta: &PointI64) -> RectI64 {
    try_translate(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{assign_translate, translate, try_assign_translate, try_translate};
    use crate::cartesian::{point::point_i64::PointI64, rect::rect_i64::RectI64};

    #[test]
    fn test_try_assign_translate() {
        let mut r = RectI64::of(0, 0, 10, 10);
        assert_eq!(try_assign_translate(&mut r, &PointI64::of(10, 20)), Some(()));
        assert_eq!(r, RectI64::of(10, 20, 20, 30));
        assert_eq!(try_assign_translate(&mut r, &PointI64::of(-20, -15)), Some(()));
        assert_eq!(r, RectI64::of(-10, 5, 0, 15));
    }

    #[test]
    fn try_assign_translate_small_rect_to_bounds() {
        let mut min_r = RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MIN + 12, i64::MIN + 15);
        assert_eq!(try_assign_translate(&mut min_r, &PointI64::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10));

        let mut max_r = RectI64::of(i64::MAX - 12, i64::MAX - 15, i64::MAX - 2, i64::MAX - 5);
        assert_eq!(try_assign_translate(&mut max_r, &PointI64::of(2, 5)), Some(()));
        assert_eq!(max_r, RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX, i64::MAX));
    }

    #[test]
    fn try_assign_translate_big_rect_to_bounds() {
        let mut min_r = RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX);
        assert_eq!(try_assign_translate(&mut min_r, &PointI64::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5));

        let mut max_r = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5);
        assert_eq!(try_assign_translate(&mut max_r, &PointI64::of(2, 5)), Some(()));
        assert_eq!(max_r, RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX));
    }

    #[test]
    fn try_assign_translate_small_rect_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MIN + 20, i64::MIN + 30);
        assert_eq!(try_assign_translate(&mut r_min, &PointI64::of(-20, -20)), None);
        assert_eq!(r_min, RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MIN + 20, i64::MIN + 30));

        let mut r_max = RectI64::of(i64::MAX - 20, i64::MAX - 30, i64::MAX - 5, i64::MAX - 10);
        assert_eq!(try_assign_translate(&mut r_max, &PointI64::of(20, 20)), None);
        assert_eq!(r_max, RectI64::of(i64::MAX - 20, i64::MAX - 30, i64::MAX - 5, i64::MAX - 10));
    }

    #[test]
    fn try_assign_translate_big_rect_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MAX, i64::MAX);
        assert_eq!(try_assign_translate(&mut r_min, &PointI64::of(-20, -20)), None);
        assert_eq!(r_min, RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MAX, i64::MAX));

        let mut r_max = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 5, i64::MAX - 10);
        assert_eq!(try_assign_translate(&mut r_max, &PointI64::of(20, 20)), None);
        assert_eq!(r_max, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 5, i64::MAX - 10));
    }

    #[test]
    fn try_assign_translate_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MIN + 10, i64::MIN + 10);
        assert_eq!(try_assign_translate(&mut r_min, &PointI64::min()), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointI64::of(i64::MIN, 0)), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointI64::of(0, i64::MIN)), None);
        assert_eq!(r_min, RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MIN + 10, i64::MIN + 10));

        let mut r_max = RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX - 1, i64::MAX - 1);
        assert_eq!(try_assign_translate(&mut r_max, &PointI64::max()), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointI64::of(i64::MAX, 0)), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointI64::of(0, i64::MAX)), None);
        assert_eq!(r_max, RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX - 1, i64::MAX - 1));
    }

    #[test]
    fn try_assign_translate_big_rect_limits_out_of_bounds() {
        let mut r = RectI64::largest();
        assert_eq!(try_assign_translate(&mut r, &PointI64::min()), None);
        assert_eq!(try_assign_translate(&mut r, &PointI64::max()), None);
        assert_eq!(r, RectI64::largest());

        let mut r_min = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1);
        assert_eq!(try_assign_translate(&mut r_min, &PointI64::max()), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointI64::of(i64::MAX, 0)), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointI64::of(0, i64::MAX)), None);
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1));

        let mut r_max = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX);
        assert_eq!(try_assign_translate(&mut r_max, &PointI64::min()), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointI64::of(i64::MIN, 0)), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointI64::of(0, i64::MIN)), None);
        assert_eq!(r_max, RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX));
    }

    #[test]
    fn test_try_translate() {
        assert_eq!(try_translate(&RectI64::of(0, 0, 10, 10), &PointI64::of(10, 20)), Some(RectI64::of(10, 20, 20, 30)));
        assert_eq!(try_translate(&RectI64::of(10, 20, 20, 30), &PointI64::of(-20, -15)), Some(RectI64::of(-10, 5, 0, 15)));
    }

    #[test]
    fn try_translate_small_rect_to_bounds() {
        assert_eq!(
            try_translate(&RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MIN + 12, i64::MIN + 15), &PointI64::of(-2, -5)),
            Some(RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10))
        );
        assert_eq!(
            try_translate(&RectI64::of(i64::MAX - 12, i64::MAX - 15, i64::MAX - 2, i64::MAX - 5), &PointI64::of(2, 5)),
            Some(RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX, i64::MAX))
        );
    }

    #[test]
    fn try_translate_big_rect_to_bounds() {
        assert_eq!(
            try_translate(&RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX), &PointI64::of(-2, -5)),
            Some(RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5))
        );
        assert_eq!(
            try_translate(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5), &PointI64::of(2, 5)),
            Some(RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX))
        );
    }

    #[test]
    fn try_translate_small_rect_out_of_bounds() {
        assert_eq!(try_translate(&RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MIN + 20, i64::MIN + 30), &PointI64::of(-20, -20)), None);
        assert_eq!(try_translate(&RectI64::of(i64::MAX - 20, i64::MAX - 30, i64::MAX - 5, i64::MAX - 10), &PointI64::of(20, 20)), None);
    }

    #[test]
    fn try_translate_big_rect_out_of_bounds() {
        assert_eq!(try_translate(&RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MAX, i64::MAX), &PointI64::of(-20, -20)), None);
        assert_eq!(try_translate(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 5, i64::MAX - 10), &PointI64::of(20, 20)), None);
    }

    #[test]
    fn try_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(try_translate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MIN + 10, i64::MIN + 10), &PointI64::min()), None);
        assert_eq!(try_translate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MIN + 10, i64::MIN + 10), &PointI64::of(i64::MIN, 0)), None);
        assert_eq!(try_translate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MIN + 10, i64::MIN + 10), &PointI64::of(0, i64::MIN)), None);
        assert_eq!(try_translate(&RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX - 1, i64::MAX - 1), &PointI64::max()), None);
        assert_eq!(try_translate(&RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX - 1, i64::MAX - 1), &PointI64::of(i64::MAX, 0)), None);
        assert_eq!(try_translate(&RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX - 1, i64::MAX - 1), &PointI64::of(0, i64::MAX)), None);
    }

    #[test]
    fn try_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(try_translate(&RectI64::largest(), &PointI64::min()), None);
        assert_eq!(try_translate(&RectI64::largest(), &PointI64::max()), None);
        assert_eq!(try_translate(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1), &PointI64::max()), None);
        assert_eq!(try_translate(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1), &PointI64::of(i64::MAX, 0)), None);
        assert_eq!(try_translate(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1), &PointI64::of(0, i64::MAX)), None);
        assert_eq!(try_translate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX), &PointI64::min()), None);
        assert_eq!(try_translate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX), &PointI64::of(i64::MIN, 0)), None);
        assert_eq!(try_translate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX), &PointI64::of(0, i64::MIN)), None);
    }

    #[test]
    fn test_assign_translate() {
        let mut r = RectI64::of(0, 0, 10, 10);
        assign_translate(&mut r, &PointI64::of(10, 20));
        assert_eq!(r, RectI64::of(10, 20, 20, 30));
        assign_translate(&mut r, &PointI64::of(-20, -15));
        assert_eq!(r, RectI64::of(-10, 5, 0, 15));
    }

    #[test]
    fn test_translate() {
        assert_eq!(translate(&RectI64::of(0, 0, 10, 10), &PointI64::of(10, 20)), RectI64::of(10, 20, 20, 30));
        assert_eq!(translate(&RectI64::of(10, 20, 20, 30), &PointI64::of(-20, -15)), RectI64::of(-10, 5, 0, 15));
    }
}
