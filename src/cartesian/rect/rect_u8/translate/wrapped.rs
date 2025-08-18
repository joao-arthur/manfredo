use crate::cartesian::{
    point::{point_i8::PointI8, point_u8::PointU8},
    rect::rect_u8::{RectU8, delta_x, delta_y},
};

pub fn assign_translate(r: &mut RectU8, delta: &PointI8) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let min_x = r.min.x.wrapping_add_signed(delta.x);
    let min_y = r.min.y.wrapping_add_signed(delta.y);
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = min_x.wrapping_add(dx);
    r.max.y = min_y.wrapping_add(dy);
}

pub fn translate(r: &RectU8, delta: &PointI8) -> RectU8 {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let min_x = r.min.x.wrapping_add_signed(delta.x);
    let min_y = r.min.y.wrapping_add_signed(delta.y);
    let max_x = min_x.wrapping_add(dx);
    let max_y = min_y.wrapping_add(dy);
    RectU8 { min: PointU8 { x: min_x, y: min_y }, max: PointU8 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{point::point_i8::PointI8, rect::rect_u8::RectU8};

    use super::{assign_translate, translate};

    #[test]
    fn test_assign_translate() {
        let mut r = RectU8::of(0, 0, 12, 15);
        assign_translate(&mut r, &PointI8::of(5, 4));
        assert_eq!(r, RectU8::of(5, 4, 17, 19));
        assign_translate(&mut r, &PointI8::of(-4, -2));
        assert_eq!(r, RectU8::of(1, 2, 13, 17));
    }

    #[test]
    fn assign_translate_small_rect_to_bounds() {
        let mut min_r = RectU8::of(2, 5, 12, 15);
        assign_translate(&mut min_r, &PointI8::of(-2, -5));
        assert_eq!(min_r, RectU8::of(0, 0, 10, 10));

        let mut max_r = RectU8::of(u8::MAX - 12, u8::MAX - 15, u8::MAX - 2, u8::MAX - 5);
        assign_translate(&mut max_r, &PointI8::of(2, 5));
        assert_eq!(max_r, RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX, u8::MAX));
    }

    #[test]
    fn assign_translate_big_rect_to_bounds() {
        let mut min_r = RectU8::of(2, 5, u8::MAX, u8::MAX);
        assign_translate(&mut min_r, &PointI8::of(-2, -5));
        assert_eq!(min_r, RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5));

        let mut max_r = RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5);
        assign_translate(&mut max_r, &PointI8::of(2, 5));
        assert_eq!(max_r, RectU8::of(2, 5, u8::MAX, u8::MAX));
    }

    #[test]
    fn assign_translate_small_rect_out_of_bounds() {
        let mut r_min = RectU8::of(10, 5, 20, 30);
        assign_translate(&mut r_min, &PointI8::of(-20, -20));
        assert_eq!(r_min, RectU8::of(u8::MAX - 9, u8::MAX - 14, 0, 10));

        let mut r_max = RectU8::of(u8::MAX - 20, u8::MAX - 30, u8::MAX - 5, u8::MAX - 10);
        assign_translate(&mut r_max, &PointI8::of(20, 20));
        assert_eq!(r_max, RectU8::of(u8::MAX, u8::MAX - 10, 14, 9));
    }

    #[test]
    fn assign_translate_big_rect_out_of_bounds() {
        let mut r_min = RectU8::of(10, 5, u8::MAX, u8::MAX);
        assign_translate(&mut r_min, &PointI8::of(-20, -20));
        assert_eq!(r_min, RectU8::of(u8::MAX - 9, u8::MAX - 14, u8::MAX - 20, u8::MAX - 20));

        let mut r_max = RectU8::of(0, 0, u8::MAX - 5, u8::MAX - 10);
        assign_translate(&mut r_max, &PointI8::of(20, 20));
        assert_eq!(r_max, RectU8::of(20, 20, 14, 9));
    }

    #[test]
    fn assign_translate_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU8::of(1, 1, 10, 10);
        assign_translate(&mut r_min, &PointI8::min());
        assert_eq!(r_min, RectU8::of((i8::MAX as u8) + 2, (i8::MAX as u8) + 2, (i8::MAX as u8) + 11, (i8::MAX as u8) + 11));

        let mut r_max = RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX - 1, u8::MAX - 1);
        assign_translate(&mut r_max, &PointI8::max());
        assert_eq!(r_max, RectU8::of((i8::MAX as u8) - 11, (i8::MAX as u8) - 11, (i8::MAX as u8) - 2, (i8::MAX as u8) - 2));
    }

    #[test]
    fn assign_translate_big_rect_limits_out_of_bounds() {
        let mut r1 = RectU8::largest();
        assign_translate(&mut r1, &PointI8::min());
        assert_eq!(r1, RectU8::of((i8::MAX as u8) + 1, (i8::MAX as u8) + 1, i8::MAX as u8, i8::MAX as u8));

        let mut r2 = RectU8::largest();
        assign_translate(&mut r2, &PointI8::max());
        assert_eq!(r2, RectU8::of(i8::MAX as u8, i8::MAX as u8, (i8::MAX as u8) - 1, (i8::MAX as u8) - 1));

        let mut r_min = RectU8::of(1, 1, u8::MAX, u8::MAX);
        assign_translate(&mut r_min, &PointI8::min());
        assert_eq!(r_min, RectU8::of((i8::MAX as u8) + 2, (i8::MAX as u8) + 2, i8::MAX as u8, i8::MAX as u8));

        let mut r_max = RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1);
        assign_translate(&mut r_max, &PointI8::max());
        assert_eq!(r_max, RectU8::of(i8::MAX as u8, i8::MAX as u8, (i8::MAX as u8) - 2, (i8::MAX as u8) - 2));
    }

    #[test]
    fn test_translate() {
        assert_eq!(translate(&RectU8::of(0, 0, 12, 15), &PointI8::of(5, 4)), RectU8::of(5, 4, 17, 19));
        assert_eq!(translate(&RectU8::of(5, 4, 17, 19), &PointI8::of(-4, -2)), RectU8::of(1, 2, 13, 17));
    }

    #[test]
    fn translate_small_rect_to_bounds() {
        assert_eq!(translate(&RectU8::of(2, 5, 12, 15), &PointI8::of(-2, -5)), RectU8::of(0, 0, 10, 10));
        assert_eq!(
            translate(&RectU8::of(u8::MAX - 12, u8::MAX - 15, u8::MAX - 2, u8::MAX - 5), &PointI8::of(2, 5)),
            RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX, u8::MAX)
        );
    }

    #[test]
    fn translate_big_rect_to_bounds() {
        assert_eq!(translate(&RectU8::of(2, 5, u8::MAX, u8::MAX), &PointI8::of(-2, -5)), RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5));
        assert_eq!(translate(&RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5), &PointI8::of(2, 5)), RectU8::of(2, 5, u8::MAX, u8::MAX));
    }

    #[test]
    fn translate_small_rect_out_of_bounds() {
        assert_eq!(translate(&RectU8::of(10, 5, 20, 30), &PointI8::of(-20, -20)), RectU8::of(u8::MAX - 9, u8::MAX - 14, 0, 10));
        assert_eq!(
            translate(&RectU8::of(u8::MAX - 20, u8::MAX - 30, u8::MAX - 5, u8::MAX - 10), &PointI8::of(20, 20)),
            RectU8::of(u8::MAX, u8::MAX - 10, 14, 9)
        );
    }

    #[test]
    fn translate_big_rect_out_of_bounds() {
        assert_eq!(
            translate(&RectU8::of(10, 5, u8::MAX, u8::MAX), &PointI8::of(-20, -20)),
            RectU8::of(u8::MAX - 9, u8::MAX - 14, u8::MAX - 20, u8::MAX - 20)
        );
        assert_eq!(translate(&RectU8::of(0, 0, u8::MAX - 5, u8::MAX - 10), &PointI8::of(20, 20)), RectU8::of(20, 20, 14, 9));
    }

    #[test]
    fn translate_small_rect_limits_out_of_bounds() {
        assert_eq!(
            translate(&RectU8::of(1, 1, 10, 10), &PointI8::min()),
            RectU8::of((i8::MAX as u8) + 2, (i8::MAX as u8) + 2, (i8::MAX as u8) + 11, (i8::MAX as u8) + 11)
        );
        assert_eq!(
            translate(&RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX - 1, u8::MAX - 1), &PointI8::max()),
            RectU8::of((i8::MAX as u8) - 11, (i8::MAX as u8) - 11, (i8::MAX as u8) - 2, (i8::MAX as u8) - 2)
        );
    }

    #[test]
    fn translate_big_rect_limits_out_of_bounds() {
        assert_eq!(
            translate(&RectU8::largest(), &PointI8::min()),
            RectU8::of((i8::MAX as u8) + 1, (i8::MAX as u8) + 1, i8::MAX as u8, i8::MAX as u8)
        );
        assert_eq!(
            translate(&RectU8::largest(), &PointI8::max()),
            RectU8::of(i8::MAX as u8, i8::MAX as u8, (i8::MAX as u8) - 1, (i8::MAX as u8) - 1)
        );
        assert_eq!(
            translate(&RectU8::of(1, 1, u8::MAX, u8::MAX), &PointI8::min()),
            RectU8::of((i8::MAX as u8) + 2, (i8::MAX as u8) + 2, i8::MAX as u8, i8::MAX as u8)
        );
        assert_eq!(
            translate(&RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1), &PointI8::max()),
            RectU8::of(i8::MAX as u8, i8::MAX as u8, (i8::MAX as u8) - 2, (i8::MAX as u8) - 2)
        );
    }
}
