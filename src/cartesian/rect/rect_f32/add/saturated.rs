use crate::cartesian::{point::point_f32::{PointF32, MIN, MAX}, rect::rect_f32::RectF32};

pub fn assign_add(r: &mut RectF32, delta: &RectF32) {
    r.min.x = (r.min.x + delta.min.x).clamp(MIN, MAX);
    r.min.y = (r.min.y + delta.min.y).clamp(MIN, MAX);
    r.max.x = (r.max.x + delta.max.x).clamp(MIN, MAX);
    r.max.y = (r.max.y + delta.max.y).clamp(MIN, MAX);
}

pub fn add(r: &RectF32, delta: &RectF32) -> RectF32 {
    let min_x = (r.min.x + delta.min.x).clamp(MIN, MAX);
    let min_y = (r.min.y + delta.min.y).clamp(MIN, MAX);
    let max_x = (r.max.x + delta.max.x).clamp(MIN, MAX);
    let max_y = (r.max.y + delta.max.y).clamp(MIN, MAX);
    RectF32 { min: PointF32 { x: min_x, y: min_y }, max: PointF32 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{rect::rect_f32::RectF32, point::point_f32::{MIN, MAX}};

    use super::{add, assign_add};

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
        let mut r = RectF32::of(MIN + 2.0, MIN + 5.0, MAX - 2.0, MAX - 5.0);
        assign_add(&mut r, &RectF32::of(-2.0, -5.0, 2.0, 5.0));
        assert_eq!(r, RectF32::largest());

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
        assert_eq!(r_min, RectF32::of(MIN, MIN, MIN + 20.0, MIN + 30.0));

        let mut r_max = RectF32::of(MAX - 20.0, MAX - 30.0, MAX - 5.0, MAX - 10.0);
        assign_add(&mut r_max, &RectF32::of(0.0, 0.0, 20.0, 20.0));
        assert_eq!(r_max, RectF32::of(MAX - 20.0, MAX - 30.0, MAX, MAX));
    }

    #[test]
    fn assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectF32::of(MIN + 10.0, MIN + 5.0, MAX, MAX);
        assign_add(&mut r_min, &RectF32::of(-20.0, -20.0, 0.0, 0.0));
        assert_eq!(r_min, RectF32::largest());

        let mut r_max = RectF32::of(MIN, MIN, MAX - 5.0, MAX - 10.0);
        assign_add(&mut r_max, &RectF32::of(0.0, 0.0, 20.0, 20.0));
        assert_eq!(r_max, RectF32::largest());
    }

    #[test]
    fn assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectF32::of(MIN + 1.0, MIN + 1.0, MIN + 10.0, MIN + 10.0);
        assign_add(&mut r_min, &RectF32::min());
        assert_eq!(r_min, RectF32::min());

        let mut r_max = RectF32::of(MAX - 10.0, MAX - 10.0, MAX - 1.0, MAX - 1.0);
        assign_add(&mut r_max, &RectF32::max());
        assert_eq!(r_max, RectF32::max());
    }

    #[test]
    fn assign_add_big_rect_limits_out_of_bounds() {
        let mut r = RectF32::largest();
        assign_add(&mut r, &RectF32::largest());
        assert_eq!(r, RectF32::largest());

        let mut r_large = RectF32::of(MIN + 1.0, MIN + 1.0, MAX - 1.0, MAX - 1.0);
        assign_add(&mut r_large, &RectF32::largest());
        assert_eq!(r_large, RectF32::largest());

        let mut r_min_x = RectF32::of(MIN + 1.0, MIN + 1.0, MAX - 1.0, MAX - 1.0);
        assign_add(&mut r_min_x, &RectF32::of(MIN, 0.0, 0.0, 0.0));
        assert_eq!(r_min_x, RectF32::of(MIN, MIN + 1.0, MAX - 1.0, MAX - 1.0));

        let mut r_min_y = RectF32::of(MIN + 1.0, MIN + 1.0, MAX - 1.0, MAX - 1.0);
        assign_add(&mut r_min_y, &RectF32::of(0.0, MIN, 0.0, 0.0));
        assert_eq!(r_min_y, RectF32::of(MIN + 1.0, MIN, MAX - 1.0, MAX - 1.0));

        let mut r_max_x = RectF32::of(MIN + 1.0, MIN + 1.0, MAX - 1.0, MAX - 1.0);
        assign_add(&mut r_max_x, &RectF32::of(0.0, 0.0, MAX, 0.0));
        assert_eq!(r_max_x, RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX - 1.0));

        let mut r_max_y = RectF32::of(MIN + 1.0, MIN + 1.0, MAX - 1.0, MAX - 1.0);
        assign_add(&mut r_max_y, &RectF32::of(0.0, 0.0, 0.0, MAX));
        assert_eq!(r_max_y, RectF32::of(MIN + 1.0, MIN + 1.0, MAX - 1.0, MAX));
    }

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
            RectF32::of(MIN, MIN, MIN + 20.0, MIN + 30.0)
        );
        assert_eq!(
            add(&RectF32::of(MAX - 20.0, MAX - 30.0, MAX - 5.0, MAX - 10.0), &RectF32::of(0.0, 0.0, 20.0, 20.0)),
            RectF32::of(MAX - 20.0, MAX - 30.0, MAX, MAX)
        );
    }

    #[test]
    fn add_big_rect_out_of_bounds() {
        assert_eq!(add(&RectF32::of(MIN + 10.0, MIN + 5.0, MAX, MAX), &RectF32::of(-20.0, -20.0, 0.0, 0.0)), RectF32::largest());
        assert_eq!(add(&RectF32::of(MIN, MIN, MAX - 5.0, MAX - 10.0), &RectF32::of(0.0, 0.0, 20.0, 20.0)), RectF32::largest());
    }

    #[test]
    fn add_small_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectF32::of(MIN + 1.0, MIN + 1.0, MIN + 10.0, MIN + 10.0), &RectF32::min()), RectF32::min());
        assert_eq!(add(&RectF32::of(MAX - 10.0, MAX - 10.0, MAX - 1.0, MAX - 1.0), &RectF32::max()), RectF32::max());
    }

    #[test]
    fn add_big_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectF32::largest(), &RectF32::largest()), RectF32::largest());

        let r_large = RectF32::of(MIN + 1.0, MIN + 1.0, MAX - 1.0, MAX - 1.0);
        assert_eq!(add(&r_large, &RectF32::largest()), RectF32::largest());
        assert_eq!(add(&r_large, &RectF32::of(MIN, 0.0, 0.0, 0.0)), RectF32::of(MIN, MIN + 1.0, MAX - 1.0, MAX - 1.0));
        assert_eq!(add(&r_large, &RectF32::of(0.0, MIN, 0.0, 0.0)), RectF32::of(MIN + 1.0, MIN, MAX - 1.0, MAX - 1.0));
        assert_eq!(add(&r_large, &RectF32::of(0.0, 0.0, MAX, 0.0)), RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX - 1.0));
        assert_eq!(add(&r_large, &RectF32::of(0.0, 0.0, 0.0, MAX)), RectF32::of(MIN + 1.0, MIN + 1.0, MAX - 1.0, MAX));
    }
}
