use crate::cartesian::{
    point::point_i32::PointI32,
    rect::rect_i32::{RectI32, delta_x, delta_y},
};

pub fn assign_translate(r: &mut RectI32, delta: &PointI32) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i64::from(r.min.x) + i64::from(delta.x);
    let temp_min_y = i64::from(r.min.y) + i64::from(delta.y);
    let min_x = temp_min_x.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(dx));
    let min_y = temp_min_y.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(dy));
    r.min.x = min_x as i32;
    r.min.y = min_y as i32;
    r.max.x = (min_x + i64::from(dx)) as i32;
    r.max.y = (min_y + i64::from(dy)) as i32;
}

pub fn translate(r: &RectI32, delta: &PointI32) -> RectI32 {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i64::from(r.min.x) + i64::from(delta.x);
    let temp_min_y = i64::from(r.min.y) + i64::from(delta.y);
    let min_x = temp_min_x.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(dx));
    let min_y = temp_min_y.clamp(i64::from(i32::MIN), i64::from(i32::MAX) - i64::from(dy));
    let max_x = min_x + i64::from(dx);
    let max_y = min_y + i64::from(dy);
    RectI32 { min: PointI32 { x: min_x as i32, y: min_y as i32 }, max: PointI32 { x: max_x as i32, y: max_y as i32 } }
}

#[cfg(test)]
mod tests {
    use super::{assign_translate, translate};
    use crate::cartesian::{point::point_i32::PointI32, rect::rect_i32::RectI32};

    #[test]
    fn test_assign_translate() {
        let mut r = RectI32::of(0, 0, 10, 10);
        assign_translate(&mut r, &PointI32::of(10, 20));
        assert_eq!(r, RectI32::of(10, 20, 20, 30));
        assign_translate(&mut r, &PointI32::of(-20, -15));
        assert_eq!(r, RectI32::of(-10, 5, 0, 15));
    }

