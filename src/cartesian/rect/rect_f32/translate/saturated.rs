use crate::cartesian::{
    point::point_f32::{MAX, MIN, PointF32},
    rect::rect_f32::{RectF32, delta_x, delta_y},
};

pub fn assign_translate(r: &mut RectF32, delta: &PointF32) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = r.min.x + delta.x;
    let temp_min_y = r.min.y + delta.y;
    let min_x = temp_min_x.clamp(MIN, MAX - dx);
    let min_y = temp_min_y.clamp(MIN, MAX - dy);
    let max_x = min_x + dx;
    let max_y = min_y + dy;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
}

pub fn translate(r: &RectF32, delta: &PointF32) -> RectF32 {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = r.min.x + delta.x;
    let temp_min_y = r.min.y + delta.y;
    let min_x = temp_min_x.clamp(MIN, MAX - dx);
    let min_y = temp_min_y.clamp(MIN, MAX - dy);
    let max_x = min_x + dx;
    let max_y = min_y + dy;
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
        let mut r = RectF32::of(0.0, 0.0, 10.0, 10.0);
        assign_translate(&mut r, &PointF32::of(10.0, 20.0));
        assert_eq!(r, RectF32::of(10.0, 20.0, 20.0, 30.0));
        assign_translate(&mut r, &PointF32::of(-20.0, -15.0));
        assert_eq!(r, RectF32::of(-10.0, 5.0, 0.0, 15.0));
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
        let mut min_r = RectF32::of(MIN + 2.0, MIN + 5.0, 0.0, 0.0);
        assign_translate(&mut min_r, &PointF32::of(-2.0, -5.0));
        assert_eq!(min_r, RectF32::of(MIN, MIN, -2.0, -5.0));

        let mut max_r = RectF32::of(0.0, 0.0, MAX - 2.0, MAX - 5.0);
        assign_translate(&mut max_r, &PointF32::of(2.0, 5.0));
        assert_eq!(max_r, RectF32::of(2.0, 5.0, MAX, MAX));
    }

    #[test]
    fn assign_translate_small_rect_out_of_bounds() {
        let mut r_min = RectF32::of(MIN + 10.0, MIN + 5.0, MIN + 20.0, MIN + 30.0);
        assign_translate(&mut r_min, &PointF32::of(-20.0, -20.0));
        assert_eq!(r_min, RectF32::of(MIN, MIN, MIN + 10.0, MIN + 25.0));

        let mut r_max = RectF32::of(MAX - 20.0, MAX - 30.0, MAX - 5.0, MAX - 10.0);
        assign_translate(&mut r_max, &PointF32::of(20.0, 20.0));
        assert_eq!(r_max, RectF32::of(MAX - 15.0, MAX - 20.0, MAX, MAX));
    }

    #[test]
    fn assign_translate_big_rect_out_of_bounds() {
        let mut r_min = RectF32::of(MIN + 10.0, MIN + 5.0, 0.0, 0.0);
        assign_translate(&mut r_min, &PointF32::of(-20.0, -20.0));
        assert_eq!(r_min, RectF32::of(MIN, MIN, -10.0, -5.0));

        let mut r_max = RectF32::of(0.0, 0.0, MAX - 5.0, MAX - 10.0);
        assign_translate(&mut r_max, &PointF32::of(20.0, 20.0));
        assert_eq!(r_max, RectF32::of(5.0, 10.0, MAX, MAX));
    }

    #[test]
    fn assign_translate_small_rect_limits_out_of_bounds() {
        let mut r_min = RectF32::of(MIN + 1.0, MIN + 1.0, MIN + 10.0, MIN + 10.0);
        assign_translate(&mut r_min, &PointF32::min());
        assert_eq!(r_min, RectF32::of(MIN, MIN, MIN + 9.0, MIN + 9.0));

        let mut r_max = RectF32::of(MAX - 10.0, MAX - 10.0, MAX - 1.0, MAX - 1.0);
        assign_translate(&mut r_max, &PointF32::max());
        assert_eq!(r_max, RectF32::of(MAX - 9.0, MAX - 9.0, MAX, MAX));
    }

    #[test]
    fn assign_translate_big_rect_limits_out_of_bounds() {
        let mut r_min = RectF32::of(MIN, MIN, 0.0, 0.0);
        assign_translate(&mut r_min, &PointF32::min());
        assert_eq!(r_min, RectF32::of(MIN, MIN, 0.0, 0.0));

        let mut r_max = RectF32::of(0.0, 0.0, MAX, MAX);
        assign_translate(&mut r_max, &PointF32::max());
        assert_eq!(r_max, RectF32::of(0.0, 0.0, MAX, MAX));

        let mut r_almost_min = RectF32::of(MIN + 1.0, MIN + 1.0, 0.0, 0.0);
        assign_translate(&mut r_almost_min, &PointF32::min());
        assert_eq!(r_almost_min, RectF32::of(MIN, MIN, -1.0, -1.0));

        let mut r_almost_max = RectF32::of(0.0, 0.0, MAX - 1.0, MAX - 1.0);
        assign_translate(&mut r_almost_max, &PointF32::max());
        assert_eq!(r_almost_max, RectF32::of(1.0, 1.0, MAX, MAX));
    }

    #[test]
    fn test_translate() {
        assert_eq!(translate(&RectF32::of(0.0, 0.0, 10.0, 10.0), &PointF32::of(10.0, 20.0)), RectF32::of(10.0, 20.0, 20.0, 30.0));
        assert_eq!(translate(&RectF32::of(10.0, 20.0, 20.0, 30.0), &PointF32::of(-20.0, -15.0)), RectF32::of(-10.0, 5.0, 0.0, 15.0));
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
        assert_eq!(translate(&RectF32::of(MIN + 2.0, MIN + 5.0, 0.0, 0.0), &PointF32::of(-2.0, -5.0)), RectF32::of(MIN, MIN, -2.0, -5.0));
        assert_eq!(translate(&RectF32::of(0.0, 0.0, MAX - 2.0, MAX - 5.0), &PointF32::of(2.0, 5.0)), RectF32::of(2.0, 5.0, MAX, MAX));
    }

    #[test]
    fn translate_small_rect_out_of_bounds() {
        assert_eq!(
            translate(&RectF32::of(MIN + 10.0, MIN + 5.0, MIN + 20.0, MIN + 30.0), &PointF32::of(-20.0, -20.0)),
            RectF32::of(MIN, MIN, MIN + 10.0, MIN + 25.0)
        );
        assert_eq!(
            translate(&RectF32::of(MAX - 20.0, MAX - 30.0, MAX - 5.0, MAX - 10.0), &PointF32::of(20.0, 20.0)),
            RectF32::of(MAX - 15.0, MAX - 20.0, MAX, MAX)
        );
    }

    #[test]
    fn translate_big_rect_out_of_bounds() {
        assert_eq!(translate(&RectF32::of(MIN + 10.0, MIN + 5.0, 0.0, 0.0), &PointF32::of(-20.0, -20.0)), RectF32::of(MIN, MIN, -10.0, -5.0));
        assert_eq!(translate(&RectF32::of(0.0, 0.0, MAX - 5.0, MAX - 10.0), &PointF32::of(20.0, 20.0)), RectF32::of(5.0, 10.0, MAX, MAX));
    }

    #[test]
    fn translate_small_rect_limits_out_of_bounds() {
        assert_eq!(
            translate(&RectF32::of(MIN + 1.0, MIN + 1.0, MIN + 10.0, MIN + 10.0), &PointF32::min()),
            RectF32::of(MIN, MIN, MIN + 9.0, MIN + 9.0)
        );
        assert_eq!(
            translate(&RectF32::of(MAX - 10.0, MAX - 10.0, MAX - 1.0, MAX - 1.0), &PointF32::max()),
            RectF32::of(MAX - 9.0, MAX - 9.0, MAX, MAX)
        );
    }

    #[test]
    fn translate_big_rect_limits_out_of_bounds() {
        assert_eq!(translate(&RectF32::of(MIN, MIN, 0.0, 0.0), &PointF32::min()), RectF32::of(MIN, MIN, 0.0, 0.0));
        assert_eq!(translate(&RectF32::of(0.0, 0.0, MAX, MAX), &PointF32::max()), RectF32::of(0.0, 0.0, MAX, MAX));
        assert_eq!(translate(&RectF32::of(MIN + 1.0, MIN + 1.0, 0.0, 0.0), &PointF32::min()), RectF32::of(MIN, MIN, -1.0, -1.0));
        assert_eq!(translate(&RectF32::of(0.0, 0.0, MAX - 1.0, MAX - 1.0), &PointF32::max()), RectF32::of(1.0, 1.0, MAX, MAX));
    }
}
