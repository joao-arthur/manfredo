use crate::cartesian::{point::point_i64::PointI64, rect::rect_i64::RectI64};

pub fn assign_add(r: &mut RectI64, delta: &RectI64) {
    let min_x = r.min.x.wrapping_add(delta.min.x);
    let min_y = r.min.y.wrapping_add(delta.min.y);
    let max_x = r.max.x.wrapping_add(delta.max.x);
    let max_y = r.max.y.wrapping_add(delta.max.y);
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
}

#[cfg(test)]
mod tests {
    use super::assign_add;
    use crate::cartesian::rect::rect_i64::RectI64;

    #[test]
    fn test_assign_add() {
        let mut r = RectI64::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI64::of(5, 4, 3, 2));
        assert_eq!(r, RectI64::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI64::of(-14, -13, -12, -11));
        assert_eq!(r, RectI64::of(-9, -9, 3, 6));
    }

    #[test]
    fn assign_add_small_rect_to_bounds() {
        let mut min_r = RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MIN + 12, i64::MIN + 15);
        assign_add(&mut min_r, &RectI64::of(-2, -5, 9, 7));
        assert_eq!(min_r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 21, i64::MIN + 22));

        let mut max_r = RectI64::of(i64::MAX - 12, i64::MAX - 15, i64::MAX - 2, i64::MAX - 5);
        assign_add(&mut max_r, &RectI64::of(-9, -7, 2, 5));
        assert_eq!(max_r, RectI64::of(i64::MAX - 21, i64::MAX - 22, i64::MAX, i64::MAX));
    }

    #[test]
    fn assign_add_big_rect_to_bounds() {
        let mut min_r = RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX - 2, i64::MAX - 5);
        assign_add(&mut min_r, &RectI64::of(-2, -5, 2, 5));
        assert_eq!(min_r, RectI64::largest());

        let mut min_r = RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX);
        assign_add(&mut min_r, &RectI64::of(-2, -5, 0, 0));
        assert_eq!(min_r, RectI64::largest());

        let mut max_r = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5);
        assign_add(&mut max_r, &RectI64::of(0, 0, 2, 5));
        assert_eq!(max_r, RectI64::largest());
    }

    #[test]
    fn assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MIN + 20, i64::MIN + 30);
        assign_add(&mut r_min, &RectI64::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectI64::of(i64::MAX - 9, i64::MAX - 14, i64::MIN + 20, i64::MIN + 30));

        let mut r_max = RectI64::of(i64::MAX - 20, i64::MAX - 30, i64::MAX - 5, i64::MAX - 10);
        assign_add(&mut r_max, &RectI64::of(0, 0, 20, 20));
        assert_eq!(r_max, RectI64::of(i64::MAX - 20, i64::MAX - 30, i64::MIN + 14, i64::MIN + 9));
    }

    #[test]
    fn assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MAX, i64::MAX);
        assign_add(&mut r_min, &RectI64::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectI64::of(i64::MAX - 9, i64::MAX - 14, i64::MAX, i64::MAX));

        let mut r_max = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 5, i64::MAX - 10);
        assign_add(&mut r_max, &RectI64::of(0, 0, 20, 20));
        assert_eq!(r_max, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 14, i64::MIN + 9));
    }

    #[test]
    fn assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MIN + 10, i64::MIN + 10);
        assign_add(&mut r_min, &RectI64::min());
        assert_eq!(r_min, RectI64::of(1, 1, 10, 10));

        let mut r_max = RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX - 1, i64::MAX - 1);
        assign_add(&mut r_max, &RectI64::max());
        assert_eq!(r_max, RectI64::of(-12, -12, -3, -3));
    }

    #[test]
    fn assign_add_big_rect_limits_out_of_bounds() {
        let mut r1 = RectI64::largest();
        assign_add(&mut r1, &RectI64::min());
        assert_eq!(r1, RectI64::of(0, 0, -1, -1));

        let mut r2 = RectI64::largest();
        assign_add(&mut r2, &RectI64::max());
        assert_eq!(r2, RectI64::of(-1, -1, -2, -2));

        let mut r_min = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX);
        assign_add(&mut r_min, &RectI64::min());
        assert_eq!(r_min, RectI64::of(1, 1, -1, -1));

        let mut r_max = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1);
        assign_add(&mut r_max, &RectI64::max());
        assert_eq!(r_max, RectI64::of(-1, -1, -3, -3));
    }
}