    #[test]
    fn assign_translate_small_rect_to_bounds() {
        let mut min_r = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MIN + 12, i32::MIN + 15);
        assign_translate(&mut min_r, &PointI32::of(-2, -5));
        assert_eq!(min_r, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 10, i32::MIN + 10));

        let mut max_r = RectI32::of(i32::MAX - 12, i32::MAX - 15, i32::MAX - 2, i32::MAX - 5);
        assign_translate(&mut max_r, &PointI32::of(2, 5));
        assert_eq!(max_r, RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX, i32::MAX));
    }

    #[test]
    fn assign_translate_big_rect_to_bounds() {
        let mut min_r = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX);
        assign_translate(&mut min_r, &PointI32::of(-2, -5));
        assert_eq!(min_r, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5));

        let mut max_r = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5);
        assign_translate(&mut max_r, &PointI32::of(2, 5));
        assert_eq!(max_r, RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX));
    }

    #[test]
    fn assign_translate_small_rect_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MIN + 20, i32::MIN + 30);
        assign_translate(&mut r_min, &PointI32::of(-20, -20));
        assert_eq!(r_min, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 10, i32::MIN + 25));

        let mut r_max = RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX - 5, i32::MAX - 10);
        assign_translate(&mut r_max, &PointI32::of(20, 20));
        assert_eq!(r_max, RectI32::of(i32::MAX - 15, i32::MAX - 20, i32::MAX, i32::MAX));
    }

    #[test]
    fn assign_translate_big_rect_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MAX, i32::MAX);
        assign_translate(&mut r_min, &PointI32::of(-20, -20));
        assert_eq!(r_min, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 10, i32::MAX - 5));

        let mut r_max = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 5, i32::MAX - 10);
        assign_translate(&mut r_max, &PointI32::of(20, 20));
        assert_eq!(r_max, RectI32::of(i32::MIN + 5, i32::MIN + 10, i32::MAX, i32::MAX));
    }

    #[test]
    fn assign_translate_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10);
        assign_translate(&mut r_min, &PointI32::min());
        assert_eq!(r_min, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 9, i32::MIN + 9));

        let mut r_max = RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1);
        assign_translate(&mut r_max, &PointI32::max());
        assert_eq!(r_max, RectI32::of(i32::MAX - 9, i32::MAX - 9, i32::MAX, i32::MAX));
    }

    #[test]
    fn assign_translate_big_rect_limits_out_of_bounds() {
        let mut r_min = RectI32::largest();
        assign_translate(&mut r_min, &PointI32::min());
        assert_eq!(r_min, RectI32::largest());
        assign_translate(&mut r_min, &PointI32::max());
        assert_eq!(r_min, RectI32::largest());

        let mut r_min = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX);
        assign_translate(&mut r_min, &PointI32::min());
        assert_eq!(r_min, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1));

        let mut r_max = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1);
        assign_translate(&mut r_max, &PointI32::max());
        assert_eq!(r_max, RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX));
    }

    #[test]
    fn test_translate() {
        assert_eq!(translate(&RectI32::of(0, 0, 10, 10), &PointI32::of(10, 20)), RectI32::of(10, 20, 20, 30));
        assert_eq!(translate(&RectI32::of(10, 20, 20, 30), &PointI32::of(-20, -15)), RectI32::of(-10, 5, 0, 15));
    }

    #[test]
    fn translate_small_rect_to_bounds() {
        assert_eq!(
            translate(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MIN + 12, i32::MIN + 15), &PointI32::of(-2, -5)),
            RectI32::of(i32::MIN, i32::MIN, i32::MIN + 10, i32::MIN + 10)
        );
        assert_eq!(
            translate(&RectI32::of(i32::MAX - 12, i32::MAX - 15, i32::MAX - 2, i32::MAX - 5), &PointI32::of(2, 5)),
            RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX, i32::MAX)
        );
    }

    #[test]
    fn translate_big_rect_to_bounds() {
        assert_eq!(
            translate(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX), &PointI32::of(-2, -5)),
            RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5)
        );
        assert_eq!(
            translate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5), &PointI32::of(2, 5)),
            RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX)
        );
    }

    #[test]
    fn translate_small_rect_out_of_bounds() {
        assert_eq!(
            translate(&RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MIN + 20, i32::MIN + 30), &PointI32::of(-20, -20)),
            RectI32::of(i32::MIN, i32::MIN, i32::MIN + 10, i32::MIN + 25)
        );
        assert_eq!(
            translate(&RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX - 5, i32::MAX - 10), &PointI32::of(20, 20)),
            RectI32::of(i32::MAX - 15, i32::MAX - 20, i32::MAX, i32::MAX)
        );
    }

    #[test]
    fn translate_big_rect_out_of_bounds() {
        assert_eq!(
            translate(&RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MAX, i32::MAX), &PointI32::of(-20, -20)),
            RectI32::of(i32::MIN, i32::MIN, i32::MAX - 10, i32::MAX - 5)
        );
        assert_eq!(
            translate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 5, i32::MAX - 10), &PointI32::of(20, 20)),
            RectI32::of(i32::MIN + 5, i32::MIN + 10, i32::MAX, i32::MAX)
        );
    }

    #[test]
    fn translate_small_rect_limits_out_of_bounds() {
        assert_eq!(
            translate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10), &PointI32::min()),
            RectI32::of(i32::MIN, i32::MIN, i32::MIN + 9, i32::MIN + 9)
        );
        assert_eq!(
            translate(&RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1), &PointI32::max()),
            RectI32::of(i32::MAX - 9, i32::MAX - 9, i32::MAX, i32::MAX)
        );
    }

    #[test]
    fn translate_big_rect_limits_out_of_bounds() {
        assert_eq!(translate(&RectI32::largest(), &PointI32::min()), RectI32::largest());
        assert_eq!(translate(&RectI32::largest(), &PointI32::max()), RectI32::largest());
        assert_eq!(
            translate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX), &PointI32::min()),
            RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1)
        );
        assert_eq!(
            translate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1), &PointI32::max()),
            RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX)
        );
    }
}
