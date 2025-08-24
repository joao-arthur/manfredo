use crate::cartesian::{
    point::point_i64::PointI64,
    rect::rect_i64::{RectI64, delta_x, delta_y},
};

pub fn saturating_translate_assign(r: &mut RectI64, delta: &PointI64) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i128::from(r.min.x) + i128::from(delta.x);
    let temp_min_y = i128::from(r.min.y) + i128::from(delta.y);
    let min_x = temp_min_x.clamp(i128::from(i64::MIN), i128::from(i64::MAX) - i128::from(dx));
    let min_y = temp_min_y.clamp(i128::from(i64::MIN), i128::from(i64::MAX) - i128::from(dy));
    r.min.x = min_x as i64;
    r.min.y = min_y as i64;
    r.max.x = (min_x + i128::from(dx)) as i64;
    r.max.y = (min_y + i128::from(dy)) as i64;
}

pub fn saturating_translate(r: &RectI64, delta: &PointI64) -> RectI64 {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i128::from(r.min.x) + i128::from(delta.x);
    let temp_min_y = i128::from(r.min.y) + i128::from(delta.y);
    let min_x = temp_min_x.clamp(i128::from(i64::MIN), i128::from(i64::MAX) - i128::from(dx));
    let min_y = temp_min_y.clamp(i128::from(i64::MIN), i128::from(i64::MAX) - i128::from(dy));
    let max_x = min_x + i128::from(dx);
    let max_y = min_y + i128::from(dy);
    RectI64 { min: PointI64 { x: min_x as i64, y: min_y as i64 }, max: PointI64 { x: max_x as i64, y: max_y as i64 } }
}

#[cfg(test)]
mod tests {
    use super::{saturating_translate_assign, saturating_translate};
    use crate::cartesian::{point::point_i64::PointI64, rect::rect_i64::RectI64};

    #[test]
    fn test_saturating_translate_assign() {
        let mut r = RectI64::of(0, 0, 10, 10);
        saturating_translate_assign(&mut r, &PointI64::of(10, 20));
        assert_eq!(r, RectI64::of(10, 20, 20, 30));
        saturating_translate_assign(&mut r, &PointI64::of(-20, -15));
        assert_eq!(r, RectI64::of(-10, 5, 0, 15));
    }

