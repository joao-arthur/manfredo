use crate::cartesian::{
    point::point_f32::{MAX, MIN, PointF32},
    rect::rect_f32::RectF32,
};

pub fn add(r: &RectF32, delta: &RectF32) -> RectF32 {
    let mut min_x = r.min.x;
    let mut min_y = r.min.y;
    let mut max_x = r.max.x;
    let mut max_y = r.max.y;
    if min_x + delta.min.x > MAX {
        let diff_min_x = MAX - min_x;
        let delta_x_adjusted = delta.min.x - diff_min_x - 1.0;
        min_x = MIN + delta_x_adjusted;
    } else if min_x + delta.min.x < MIN {
        let diff_min_x = MIN - min_x;
        let delta_x_adjusted = delta.min.x - diff_min_x + 1.0;
        min_x = MAX + delta_x_adjusted;
    } else {
        min_x += delta.min.x;
    }
    if min_y + delta.min.y > MAX {
        let diff_min_y = MAX - min_y;
        let delta_y_adjusted = delta.min.y - diff_min_y - 1.0;
        min_y = MIN + delta_y_adjusted;
    } else if min_y + delta.min.y < MIN {
        let diff_min_y = MIN - min_y;
        let delta_y_adjusted = delta.min.y - diff_min_y + 1.0;
        min_y = MAX + delta_y_adjusted;
    } else {
        min_y += delta.min.y;
    }
    if max_x + delta.max.x > MAX {
        let diff_min_x = MAX - max_x;
        let delta_x_adjusted = delta.max.x - diff_min_x - 1.0;
        max_x = MIN + delta_x_adjusted;
    } else if max_x + delta.max.x < MIN {
        let diff_min_x = MIN - max_x;
        let delta_x_adjusted = delta.max.x - diff_min_x + 1.0;
        max_x = MAX + delta_x_adjusted;
    } else {
        max_x += delta.max.x;
    }
    if max_y + delta.max.y > MAX {
        let diff_min_y = MAX - max_y;
        let delta_y_adjusted = delta.max.y - diff_min_y - 1.0;
        max_y = MIN + delta_y_adjusted;
    } else if max_y + delta.max.y < MIN {
        let diff_min_y = MIN - max_y;
        let delta_y_adjusted = delta.max.y - diff_min_y + 1.0;
        max_y = MAX + delta_y_adjusted;
    } else {
        max_y += delta.max.y;
    }
    RectF32 { min: PointF32 { x: min_x, y: min_y }, max: PointF32 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use super::add;
    use crate::cartesian::{
        point::point_f32::{MAX, MIN},
        rect::rect_f32::RectF32,
    };

    #[test]
    fn test_add() {
        assert_eq!(add(&RectF32::of(0.0, 0.0, 12.0, 15.0), &RectF32::of(5.0, 4.0, 3.0, 2.0)), RectF32::of(5.0, 4.0, 15.0, 17.0));
        assert_eq!(add(&RectF32::of(5.0, 4.0, 15.0, 17.0), &RectF32::of(-14.0, -13.0, -12.0, -11.0)), RectF32::of(-9.0, -9.0, 3.0, 6.0));
    }

    #[test]
    fn add_small_rect_to_bounds() {
        assert_eq!(
            add(&RectF32::of(MIN + 2.0, MIN + 5.0, MIN + 12.0, MIN + 15.0), &RectF32::of(-2.0, -5.0, 9.0, 7.0)),
            RectF32::of(MIN, MIN, MIN + 21.0, MIN + 22.0)
        );
        assert_eq!(
            add(&RectF32::of(MAX - 12.0, MAX - 15.0, MAX - 2.0, MAX - 5.0), &RectF32::of(-9.0, -7.0, 2.0, 5.0)),
            RectF32::of(MAX - 21.0, MAX - 22.0, MAX, MAX)
        );
    }

    #[test]
    fn add_big_rect_to_bounds() {
        assert_eq!(add(&RectF32::of(MIN + 2.0, MIN + 5.0, MAX - 2.0, MAX - 5.0), &RectF32::of(-2.0, -5.0, 2.0, 5.0)), RectF32::largest());
        assert_eq!(add(&RectF32::of(MIN + 2.0, MIN + 5.0, MAX, MAX), &RectF32::of(-2.0, -5.0, 0.0, 0.0)), RectF32::largest());
        assert_eq!(add(&RectF32::of(MIN, MIN, MAX - 2.0, MAX - 5.0), &RectF32::of(0.0, 0.0, 2.0, 5.0)), RectF32::largest());
    }

    #[test]
    fn add_small_rect_out_of_bounds() {
        assert_eq!(
            add(&RectF32::of(MIN + 10.0, MIN + 5.0, MIN + 20.0, MIN + 30.0), &RectF32::of(-20.0, -20.0, 0.0, 0.0)),
            RectF32::of(MAX - 9.0, MAX - 14.0, MIN + 20.0, MIN + 30.0)
        );
        assert_eq!(
            add(&RectF32::of(MAX - 20.0, MAX - 30.0, MAX - 5.0, MAX - 10.0), &RectF32::of(0.0, 0.0, 20.0, 20.0)),
            RectF32::of(MAX - 20.0, MAX - 30.0, MIN + 14.0, MIN + 9.0)
        );
    }

    #[test]
    fn add_big_rect_out_of_bounds() {
        assert_eq!(
            add(&RectF32::of(MIN + 10.0, MIN + 5.0, MAX, MAX), &RectF32::of(-20.0, -20.0, 0.0, 0.0)),
            RectF32::of(MAX - 9.0, MAX - 14.0, MAX, MAX)
        );
        assert_eq!(
            add(&RectF32::of(MIN, MIN, MAX - 5.0, MAX - 10.0), &RectF32::of(0.0, 0.0, 20.0, 20.0)),
            RectF32::of(MIN, MIN, MIN + 14.0, MIN + 9.0)
        );
    }

    #[test]
    fn add_small_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectF32::of(MIN + 1.0, MIN + 1.0, MIN + 10.0, MIN + 10.0), &RectF32::min()), RectF32::of(1.0, 1.0, 10.0, 10.0));
        assert_eq!(add(&RectF32::of(MAX - 10.0, MAX - 10.0, MAX - 1.0, MAX - 1.0), &RectF32::max()), RectF32::of(-12.0, -12.0, -3.0, -3.0));
    }

    #[test]
    fn add_big_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectF32::largest(), &RectF32::min()), RectF32::of(0.0, 0.0, -1.0, -1.0));
        assert_eq!(add(&RectF32::largest(), &RectF32::max()), RectF32::of(-1.0, -1.0, -2.0, -2.0));
        assert_eq!(add(&RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX), &RectF32::min()), RectF32::of(1.0, 1.0, -1.0, -1.0));
        assert_eq!(add(&RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0), &RectF32::max()), RectF32::of(-1.0, -1.0, -3.0, -3.0));
    }
}
