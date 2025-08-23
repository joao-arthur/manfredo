use crate::cartesian::{
    point::point_f32::{MAX, MIN, PointF32},
    rect::rect_f32::RectF32,
};

pub fn assign_translate(r: &mut RectF32, delta: &PointF32) {
    if r.min.x + delta.x > MAX {
        let diff_min_x = MAX - r.min.x;
        let delta_x_adjusted = delta.x - diff_min_x - 1.0;
        r.min.x = MIN + delta_x_adjusted;
    } else if r.min.x + delta.x < MIN {
        let diff_min_x = MIN - r.min.x;
        let delta_x_adjusted = delta.x - diff_min_x + 1.0;
        r.min.x = MAX + delta_x_adjusted;
    } else {
        r.min.x += delta.x;
    }
    if r.min.y + delta.y > MAX {
        let diff_min_y = MAX - r.min.y;
        let delta_y_adjusted = delta.y - diff_min_y - 1.0;
        r.min.y = MIN + delta_y_adjusted;
    } else if r.min.y + delta.y < MIN {
        let diff_min_y = MIN - r.min.y;
        let delta_y_adjusted = delta.y - diff_min_y + 1.0;
        r.min.y = MAX + delta_y_adjusted;
    } else {
        r.min.y += delta.y;
    }
    if r.max.x + delta.x > MAX {
        let diff_min_x = MAX - r.max.x;
        let delta_x_adjusted = delta.x - diff_min_x - 1.0;
        r.max.x = MIN + delta_x_adjusted;
    } else if r.max.x + delta.x < MIN {
        let diff_min_x = MIN - r.max.x;
        let delta_x_adjusted = delta.x - diff_min_x + 1.0;
        r.max.x = MAX + delta_x_adjusted;
    } else {
        r.max.x += delta.x;
    }
    if r.max.y + delta.y > MAX {
        let diff_min_y = MAX - r.max.y;
        let delta_y_adjusted = delta.y - diff_min_y - 1.0;
        r.max.y = MIN + delta_y_adjusted;
    } else if r.max.y + delta.y < MIN {
        let diff_min_y = MIN - r.max.y;
        let delta_y_adjusted = delta.y - diff_min_y + 1.0;
        r.max.y = MAX + delta_y_adjusted;
    } else {
        r.max.y += delta.y;
    }
}

