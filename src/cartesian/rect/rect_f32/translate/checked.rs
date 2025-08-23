use crate::cartesian::{
    point::point_f32::{MAX, MIN, PointF32},
    rect::rect_f32::RectF32,
};

pub fn try_assign_translate(r: &mut RectF32, delta: &PointF32) -> Option<()> {
    if delta.x < MIN - r.min.x || delta.y < MIN - r.min.y || delta.x > MAX - r.max.x || delta.y > MAX - r.max.y {
        return None;
    }
    r.min.x = r.min.x + delta.x;
    r.min.y = r.min.y + delta.y;
    r.max.x = r.max.x + delta.x;
    r.max.y = r.max.y + delta.y;
    Some(())
}

pub fn try_translate(r: &RectF32, delta: &PointF32) -> Option<RectF32> {
    if delta.x < MIN - r.min.x || delta.y < MIN - r.min.y || delta.x > MAX - r.max.x || delta.y > MAX - r.max.y {
        return None;
    }
    let min_x = r.min.x + delta.x;
    let min_y = r.min.y + delta.y;
    let max_x = r.max.x + delta.x;
    let max_y = r.max.y + delta.y;
    Some(RectF32 { min: PointF32 { x: min_x, y: min_y }, max: PointF32 { x: max_x, y: max_y } })
}

pub fn assign_translate(r: &mut RectF32, delta: &PointF32) {
    try_assign_translate(r, delta).unwrap()
}

