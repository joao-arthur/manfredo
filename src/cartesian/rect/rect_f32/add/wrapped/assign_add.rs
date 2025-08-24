use crate::cartesian::{
    point::point_f32::{MAX, MIN},
    rect::rect_f32::RectF32,
};

pub fn assign_add(r: &mut RectF32, delta: &RectF32) {
    if r.min.x + delta.min.x > MAX {
        let diff_min_x = MAX - r.min.x;
        let delta_x_adjusted = delta.min.x - diff_min_x - 1.0;
        r.min.x = MIN + delta_x_adjusted;
    } else if r.min.x + delta.min.x < MIN {
        let diff_min_x = MIN - r.min.x;
        let delta_x_adjusted = delta.min.x - diff_min_x + 1.0;
        r.min.x = MAX + delta_x_adjusted;
    } else {
        r.min.x += delta.min.x;
    }
    if r.min.y + delta.min.y > MAX {
        let diff_min_y = MAX - r.min.y;
        let delta_y_adjusted = delta.min.y - diff_min_y - 1.0;
        r.min.y = MIN + delta_y_adjusted;
    } else if r.min.y + delta.min.y < MIN {
        let diff_min_y = MIN - r.min.y;
        let delta_y_adjusted = delta.min.y - diff_min_y + 1.0;
        r.min.y = MAX + delta_y_adjusted;
    } else {
        r.min.y += delta.min.y;
    }
    if r.max.x + delta.max.x > MAX {
        let diff_min_x = MAX - r.max.x;
        let delta_x_adjusted = delta.max.x - diff_min_x - 1.0;
        r.max.x = MIN + delta_x_adjusted;
    } else if r.max.x + delta.max.x < MIN {
        let diff_min_x = MIN - r.max.x;
        let delta_x_adjusted = delta.max.x - diff_min_x + 1.0;
        r.max.x = MAX + delta_x_adjusted;
    } else {
        r.max.x += delta.max.x;
    }
    if r.max.y + delta.max.y > MAX {
        let diff_min_y = MAX - r.max.y;
        let delta_y_adjusted = delta.max.y - diff_min_y - 1.0;
        r.max.y = MIN + delta_y_adjusted;
    } else if r.max.y + delta.max.y < MIN {
        let diff_min_y = MIN - r.max.y;
        let delta_y_adjusted = delta.max.y - diff_min_y + 1.0;
        r.max.y = MAX + delta_y_adjusted;
    } else {
        r.max.y += delta.max.y;
    }
}

#[cfg(test)]
mod tests {
    use super::assign_add;
    use crate::cartesian::{
        point::point_f32::{MAX, MIN},
        rect::rect_f32::RectF32,
    };

    #[test]
    fn test_assign_add() {
        let mut r = RectF32::of(0.0, 0.0, 12.0, 15.0);
        assign_add(&mut r, &RectF32::of(5.0, 4.0, 3.0, 2.0));
        assert_eq!(r, RectF32::of(5.0, 4.0, 15.0, 17.0));
        assign_add(&mut r, &RectF32::of(-14.0, -13.0, -12.0, -11.0));
        assert_eq!(r, RectF32::of(-9.0, -9.0, 3.0, 6.0));
    }

    #[test]
    fn assign_add_small_rect_to_bounds() {
        let mut min_r = RectF32::of(MIN + 2.0, MIN + 5.0, MIN + 12.0, MIN + 15.0);
        assign_add(&mut min_r, &RectF32::of(-2.0, -5.0, 9.0, 7.0));
        assert_eq!(min_r, RectF32::of(MIN, MIN, MIN + 21.0, MIN + 22.0));

        let mut max_r = RectF32::of(MAX - 12.0, MAX - 15.0, MAX - 2.0, MAX - 5.0);
        assign_add(&mut max_r, &RectF32::of(-9.0, -7.0, 2.0, 5.0));
        assert_eq!(max_r, RectF32::of(MAX - 21.0, MAX - 22.0, MAX, MAX));
    }

