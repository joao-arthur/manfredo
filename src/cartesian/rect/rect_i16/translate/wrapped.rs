use crate::cartesian::{
    point::point_i16::PointI16,
    rect::rect_i16::{RectI16, delta_x, delta_y},
};

pub fn wrapping_translate_assign(r: &mut RectI16, delta: &PointI16) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let min_x = r.min.x.wrapping_add(delta.x);
    let min_y = r.min.y.wrapping_add(delta.y);
    let max_x = min_x.wrapping_add_unsigned(dx);
    let max_y = min_y.wrapping_add_unsigned(dy);
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
}

pub fn wrapping_translate(r: &RectI16, delta: &PointI16) -> RectI16 {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let min_x = r.min.x.wrapping_add(delta.x);
    let min_y = r.min.y.wrapping_add(delta.y);
    let max_x = min_x.wrapping_add_unsigned(dx);
    let max_y = min_y.wrapping_add_unsigned(dy);
    RectI16 { min: PointI16 { x: min_x, y: min_y }, max: PointI16 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use super::{wrapping_translate_assign, wrapping_translate};
    use crate::cartesian::{point::point_i16::PointI16, rect::rect_i16::RectI16};

    #[test]
    fn test_wrapping_translate_assign() {
        let mut r = RectI16::of(0, 0, 12, 15);
        wrapping_translate_assign(&mut r, &PointI16::of(5, 4));
        assert_eq!(r, RectI16::of(5, 4, 17, 19));
        wrapping_translate_assign(&mut r, &PointI16::of(-4, -2));
        assert_eq!(r, RectI16::of(1, 2, 13, 17));
    }

    #[test]
    fn wrapping_translate_assign_small_rect_to_bounds() {
        let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MIN + 12, i16::MIN + 15);
        wrapping_translate_assign(&mut min_r, &PointI16::of(-2, -5));
        assert_eq!(min_r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 10));

        let mut max_r = RectI16::of(i16::MAX - 12, i16::MAX - 15, i16::MAX - 2, i16::MAX - 5);
        wrapping_translate_assign(&mut max_r, &PointI16::of(2, 5));
        assert_eq!(max_r, RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX, i16::MAX));
    }

    #[test]
    fn wrapping_translate_assign_big_rect_to_bounds() {
        let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX);
        wrapping_translate_assign(&mut min_r, &PointI16::of(-2, -5));
        assert_eq!(min_r, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5));

        let mut max_r = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5);
        wrapping_translate_assign(&mut max_r, &PointI16::of(2, 5));
        assert_eq!(max_r, RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX));
    }

    #[test]
    fn wrapping_translate_assign_small_rect_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30);
        wrapping_translate_assign(&mut r_min, &PointI16::of(-20, -20));
        assert_eq!(r_min, RectI16::of(i16::MAX - 9, i16::MAX - 14, i16::MIN, i16::MIN + 10));

        let mut r_max = RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10);
        wrapping_translate_assign(&mut r_max, &PointI16::of(20, 20));
        assert_eq!(r_max, RectI16::of(i16::MAX, i16::MAX - 10, i16::MIN + 14, i16::MIN + 9));
    }

    #[test]
    fn wrapping_translate_assign_big_rect_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX);
        wrapping_translate_assign(&mut r_min, &PointI16::of(-20, -20));
        assert_eq!(r_min, RectI16::of(i16::MAX - 9, i16::MAX - 14, i16::MAX - 20, i16::MAX - 20));

        let mut r_max = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10);
        wrapping_translate_assign(&mut r_max, &PointI16::of(20, 20));
        assert_eq!(r_max, RectI16::of(i16::MIN + 20, i16::MIN + 20, i16::MIN + 14, i16::MIN + 9));
    }

    #[test]
    fn wrapping_translate_assign_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10);
        wrapping_translate_assign(&mut r_min, &PointI16::min());
        assert_eq!(r_min, RectI16::of(1, 1, 10, 10));

        let mut r_max = RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1);
        wrapping_translate_assign(&mut r_max, &PointI16::max());
        assert_eq!(r_max, RectI16::of(-12, -12, -3, -3));
    }

    #[test]
    fn wrapping_translate_assign_big_rect_limits_out_of_bounds() {
        let mut r1 = RectI16::largest();
        wrapping_translate_assign(&mut r1, &PointI16::min());
        assert_eq!(r1, RectI16::of(0, 0, -1, -1));

        let mut r2 = RectI16::largest();
        wrapping_translate_assign(&mut r2, &PointI16::max());
        assert_eq!(r2, RectI16::of(-1, -1, -2, -2));

        let mut r_min = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX);
        wrapping_translate_assign(&mut r_min, &PointI16::min());
        assert_eq!(r_min, RectI16::of(1, 1, -1, -1));

        let mut r_max = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1);
        wrapping_translate_assign(&mut r_max, &PointI16::max());
        assert_eq!(r_max, RectI16::of(-1, -1, -3, -3));
    }

    #[test]
    fn test_wrapping_translate() {
        assert_eq!(wrapping_translate(&RectI16::of(0, 0, 12, 15), &PointI16::of(5, 4)), RectI16::of(5, 4, 17, 19));
        assert_eq!(wrapping_translate(&RectI16::of(5, 4, 17, 19), &PointI16::of(-4, -2)), RectI16::of(1, 2, 13, 17));
    }

    #[test]
    fn wrapping_translate_small_rect_to_bounds() {
        assert_eq!(
            wrapping_translate(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MIN + 12, i16::MIN + 15), &PointI16::of(-2, -5)),
            RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 10)
        );
        assert_eq!(
            wrapping_translate(&RectI16::of(i16::MAX - 12, i16::MAX - 15, i16::MAX - 2, i16::MAX - 5), &PointI16::of(2, 5)),
            RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX, i16::MAX)
        );
    }

    #[test]
    fn wrapping_translate_big_rect_to_bounds() {
        assert_eq!(
            wrapping_translate(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX), &PointI16::of(-2, -5)),
            RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5)
        );
        assert_eq!(
            wrapping_translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5), &PointI16::of(2, 5)),
            RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX)
        );
    }

    #[test]
    fn wrapping_translate_small_rect_out_of_bounds() {
        assert_eq!(
            wrapping_translate(&RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30), &PointI16::of(-20, -20)),
            RectI16::of(i16::MAX - 9, i16::MAX - 14, i16::MIN, i16::MIN + 10)
        );
        assert_eq!(
            wrapping_translate(&RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10), &PointI16::of(20, 20)),
            RectI16::of(i16::MAX, i16::MAX - 10, i16::MIN + 14, i16::MIN + 9)
        );
    }

    #[test]
    fn wrapping_translate_big_rect_out_of_bounds() {
        assert_eq!(
            wrapping_translate(&RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX), &PointI16::of(-20, -20)),
            RectI16::of(i16::MAX - 9, i16::MAX - 14, i16::MAX - 20, i16::MAX - 20)
        );
        assert_eq!(
            wrapping_translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10), &PointI16::of(20, 20)),
            RectI16::of(i16::MIN + 20, i16::MIN + 20, i16::MIN + 14, i16::MIN + 9)
        );
    }

    #[test]
    fn wrapping_translate_small_rect_limits_out_of_bounds() {
        assert_eq!(wrapping_translate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10), &PointI16::min()), RectI16::of(1, 1, 10, 10));
        assert_eq!(
            wrapping_translate(&RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1), &PointI16::max()),
            RectI16::of(-12, -12, -3, -3)
        );
    }

    #[test]
    fn wrapping_translate_big_rect_limits_out_of_bounds() {
        assert_eq!(wrapping_translate(&RectI16::largest(), &PointI16::min()), RectI16::of(0, 0, -1, -1));
        assert_eq!(wrapping_translate(&RectI16::largest(), &PointI16::max()), RectI16::of(-1, -1, -2, -2));
        assert_eq!(wrapping_translate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX), &PointI16::min()), RectI16::of(1, 1, -1, -1));
        assert_eq!(wrapping_translate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1), &PointI16::max()), RectI16::of(-1, -1, -3, -3));
    }
}
