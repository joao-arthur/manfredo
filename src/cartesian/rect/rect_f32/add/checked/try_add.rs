use crate::cartesian::{
    point::point_f32::{MAX, MIN, PointF32},
    rect::rect_f32::RectF32,
};

pub fn try_checked_add(r: &RectF32, delta: &RectF32) -> Option<RectF32> {
    if delta.min.x < MIN - r.min.x || delta.min.y < MIN - r.min.y || delta.max.x > MAX - r.max.x || delta.max.y > MAX - r.max.y {
        return None;
    }
    let min_x = r.min.x + delta.min.x;
    let min_y = r.min.y + delta.min.y;
    let max_x = r.max.x + delta.max.x;
    let max_y = r.max.y + delta.max.y;
    Some(RectF32 { min: PointF32 { x: min_x, y: min_y }, max: PointF32 { x: max_x, y: max_y } })
}

#[cfg(test)]
mod tests {
    use super::try_checked_add;
    use crate::cartesian::{
        point::point_f32::{MAX, MIN},
        rect::rect_f32::RectF32,
    };

    #[test]
    fn test_try_checked_add() {
        assert_eq!(try_checked_add(&RectF32::of(0.0, 0.0, 12.0, 15.0), &RectF32::of(5.0, 4.0, 3.0, 2.0)), Some(RectF32::of(5.0, 4.0, 15.0, 17.0)));
        assert_eq!(
            try_checked_add(&RectF32::of(5.0, 4.0, 15.0, 17.0), &RectF32::of(-14.0, -13.0, -12.0, -11.0)),
            Some(RectF32::of(-9.0, -9.0, 3.0, 6.0))
        );
    }

    #[test]
    fn try_checked_add_small_rect_to_bounds() {
        assert_eq!(
            try_checked_add(&RectF32::of(MIN + 2.0, MIN + 5.0, MIN + 12.0, MIN + 15.0), &RectF32::of(-2.0, -5.0, 10.0, 20.0)),
            Some(RectF32::of(MIN, MIN, MIN + 22.0, MIN + 35.0))
        );
        assert_eq!(
            try_checked_add(&RectF32::of(MAX - 12.0, MAX - 15.0, MAX - 2.0, MAX - 5.0), &RectF32::of(-1.0, -4.0, 2.0, 5.0)),
            Some(RectF32::of(MAX - 13.0, MAX - 19.0, MAX, MAX))
        );
    }

    #[test]
    fn try_checked_add_to_bounds() {
        assert_eq!(
            try_checked_add(&RectF32::of(MIN + 2.0, MIN + 5.0, MAX - 2.0, MAX - 5.0), &RectF32::of(-2.0, -5.0, 2.0, 5.0)),
            Some(RectF32::largest())
        );
        assert_eq!(try_checked_add(&RectF32::of(MIN + 2.0, MIN + 5.0, MAX, MAX), &RectF32::of(-2.0, -5.0, 0.0, 0.0)), Some(RectF32::largest()));
        assert_eq!(try_checked_add(&RectF32::of(MIN, MIN, MAX - 2.0, MAX - 5.0), &RectF32::of(0.0, 0.0, 2.0, 5.0)), Some(RectF32::largest()));
    }

    #[test]
    fn try_checked_add_out_of_bounds() {
        let r = RectF32::largest();
        assert_eq!(try_checked_add(&r, &RectF32::of(-1.0, 0.0, 0.0, 0.0)), None);
        assert_eq!(try_checked_add(&r, &RectF32::of(0.0, -1.0, 0.0, 0.0)), None);
        assert_eq!(try_checked_add(&r, &RectF32::of(0.0, 0.0, 1.0, 0.0)), None);
        assert_eq!(try_checked_add(&r, &RectF32::of(0.0, 0.0, 0.0, 1.0)), None);

        let r_min = RectF32::largest_min();
        assert_eq!(try_checked_add(&r_min, &RectF32::of(-1.0, -1.0, 1.0, 1.0)), None);

        let r_max = RectF32::largest_max();
        assert_eq!(try_checked_add(&r_max, &RectF32::of(-1.0, -1.0, 1.0, 1.0)), None);
    }

