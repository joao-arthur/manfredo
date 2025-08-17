use crate::cartesian::{
    point::{point_i32::PointI32, point_u32::PointU32},
    rect::rect_u32::{RectU32, delta_x, delta_y},
};

pub fn assign_add(r: &mut RectU32, delta: &PointI32) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let min_x = r.min.x.wrapping_add(delta.x as u32);
    let min_y = r.min.y.wrapping_add(delta.y as u32);
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = min_x.wrapping_add(dx);
    r.max.y = min_y.wrapping_add(dy);
}

pub fn add(r: &RectU32, delta: &PointI32) -> RectU32 {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let min_x = r.min.x.wrapping_add(delta.x as u32);
    let min_y = r.min.y.wrapping_add(delta.y as u32);
    let max_x = min_x.wrapping_add(dx);
    let max_y = min_y.wrapping_add(dy);
    RectU32 { min: PointU32 { x: min_x, y: min_y }, max: PointU32 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{point::point_i32::PointI32, rect::rect_u32::RectU32};

    use super::{add, assign_add};

    #[test]
    fn test_assign_add() {
        let mut r = RectU32::of(0, 0, 12, 15);
        assign_add(&mut r, &PointI32::of(5, 4));
        assert_eq!(r, RectU32::of(5, 4, 17, 19));
        assign_add(&mut r, &PointI32::of(-4, -2));
        assert_eq!(r, RectU32::of(1, 2, 13, 17));
    }

    #[test]
    fn assign_add_small_rect_to_bounds() {
        let mut min_r = RectU32::of(2, 5, 12, 15);
        assign_add(&mut min_r, &PointI32::of(-2, -5));
        assert_eq!(min_r, RectU32::of(0, 0, 10, 10));

        let mut max_r = RectU32::of(u32::MAX - 12, u32::MAX - 15, u32::MAX - 2, u32::MAX - 5);
        assign_add(&mut max_r, &PointI32::of(2, 5));
        assert_eq!(max_r, RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX, u32::MAX));
    }

    #[test]
    fn assign_add_big_rect_to_bounds() {
        let mut min_r = RectU32::of(2, 5, u32::MAX, u32::MAX);
        assign_add(&mut min_r, &PointI32::of(-2, -5));
        assert_eq!(min_r, RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5));

        let mut max_r = RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5);
        assign_add(&mut max_r, &PointI32::of(2, 5));
        assert_eq!(max_r, RectU32::of(2, 5, u32::MAX, u32::MAX));
    }

    #[test]
    fn assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectU32::of(10, 5, 20, 30);
        assign_add(&mut r_min, &PointI32::of(-20, -20));
        assert_eq!(r_min, RectU32::of(u32::MAX - 9, u32::MAX - 14, 0, 10));

        let mut r_max = RectU32::of(u32::MAX - 20, u32::MAX - 30, u32::MAX - 5, u32::MAX - 10);
        assign_add(&mut r_max, &PointI32::of(20, 20));
        assert_eq!(r_max, RectU32::of(u32::MAX, u32::MAX - 10, 14, 9));
    }

    #[test]
    fn assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectU32::of(10, 5, u32::MAX, u32::MAX);
        assign_add(&mut r_min, &PointI32::of(-20, -20));
        assert_eq!(r_min, RectU32::of(u32::MAX - 9, u32::MAX - 14, u32::MAX - 20, u32::MAX - 20));

        let mut r_max = RectU32::of(0, 0, u32::MAX - 5, u32::MAX - 10);
        assign_add(&mut r_max, &PointI32::of(20, 20));
        assert_eq!(r_max, RectU32::of(20, 20, 14, 9));
    }

    #[test]
    fn assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU32::of(1, 1, 10, 10);
        assign_add(&mut r_min, &PointI32::min());
        assert_eq!(r_min, RectU32::of((i32::MAX as u32) + 2, (i32::MAX as u32) + 2, (i32::MAX as u32) + 11, (i32::MAX as u32) + 11));

        let mut r_max = RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1);
        assign_add(&mut r_max, &PointI32::max());
        assert_eq!(r_max, RectU32::of((i32::MAX as u32) - 11, (i32::MAX as u32) - 11, (i32::MAX as u32) - 2, (i32::MAX as u32) - 2));
    }

    #[test]
    fn assign_add_big_rect_limits_out_of_bounds() {
        let mut r1 = RectU32::largest();
        assign_add(&mut r1, &PointI32::min());
        assert_eq!(r1, RectU32::of((i32::MAX as u32) + 1, (i32::MAX as u32) + 1, i32::MAX as u32, i32::MAX as u32));

        let mut r2 = RectU32::largest();
        assign_add(&mut r2, &PointI32::max());
        assert_eq!(r2, RectU32::of(i32::MAX as u32, i32::MAX as u32, (i32::MAX as u32) - 1, (i32::MAX as u32) - 1));

        let mut r_min = RectU32::of(1, 1, u32::MAX, u32::MAX);
        assign_add(&mut r_min, &PointI32::min());
        assert_eq!(r_min, RectU32::of((i32::MAX as u32) + 2, (i32::MAX as u32) + 2, i32::MAX as u32, i32::MAX as u32));

        let mut r_max = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
        assign_add(&mut r_max, &PointI32::max());
        assert_eq!(r_max, RectU32::of(i32::MAX as u32, i32::MAX as u32, (i32::MAX as u32) - 2, (i32::MAX as u32) - 2));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&RectU32::of(0, 0, 12, 15), &PointI32::of(5, 4)), RectU32::of(5, 4, 17, 19));
        assert_eq!(add(&RectU32::of(5, 4, 17, 19), &PointI32::of(-4, -2)), RectU32::of(1, 2, 13, 17));
    }

    #[test]
    fn add_small_rect_to_bounds() {
        assert_eq!(add(&RectU32::of(2, 5, 12, 15), &PointI32::of(-2, -5)), RectU32::of(0, 0, 10, 10));
        assert_eq!(
            add(&RectU32::of(u32::MAX - 12, u32::MAX - 15, u32::MAX - 2, u32::MAX - 5), &PointI32::of(2, 5)),
            RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX, u32::MAX)
        );
    }

    #[test]
    fn add_big_rect_to_bounds() {
        assert_eq!(add(&RectU32::of(2, 5, u32::MAX, u32::MAX), &PointI32::of(-2, -5)), RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5));
        assert_eq!(add(&RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5), &PointI32::of(2, 5)), RectU32::of(2, 5, u32::MAX, u32::MAX));
    }

    #[test]
    fn add_small_rect_out_of_bounds() {
        assert_eq!(add(&RectU32::of(10, 5, 20, 30), &PointI32::of(-20, -20)), RectU32::of(u32::MAX - 9, u32::MAX - 14, 0, 10));
        assert_eq!(
            add(&RectU32::of(u32::MAX - 20, u32::MAX - 30, u32::MAX - 5, u32::MAX - 10), &PointI32::of(20, 20)),
            RectU32::of(u32::MAX, u32::MAX - 10, 14, 9)
        );
    }

    #[test]
    fn add_big_rect_out_of_bounds() {
        assert_eq!(
            add(&RectU32::of(10, 5, u32::MAX, u32::MAX), &PointI32::of(-20, -20)),
            RectU32::of(u32::MAX - 9, u32::MAX - 14, u32::MAX - 20, u32::MAX - 20)
        );
        assert_eq!(add(&RectU32::of(0, 0, u32::MAX - 5, u32::MAX - 10), &PointI32::of(20, 20)), RectU32::of(20, 20, 14, 9));
    }

    #[test]
    fn add_small_rect_limits_out_of_bounds() {
        assert_eq!(
            add(&RectU32::of(1, 1, 10, 10), &PointI32::min()),
            RectU32::of((i32::MAX as u32) + 2, (i32::MAX as u32) + 2, (i32::MAX as u32) + 11, (i32::MAX as u32) + 11)
        );
        assert_eq!(
            add(&RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1), &PointI32::max()),
            RectU32::of((i32::MAX as u32) - 11, (i32::MAX as u32) - 11, (i32::MAX as u32) - 2, (i32::MAX as u32) - 2)
        );
    }

    #[test]
    fn add_big_rect_limits_out_of_bounds() {
        assert_eq!(
            add(&RectU32::largest(), &PointI32::min()),
            RectU32::of((i32::MAX as u32) + 1, (i32::MAX as u32) + 1, i32::MAX as u32, i32::MAX as u32)
        );
        assert_eq!(
            add(&RectU32::largest(), &PointI32::max()),
            RectU32::of(i32::MAX as u32, i32::MAX as u32, (i32::MAX as u32) - 1, (i32::MAX as u32) - 1)
        );
        assert_eq!(
            add(&RectU32::of(1, 1, u32::MAX, u32::MAX), &PointI32::min()),
            RectU32::of((i32::MAX as u32) + 2, (i32::MAX as u32) + 2, i32::MAX as u32, i32::MAX as u32)
        );
        assert_eq!(
            add(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1), &PointI32::max()),
            RectU32::of(i32::MAX as u32, i32::MAX as u32, (i32::MAX as u32) - 2, (i32::MAX as u32) - 2)
        );
    }
}
