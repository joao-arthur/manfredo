use crate::cartesian::{
    point::point_u32::PointU32,
    rect::{rect_i32::RectI32, rect_u32::RectU32},
};

pub fn assign_add(r: &mut RectU32, delta: &RectI32) {
    let min_x = r.min.x.wrapping_add_signed(delta.min.x);
    let min_y = r.min.y.wrapping_add_signed(delta.min.y);
    let max_x = r.max.x.wrapping_add_signed(delta.max.x);
    let max_y = r.max.y.wrapping_add_signed(delta.max.y);
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
}

pub fn add(r: &RectU32, delta: &RectI32) -> RectU32 {
    let min_x = r.min.x.wrapping_add_signed(delta.min.x);
    let min_y = r.min.y.wrapping_add_signed(delta.min.y);
    let max_x = r.max.x.wrapping_add_signed(delta.max.x);
    let max_y = r.max.y.wrapping_add_signed(delta.max.y);
    RectU32 { min: PointU32 { x: min_x, y: min_y }, max: PointU32 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::{rect_i32::RectI32, rect_u32::RectU32};

    use super::{add, assign_add};

    #[test]
    fn test_assign_add() {
        let mut r = RectU32::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI32::of(5, 4, 3, 2));
        assert_eq!(r, RectU32::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI32::of(-4, -3, -2, -1));
        assert_eq!(r, RectU32::of(1, 1, 13, 16));
    }

    #[test]
    fn assign_add_small_rect_to_bounds() {
        let mut min_r = RectU32::of(2, 5, 12, 15);
        assign_add(&mut min_r, &RectI32::of(-2, -5, 9, 7));
        assert_eq!(min_r, RectU32::of(0, 0, 21, 22));

        let mut max_r = RectU32::of(u32::MAX - 12, u32::MAX - 15, u32::MAX - 2, u32::MAX - 5);
        assign_add(&mut max_r, &RectI32::of(-9, -7, 2, 5));
        assert_eq!(max_r, RectU32::of(u32::MAX - 21, u32::MAX - 22, u32::MAX, u32::MAX));
    }

    #[test]
    fn assign_add_big_rect_to_bounds() {
        let mut min_r = RectU32::of(2, 5, u32::MAX - 2, u32::MAX - 5);
        assign_add(&mut min_r, &RectI32::of(-2, -5, 2, 5));
        assert_eq!(min_r, RectU32::largest());

        let mut min_r = RectU32::of(2, 5, u32::MAX, u32::MAX);
        assign_add(&mut min_r, &RectI32::of(-2, -5, 0, 0));
        assert_eq!(min_r, RectU32::largest());

        let mut max_r = RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5);
        assign_add(&mut max_r, &RectI32::of(0, 0, 2, 5));
        assert_eq!(max_r, RectU32::largest());
    }

    #[test]
    fn assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectU32::of(10, 5, 20, 30);
        assign_add(&mut r_min, &RectI32::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectU32::of(u32::MAX - 9, u32::MAX - 14, 20, 30));

        let mut r_max = RectU32::of(u32::MAX - 20, u32::MAX - 30, u32::MAX - 5, u32::MAX - 10);
        assign_add(&mut r_max, &RectI32::of(0, 0, 20, 20));
        assert_eq!(r_max, RectU32::of(u32::MAX - 20, u32::MAX - 30, 14, 9));
    }

    #[test]
    fn assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectU32::of(10, 5, u32::MAX, u32::MAX);
        assign_add(&mut r_min, &RectI32::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectU32::of(u32::MAX - 9, u32::MAX - 14, u32::MAX, u32::MAX));

        let mut r_max = RectU32::of(0, 0, u32::MAX - 5, u32::MAX - 10);
        assign_add(&mut r_max, &RectI32::of(0, 0, 20, 20));
        assert_eq!(r_max, RectU32::of(0, 0, 14, 9));
    }

    #[test]
    fn assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU32::of(1, 1, 10, 10);
        assign_add(&mut r_min, &RectI32::min());
        assert_eq!(r_min, RectU32::of((i32::MAX as u32) + 2, (i32::MAX as u32) + 2, (i32::MAX as u32) + 11, (i32::MAX as u32) + 11));

        let mut r_max = RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1);
        assign_add(&mut r_max, &RectI32::max());
        assert_eq!(r_max, RectU32::of((i32::MAX as u32) - 11, (i32::MAX as u32) - 11, (i32::MAX as u32) - 2, (i32::MAX as u32) - 2));
    }

    #[test]
    fn assign_add_big_rect_limits_out_of_bounds() {
        let mut r1 = RectU32::largest();
        assign_add(&mut r1, &RectI32::min());
        assert_eq!(r1, RectU32::of((i32::MAX as u32) + 1, (i32::MAX as u32) + 1, i32::MAX as u32, i32::MAX as u32));

        let mut r2 = RectU32::largest();
        assign_add(&mut r2, &RectI32::max());
        assert_eq!(r2, RectU32::of(i32::MAX as u32, i32::MAX as u32, (i32::MAX as u32) - 1, (i32::MAX as u32) - 1));

        let mut r_min = RectU32::of(1, 1, u32::MAX, u32::MAX);
        assign_add(&mut r_min, &RectI32::min());
        assert_eq!(r_min, RectU32::of((i32::MAX as u32) + 2, (i32::MAX as u32) + 2, i32::MAX as u32, i32::MAX as u32));

        let mut r_max = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
        assign_add(&mut r_max, &RectI32::max());
        assert_eq!(r_max, RectU32::of(i32::MAX as u32, i32::MAX as u32, (i32::MAX as u32) - 2, (i32::MAX as u32) - 2));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&RectU32::of(0, 0, 12, 15), &RectI32::of(5, 4, 3, 2)), RectU32::of(5, 4, 15, 17));
        assert_eq!(add(&RectU32::of(5, 4, 15, 17), &RectI32::of(-4, -3, -2, -1)), RectU32::of(1, 1, 13, 16));
    }

    #[test]
    fn add_small_rect_to_bounds() {
        assert_eq!(add(&RectU32::of(2, 5, 12, 15), &RectI32::of(-2, -5, 9, 7)), RectU32::of(0, 0, 21, 22));
        assert_eq!(
            add(&RectU32::of(u32::MAX - 12, u32::MAX - 15, u32::MAX - 2, u32::MAX - 5), &RectI32::of(-9, -7, 2, 5)),
            RectU32::of(u32::MAX - 21, u32::MAX - 22, u32::MAX, u32::MAX)
        );
    }

    #[test]
    fn add_big_rect_to_bounds() {
        assert_eq!(add(&RectU32::of(2, 5, u32::MAX - 2, u32::MAX - 5), &RectI32::of(-2, -5, 2, 5)), RectU32::largest());
        assert_eq!(add(&RectU32::of(2, 5, u32::MAX, u32::MAX), &RectI32::of(-2, -5, 0, 0)), RectU32::largest());
        assert_eq!(add(&RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5), &RectI32::of(0, 0, 2, 5)), RectU32::largest());
    }

    #[test]
    fn add_small_rect_out_of_bounds() {
        assert_eq!(add(&RectU32::of(10, 5, 20, 30), &RectI32::of(-20, -20, 0, 0)), RectU32::of(u32::MAX - 9, u32::MAX - 14, 20, 30));
        assert_eq!(
            add(&RectU32::of(u32::MAX - 20, u32::MAX - 30, u32::MAX - 5, u32::MAX - 10), &RectI32::of(0, 0, 20, 20)),
            RectU32::of(u32::MAX - 20, u32::MAX - 30, 14, 9)
        );
    }

    #[test]
    fn add_big_rect_out_of_bounds() {
        assert_eq!(add(&RectU32::of(10, 5, u32::MAX, u32::MAX), &RectI32::of(-20, -20, 0, 0)), RectU32::of(u32::MAX - 9, u32::MAX - 14, u32::MAX, u32::MAX));
        assert_eq!(add(&RectU32::of(0, 0, u32::MAX - 5, u32::MAX - 10), &RectI32::of(0, 0, 20, 20)), RectU32::of(0, 0, 14, 9));
    }

    #[test]
    fn add_small_rect_limits_out_of_bounds() {
        assert_eq!(
            add(&RectU32::of(1, 1, 10, 10), &RectI32::min()),
            RectU32::of((i32::MAX as u32) + 2, (i32::MAX as u32) + 2, (i32::MAX as u32) + 11, (i32::MAX as u32) + 11)
        );
        assert_eq!(
            add(&RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1), &RectI32::max()),
            RectU32::of((i32::MAX as u32) - 11, (i32::MAX as u32) - 11, (i32::MAX as u32) - 2, (i32::MAX as u32) - 2)
        );
    }

    #[test]
    fn add_big_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectU32::largest(), &RectI32::min()), RectU32::of((i32::MAX as u32) + 1, (i32::MAX as u32) + 1, i32::MAX as u32, i32::MAX as u32));
        assert_eq!(add(&RectU32::largest(), &RectI32::max()), RectU32::of(i32::MAX as u32, i32::MAX as u32, (i32::MAX as u32) - 1, (i32::MAX as u32) - 1));
        assert_eq!(
            add(&RectU32::of(1, 1, u32::MAX, u32::MAX), &RectI32::min()),
            RectU32::of((i32::MAX as u32) + 2, (i32::MAX as u32) + 2, i32::MAX as u32, i32::MAX as u32)
        );
        assert_eq!(
            add(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1), &RectI32::max()),
            RectU32::of(i32::MAX as u32, i32::MAX as u32, (i32::MAX as u32) - 2, (i32::MAX as u32) - 2)
        );
    }
}