pub fn translate(r: &RectF32, delta: &PointF32) -> RectF32 {
    let mut min_x = r.min.x;
    let mut min_y = r.min.y;
    let mut max_x = r.max.x;
    let mut max_y = r.max.y;

    if min_x + delta.x > MAX {
        let diff_min_x = MAX - min_x;
        let delta_x_adjusted = delta.x - diff_min_x - 1.0;
        min_x = MIN + delta_x_adjusted;
    } else if min_x + delta.x < MIN {
        let diff_min_x = MIN - min_x;
        let delta_x_adjusted = delta.x - diff_min_x + 1.0;
        min_x = MAX + delta_x_adjusted;
    } else {
        min_x += delta.x;
    }
    if min_y + delta.y > MAX {
        let diff_min_y = MAX - min_y;
        let delta_y_adjusted = delta.y - diff_min_y - 1.0;
        min_y = MIN + delta_y_adjusted;
    } else if min_y + delta.y < MIN {
        let diff_min_y = MIN - min_y;
        let delta_y_adjusted = delta.y - diff_min_y + 1.0;
        min_y = MAX + delta_y_adjusted;
    } else {
        min_y += delta.y;
    }
    if max_x + delta.x > MAX {
        let diff_min_x = MAX - max_x;
        let delta_x_adjusted = delta.x - diff_min_x - 1.0;
        max_x = MIN + delta_x_adjusted;
    } else if max_x + delta.x < MIN {
        let diff_min_x = MIN - max_x;
        let delta_x_adjusted = delta.x - diff_min_x + 1.0;
        max_x = MAX + delta_x_adjusted;
    } else {
        max_x += delta.x;
    }
    if max_y + delta.y > MAX {
        let diff_min_y = MAX - max_y;
        let delta_y_adjusted = delta.y - diff_min_y - 1.0;
        max_y = MIN + delta_y_adjusted;
    } else if max_y + delta.y < MIN {
        let diff_min_y = MIN - max_y;
        let delta_y_adjusted = delta.y - diff_min_y + 1.0;
        max_y = MAX + delta_y_adjusted;
    } else {
        max_y += delta.y;
    }

    RectF32 { min: PointF32 { x: min_x, y: min_y }, max: PointF32 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{
        point::point_f32::{MAX, MIN, PointF32},
        rect::rect_f32::RectF32,
    };

    use super::{assign_translate, translate};

    #[test]
    fn test_assign_translate() {
        let mut r = RectF32::of(0.0, 0.0, 12.0, 15.0);
        assign_translate(&mut r, &PointF32::of(5.0, 4.0));
        assert_eq!(r, RectF32::of(5.0, 4.0, 17.0, 19.0));
        assign_translate(&mut r, &PointF32::of(-4.0, -2.0));
        assert_eq!(r, RectF32::of(1.0, 2.0, 13.0, 17.0));
    }

    #[test]
    fn assign_translate_small_rect_to_bounds() {
        let mut min_r = RectF32::of(MIN + 2.0, MIN + 5.0, MIN + 12.0, MIN + 15.0);
        assign_translate(&mut min_r, &PointF32::of(-2.0, -5.0));
        assert_eq!(min_r, RectF32::of(MIN, MIN, MIN + 10.0, MIN + 10.0));

        let mut max_r = RectF32::of(MAX - 12.0, MAX - 15.0, MAX - 2.0, MAX - 5.0);
        assign_translate(&mut max_r, &PointF32::of(2.0, 5.0));
        assert_eq!(max_r, RectF32::of(MAX - 10.0, MAX - 10.0, MAX, MAX));
    }

    #[test]
    fn assign_translate_big_rect_to_bounds() {
        let mut min_r = RectF32::of(MIN + 2.0, MIN + 5.0, MAX, MAX);
        assign_translate(&mut min_r, &PointF32::of(-2.0, -5.0));
        assert_eq!(min_r, RectF32::of(MIN, MIN, MAX - 2.0, MAX - 5.0));

        let mut max_r = RectF32::of(MIN, MIN, MAX - 2.0, MAX - 5.0);
        assign_translate(&mut max_r, &PointF32::of(2.0, 5.0));
        assert_eq!(max_r, RectF32::of(MIN + 2.0, MIN + 5.0, MAX, MAX));
    }

    #[test]
    fn assign_translate_small_rect_out_of_bounds() {
        let mut r_min = RectF32::of(MIN + 10.0, MIN + 5.0, MIN + 20.0, MIN + 30.0);
        assign_translate(&mut r_min, &PointF32::of(-20.0, -20.0));
        assert_eq!(r_min, RectF32::of(MAX - 9.0, MAX - 14.0, MIN, MIN + 10.0));

        let mut r_max = RectF32::of(MAX - 20.0, MAX - 30.0, MAX - 5.0, MAX - 10.0);
        assign_translate(&mut r_max, &PointF32::of(20.0, 20.0));
        assert_eq!(r_max, RectF32::of(MAX, MAX - 10.0, MIN + 14.0, MIN + 9.0));
    }

    #[test]
    fn assign_translate_big_rect_out_of_bounds() {
        let mut r_min = RectF32::of(MIN + 10.0, MIN + 5.0, MAX, MAX);
        assign_translate(&mut r_min, &PointF32::of(-20.0, -20.0));
        assert_eq!(r_min, RectF32::of(MAX - 9.0, MAX - 14.0, MAX - 20.0, MAX - 20.0));

        let mut r_max = RectF32::of(MIN, MIN, MAX - 5.0, MAX - 10.0);
        assign_translate(&mut r_max, &PointF32::of(20.0, 20.0));
        assert_eq!(r_max, RectF32::of(MIN + 20.0, MIN + 20.0, MIN + 14.0, MIN + 9.0));
    }

    #[test]
    fn assign_translate_small_rect_limits_out_of_bounds() {
        let mut r_min = RectF32::of(MIN + 1.0, MIN + 1.0, MIN + 10.0, MIN + 10.0);
        assign_translate(&mut r_min, &PointF32::min());
        assert_eq!(r_min, RectF32::of(1.0, 1.0, 10.0, 10.0));

        let mut r_max = RectF32::of(MAX - 10.0, MAX - 10.0, MAX - 1.0, MAX - 1.0);
        assign_translate(&mut r_max, &PointF32::max());
        assert_eq!(r_max, RectF32::of(-12.0, -12.0, -3.0, -3.0));
    }

    #[test]
    fn assign_translate_big_rect_limits_out_of_bounds() {
        let mut r1 = RectF32::largest();
        assign_translate(&mut r1, &PointF32::min());
        assert_eq!(r1, RectF32::of(0.0, 0.0, -1.0, -1.0));

        let mut r2 = RectF32::largest();
        assign_translate(&mut r2, &PointF32::max());
        assert_eq!(r2, RectF32::of(-1.0, -1.0, -2.0, -2.0));

        let mut r_min = RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX);
        assign_translate(&mut r_min, &PointF32::min());
        assert_eq!(r_min, RectF32::of(1.0, 1.0, -1.0, -1.0));

        let mut r_max = RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0);
        assign_translate(&mut r_max, &PointF32::max());
        assert_eq!(r_max, RectF32::of(-1.0, -1.0, -3.0, -3.0));
    }

    #[test]
    fn test_translate() {
        assert_eq!(translate(&RectF32::of(0.0, 0.0, 12.0, 15.0), &PointF32::of(5.0, 4.0)), RectF32::of(5.0, 4.0, 17.0, 19.0));
        assert_eq!(translate(&RectF32::of(5.0, 4.0, 17.0, 19.0), &PointF32::of(-4.0, -2.0)), RectF32::of(1.0, 2.0, 13.0, 17.0));
    }

    #[test]
    fn translate_small_rect_to_bounds() {
        assert_eq!(
            translate(&RectF32::of(MIN + 2.0, MIN + 5.0, MIN + 12.0, MIN + 15.0), &PointF32::of(-2.0, -5.0)),
            RectF32::of(MIN, MIN, MIN + 10.0, MIN + 10.0)
        );
        assert_eq!(
            translate(&RectF32::of(MAX - 12.0, MAX - 15.0, MAX - 2.0, MAX - 5.0), &PointF32::of(2.0, 5.0)),
            RectF32::of(MAX - 10.0, MAX - 10.0, MAX, MAX)
        );
    }

    #[test]
    fn translate_big_rect_to_bounds() {
        assert_eq!(translate(&RectF32::of(MIN + 2.0, MIN + 5.0, MAX, MAX), &PointF32::of(-2.0, -5.0)), RectF32::of(MIN, MIN, MAX - 2.0, MAX - 5.0));
        assert_eq!(translate(&RectF32::of(MIN, MIN, MAX - 2.0, MAX - 5.0), &PointF32::of(2.0, 5.0)), RectF32::of(MIN + 2.0, MIN + 5.0, MAX, MAX));
    }

    #[test]
    fn translate_small_rect_out_of_bounds() {
        assert_eq!(
            translate(&RectF32::of(MIN + 10.0, MIN + 5.0, MIN + 20.0, MIN + 30.0), &PointF32::of(-20.0, -20.0)),
            RectF32::of(MAX - 9.0, MAX - 14.0, MIN, MIN + 10.0)
        );
        assert_eq!(
            translate(&RectF32::of(MAX - 20.0, MAX - 30.0, MAX - 5.0, MAX - 10.0), &PointF32::of(20.0, 20.0)),
            RectF32::of(MAX, MAX - 10.0, MIN + 14.0, MIN + 9.0)
        );
    }

    #[test]
    fn translate_big_rect_out_of_bounds() {
        assert_eq!(
            translate(&RectF32::of(MIN + 10.0, MIN + 5.0, MAX, MAX), &PointF32::of(-20.0, -20.0)),
            RectF32::of(MAX - 9.0, MAX - 14.0, MAX - 20.0, MAX - 20.0)
        );
        assert_eq!(
            translate(&RectF32::of(MIN, MIN, MAX - 5.0, MAX - 10.0), &PointF32::of(20.0, 20.0)),
            RectF32::of(MIN + 20.0, MIN + 20.0, MIN + 14.0, MIN + 9.0)
        );
    }

    #[test]
    fn translate_small_rect_limits_out_of_bounds() {
        assert_eq!(translate(&RectF32::of(MIN + 1.0, MIN + 1.0, MIN + 10.0, MIN + 10.0), &PointF32::min()), RectF32::of(1.0, 1.0, 10.0, 10.0));
        assert_eq!(translate(&RectF32::of(MAX - 10.0, MAX - 10.0, MAX - 1.0, MAX - 1.0), &PointF32::max()), RectF32::of(-12.0, -12.0, -3.0, -3.0));
    }

    #[test]
    fn translate_big_rect_limits_out_of_bounds() {
        assert_eq!(translate(&RectF32::largest(), &PointF32::min()), RectF32::of(0.0, 0.0, -1.0, -1.0));
        assert_eq!(translate(&RectF32::largest(), &PointF32::max()), RectF32::of(-1.0, -1.0, -2.0, -2.0));
        assert_eq!(translate(&RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX), &PointF32::min()), RectF32::of(1.0, 1.0, -1.0, -1.0));
        assert_eq!(translate(&RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0), &PointF32::max()), RectF32::of(-1.0, -1.0, -3.0, -3.0));
    }
}
