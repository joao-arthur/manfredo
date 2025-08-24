use crate::cartesian::{point::point_i32::PointI32, rect::rect_i32::RectI32};

pub fn try_assign_translate(r: &mut RectI32, delta: &PointI32) -> Option<()> {
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

pub fn try_translate(r: &RectI32, delta: &PointI32) -> Option<RectI32> {
    let min_x = r.min.x.checked_add(delta.x)?;
    let min_y = r.min.y.checked_add(delta.y)?;
    let max_x = r.max.x.checked_add(delta.x)?;
    let max_y = r.max.y.checked_add(delta.y)?;
    Some(RectI32 { min: PointI32 { x: min_x, y: min_y }, max: PointI32 { x: max_x, y: max_y } })
}

pub fn assign_translate(r: &mut RectI32, delta: &PointI32) {
    try_assign_translate(r, delta).unwrap()
}

pub fn translate(r: &RectI32, delta: &PointI32) -> RectI32 {
    try_translate(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{assign_translate, translate, try_assign_translate, try_translate};
    use crate::cartesian::{point::point_i32::PointI32, rect::rect_i32::RectI32};

    #[test]
    fn test_try_assign_translate() {
        let mut r = RectI32::of(0, 0, 10, 10);
        assert_eq!(try_assign_translate(&mut r, &PointI32::of(10, 20)), Some(()));
        assert_eq!(r, RectI32::of(10, 20, 20, 30));
        assert_eq!(try_assign_translate(&mut r, &PointI32::of(-20, -15)), Some(()));
        assert_eq!(r, RectI32::of(-10, 5, 0, 15));
    }

    #[test]
    fn try_assign_translate_small_rect_to_bounds() {
        let mut min_r = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MIN + 12, i32::MIN + 15);
        assert_eq!(try_assign_translate(&mut min_r, &PointI32::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 10, i32::MIN + 10));

        let mut max_r = RectI32::of(i32::MAX - 12, i32::MAX - 15, i32::MAX - 2, i32::MAX - 5);
        assert_eq!(try_assign_translate(&mut max_r, &PointI32::of(2, 5)), Some(()));
        assert_eq!(max_r, RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX, i32::MAX));
    }

    #[test]
    fn try_assign_translate_big_rect_to_bounds() {
        let mut min_r = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX);
        assert_eq!(try_assign_translate(&mut min_r, &PointI32::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5));

        let mut max_r = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5);
        assert_eq!(try_assign_translate(&mut max_r, &PointI32::of(2, 5)), Some(()));
        assert_eq!(max_r, RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX));
    }

    #[test]
    fn try_assign_translate_small_rect_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MIN + 20, i32::MIN + 30);
        assert_eq!(try_assign_translate(&mut r_min, &PointI32::of(-20, -20)), None);
        assert_eq!(r_min, RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MIN + 20, i32::MIN + 30));

        let mut r_max = RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX - 5, i32::MAX - 10);
        assert_eq!(try_assign_translate(&mut r_max, &PointI32::of(20, 20)), None);
        assert_eq!(r_max, RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX - 5, i32::MAX - 10));
    }

    #[test]
    fn try_assign_translate_big_rect_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MAX, i32::MAX);
        assert_eq!(try_assign_translate(&mut r_min, &PointI32::of(-20, -20)), None);
        assert_eq!(r_min, RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MAX, i32::MAX));

        let mut r_max = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 5, i32::MAX - 10);
        assert_eq!(try_assign_translate(&mut r_max, &PointI32::of(20, 20)), None);
        assert_eq!(r_max, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 5, i32::MAX - 10));
    }

    #[test]
    fn try_assign_translate_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10);
        assert_eq!(try_assign_translate(&mut r_min, &PointI32::min()), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointI32::of(0, i32::MIN)), None);
        assert_eq!(r_min, RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10));

        let mut r_max = RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1);
        assert_eq!(try_assign_translate(&mut r_max, &PointI32::max()), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointI32::of(0, i32::MAX)), None);
        assert_eq!(r_max, RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1));
    }

    #[test]
    fn try_assign_translate_big_rect_limits_out_of_bounds() {
        let mut r = RectI32::largest();
        assert_eq!(try_assign_translate(&mut r, &PointI32::min()), None);
        assert_eq!(try_assign_translate(&mut r, &PointI32::max()), None);
        assert_eq!(r, RectI32::largest());

        let mut r_min = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1);
        assert_eq!(try_assign_translate(&mut r_min, &PointI32::max()), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointI32::of(0, i32::MAX)), None);
        assert_eq!(r_min, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1));

        let mut r_max = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX);
        assert_eq!(try_assign_translate(&mut r_max, &PointI32::min()), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointI32::of(0, i32::MIN)), None);
        assert_eq!(r_max, RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX));
    }

    #[test]
    fn test_try_translate() {
        assert_eq!(try_translate(&RectI32::of(0, 0, 10, 10), &PointI32::of(10, 20)), Some(RectI32::of(10, 20, 20, 30)));
        assert_eq!(try_translate(&RectI32::of(10, 20, 20, 30), &PointI32::of(-20, -15)), Some(RectI32::of(-10, 5, 0, 15)));
    }

    #[test]
    fn try_translate_small_rect_to_bounds() {
        assert_eq!(
            try_translate(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MIN + 12, i32::MIN + 15), &PointI32::of(-2, -5)),
            Some(RectI32::of(i32::MIN, i32::MIN, i32::MIN + 10, i32::MIN + 10))
        );
        assert_eq!(
            try_translate(&RectI32::of(i32::MAX - 12, i32::MAX - 15, i32::MAX - 2, i32::MAX - 5), &PointI32::of(2, 5)),
            Some(RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX, i32::MAX))
        );
    }

    #[test]
    fn try_translate_big_rect_to_bounds() {
        assert_eq!(
            try_translate(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX), &PointI32::of(-2, -5)),
            Some(RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5))
        );
        assert_eq!(
            try_translate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5), &PointI32::of(2, 5)),
            Some(RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX))
        );
    }

    #[test]
    fn try_translate_small_rect_out_of_bounds() {
        assert_eq!(try_translate(&RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MIN + 20, i32::MIN + 30), &PointI32::of(-20, -20)), None);
        assert_eq!(try_translate(&RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX - 5, i32::MAX - 10), &PointI32::of(20, 20)), None);
    }

    #[test]
    fn try_translate_big_rect_out_of_bounds() {
        assert_eq!(try_translate(&RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MAX, i32::MAX), &PointI32::of(-20, -20)), None);
        assert_eq!(try_translate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 5, i32::MAX - 10), &PointI32::of(20, 20)), None);
    }

    #[test]
    fn try_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(try_translate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10), &PointI32::min()), None);
        assert_eq!(try_translate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10), &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(try_translate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10), &PointI32::of(0, i32::MIN)), None);
        assert_eq!(try_translate(&RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1), &PointI32::max()), None);
        assert_eq!(try_translate(&RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1), &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(try_translate(&RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1), &PointI32::of(0, i32::MAX)), None);
    }

    #[test]
    fn try_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(try_translate(&RectI32::largest(), &PointI32::min()), None);
        assert_eq!(try_translate(&RectI32::largest(), &PointI32::max()), None);
        assert_eq!(try_translate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1), &PointI32::max()), None);
        assert_eq!(try_translate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1), &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(try_translate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1), &PointI32::of(0, i32::MAX)), None);
        assert_eq!(try_translate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX), &PointI32::min()), None);
        assert_eq!(try_translate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX), &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(try_translate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX), &PointI32::of(0, i32::MIN)), None);
    }

    #[test]
    fn test_assign_translate() {
        let mut r = RectI32::of(0, 0, 10, 10);
        assign_translate(&mut r, &PointI32::of(10, 20));
        assert_eq!(r, RectI32::of(10, 20, 20, 30));
        assign_translate(&mut r, &PointI32::of(-20, -15));
        assert_eq!(r, RectI32::of(-10, 5, 0, 15));
    }

    #[test]
    fn test_translate() {
        assert_eq!(translate(&RectI32::of(0, 0, 10, 10), &PointI32::of(10, 20)), RectI32::of(10, 20, 20, 30));
        assert_eq!(translate(&RectI32::of(10, 20, 20, 30), &PointI32::of(-20, -15)), RectI32::of(-10, 5, 0, 15));
    }
}