pub fn translate(r: &RectF32, delta: &PointF32) -> RectF32 {
    try_translate(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{
        point::point_f32::{MAX, MIN, PointF32},
        rect::rect_f32::RectF32,
    };

    use super::{assign_translate, translate, try_assign_translate, try_translate};

    #[test]
    fn test_try_assign_translate() {
        let mut r = RectF32::of(0.0, 0.0, 10.0, 10.0);
        assert_eq!(try_assign_translate(&mut r, &PointF32::of(10.0, 20.0)), Some(()));
        assert_eq!(r, RectF32::of(10.0, 20.0, 20.0, 30.0));
        assert_eq!(try_assign_translate(&mut r, &PointF32::of(-20.0, -15.0)), Some(()));
        assert_eq!(r, RectF32::of(-10.0, 5.0, 0.0, 15.0));
    }

    #[test]
    fn try_assign_translate_small_rect_to_bounds() {
        let mut min_r = RectF32::of(MIN + 2.0, MIN + 5.0, MIN + 12.0, MIN + 15.0);
        assert_eq!(try_assign_translate(&mut min_r, &PointF32::of(-2.0, -5.0)), Some(()));
        assert_eq!(min_r, RectF32::of(MIN, MIN, MIN + 10.0, MIN + 10.0));

        let mut max_r = RectF32::of(MAX - 12.0, MAX - 15.0, MAX - 2.0, MAX - 5.0);
        assert_eq!(try_assign_translate(&mut max_r, &PointF32::of(2.0, 5.0)), Some(()));
        assert_eq!(max_r, RectF32::of(MAX - 10.0, MAX - 10.0, MAX, MAX));
    }

    #[test]
    fn try_assign_translate_big_rect_to_bounds() {
        let mut min_r = RectF32::of(MIN + 2.0, MIN + 5.0, MAX, MAX);
        assert_eq!(try_assign_translate(&mut min_r, &PointF32::of(-2.0, -5.0)), Some(()));
        assert_eq!(min_r, RectF32::of(MIN, MIN, MAX - 2.0, MAX - 5.0));

        let mut max_r = RectF32::of(MIN, MIN, MAX - 2.0, MAX - 5.0);
        assert_eq!(try_assign_translate(&mut max_r, &PointF32::of(2.0, 5.0)), Some(()));
        assert_eq!(max_r, RectF32::of(MIN + 2.0, MIN + 5.0, MAX, MAX));
    }

    #[test]
    fn try_assign_translate_small_rect_out_of_bounds() {
        let mut r_min = RectF32::of(MIN + 10.0, MIN + 5.0, MIN + 20.0, MIN + 30.0);
        assert_eq!(try_assign_translate(&mut r_min, &PointF32::of(-20.0, -20.0)), None);
        assert_eq!(r_min, RectF32::of(MIN + 10.0, MIN + 5.0, MIN + 20.0, MIN + 30.0));

        let mut r_max = RectF32::of(MAX - 20.0, MAX - 30.0, MAX - 5.0, MAX - 10.0);
        assert_eq!(try_assign_translate(&mut r_max, &PointF32::of(20.0, 20.0)), None);
        assert_eq!(r_max, RectF32::of(MAX - 20.0, MAX - 30.0, MAX - 5.0, MAX - 10.0));
    }

    #[test]
    fn try_assign_translate_big_rect_out_of_bounds() {
        let mut r_min = RectF32::of(MIN + 10.0, MIN + 5.0, MAX, MAX);
        assert_eq!(try_assign_translate(&mut r_min, &PointF32::of(-20.0, -20.0)), None);
        assert_eq!(r_min, RectF32::of(MIN + 10.0, MIN + 5.0, MAX, MAX));

        let mut r_max = RectF32::of(MIN, MIN, MAX - 5.0, MAX - 10.0);
        assert_eq!(try_assign_translate(&mut r_max, &PointF32::of(20.0, 20.0)), None);
        assert_eq!(r_max, RectF32::of(MIN, MIN, MAX - 5.0, MAX - 10.0));
    }

    #[test]
    fn try_assign_translate_small_rect_limits_out_of_bounds() {
        let mut r_min = RectF32::of(MIN + 1.0, MIN + 1.0, MIN + 10.0, MIN + 10.0);
        assert_eq!(try_assign_translate(&mut r_min, &PointF32::min()), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointF32::of(MIN, 0.0)), None);
        assert_eq!(try_assign_translate(&mut r_min, &PointF32::of(0.0, MIN)), None);
        assert_eq!(r_min, RectF32::of(MIN + 1.0, MIN + 1.0, MIN + 10.0, MIN + 10.0));

        let mut r_max = RectF32::of(MAX - 10.0, MAX - 10.0, MAX - 1.0, MAX - 1.0);
        assert_eq!(try_assign_translate(&mut r_max, &PointF32::max()), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointF32::of(MAX, 0.0)), None);
        assert_eq!(try_assign_translate(&mut r_max, &PointF32::of(0.0, MAX)), None);
        assert_eq!(r_max, RectF32::of(MAX - 10.0, MAX - 10.0, MAX - 1.0, MAX - 1.0));
    }

    #[test]
    fn try_assign_translate_big_rect_limits_out_of_bounds() {
        let mut r = RectF32::largest();
        assert_eq!(try_assign_translate(&mut r, &PointF32::min()), None);
        assert_eq!(try_assign_translate(&mut r, &PointF32::max()), None);
        assert_eq!(r, RectF32::largest());

        let mut r_min = RectF32::largest_min();
        assert_eq!(try_assign_translate(&mut r_min, &PointF32::min()), None);
        assert_eq!(r_min, RectF32::largest_min());

        let mut r_max = RectF32::largest_max();
        assert_eq!(try_assign_translate(&mut r_max, &PointF32::max()), None);
        assert_eq!(r_max, RectF32::largest_max());

        let mut r_min_2 = RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0);
        assert_eq!(try_assign_translate(&mut r_min_2, &PointF32::max()), None);
        assert_eq!(try_assign_translate(&mut r_min_2, &PointF32::of(MAX, 0.0)), None);
        assert_eq!(try_assign_translate(&mut r_min_2, &PointF32::of(0.0, MAX)), None);
        assert_eq!(r_min_2, RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0));

        let mut r_max_2 = RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX);
        assert_eq!(try_assign_translate(&mut r_max_2, &PointF32::min()), None);
        assert_eq!(try_assign_translate(&mut r_max_2, &PointF32::of(MIN, 0.0)), None);
        assert_eq!(try_assign_translate(&mut r_max_2, &PointF32::of(0.0, MIN)), None);
        assert_eq!(r_max_2, RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX));
    }

    #[test]
    fn test_try_translate() {
        assert_eq!(try_translate(&RectF32::of(0.0, 0.0, 10.0, 10.0), &PointF32::of(10.0, 20.0)), Some(RectF32::of(10.0, 20.0, 20.0, 30.0)));
        assert_eq!(try_translate(&RectF32::of(10.0, 20.0, 20.0, 30.0), &PointF32::of(-20.0, -15.0)), Some(RectF32::of(-10.0, 5.0, 0.0, 15.0)));
    }

    #[test]
    fn try_translate_small_rect_to_bounds() {
        assert_eq!(
            try_translate(&RectF32::of(MIN + 2.0, MIN + 5.0, MIN + 12.0, MIN + 15.0), &PointF32::of(-2.0, -5.0)),
            Some(RectF32::of(MIN, MIN, MIN + 10.0, MIN + 10.0))
        );
        assert_eq!(
            try_translate(&RectF32::of(MAX - 12.0, MAX - 15.0, MAX - 2.0, MAX - 5.0), &PointF32::of(2.0, 5.0)),
            Some(RectF32::of(MAX - 10.0, MAX - 10.0, MAX, MAX))
        );
    }

    #[test]
    fn try_translate_big_rect_to_bounds() {
        assert_eq!(
            try_translate(&RectF32::of(MIN + 2.0, MIN + 5.0, MAX, MAX), &PointF32::of(-2.0, -5.0)),
            Some(RectF32::of(MIN, MIN, MAX - 2.0, MAX - 5.0))
        );
        assert_eq!(
            try_translate(&RectF32::of(MIN, MIN, MAX - 2.0, MAX - 5.0), &PointF32::of(2.0, 5.0)),
            Some(RectF32::of(MIN + 2.0, MIN + 5.0, MAX, MAX))
        );
    }

    #[test]
    fn try_translate_small_rect_out_of_bounds() {
        assert_eq!(try_translate(&RectF32::of(MIN + 10.0, MIN + 5.0, MIN + 20.0, MIN + 30.0), &PointF32::of(-20.0, -20.0)), None);
        assert_eq!(try_translate(&RectF32::of(MAX - 20.0, MAX - 30.0, MAX - 5.0, MAX - 10.0), &PointF32::of(20.0, 20.0)), None);
    }

    #[test]
    fn try_translate_big_rect_out_of_bounds() {
        assert_eq!(try_translate(&RectF32::of(MIN + 10.0, MIN + 5.0, MAX, MAX), &PointF32::of(-20.0, -20.0)), None);
        assert_eq!(try_translate(&RectF32::of(MIN, MIN, MAX - 5.0, MAX - 10.0), &PointF32::of(20.0, 20.0)), None);
    }

    #[test]
    fn try_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(try_translate(&RectF32::of(MIN + 1.0, MIN + 1.0, MIN + 10.0, MIN + 10.0), &PointF32::min()), None);
        assert_eq!(try_translate(&RectF32::of(MIN + 1.0, MIN + 1.0, MIN + 10.0, MIN + 10.0), &PointF32::of(MIN, 0.0)), None);
        assert_eq!(try_translate(&RectF32::of(MIN + 1.0, MIN + 1.0, MIN + 10.0, MIN + 10.0), &PointF32::of(0.0, MIN)), None);
        assert_eq!(try_translate(&RectF32::of(MAX - 10.0, MAX - 10.0, MAX - 1.0, MAX - 1.0), &PointF32::max()), None);
        assert_eq!(try_translate(&RectF32::of(MAX - 10.0, MAX - 10.0, MAX - 1.0, MAX - 1.0), &PointF32::of(MAX, 0.0)), None);
        assert_eq!(try_translate(&RectF32::of(MAX - 10.0, MAX - 10.0, MAX - 1.0, MAX - 1.0), &PointF32::of(0.0, MAX)), None);
    }

    #[test]
    fn try_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(try_translate(&RectF32::largest(), &PointF32::min()), None);
        assert_eq!(try_translate(&RectF32::largest(), &PointF32::max()), None);
        assert_eq!(try_translate(&RectF32::largest_min(), &PointF32::min()), None);
        assert_eq!(try_translate(&RectF32::largest_max(), &PointF32::max()), None);
        assert_eq!(try_translate(&RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0), &PointF32::max()), None);
        assert_eq!(try_translate(&RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0), &PointF32::of(MAX, 0.0)), None);
        assert_eq!(try_translate(&RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0), &PointF32::of(0.0, MAX)), None);
        assert_eq!(try_translate(&RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX), &PointF32::min()), None);
        assert_eq!(try_translate(&RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX), &PointF32::of(MIN, 0.0)), None);
        assert_eq!(try_translate(&RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX), &PointF32::of(0.0, MIN)), None);
    }

    #[test]
    fn test_assign_translate() {
        let mut r = RectF32::of(0.0, 0.0, 10.0, 10.0);
        assign_translate(&mut r, &PointF32::of(10.0, 20.0));
        assert_eq!(r, RectF32::of(10.0, 20.0, 20.0, 30.0));
        assign_translate(&mut r, &PointF32::of(-20.0, -15.0));
        assert_eq!(r, RectF32::of(-10.0, 5.0, 0.0, 15.0));
    }

    #[test]
    fn test_translate() {
        assert_eq!(translate(&RectF32::of(0.0, 0.0, 10.0, 10.0), &PointF32::of(10.0, 20.0)), RectF32::of(10.0, 20.0, 20.0, 30.0));
        assert_eq!(translate(&RectF32::of(10.0, 20.0, 20.0, 30.0), &PointF32::of(-20.0, -15.0)), RectF32::of(-10.0, 5.0, 0.0, 15.0));
    }
}
