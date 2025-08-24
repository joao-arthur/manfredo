use crate::cartesian::{point::point_i16::PointI16, rect::rect_i16::RectI16};

pub fn try_assign_translate(r: &mut RectI16, delta: &PointI16) -> Option<()> {
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

pub fn try_translate(r: &RectI16, delta: &PointI16) -> Option<RectI16> {
    let min_x = r.min.x.checked_add(delta.x)?;
    let min_y = r.min.y.checked_add(delta.y)?;
    let max_x = r.max.x.checked_add(delta.x)?;
    let max_y = r.max.y.checked_add(delta.y)?;
    Some(RectI16 { min: PointI16 { x: min_x, y: min_y }, max: PointI16 { x: max_x, y: max_y } })
}

pub fn assign_translate(r: &mut RectI16, delta: &PointI16) {
    try_assign_translate(r, delta).unwrap()
}

pub fn translate(r: &RectI16, delta: &PointI16) -> RectI16 {
    try_translate(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{assign_translate, translate, try_assign_translate, try_translate};
    use crate::cartesian::{point::point_i16::PointI16, rect::rect_i16::RectI16};

    #[test]
    fn test_try_assign_translate() {
        let mut r = RectI16::of(0, 0, 10, 10);
        assert_eq!(try_assign_translate(&mut r, &PointI16::of(10, 20)), Some(()));
        assert_eq!(r, RectI16::of(10, 20, 20, 30));
        assert_eq!(try_assign_translate(&mut r, &PointI16::of(-20, -15)), Some(()));
        assert_eq!(r, RectI16::of(-10, 5, 0, 15));
    }

    #[test]
    fn try_assign_translate_small_rect_to_bounds() {
        let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MIN + 12, i16::MIN + 15);
        assert_eq!(try_assign_translate(&mut min_r, &PointI16::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 10));

        let mut max_r = RectI16::of(i16::MAX - 12, i16::MAX - 15, i16::MAX - 2, i16::MAX - 5);
        assert_eq!(try_assign_translate(&mut max_r, &PointI16::of(2, 5)), Some(()));
        assert_eq!(max_r, RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX, i16::MAX));
    }

    #[test]
    fn try_assign_translate_big_rect_to_bounds() {
        let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX);
        assert_eq!(try_assign_translate(&mut min_r, &PointI16::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5));

        let mut max_r = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5);
        assert_eq!(try_assign_translate(&mut max_r, &PointI16::of(2, 5)), Some(()));
        assert_eq!(max_r, RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX));
    }

    #[test]
    fn try_assign_translate_small_rect_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30);
        assert_eq!(try_assign_translate(&mut r_min, &PointI16::of(-20, -20)), None);
        assert_eq!(r_min, RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30));

        let mut r_max = RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10);
        assert_eq!(try_assign_translate(&mut r_max, &PointI16::of(20, 20)), None);
        assert_eq!(r_max, RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10));
    }

    #[test]
    fn try_assign_translate_big_rect_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX);
        assert_eq!(try_assign_translate(&mut r_min, &PointI16::of(-20, -20)), None);
        assert_eq!(r_min, RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX));

        let mut r_max = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10);
        assert_eq!(try_assign_translate(&mut r_max, &PointI16::of(20, 20)), None);
        assert_eq!(r_max, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10));
    }

    #[test]
    fn try_assign_translate_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10);
        assert_eq!(try_assign_translate(&mut r_min, &PointI16::min()), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointI16::of(i16::MIN, 0)), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointI16::of(0, i16::MIN)), None);
        assert_eq!(r_min, RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10));

        let mut r_max = RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1);
        assert_eq!(try_assign_translate(&mut r_max, &PointI16::max()), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointI16::of(i16::MAX, 0)), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointI16::of(0, i16::MAX)), None);
        assert_eq!(r_max, RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1));
    }

    #[test]
    fn try_assign_translate_big_rect_limits_out_of_bounds() {
        let mut r = RectI16::largest();
        assert_eq!(try_assign_translate(&mut r, &PointI16::min()), None);
        assert_eq!(try_assign_translate(&mut r, &PointI16::max()), None);
        assert_eq!(r, RectI16::largest());

        let mut r_min = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1);
        assert_eq!(try_assign_translate(&mut r_min, &PointI16::max()), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointI16::of(i16::MAX, 0)), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointI16::of(0, i16::MAX)), None);
        assert_eq!(r_min, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1));

        let mut r_max = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX);
        assert_eq!(try_assign_translate(&mut r_max, &PointI16::min()), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointI16::of(i16::MIN, 0)), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointI16::of(0, i16::MIN)), None);
        assert_eq!(r_max, RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX));
    }

    #[test]
    fn test_try_translate() {
        assert_eq!(try_translate(&RectI16::of(0, 0, 10, 10), &PointI16::of(10, 20)), Some(RectI16::of(10, 20, 20, 30)));
        assert_eq!(try_translate(&RectI16::of(10, 20, 20, 30), &PointI16::of(-20, -15)), Some(RectI16::of(-10, 5, 0, 15)));
    }

    #[test]
    fn try_translate_small_rect_to_bounds() {
        assert_eq!(
            try_translate(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MIN + 12, i16::MIN + 15), &PointI16::of(-2, -5)),
            Some(RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 10))
        );
        assert_eq!(
            try_translate(&RectI16::of(i16::MAX - 12, i16::MAX - 15, i16::MAX - 2, i16::MAX - 5), &PointI16::of(2, 5)),
            Some(RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX, i16::MAX))
        );
    }

    #[test]
    fn try_translate_big_rect_to_bounds() {
        assert_eq!(
            try_translate(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX), &PointI16::of(-2, -5)),
            Some(RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5))
        );
        assert_eq!(
            try_translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5), &PointI16::of(2, 5)),
            Some(RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX))
        );
    }

    #[test]
    fn try_translate_small_rect_out_of_bounds() {
        assert_eq!(try_translate(&RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30), &PointI16::of(-20, -20)), None);
        assert_eq!(try_translate(&RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10), &PointI16::of(20, 20)), None);
    }

    #[test]
    fn try_translate_big_rect_out_of_bounds() {
        assert_eq!(try_translate(&RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX), &PointI16::of(-20, -20)), None);
        assert_eq!(try_translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10), &PointI16::of(20, 20)), None);
    }

    #[test]
    fn try_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(try_translate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10), &PointI16::min()), None);
        assert_eq!(try_translate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10), &PointI16::of(i16::MIN, 0)), None);
        assert_eq!(try_translate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10), &PointI16::of(0, i16::MIN)), None);
        assert_eq!(try_translate(&RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1), &PointI16::max()), None);
        assert_eq!(try_translate(&RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1), &PointI16::of(i16::MAX, 0)), None);
        assert_eq!(try_translate(&RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1), &PointI16::of(0, i16::MAX)), None);
    }

    #[test]
    fn try_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(try_translate(&RectI16::largest(), &PointI16::min()), None);
        assert_eq!(try_translate(&RectI16::largest(), &PointI16::max()), None);
        assert_eq!(try_translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1), &PointI16::max()), None);
        assert_eq!(try_translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1), &PointI16::of(i16::MAX, 0)), None);
        assert_eq!(try_translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1), &PointI16::of(0, i16::MAX)), None);
        assert_eq!(try_translate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX), &PointI16::min()), None);
        assert_eq!(try_translate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX), &PointI16::of(i16::MIN, 0)), None);
        assert_eq!(try_translate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX), &PointI16::of(0, i16::MIN)), None);
    }

    #[test]
    fn test_assign_translate() {
        let mut r = RectI16::of(0, 0, 10, 10);
        assign_translate(&mut r, &PointI16::of(10, 20));
        assert_eq!(r, RectI16::of(10, 20, 20, 30));
        assign_translate(&mut r, &PointI16::of(-20, -15));
        assert_eq!(r, RectI16::of(-10, 5, 0, 15));
    }

    #[test]
    fn test_translate() {
        assert_eq!(translate(&RectI16::of(0, 0, 10, 10), &PointI16::of(10, 20)), RectI16::of(10, 20, 20, 30));
        assert_eq!(translate(&RectI16::of(10, 20, 20, 30), &PointI16::of(-20, -15)), RectI16::of(-10, 5, 0, 15));
    }
}
