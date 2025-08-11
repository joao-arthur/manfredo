use super::{MAX, MIN, PointF64};

pub fn assign_add(p: &mut PointF64, delta: &PointF64) {
    if p.x + delta.x > MAX {
        let diff_min_x = MAX - p.x;
        let delta_x_adjusted = delta.x - diff_min_x - 1.0;
        p.x = MIN + delta_x_adjusted;
    } else if p.x + delta.x < MIN {
        let diff_min_x = MIN - p.x;
        let delta_x_adjusted = delta.x - diff_min_x + 1.0;
        p.x = MAX + delta_x_adjusted;
    } else {
        p.x += delta.x;
    }
    if p.y + delta.y > MAX {
        let diff_min_y = MAX - p.y;
        let delta_y_adjusted = delta.y - diff_min_y - 1.0;
        p.y = MIN + delta_y_adjusted;
    } else if p.y + delta.y < MIN {
        let diff_min_y = MIN - p.y;
        let delta_y_adjusted = delta.y - diff_min_y + 1.0;
        p.y = MAX + delta_y_adjusted;
    } else {
        p.y += delta.y;
    }
}

pub fn add(p: &PointF64, delta: &PointF64) -> PointF64 {
    let mut x = p.x;
    let mut y = p.y;
    if x + delta.x > MAX {
        let diff_min_x = MAX - x;
        let delta_x_adjusted = delta.x - diff_min_x - 1.0;
        x = MIN + delta_x_adjusted;
    } else if x + delta.x < MIN {
        let diff_min_x = MIN - x;
        let delta_x_adjusted = delta.x - diff_min_x + 1.0;
        x = MAX + delta_x_adjusted;
    } else {
        x += delta.x;
    }
    if y + delta.y > MAX {
        let diff_min_y = MAX - y;
        let delta_y_adjusted = delta.y - diff_min_y - 1.0;
        y = MIN + delta_y_adjusted;
    } else if y + delta.y < MIN {
        let diff_min_y = MIN - y;
        let delta_y_adjusted = delta.y - diff_min_y + 1.0;
        y = MAX + delta_y_adjusted;
    } else {
        y += delta.y;
    }
    PointF64 { x, y }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_f64::{MAX, MIN, PointF64};

    use super::{add, assign_add};

    #[test]
    fn test_assign_add() {
        let mut p = PointF64::of(0.0, 0.0);
        assign_add(&mut p, &PointF64::of(10.0, 13.0));
        assert_eq!(p, PointF64::of(10.0, 13.0));
        assign_add(&mut p, &PointF64::of(-5.0, -3.0));
        assert_eq!(p, PointF64::of(5.0, 10.0));
    }

    #[test]
    fn assign_add_to_bounds() {
        let mut p_min = PointF64::of(MIN + 2.0, MIN + 5.0);
        assign_add(&mut p_min, &PointF64::of(-2.0, -5.0));
        assert_eq!(p_min, PointF64::min());

        let mut m_max = PointF64::of(MAX - 2.0, MAX - 5.0);
        assign_add(&mut m_max, &PointF64::of(2.0, 5.0));
        assert_eq!(m_max, PointF64::max());
    }

    #[test]
    fn assign_add_out_of_bounds() {
        let mut p_min = PointF64::of(MIN + 2.0, MIN + 5.0);
        assign_add(&mut p_min, &PointF64::of(-10.0, -10.0));
        assert_eq!(p_min, PointF64::of(MAX - 7.0, MAX - 4.0));

        let mut m_max = PointF64::of(MAX - 2.0, MAX - 5.0);
        assign_add(&mut m_max, &PointF64::of(10.0, 10.0));
        assert_eq!(m_max, PointF64::of(MIN + 7.0, MIN + 4.0));
    }

    #[test]
    fn assign_add_limits_out_of_bounds_out_of_bounds() {
        let mut p_min = PointF64::of(MIN + 1.0, MIN + 1.0);
        assign_add(&mut p_min, &PointF64::min());
        assert_eq!(p_min, PointF64::of(1.0, 1.0));

        let mut m_max = PointF64::of(MAX - 1.0, MAX - 1.0);
        assign_add(&mut m_max, &PointF64::max());
        assert_eq!(m_max, PointF64::of(-3.0, -3.0));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&PointF64::of(0.0, 0.0), &PointF64::of(10.0, 13.0)), PointF64::of(10.0, 13.0));
        assert_eq!(add(&PointF64::of(10.0, 10.0), &PointF64::of(-5.0, -3.0)), PointF64::of(5.0, 7.0));
    }

    #[test]
    fn add_to_bounds() {
        assert_eq!(add(&PointF64::of(MIN + 2.0, MIN + 5.0), &PointF64::of(-2.0, -5.0)), PointF64::min());
        assert_eq!(add(&PointF64::of(MAX - 2.0, MAX - 5.0), &PointF64::of(2.0, 5.0)), PointF64::max());
    }

    #[test]
    fn add_out_of_bounds() {
        assert_eq!(add(&PointF64::of(MIN + 2.0, MIN + 5.0), &PointF64::of(-10.0, -10.0)), PointF64::of(MAX - 7.0, MAX - 4.0));
        assert_eq!(add(&PointF64::of(MAX - 2.0, MAX - 5.0), &PointF64::of(10.0, 10.0)), PointF64::of(MIN + 7.0, MIN + 4.0));
    }

    #[test]
    fn add_limits_out_of_bounds() {
        assert_eq!(add(&PointF64::of(MIN + 1.0, MIN + 1.0), &PointF64::min()), PointF64::of(1.0, 1.0));
        assert_eq!(add(&PointF64::of(MAX - 1.0, MAX - 1.0), &PointF64::max()), PointF64::of(-3.0, -3.0));
    }
}
