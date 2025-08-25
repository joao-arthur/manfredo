use crate::cartesian::{
    point::{point_i64::PointI64, point_u64::PointU64},
    rect::rect_u64::{RectU64, delta_x, delta_y},
};

pub fn saturating_translate_assign(r: &mut RectU64, delta: &PointI64) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i128::from(r.min.x) + i128::from(delta.x);
    let temp_min_y = i128::from(r.min.y) + i128::from(delta.y);
    let clamped_x = temp_min_x.clamp(0, i128::from(u64::MAX) - i128::from(dx));
    let clamped_y = temp_min_y.clamp(0, i128::from(u64::MAX) - i128::from(dy));
    let min_x = clamped_x as u64;
    let min_y = clamped_y as u64;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = min_x + dx;
    r.max.y = min_y + dy;
}

pub fn saturating_translate(r: &RectU64, delta: &PointI64) -> RectU64 {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i128::from(r.min.x) + i128::from(delta.x);
    let temp_min_y = i128::from(r.min.y) + i128::from(delta.y);
    let clamped_x = temp_min_x.clamp(0, i128::from(u64::MAX) - i128::from(dx));
    let clamped_y = temp_min_y.clamp(0, i128::from(u64::MAX) - i128::from(dy));
    let min_x = clamped_x as u64;
    let min_y = clamped_y as u64;
    let max_x = min_x + dx;
    let max_y = min_y + dy;
    RectU64 { min: PointU64 { x: min_x, y: min_y }, max: PointU64 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use super::{saturating_translate, saturating_translate_assign};
    use crate::cartesian::{point::point_i64::PointI64, rect::rect_u64::RectU64};

    #[test]
    fn test_saturating_translate_assign() {
        let mut r = RectU64::of(0, 0, 12, 15);
        saturating_translate_assign(&mut r, &PointI64::of(5, 4));
        assert_eq!(r, RectU64::of(5, 4, 17, 19));
        saturating_translate_assign(&mut r, &PointI64::of(-4, -2));
        assert_eq!(r, RectU64::of(1, 2, 13, 17));
    }

    #[test]
    fn saturating_translate_assign_small_rect_to_bounds() {
        let mut min_r = RectU64::of(2, 5, 12, 15);
        saturating_translate_assign(&mut min_r, &PointI64::of(-2, -5));
        assert_eq!(min_r, RectU64::of(0, 0, 10, 10));

        let mut max_r = RectU64::of(u64::MAX - 12, u64::MAX - 15, u64::MAX - 2, u64::MAX - 5);
        saturating_translate_assign(&mut max_r, &PointI64::of(2, 5));
        assert_eq!(max_r, RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_to_bounds() {
        let mut min_r = RectU64::of(2, 5, u64::MAX, u64::MAX);
        saturating_translate_assign(&mut min_r, &PointI64::of(-2, -5));
        assert_eq!(min_r, RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5));

        let mut max_r = RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5);
        saturating_translate_assign(&mut max_r, &PointI64::of(2, 5));
        assert_eq!(max_r, RectU64::of(2, 5, u64::MAX, u64::MAX));
    }

    #[test]
    fn saturating_translate_assign_small_rect_out_of_bounds() {
        let mut r_min = RectU64::of(10, 5, 20, 30);
        saturating_translate_assign(&mut r_min, &PointI64::of(-20, -20));
        assert_eq!(r_min, RectU64::of(0, 0, 10, 25));

        let mut r_max = RectU64::of(u64::MAX - 20, u64::MAX - 30, u64::MAX - 5, u64::MAX - 10);
        saturating_translate_assign(&mut r_max, &PointI64::of(20, 20));
        assert_eq!(r_max, RectU64::of(u64::MAX - 15, u64::MAX - 20, u64::MAX, u64::MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_out_of_bounds() {
        let mut r_min = RectU64::of(10, 5, u64::MAX, u64::MAX);
        saturating_translate_assign(&mut r_min, &PointI64::of(-20, -20));
        assert_eq!(r_min, RectU64::of(0, 0, u64::MAX - 10, u64::MAX - 5));

        let mut r_max = RectU64::of(0, 0, u64::MAX - 5, u64::MAX - 10);
        saturating_translate_assign(&mut r_max, &PointI64::of(20, 20));
        assert_eq!(r_max, RectU64::of(5, 10, u64::MAX, u64::MAX));
    }

    #[test]
    fn saturating_translate_assign_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU64::of(1, 1, 10, 10);
        saturating_translate_assign(&mut r_min, &PointI64::min());
        assert_eq!(r_min, RectU64::of(0, 0, 9, 9));

        let mut r_max = RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1);
        saturating_translate_assign(&mut r_max, &PointI64::max());
        assert_eq!(r_max, RectU64::of(u64::MAX - 9, u64::MAX - 9, u64::MAX, u64::MAX));
    }

    #[test]
    fn saturating_translate_assign_big_rect_limits_out_of_bounds() {
        let mut r = RectU64::largest();
        saturating_translate_assign(&mut r, &PointI64::min());
        assert_eq!(r, RectU64::largest());
        saturating_translate_assign(&mut r, &PointI64::max());
        assert_eq!(r, RectU64::largest());

        let mut r_min = RectU64::of(1, 1, u64::MAX, u64::MAX);
        saturating_translate_assign(&mut r_min, &PointI64::min());
        assert_eq!(r_min, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));

        let mut r_max = RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1);
        saturating_translate_assign(&mut r_max, &PointI64::max());
        assert_eq!(r_max, RectU64::of(1, 1, u64::MAX, u64::MAX));
    }

    #[test]
    fn test_saturating_translate() {
        assert_eq!(saturating_translate(&RectU64::of(0, 0, 12, 15), &PointI64::of(5, 4)), RectU64::of(5, 4, 17, 19));
        assert_eq!(saturating_translate(&RectU64::of(5, 4, 17, 19), &PointI64::of(-4, -2)), RectU64::of(1, 2, 13, 17));
    }

    #[test]
    fn saturating_translate_small_rect_to_bounds() {
        assert_eq!(saturating_translate(&RectU64::of(2, 5, 12, 15), &PointI64::of(-2, -5)), RectU64::of(0, 0, 10, 10));
        assert_eq!(
            saturating_translate(&RectU64::of(u64::MAX - 12, u64::MAX - 15, u64::MAX - 2, u64::MAX - 5), &PointI64::of(2, 5)),
            RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX)
        );
    }

    #[test]
    fn saturating_translate_big_rect_to_bounds() {
        assert_eq!(
            saturating_translate(&RectU64::of(2, 5, u64::MAX, u64::MAX), &PointI64::of(-2, -5)),
            RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5)
        );
        assert_eq!(saturating_translate(&RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5), &PointI64::of(2, 5)), RectU64::of(2, 5, u64::MAX, u64::MAX));
    }

    #[test]
    fn saturating_translate_small_rect_out_of_bounds() {
        assert_eq!(saturating_translate(&RectU64::of(10, 5, 20, 30), &PointI64::of(-20, -20)), RectU64::of(0, 0, 10, 25));
        assert_eq!(
            saturating_translate(&RectU64::of(u64::MAX - 20, u64::MAX - 30, u64::MAX - 5, u64::MAX - 10), &PointI64::of(20, 20)),
            RectU64::of(u64::MAX - 15, u64::MAX - 20, u64::MAX, u64::MAX)
        );
    }

    #[test]
    fn saturating_translate_big_rect_out_of_bounds() {
        assert_eq!(
            saturating_translate(&RectU64::of(10, 5, u64::MAX, u64::MAX), &PointI64::of(-20, -20)),
            RectU64::of(0, 0, u64::MAX - 10, u64::MAX - 5)
        );
        assert_eq!(
            saturating_translate(&RectU64::of(0, 0, u64::MAX - 5, u64::MAX - 10), &PointI64::of(20, 20)),
            RectU64::of(5, 10, u64::MAX, u64::MAX)
        );
    }

    #[test]
    fn saturating_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(saturating_translate(&RectU64::of(1, 1, 10, 10), &PointI64::min()), RectU64::of(0, 0, 9, 9));
        assert_eq!(
            saturating_translate(&RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1), &PointI64::max()),
            RectU64::of(u64::MAX - 9, u64::MAX - 9, u64::MAX, u64::MAX)
        );
    }

    #[test]
    fn saturating_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(saturating_translate(&RectU64::of(1, 1, u64::MAX, u64::MAX), &PointI64::min()), RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));
        assert_eq!(saturating_translate(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1), &PointI64::max()), RectU64::of(1, 1, u64::MAX, u64::MAX));
    }
}