    #[test]
    fn saturating_translate_assign_small_rect_to_bounds() {
        let mut min_r = RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MIN + 12, i64::MIN + 15);
        saturating_translate_assign(&mut min_r, &PointI64::of(-2, -5));
        assert_eq!(min_r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10));

        let mut max_r = RectI64::of(i64::MAX - 12, i64::MAX - 15, i64::MAX - 2, i64::MAX - 5);
        saturating_translate_assign(&mut max_r, &PointI64::of(2, 5));
        assert_eq!(max_r, RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX, i64::MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_to_bounds() {
        let mut min_r = RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX);
        saturating_translate_assign(&mut min_r, &PointI64::of(-2, -5));
        assert_eq!(min_r, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5));

        let mut max_r = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5);
        saturating_translate_assign(&mut max_r, &PointI64::of(2, 5));
        assert_eq!(max_r, RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX));
    }

    #[test]
    fn saturating_translate_assign_small_rect_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MIN + 20, i64::MIN + 30);
        saturating_translate_assign(&mut r_min, &PointI64::of(-20, -20));
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 25));

        let mut r_max = RectI64::of(i64::MAX - 20, i64::MAX - 30, i64::MAX - 5, i64::MAX - 10);
        saturating_translate_assign(&mut r_max, &PointI64::of(20, 20));
        assert_eq!(r_max, RectI64::of(i64::MAX - 15, i64::MAX - 20, i64::MAX, i64::MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MAX, i64::MAX);
        saturating_translate_assign(&mut r_min, &PointI64::of(-20, -20));
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 10, i64::MAX - 5));

        let mut r_max = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 5, i64::MAX - 10);
        saturating_translate_assign(&mut r_max, &PointI64::of(20, 20));
        assert_eq!(r_max, RectI64::of(i64::MIN + 5, i64::MIN + 10, i64::MAX, i64::MAX));
    }

    #[test]
    fn saturating_translate_assign_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MIN + 10, i64::MIN + 10);
        saturating_translate_assign(&mut r_min, &PointI64::min());
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 9, i64::MIN + 9));

        let mut r_max = RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX - 1, i64::MAX - 1);
        saturating_translate_assign(&mut r_max, &PointI64::max());
        assert_eq!(r_max, RectI64::of(i64::MAX - 9, i64::MAX - 9, i64::MAX, i64::MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_limits_out_of_bounds() {
        let mut r_min = RectI64::largest();
        saturating_translate_assign(&mut r_min, &PointI64::min());
        assert_eq!(r_min, RectI64::largest());
        saturating_translate_assign(&mut r_min, &PointI64::max());
        assert_eq!(r_min, RectI64::largest());

        let mut r_min = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX);
        saturating_translate_assign(&mut r_min, &PointI64::min());
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1));

        let mut r_max = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1);
        saturating_translate_assign(&mut r_max, &PointI64::max());
        assert_eq!(r_max, RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX));
    }

    #[test]
    fn test_saturating_translate() {
        assert_eq!(saturating_translate(&RectI64::of(0, 0, 10, 10), &PointI64::of(10, 20)), RectI64::of(10, 20, 20, 30));
        assert_eq!(saturating_translate(&RectI64::of(10, 20, 20, 30), &PointI64::of(-20, -15)), RectI64::of(-10, 5, 0, 15));
    }

    #[test]
    fn saturating_translate_small_rect_to_bounds() {
        assert_eq!(
            saturating_translate(&RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MIN + 12, i64::MIN + 15), &PointI64::of(-2, -5)),
            RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10)
        );
        assert_eq!(
            saturating_translate(&RectI64::of(i64::MAX - 12, i64::MAX - 15, i64::MAX - 2, i64::MAX - 5), &PointI64::of(2, 5)),
            RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX, i64::MAX)
        );
    }

    #[test]
    fn saturating_translate_big_rect_to_bounds() {
        assert_eq!(
            saturating_translate(&RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX), &PointI64::of(-2, -5)),
            RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5)
        );
        assert_eq!(
            saturating_translate(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5), &PointI64::of(2, 5)),
            RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX)
        );
    }

    #[test]
    fn saturating_translate_small_rect_out_of_bounds() {
        assert_eq!(
            saturating_translate(&RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MIN + 20, i64::MIN + 30), &PointI64::of(-20, -20)),
            RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 25)
        );
        assert_eq!(
            saturating_translate(&RectI64::of(i64::MAX - 20, i64::MAX - 30, i64::MAX - 5, i64::MAX - 10), &PointI64::of(20, 20)),
            RectI64::of(i64::MAX - 15, i64::MAX - 20, i64::MAX, i64::MAX)
        );
    }

    #[test]
    fn saturating_translate_big_rect_out_of_bounds() {
        assert_eq!(
            saturating_translate(&RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MAX, i64::MAX), &PointI64::of(-20, -20)),
            RectI64::of(i64::MIN, i64::MIN, i64::MAX - 10, i64::MAX - 5)
        );
        assert_eq!(
            saturating_translate(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 5, i64::MAX - 10), &PointI64::of(20, 20)),
            RectI64::of(i64::MIN + 5, i64::MIN + 10, i64::MAX, i64::MAX)
        );
    }

    #[test]
    fn saturating_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(
            saturating_translate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MIN + 10, i64::MIN + 10), &PointI64::min()),
            RectI64::of(i64::MIN, i64::MIN, i64::MIN + 9, i64::MIN + 9)
        );
        assert_eq!(
            saturating_translate(&RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX - 1, i64::MAX - 1), &PointI64::max()),
            RectI64::of(i64::MAX - 9, i64::MAX - 9, i64::MAX, i64::MAX)
        );
    }

    #[test]
    fn saturating_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(saturating_translate(&RectI64::largest(), &PointI64::min()), RectI64::largest());
        assert_eq!(saturating_translate(&RectI64::largest(), &PointI64::max()), RectI64::largest());
        assert_eq!(
            saturating_translate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX), &PointI64::min()),
            RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1)
        );
        assert_eq!(
            saturating_translate(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1), &PointI64::max()),
            RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX)
        );
    }
}
