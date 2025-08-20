use crate::cartesian::{point::point_i16::PointI16, rect::rect_i16::RectI16};

pub fn assign_add(r: &mut RectI16, delta: &RectI16) {
    let min_x = r.min.x.saturating_add(delta.min.x);
    let min_y = r.min.y.saturating_add(delta.min.y);
    let max_x = r.max.x.saturating_add(delta.max.x);
    let max_y = r.max.y.saturating_add(delta.max.y);
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
}

pub fn add(r: &RectI16, delta: &RectI16) -> RectI16 {
    let min_x = r.min.x.saturating_add(delta.min.x);
    let min_y = r.min.y.saturating_add(delta.min.y);
    let max_x = r.max.x.saturating_add(delta.max.x);
    let max_y = r.max.y.saturating_add(delta.max.y);
    RectI16 { min: PointI16 { x: min_x, y: min_y }, max: PointI16 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_i16::RectI16;

    use super::{add, assign_add};

    #[test]
    fn test_assign_add() {
        let mut r = RectI16::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI16::of(5, 4, 3, 2));
        assert_eq!(r, RectI16::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI16::of(-14, -13, -12, -11));
        assert_eq!(r, RectI16::of(-9, -9, 3, 6));
    }

    #[test]
    fn assign_add_small_rect_to_bounds() {
        let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MIN + 12, i16::MIN + 15);
        assign_add(&mut min_r, &RectI16::of(-2, -5, 9, 7));
        assert_eq!(min_r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 21, i16::MIN + 22));

        let mut max_r = RectI16::of(i16::MAX - 12, i16::MAX - 15, i16::MAX - 2, i16::MAX - 5);
        assign_add(&mut max_r, &RectI16::of(-9, -7, 2, 5));
        assert_eq!(max_r, RectI16::of(i16::MAX - 21, i16::MAX - 22, i16::MAX, i16::MAX));
    }

    #[test]
    fn assign_add_big_rect_to_bounds() {
        let mut r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX - 2, i16::MAX - 5);
        assign_add(&mut r, &RectI16::of(-2, -5, 2, 5));
        assert_eq!(r, RectI16::largest());

        let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX);
        assign_add(&mut min_r, &RectI16::of(-2, -5, 0, 0));
        assert_eq!(min_r, RectI16::largest());

        let mut max_r = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5);
        assign_add(&mut max_r, &RectI16::of(0, 0, 2, 5));
        assert_eq!(max_r, RectI16::largest());
    }

    #[test]
    fn assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30);
        assign_add(&mut r_min, &RectI16::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 20, i16::MIN + 30));

        let mut r_max = RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10);
        assign_add(&mut r_max, &RectI16::of(0, 0, 20, 20));
        assert_eq!(r_max, RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX, i16::MAX));
    }

    #[test]
    fn assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX);
        assign_add(&mut r_min, &RectI16::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectI16::largest());

        let mut r_max = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10);
        assign_add(&mut r_max, &RectI16::of(0, 0, 20, 20));
        assert_eq!(r_max, RectI16::largest());
    }

    #[test]
    fn assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10);
        assign_add(&mut r_min, &RectI16::min());
        assert_eq!(r_min, RectI16::min());

        let mut r_max = RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1);
        assign_add(&mut r_max, &RectI16::max());
        assert_eq!(r_max, RectI16::max());
    }

    #[test]
    fn assign_add_big_rect_limits_out_of_bounds() {
        let mut r = RectI16::largest();
        assign_add(&mut r, &RectI16::largest());
        assert_eq!(r, RectI16::largest());

        let mut r_large = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assign_add(&mut r_large, &RectI16::largest());
        assert_eq!(r_large, RectI16::largest());

        let mut r_min_x = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assign_add(&mut r_min_x, &RectI16::of(i16::MIN, 0, 0, 0));
        assert_eq!(r_min_x, RectI16::of(i16::MIN, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1));

        let mut r_min_y = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assign_add(&mut r_min_y, &RectI16::of(0, i16::MIN, 0, 0));
        assert_eq!(r_min_y, RectI16::of(i16::MIN + 1, i16::MIN, i16::MAX - 1, i16::MAX - 1));

        let mut r_max_x = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assign_add(&mut r_max_x, &RectI16::of(0, 0, i16::MAX, 0));
        assert_eq!(r_max_x, RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX - 1));

        let mut r_max_y = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assign_add(&mut r_max_y, &RectI16::of(0, 0, 0, i16::MAX));
        assert_eq!(r_max_y, RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&RectI16::of(0, 0, 12, 15), &RectI16::of(5, 4, 3, 2)), RectI16::of(5, 4, 15, 17));
        assert_eq!(add(&RectI16::of(5, 4, 15, 17), &RectI16::of(-14, -13, -12, -11)), RectI16::of(-9, -9, 3, 6));
    }

    #[test]
    fn add_small_rect_to_bounds() {
        assert_eq!(
            add(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MIN + 12, i16::MIN + 15), &RectI16::of(-2, -5, 9, 7)),
            RectI16::of(i16::MIN, i16::MIN, i16::MIN + 21, i16::MIN + 22)
        );
        assert_eq!(
            add(&RectI16::of(i16::MAX - 12, i16::MAX - 15, i16::MAX - 2, i16::MAX - 5), &RectI16::of(-9, -7, 2, 5)),
            RectI16::of(i16::MAX - 21, i16::MAX - 22, i16::MAX, i16::MAX)
        );
    }

    #[test]
    fn add_big_rect_to_bounds() {
        assert_eq!(add(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX - 2, i16::MAX - 5), &RectI16::of(-2, -5, 2, 5)), RectI16::largest());
        assert_eq!(add(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX), &RectI16::of(-2, -5, 0, 0)), RectI16::largest());
        assert_eq!(add(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5), &RectI16::of(0, 0, 2, 5)), RectI16::largest());
    }

    #[test]
    fn add_small_rect_out_of_bounds() {
        assert_eq!(
            add(&RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30), &RectI16::of(-20, -20, 0, 0)),
            RectI16::of(i16::MIN, i16::MIN, i16::MIN + 20, i16::MIN + 30)
        );
        assert_eq!(
            add(&RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10), &RectI16::of(0, 0, 20, 20)),
            RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX, i16::MAX)
        );
    }

    #[test]
    fn add_big_rect_out_of_bounds() {
        assert_eq!(add(&RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX), &RectI16::of(-20, -20, 0, 0)), RectI16::largest());
        assert_eq!(add(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10), &RectI16::of(0, 0, 20, 20)), RectI16::largest());
    }

    #[test]
    fn add_small_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10), &RectI16::min()), RectI16::min());
        assert_eq!(add(&RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1), &RectI16::max()), RectI16::max());
    }

    #[test]
    fn add_big_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectI16::largest(), &RectI16::largest()), RectI16::largest());

        let r_large = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assert_eq!(add(&r_large, &RectI16::largest()), RectI16::largest());
        assert_eq!(add(&r_large, &RectI16::of(i16::MIN, 0, 0, 0)), RectI16::of(i16::MIN, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1));
        assert_eq!(add(&r_large, &RectI16::of(0, i16::MIN, 0, 0)), RectI16::of(i16::MIN + 1, i16::MIN, i16::MAX - 1, i16::MAX - 1));
        assert_eq!(add(&r_large, &RectI16::of(0, 0, i16::MAX, 0)), RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX - 1));
        assert_eq!(add(&r_large, &RectI16::of(0, 0, 0, i16::MAX)), RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX));
    }
}