    #[test]
    fn assign_add_big_rect_to_bounds() {
        let mut min_r = RectF32::of(MIN + 2.0, MIN + 5.0, MAX - 2.0, MAX - 5.0);
        assign_add(&mut min_r, &RectF32::of(-2.0, -5.0, 2.0, 5.0));
        assert_eq!(min_r, RectF32::largest());

        let mut min_r = RectF32::of(MIN + 2.0, MIN + 5.0, MAX, MAX);
        assign_add(&mut min_r, &RectF32::of(-2.0, -5.0, 0.0, 0.0));
        assert_eq!(min_r, RectF32::largest());

        let mut max_r = RectF32::of(MIN, MIN, MAX - 2.0, MAX - 5.0);
        assign_add(&mut max_r, &RectF32::of(0.0, 0.0, 2.0, 5.0));
        assert_eq!(max_r, RectF32::largest());
    }

    #[test]
    fn assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectF32::of(MIN + 10.0, MIN + 5.0, MIN + 20.0, MIN + 30.0);
        assign_add(&mut r_min, &RectF32::of(-20.0, -20.0, 0.0, 0.0));
        assert_eq!(r_min, RectF32::of(MAX - 9.0, MAX - 14.0, MIN + 20.0, MIN + 30.0));

        let mut r_max = RectF32::of(MAX - 20.0, MAX - 30.0, MAX - 5.0, MAX - 10.0);
        assign_add(&mut r_max, &RectF32::of(0.0, 0.0, 20.0, 20.0));
        assert_eq!(r_max, RectF32::of(MAX - 20.0, MAX - 30.0, MIN + 14.0, MIN + 9.0));
    }

    #[test]
    fn assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectF32::of(MIN + 10.0, MIN + 5.0, MAX, MAX);
        assign_add(&mut r_min, &RectF32::of(-20.0, -20.0, 0.0, 0.0));
        assert_eq!(r_min, RectF32::of(MAX - 9.0, MAX - 14.0, MAX, MAX));

        let mut r_max = RectF32::of(MIN, MIN, MAX - 5.0, MAX - 10.0);
        assign_add(&mut r_max, &RectF32::of(0.0, 0.0, 20.0, 20.0));
        assert_eq!(r_max, RectF32::of(MIN, MIN, MIN + 14.0, MIN + 9.0));
    }

    #[test]
    fn assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectF32::of(MIN + 1.0, MIN + 1.0, MIN + 10.0, MIN + 10.0);
        assign_add(&mut r_min, &RectF32::min());
        assert_eq!(r_min, RectF32::of(1.0, 1.0, 10.0, 10.0));

        let mut r_max = RectF32::of(MAX - 10.0, MAX - 10.0, MAX - 1.0, MAX - 1.0);
        assign_add(&mut r_max, &RectF32::max());
        assert_eq!(r_max, RectF32::of(-12.0, -12.0, -3.0, -3.0));
    }

    #[test]
    fn assign_add_big_rect_limits_out_of_bounds() {
        let mut r1 = RectF32::largest();
        assign_add(&mut r1, &RectF32::min());
        assert_eq!(r1, RectF32::of(0.0, 0.0, -1.0, -1.0));

        let mut r2 = RectF32::largest();
        assign_add(&mut r2, &RectF32::max());
        assert_eq!(r2, RectF32::of(-1.0, -1.0, -2.0, -2.0));

        let mut r_min = RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX);
        assign_add(&mut r_min, &RectF32::min());
        assert_eq!(r_min, RectF32::of(1.0, 1.0, -1.0, -1.0));

        let mut r_max = RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0);
        assign_add(&mut r_max, &RectF32::max());
        assert_eq!(r_max, RectF32::of(-1.0, -1.0, -3.0, -3.0));
    }
}
