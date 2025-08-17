use crate::cartesian::{
    point::{point_i64::PointI64, point_u64::PointU64},
    rect::rect_u64::{RectU64, delta_x, delta_y},
};

pub fn assign_add(r: &mut RectU64, delta: &PointI64) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let min_x = r.min.x.wrapping_add(delta.x as u64);
    let min_y = r.min.y.wrapping_add(delta.y as u64);
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = min_x.wrapping_add(dx);
    r.max.y = min_y.wrapping_add(dy);
}

pub fn add(r: &RectU64, delta: &PointI64) -> RectU64 {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let min_x = r.min.x.wrapping_add(delta.x as u64);
    let min_y = r.min.y.wrapping_add(delta.y as u64);
    let max_x = min_x.wrapping_add(dx);
    let max_y = min_y.wrapping_add(dy);
    RectU64 { min: PointU64 { x: min_x, y: min_y }, max: PointU64 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{point::point_i64::PointI64, rect::rect_u64::RectU64};

    use super::{add, assign_add};

    #[test]
    fn test_assign_add() {
        let mut r = RectU64::of(0, 0, 12, 15);
        assign_add(&mut r, &PointI64::of(5, 4));
        assert_eq!(r, RectU64::of(5, 4, 17, 19));
        assign_add(&mut r, &PointI64::of(-4, -2));
        assert_eq!(r, RectU64::of(1, 2, 13, 17));
    }

    #[test]
    fn assign_add_small_rect_to_bounds() {
        let mut min_r = RectU64::of(2, 5, 12, 15);
        assign_add(&mut min_r, &PointI64::of(-2, -5));
        assert_eq!(min_r, RectU64::of(0, 0, 10, 10));

        let mut max_r = RectU64::of(u64::MAX - 12, u64::MAX - 15, u64::MAX - 2, u64::MAX - 5);
        assign_add(&mut max_r, &PointI64::of(2, 5));
        assert_eq!(max_r, RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX));
    }

    #[test]
    fn assign_add_big_rect_to_bounds() {
        let mut min_r = RectU64::of(2, 5, u64::MAX, u64::MAX);
        assign_add(&mut min_r, &PointI64::of(-2, -5));
        assert_eq!(min_r, RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5));

        let mut max_r = RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5);
        assign_add(&mut max_r, &PointI64::of(2, 5));
        assert_eq!(max_r, RectU64::of(2, 5, u64::MAX, u64::MAX));
    }

    #[test]
    fn assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectU64::of(10, 5, 20, 30);
        assign_add(&mut r_min, &PointI64::of(-20, -20));
        assert_eq!(r_min, RectU64::of(u64::MAX - 9, u64::MAX - 14, 0, 10));

        let mut r_max = RectU64::of(u64::MAX - 20, u64::MAX - 30, u64::MAX - 5, u64::MAX - 10);
        assign_add(&mut r_max, &PointI64::of(20, 20));
        assert_eq!(r_max, RectU64::of(u64::MAX, u64::MAX - 10, 14, 9));
    }

    #[test]
    fn assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectU64::of(10, 5, u64::MAX, u64::MAX);
        assign_add(&mut r_min, &PointI64::of(-20, -20));
        assert_eq!(r_min, RectU64::of(u64::MAX - 9, u64::MAX - 14, u64::MAX - 20, u64::MAX - 20));

        let mut r_max = RectU64::of(0, 0, u64::MAX - 5, u64::MAX - 10);
        assign_add(&mut r_max, &PointI64::of(20, 20));
        assert_eq!(r_max, RectU64::of(20, 20, 14, 9));
    }

    #[test]
    fn assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU64::of(1, 1, 10, 10);
        assign_add(&mut r_min, &PointI64::min());
        assert_eq!(r_min, RectU64::of((i64::MAX as u64) + 2, (i64::MAX as u64) + 2, (i64::MAX as u64) + 11, (i64::MAX as u64) + 11));

        let mut r_max = RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1);
        assign_add(&mut r_max, &PointI64::max());
        assert_eq!(r_max, RectU64::of((i64::MAX as u64) - 11, (i64::MAX as u64) - 11, (i64::MAX as u64) - 2, (i64::MAX as u64) - 2));
    }

    #[test]
    fn assign_add_big_rect_limits_out_of_bounds() {
        let mut r1 = RectU64::largest();
        assign_add(&mut r1, &PointI64::min());
        assert_eq!(r1, RectU64::of((i64::MAX as u64) + 1, (i64::MAX as u64) + 1, i64::MAX as u64, i64::MAX as u64));

        let mut r2 = RectU64::largest();
        assign_add(&mut r2, &PointI64::max());
        assert_eq!(r2, RectU64::of(i64::MAX as u64, i64::MAX as u64, (i64::MAX as u64) - 1, (i64::MAX as u64) - 1));

        let mut r_min = RectU64::of(1, 1, u64::MAX, u64::MAX);
        assign_add(&mut r_min, &PointI64::min());
        assert_eq!(r_min, RectU64::of((i64::MAX as u64) + 2, (i64::MAX as u64) + 2, i64::MAX as u64, i64::MAX as u64));

        let mut r_max = RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1);
        assign_add(&mut r_max, &PointI64::max());
        assert_eq!(r_max, RectU64::of(i64::MAX as u64, i64::MAX as u64, (i64::MAX as u64) - 2, (i64::MAX as u64) - 2));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&RectU64::of(0, 0, 12, 15), &PointI64::of(5, 4)), RectU64::of(5, 4, 17, 19));
        assert_eq!(add(&RectU64::of(5, 4, 17, 19), &PointI64::of(-4, -2)), RectU64::of(1, 2, 13, 17));
    }

    #[test]
    fn add_small_rect_to_bounds() {
        assert_eq!(add(&RectU64::of(2, 5, 12, 15), &PointI64::of(-2, -5)), RectU64::of(0, 0, 10, 10));
        assert_eq!(
            add(&RectU64::of(u64::MAX - 12, u64::MAX - 15, u64::MAX - 2, u64::MAX - 5), &PointI64::of(2, 5)),
            RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX)
        );
    }

    #[test]
    fn add_big_rect_to_bounds() {
        assert_eq!(add(&RectU64::of(2, 5, u64::MAX, u64::MAX), &PointI64::of(-2, -5)), RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5));
        assert_eq!(add(&RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5), &PointI64::of(2, 5)), RectU64::of(2, 5, u64::MAX, u64::MAX));
    }

    #[test]
    fn add_small_rect_out_of_bounds() {
        assert_eq!(add(&RectU64::of(10, 5, 20, 30), &PointI64::of(-20, -20)), RectU64::of(u64::MAX - 9, u64::MAX - 14, 0, 10));
        assert_eq!(
            add(&RectU64::of(u64::MAX - 20, u64::MAX - 30, u64::MAX - 5, u64::MAX - 10), &PointI64::of(20, 20)),
            RectU64::of(u64::MAX, u64::MAX - 10, 14, 9)
        );
    }

    #[test]
    fn add_big_rect_out_of_bounds() {
        assert_eq!(
            add(&RectU64::of(10, 5, u64::MAX, u64::MAX), &PointI64::of(-20, -20)),
            RectU64::of(u64::MAX - 9, u64::MAX - 14, u64::MAX - 20, u64::MAX - 20)
        );
        assert_eq!(add(&RectU64::of(0, 0, u64::MAX - 5, u64::MAX - 10), &PointI64::of(20, 20)), RectU64::of(20, 20, 14, 9));
    }

    #[test]
    fn add_small_rect_limits_out_of_bounds() {
        assert_eq!(
            add(&RectU64::of(1, 1, 10, 10), &PointI64::min()),
            RectU64::of((i64::MAX as u64) + 2, (i64::MAX as u64) + 2, (i64::MAX as u64) + 11, (i64::MAX as u64) + 11)
        );
        assert_eq!(
            add(&RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1), &PointI64::max()),
            RectU64::of((i64::MAX as u64) - 11, (i64::MAX as u64) - 11, (i64::MAX as u64) - 2, (i64::MAX as u64) - 2)
        );
    }

    #[test]
    fn add_big_rect_limits_out_of_bounds() {
        assert_eq!(
            add(&RectU64::largest(), &PointI64::min()),
            RectU64::of((i64::MAX as u64) + 1, (i64::MAX as u64) + 1, i64::MAX as u64, i64::MAX as u64)
        );
        assert_eq!(
            add(&RectU64::largest(), &PointI64::max()),
            RectU64::of(i64::MAX as u64, i64::MAX as u64, (i64::MAX as u64) - 1, (i64::MAX as u64) - 1)
        );
        assert_eq!(
            add(&RectU64::of(1, 1, u64::MAX, u64::MAX), &PointI64::min()),
            RectU64::of((i64::MAX as u64) + 2, (i64::MAX as u64) + 2, i64::MAX as u64, i64::MAX as u64)
        );
        assert_eq!(
            add(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1), &PointI64::max()),
            RectU64::of(i64::MAX as u64, i64::MAX as u64, (i64::MAX as u64) - 2, (i64::MAX as u64) - 2)
        );
    }
}
