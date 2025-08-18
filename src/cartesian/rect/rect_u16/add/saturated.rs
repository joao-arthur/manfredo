use crate::cartesian::{
    point::point_u16::PointU16,
    rect::{rect_i16::RectI16, rect_u16::RectU16},
};

pub fn assign_add(r: &mut RectU16, delta: &RectI16) {
    let min_x = r.min.x.saturating_add_signed(delta.min.x);
    let min_y = r.min.y.saturating_add_signed(delta.min.y);
    let max_x = r.max.x.saturating_add_signed(delta.max.x);
    let max_y = r.max.y.saturating_add_signed(delta.max.y);
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
}

pub fn add(r: &RectU16, delta: &RectI16) -> RectU16 {
    let min_x = r.min.x.saturating_add_signed(delta.min.x);
    let min_y = r.min.y.saturating_add_signed(delta.min.y);
    let max_x = r.max.x.saturating_add_signed(delta.max.x);
    let max_y = r.max.y.saturating_add_signed(delta.max.y);
    RectU16 { min: PointU16 { x: min_x, y: min_y }, max: PointU16 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::{rect_i16::RectI16, rect_u16::RectU16};

    use super::{add, assign_add};

    #[test]
    fn test_assign_add() {
        let mut r = RectU16::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI16::of(5, 4, 3, 2));
        assert_eq!(r, RectU16::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI16::of(-4, -3, -2, -1));
        assert_eq!(r, RectU16::of(1, 1, 13, 16));
    }

    #[test]
    fn assign_add_small_rect_to_bounds() {
        let mut min_r = RectU16::of(2, 5, 12, 15);
        assign_add(&mut min_r, &RectI16::of(-2, -5, 9, 7));
        assert_eq!(min_r, RectU16::of(0, 0, 21, 22));

        let mut max_r = RectU16::of(u16::MAX - 12, u16::MAX - 15, u16::MAX - 2, u16::MAX - 5);
        assign_add(&mut max_r, &RectI16::of(-9, -7, 2, 5));
        assert_eq!(max_r, RectU16::of(u16::MAX - 21, u16::MAX - 22, u16::MAX, u16::MAX));
    }

    #[test]
    fn assign_add_big_rect_to_bounds() {
        let mut r = RectU16::of(2, 5, u16::MAX - 2, u16::MAX - 5);
        assign_add(&mut r, &RectI16::of(-2, -5, 2, 5));
        assert_eq!(r, RectU16::largest());

        let mut min_r = RectU16::of(2, 5, u16::MAX, u16::MAX);
        assign_add(&mut min_r, &RectI16::of(-2, -5, 0, 0));
        assert_eq!(min_r, RectU16::largest());

        let mut max_r = RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5);
        assign_add(&mut max_r, &RectI16::of(0, 0, 2, 5));
        assert_eq!(max_r, RectU16::largest());
    }

    #[test]
    fn assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectU16::of(10, 5, 20, 30);
        assign_add(&mut r_min, &RectI16::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectU16::of(0, 0, 20, 30));

        let mut r_max = RectU16::of(u16::MAX - 20, u16::MAX - 30, u16::MAX - 5, u16::MAX - 10);
        assign_add(&mut r_max, &RectI16::of(0, 0, 20, 20));
        assert_eq!(r_max, RectU16::of(u16::MAX - 20, u16::MAX - 30, u16::MAX, u16::MAX));
    }

    #[test]
    fn assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectU16::of(10, 5, u16::MAX, u16::MAX);
        assign_add(&mut r_min, &RectI16::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectU16::largest());

        let mut r_max = RectU16::of(0, 0, u16::MAX - 5, u16::MAX - 10);
        assign_add(&mut r_max, &RectI16::of(0, 0, 20, 20));
        assert_eq!(r_max, RectU16::largest());
    }

    #[test]
    fn assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU16::of(1, 1, 10, 10);
        assign_add(&mut r_min, &RectI16::min());
        assert_eq!(r_min, RectU16::min());

        let mut r_max = RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX - 1, u16::MAX - 1);
        assign_add(&mut r_max, &RectI16::max());
        assert_eq!(r_max, RectU16::max());
    }

    #[test]
    fn assign_add_big_rect_limits_out_of_bounds() {
        let mut r = RectU16::largest();
        assign_add(&mut r, &RectI16::largest());
        assert_eq!(r, RectU16::largest());

        let mut r_large = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assign_add(&mut r_large, &RectI16::largest());
        assert_eq!(r_large, RectU16::largest());

        let mut r_min_x = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assign_add(&mut r_min_x, &RectI16::of(i16::MIN, 0, 0, 0));
        assert_eq!(r_min_x, RectU16::of(0, 1, u16::MAX - 1, u16::MAX - 1));

        let mut r_min_y = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assign_add(&mut r_min_y, &RectI16::of(0, i16::MIN, 0, 0));
        assert_eq!(r_min_y, RectU16::of(1, 0, u16::MAX - 1, u16::MAX - 1));

        let mut r_max_x = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assign_add(&mut r_max_x, &RectI16::of(0, 0, i16::MAX, 0));
        assert_eq!(r_max_x, RectU16::of(1, 1, u16::MAX, u16::MAX - 1));

        let mut r_max_y = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assign_add(&mut r_max_y, &RectI16::of(0, 0, 0, i16::MAX));
        assert_eq!(r_max_y, RectU16::of(1, 1, u16::MAX - 1, u16::MAX));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&RectU16::of(0, 0, 12, 15), &RectI16::of(5, 4, 3, 2)), RectU16::of(5, 4, 15, 17));
        assert_eq!(add(&RectU16::of(5, 4, 15, 17), &RectI16::of(-4, -3, -2, -1)), RectU16::of(1, 1, 13, 16));
    }

    #[test]
    fn add_small_rect_to_bounds() {
        assert_eq!(add(&RectU16::of(2, 5, 12, 15), &RectI16::of(-2, -5, 9, 7)), RectU16::of(0, 0, 21, 22));
        assert_eq!(
            add(&RectU16::of(u16::MAX - 12, u16::MAX - 15, u16::MAX - 2, u16::MAX - 5), &RectI16::of(-9, -7, 2, 5)),
            RectU16::of(u16::MAX - 21, u16::MAX - 22, u16::MAX, u16::MAX)
        );
    }

    #[test]
    fn add_big_rect_to_bounds() {
        assert_eq!(add(&RectU16::of(2, 5, u16::MAX - 2, u16::MAX - 5), &RectI16::of(-2, -5, 2, 5)), RectU16::largest());
        assert_eq!(add(&RectU16::of(2, 5, u16::MAX, u16::MAX), &RectI16::of(-2, -5, 0, 0)), RectU16::largest());
        assert_eq!(add(&RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5), &RectI16::of(0, 0, 2, 5)), RectU16::largest());
    }

    #[test]
    fn add_small_rect_out_of_bounds() {
        assert_eq!(add(&RectU16::of(10, 5, 20, 30), &RectI16::of(-20, -20, 0, 0)), RectU16::of(0, 0, 20, 30));
        assert_eq!(
            add(&RectU16::of(u16::MAX - 20, u16::MAX - 30, u16::MAX - 5, u16::MAX - 10), &RectI16::of(0, 0, 20, 20)),
            RectU16::of(u16::MAX - 20, u16::MAX - 30, u16::MAX, u16::MAX)
        );
    }

    #[test]
    fn add_big_rect_out_of_bounds() {
        assert_eq!(add(&RectU16::of(10, 5, u16::MAX, u16::MAX), &RectI16::of(-20, -20, 0, 0)), RectU16::largest());
        assert_eq!(add(&RectU16::of(0, 0, u16::MAX - 5, u16::MAX - 10), &RectI16::of(0, 0, 20, 20)), RectU16::largest());
    }

    #[test]
    fn add_small_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectU16::of(1, 1, 10, 10), &RectI16::min()), RectU16::min());
        assert_eq!(add(&RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX - 1, u16::MAX - 1), &RectI16::max()), RectU16::max());
    }

    #[test]
    fn add_big_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectU16::largest(), &RectI16::largest()), RectU16::largest());

        let r_large = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assert_eq!(add(&r_large, &RectI16::largest()), RectU16::largest());
        assert_eq!(add(&r_large, &RectI16::of(i16::MIN, 0, 0, 0)), RectU16::of(0, 1, u16::MAX - 1, u16::MAX - 1));
        assert_eq!(add(&r_large, &RectI16::of(0, i16::MIN, 0, 0)), RectU16::of(1, 0, u16::MAX - 1, u16::MAX - 1));
        assert_eq!(add(&r_large, &RectI16::of(0, 0, i16::MAX, 0)), RectU16::of(1, 1, u16::MAX, u16::MAX - 1));
        assert_eq!(add(&r_large, &RectI16::of(0, 0, 0, i16::MAX)), RectU16::of(1, 1, u16::MAX - 1, u16::MAX));
    }
}
