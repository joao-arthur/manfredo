use super::{MAX, MIN, PointF32};

pub fn assign_add(p: &mut PointF32, delta: &PointF32) {
    let temp_x = p.x + delta.x;
    let temp_y = p.y + delta.y;
    p.x = temp_x.clamp(MIN, MAX);
    p.y = temp_y.clamp(MIN, MAX);
}

pub fn add(p: &PointF32, delta: &PointF32) -> PointF32 {
    let temp_x = p.x + delta.x;
    let temp_y = p.y + delta.y;
    PointF32::of(temp_x.clamp(MIN, MAX), temp_y.clamp(MIN, MAX))
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_f32::{MAX, MIN, PointF32};

    use super::{add, assign_add};

    #[test]
    fn test_assign_add() {
        let mut p = PointF32::of(0.0, 0.0);
        assign_add(&mut p, &PointF32::of(10.0, 13.0));
        assert_eq!(p, PointF32::of(10.0, 13.0));
        assign_add(&mut p, &PointF32::of(-5.0, -3.0));
        assert_eq!(p, PointF32::of(5.0, 10.0));
    }

    #[test]
    fn assign_add_to_bounds() {
        let mut p_min = PointF32::of(MIN + 2.0, MIN + 5.0);
        assign_add(&mut p_min, &PointF32::of(-2.0, -5.0));
        assert_eq!(p_min, PointF32::min());

        let mut p_max = PointF32::of(MAX - 2.0, MAX - 5.0);
        assign_add(&mut p_max, &PointF32::of(2.0, 5.0));
        assert_eq!(p_max, PointF32::max());
    }

    #[test]
    fn assign_add_out_of_bounds() {
        let mut p_min = PointF32::of(MIN + 2.0, MIN + 5.0);
        assign_add(&mut p_min, &PointF32::of(-10.0, -10.0));
        assert_eq!(p_min, PointF32::min());

        let mut p_max = PointF32::of(MAX - 2.0, MAX - 5.0);
        assign_add(&mut p_max, &PointF32::of(10.0, 10.0));
        assert_eq!(p_max, PointF32::max());
    }

    #[test]
    fn assign_add_limits_out_of_bounds_out_of_bounds() {
        let mut p_min = PointF32::of(MIN + 1.0, MIN + 1.0);
        assign_add(&mut p_min, &PointF32::min());
        assert_eq!(p_min, PointF32::min());

        let mut p_max = PointF32::of(MAX - 1.0, MAX - 1.0);
        assign_add(&mut p_max, &PointF32::max());
        assert_eq!(p_max, PointF32::max());
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&PointF32::of(0.0, 0.0), &PointF32::of(10.0, 13.0)), PointF32::of(10.0, 13.0));
        assert_eq!(add(&PointF32::of(10.0, 10.0), &PointF32::of(-5.0, -3.0)), PointF32::of(5.0, 7.0));
    }

    #[test]
    fn add_to_bounds() {
        assert_eq!(add(&PointF32::of(MIN + 2.0, MIN + 5.0), &PointF32::of(-2.0, -5.0)), PointF32::min());
        assert_eq!(add(&PointF32::of(MAX - 2.0, MAX - 5.0), &PointF32::of(2.0, 5.0)), PointF32::max());
    }

    #[test]
    fn add_out_of_bounds() {
        assert_eq!(add(&PointF32::of(MIN + 2.0, MIN + 5.0), &PointF32::of(-10.0, -10.0)), PointF32::min());
        assert_eq!(add(&PointF32::of(MAX - 2.0, MAX - 5.0), &PointF32::of(10.0, 10.0)), PointF32::max());
    }

    #[test]
    fn add_limits_out_of_bounds() {
        assert_eq!(add(&PointF32::of(MIN + 1.0, MIN + 1.0), &PointF32::min()), PointF32::min());
        assert_eq!(add(&PointF32::of(MAX - 1.0, MAX - 1.0), &PointF32::max()), PointF32::max());
    }
}
