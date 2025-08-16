use crate::cartesian::{
    point::{point_i8::PointI8, point_u8::PointU8},
    rect::rect_u8::{RectU8, delta_x, delta_y},
};

pub fn assign_add(r: &mut RectU8, delta: &PointI8) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let min_x = r.min.x.wrapping_add(delta.x as u8);
    let min_y = r.min.y.wrapping_add(delta.y as u8);
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = min_x.wrapping_add(dx);
    r.max.y = min_y.wrapping_add(dy);
}

pub fn add(r: &RectU8, delta: &PointI8) -> RectU8 {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let min_x = r.min.x.wrapping_add(delta.x as u8);
    let min_y = r.min.y.wrapping_add(delta.y as u8);
    let max_x = min_x.wrapping_add(dx);
    let max_y = min_y.wrapping_add(dy);
    RectU8 { min: PointU8 { x: min_x, y: min_y }, max: PointU8 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{point::point_i8::PointI8, rect::rect_u8::RectU8};

    use super::{add, assign_add};

    #[test]
    fn test_assign_add() {
        let mut r = RectU8::of(0, 0, 12, 15);
        assign_add(&mut r, &PointI8::of(5, 4));
        assert_eq!(r, RectU8::of(5, 4, 17, 19));
        assign_add(&mut r, &PointI8::of(-4, -2));
        assert_eq!(r, RectU8::of(1, 2, 13, 17));
    }

    #[test]
    fn assign_add_small_rect_to_bounds() {
        let mut min_r = RectU8::of(2, 5, 12, 15);
        assign_add(&mut min_r, &PointI8::of(-2, -5));
        assert_eq!(min_r, RectU8::of(0, 0, 10, 10));

        let mut max_r = RectU8::of(u8::MAX - 12, u8::MAX - 15, u8::MAX - 2, u8::MAX - 5);
        assign_add(&mut max_r, &PointI8::of(2, 5));
        assert_eq!(max_r, RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX, u8::MAX));
    }

    #[test]
    fn assign_add_big_rect_to_bounds() {
        let mut min_r = RectU8::of(2, 5, u8::MAX, u8::MAX);
        assign_add(&mut min_r, &PointI8::of(-2, -5));
        assert_eq!(min_r, RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5));

        let mut max_r = RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5);
        assign_add(&mut max_r, &PointI8::of(2, 5));
        assert_eq!(max_r, RectU8::of(2, 5, u8::MAX, u8::MAX));
    }

    #[test]
    fn assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectU8::of(10, 5, 20, 30);
        assign_add(&mut r_min, &PointI8::of(-20, -20));
        assert_eq!(r_min, RectU8::of(u8::MAX - 9, u8::MAX - 14, 0, 10));

        let mut r_max = RectU8::of(u8::MAX - 20, u8::MAX - 30, u8::MAX - 5, u8::MAX - 10);
        assign_add(&mut r_max, &PointI8::of(20, 20));
        assert_eq!(r_max, RectU8::of(u8::MAX, u8::MAX - 10, 14, 9));
    }

    #[test]
    fn assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectU8::of(10, 5, u8::MAX, u8::MAX);
        assign_add(&mut r_min, &PointI8::of(-20, -20));
        assert_eq!(r_min, RectU8::of(u8::MAX - 9, u8::MAX - 14, u8::MAX - 20, u8::MAX - 20));

        let mut r_max = RectU8::of(0, 0, u8::MAX - 5, u8::MAX - 10);
        assign_add(&mut r_max, &PointI8::of(20, 20));
        assert_eq!(r_max, RectU8::of(20, 20, 14, 9));
    }

    #[test]
    fn assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU8::of(1, 1, 10, 10);
        assign_add(&mut r_min, &PointI8::min());
        assert_eq!(r_min, RectU8::of(129, 129, 138, 138));

        let mut r_max = RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX - 1, u8::MAX - 1);
        assign_add(&mut r_max, &PointI8::max());
        assert_eq!(r_max, RectU8::of(116, 116, 125, 125));
    }

    #[test]
    fn assign_add_big_rect_limits_out_of_bounds() {
        let mut r_min = RectU8::of(1, 1, u8::MAX, u8::MAX);
        assign_add(&mut r_min, &PointI8::min());
        assert_eq!(r_min, RectU8::of(129, 129, 127, 127));

        let mut r_max = RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1);
        assign_add(&mut r_max, &PointI8::max());
        assert_eq!(r_max, RectU8::of(127, 127, 125, 125));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&RectU8::of(0, 0, 12, 15), &PointI8::of(5, 4)), RectU8::of(5, 4, 17, 19));
        assert_eq!(add(&RectU8::of(5, 4, 17, 19), &PointI8::of(-4, -2)), RectU8::of(1, 2, 13, 17));
    }

    #[test]
    fn add_small_rect_to_bounds() {
        assert_eq!(add(&RectU8::of(2, 5, 12, 15), &PointI8::of(-2, -5)), RectU8::of(0, 0, 10, 10));
        assert_eq!(
            add(&RectU8::of(u8::MAX - 12, u8::MAX - 15, u8::MAX - 2, u8::MAX - 5), &PointI8::of(2, 5)),
            RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX, u8::MAX)
        );
    }

    #[test]
    fn add_big_rect_to_bounds() {
        assert_eq!(add(&RectU8::of(2, 5, u8::MAX, u8::MAX), &PointI8::of(-2, -5)), RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5));
        assert_eq!(add(&RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5), &PointI8::of(2, 5)), RectU8::of(2, 5, u8::MAX, u8::MAX));
    }

    #[test]
    fn add_small_rect_out_of_bounds() {
        assert_eq!(add(&RectU8::of(10, 5, 20, 30), &PointI8::of(-20, -20)), RectU8::of(u8::MAX - 9, u8::MAX - 14, 0, 10));
        assert_eq!(
            add(&RectU8::of(u8::MAX - 20, u8::MAX - 30, u8::MAX - 5, u8::MAX - 10), &PointI8::of(20, 20)),
            RectU8::of(u8::MAX, u8::MAX - 10, 14, 9)
        );
    }

    #[test]
    fn add_big_rect_out_of_bounds() {
        assert_eq!(
            add(&RectU8::of(10, 5, u8::MAX, u8::MAX), &PointI8::of(-20, -20)),
            RectU8::of(u8::MAX - 9, u8::MAX - 14, u8::MAX - 20, u8::MAX - 20)
        );
        assert_eq!(add(&RectU8::of(0, 0, u8::MAX - 5, u8::MAX - 10), &PointI8::of(20, 20)), RectU8::of(20, 20, 14, 9));
    }

    #[test]
    fn add_small_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectU8::of(1, 1, 10, 10), &PointI8::min()), RectU8::of(129, 129, 138, 138));
        assert_eq!(add(&RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX - 1, u8::MAX - 1), &PointI8::max()), RectU8::of(116, 116, 125, 125));
    }

    #[test]
    fn add_big_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectU8::of(1, 1, u8::MAX, u8::MAX), &PointI8::min()), RectU8::of(129, 129, 127, 127));
        assert_eq!(add(&RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1), &PointI8::max()), RectU8::of(127, 127, 125, 125));
    }
}