    #[test]
    fn try_checked_add_small_rect_out_of_bounds() {
        let mut r_min = RectF32::of(MIN + 10.0, MIN + 5.0, MIN + 20.0, MIN + 30.0);
        assert_eq!(try_checked_add(&mut r_min, &RectF32::of(-20.0, -20.0, 0.0, 0.0)), None);
        assert_eq!(r_min, RectF32::of(MIN + 10.0, MIN + 5.0, MIN + 20.0, MIN + 30.0));

        let mut r_max = RectF32::of(MAX - 20.0, MAX - 30.0, MAX - 5.0, MAX - 10.0);
        assert_eq!(try_checked_add(&mut r_max, &RectF32::of(0.0, 0.0, 20.0, 20.0)), None);
        assert_eq!(r_max, RectF32::of(MAX - 20.0, MAX - 30.0, MAX - 5.0, MAX - 10.0));
    }

    #[test]
    fn try_checked_add_big_rect_out_of_bounds() {
        let mut r_min = RectF32::of(MIN + 10.0, MIN + 5.0, MAX, MAX);
        assert_eq!(try_checked_add(&mut r_min, &RectF32::of(-20.0, -20.0, 0.0, 0.0)), None);
        assert_eq!(r_min, RectF32::of(MIN + 10.0, MIN + 5.0, MAX, MAX));

        let mut r_max = RectF32::of(MIN, MIN, MAX - 5.0, MAX - 10.0);
        assert_eq!(try_checked_add(&mut r_max, &RectF32::of(0.0, 0.0, 20.0, 20.0)), None);
        assert_eq!(r_max, RectF32::of(MIN, MIN, MAX - 5.0, MAX - 10.0));
    }

    #[test]
    fn try_checked_add_small_rect_limits_out_of_bounds() {
        let r_min = RectF32::of(MIN + 1.0, MIN + 1.0, MIN + 10.0, MIN + 10.0);
        assert_eq!(try_checked_add(&r_min, &RectF32::min()), None);
        assert_eq!(try_checked_add(&r_min, &RectF32::of(MIN, 0.0, 0.0, 0.0)), None);
        assert_eq!(try_checked_add(&r_min, &RectF32::of(0.0, MIN, 0.0, 0.0)), None);

        let r_max = RectF32::of(MAX - 10.0, MAX - 10.0, MAX - 1.0, MAX - 1.0);
        assert_eq!(try_checked_add(&r_max, &RectF32::max()), None);
        assert_eq!(try_checked_add(&r_max, &RectF32::of(0.0, 0.0, MAX, 0.0)), None);
        assert_eq!(try_checked_add(&r_max, &RectF32::of(0.0, 0.0, 0.0, MAX)), None);
    }

    #[test]
    fn try_checked_add_big_rect_limits_out_of_bounds() {
        let r = RectF32::largest();
        assert_eq!(try_checked_add(&r, &RectF32::largest()), None);
        assert_eq!(try_checked_add(&r, &RectF32::min()), None);
        assert_eq!(try_checked_add(&r, &RectF32::max()), None);

        let r_min = RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0);
        assert_eq!(try_checked_add(&r_min, &RectF32::max()), None);
        assert_eq!(try_checked_add(&r_min, &RectF32::of(0.0, 0.0, MAX, 0.0)), None);
        assert_eq!(try_checked_add(&r_min, &RectF32::of(0.0, 0.0, 0.0, MAX)), None);

        let r_max = RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX);
        assert_eq!(try_checked_add(&r_max, &RectF32::min()), None);
        assert_eq!(try_checked_add(&r_max, &RectF32::of(MIN, 0.0, 0.0, 0.0)), None);
        assert_eq!(try_checked_add(&r_max, &RectF32::of(0.0, MIN, 0.0, 0.0)), None);
    }
}
