use crate::cartesian::{point::point_i8::PointI8, rect::rect_i8::RectI8};

pub fn assign_add(r: &mut RectI8, delta: &RectI8) {
    let min_x = r.min.x.saturating_add(delta.min.x);
    let min_y = r.min.y.saturating_add(delta.min.y);
    let max_x = r.max.x.saturating_add(delta.max.x);
    let max_y = r.max.y.saturating_add(delta.max.y);
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
}

pub fn add(r: &RectI8, delta: &RectI8) -> RectI8 {
    let min_x = r.min.x.saturating_add(delta.min.x);
    let min_y = r.min.y.saturating_add(delta.min.y);
    let max_x = r.max.x.saturating_add(delta.max.x);
    let max_y = r.max.y.saturating_add(delta.max.y);
    RectI8 { min: PointI8 { x: min_x, y: min_y }, max: PointI8 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_i8::RectI8;

    use super::{add, assign_add};

    #[test]
    fn test_assign_add() {
        let mut r = RectI8::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI8::of(5, 4, 3, 2));
        assert_eq!(r, RectI8::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI8::of(-14, -13, -12, -11));
        assert_eq!(r, RectI8::of(-9, -9, 3, 6));
    }

    #[test]
    fn assign_add_small_rect_to_bounds() {
        let mut min_r = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MIN + 12, i8::MIN + 15);
        assign_add(&mut min_r, &RectI8::of(-2, -5, 9, 7));
        assert_eq!(min_r, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 21, i8::MIN + 22));

        let mut max_r = RectI8::of(i8::MAX - 12, i8::MAX - 15, i8::MAX - 2, i8::MAX - 5);
        assign_add(&mut max_r, &RectI8::of(-9, -7, 2, 5));
        assert_eq!(max_r, RectI8::of(i8::MAX - 21, i8::MAX - 22, i8::MAX, i8::MAX));
    }

    #[test]
    fn assign_add_big_rect_to_bounds() {
        let mut r = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX - 2, i8::MAX - 5);
        assign_add(&mut r, &RectI8::of(-2, -5, 2, 5));
        assert_eq!(r, RectI8::largest());

        let mut min_r = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX);
        assign_add(&mut min_r, &RectI8::of(-2, -5, 0, 0));
        assert_eq!(min_r, RectI8::largest());

        let mut max_r = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5);
        assign_add(&mut max_r, &RectI8::of(0, 0, 2, 5));
        assert_eq!(max_r, RectI8::largest());
    }

    #[test]
    fn assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MIN + 20, i8::MIN + 30);
        assign_add(&mut r_min, &RectI8::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 20, i8::MIN + 30));

        let mut r_max = RectI8::of(i8::MAX - 20, i8::MAX - 30, i8::MAX - 5, i8::MAX - 10);
        assign_add(&mut r_max, &RectI8::of(0, 0, 20, 20));
        assert_eq!(r_max, RectI8::of(i8::MAX - 20, i8::MAX - 30, i8::MAX, i8::MAX));
    }

    #[test]
    fn assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MAX, i8::MAX);
        assign_add(&mut r_min, &RectI8::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectI8::largest());

        let mut r_max = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 5, i8::MAX - 10);
        assign_add(&mut r_max, &RectI8::of(0, 0, 20, 20));
        assert_eq!(r_max, RectI8::largest());
    }

    #[test]
    fn assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MIN + 10, i8::MIN + 10);
        assign_add(&mut r_min, &RectI8::min());
        assert_eq!(r_min, RectI8::min());

        let mut r_max = RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX - 1, i8::MAX - 1);
        assign_add(&mut r_max, &RectI8::max());
        assert_eq!(r_max, RectI8::max());
    }

    #[test]
    fn assign_add_big_rect_limits_out_of_bounds() {
        let mut r = RectI8::largest();
        assign_add(&mut r, &RectI8::largest());
        assert_eq!(r, RectI8::largest());

        let mut r_large = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1);
        assign_add(&mut r_large, &RectI8::largest());
        assert_eq!(r_large, RectI8::largest());

        let mut r_min_x = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1);
        assign_add(&mut r_min_x, &RectI8::of(i8::MIN, 0, 0, 0));
        assert_eq!(r_min_x, RectI8::of(i8::MIN, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1));

        let mut r_min_y = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1);
        assign_add(&mut r_min_y, &RectI8::of(0, i8::MIN, 0, 0));
        assert_eq!(r_min_y, RectI8::of(i8::MIN + 1, i8::MIN, i8::MAX - 1, i8::MAX - 1));

        let mut r_max_x = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1);
        assign_add(&mut r_max_x, &RectI8::of(0, 0, i8::MAX, 0));
        assert_eq!(r_max_x, RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX - 1));

        let mut r_max_y = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1);
        assign_add(&mut r_max_y, &RectI8::of(0, 0, 0, i8::MAX));
        assert_eq!(r_max_y, RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&RectI8::of(0, 0, 12, 15), &RectI8::of(5, 4, 3, 2)), RectI8::of(5, 4, 15, 17));
        assert_eq!(add(&RectI8::of(5, 4, 15, 17), &RectI8::of(-14, -13, -12, -11)), RectI8::of(-9, -9, 3, 6));
    }

    #[test]
    fn add_small_rect_to_bounds() {
        assert_eq!(
            add(&RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MIN + 12, i8::MIN + 15), &RectI8::of(-2, -5, 9, 7)),
            RectI8::of(i8::MIN, i8::MIN, i8::MIN + 21, i8::MIN + 22)
        );
        assert_eq!(
            add(&RectI8::of(i8::MAX - 12, i8::MAX - 15, i8::MAX - 2, i8::MAX - 5), &RectI8::of(-9, -7, 2, 5)),
            RectI8::of(i8::MAX - 21, i8::MAX - 22, i8::MAX, i8::MAX)
        );
    }

    #[test]
    fn add_big_rect_to_bounds() {
        assert_eq!(add(&RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX - 2, i8::MAX - 5), &RectI8::of(-2, -5, 2, 5)), RectI8::largest());
        assert_eq!(add(&RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX), &RectI8::of(-2, -5, 0, 0)), RectI8::largest());
        assert_eq!(add(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5), &RectI8::of(0, 0, 2, 5)), RectI8::largest());
    }

    #[test]
    fn add_small_rect_out_of_bounds() {
        assert_eq!(
            add(&RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MIN + 20, i8::MIN + 30), &RectI8::of(-20, -20, 0, 0)),
            RectI8::of(i8::MIN, i8::MIN, i8::MIN + 20, i8::MIN + 30)
        );
        assert_eq!(
            add(&RectI8::of(i8::MAX - 20, i8::MAX - 30, i8::MAX - 5, i8::MAX - 10), &RectI8::of(0, 0, 20, 20)),
            RectI8::of(i8::MAX - 20, i8::MAX - 30, i8::MAX, i8::MAX)
        );
    }

    #[test]
    fn add_big_rect_out_of_bounds() {
        assert_eq!(add(&RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MAX, i8::MAX), &RectI8::of(-20, -20, 0, 0)), RectI8::largest());
        assert_eq!(add(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 5, i8::MAX - 10), &RectI8::of(0, 0, 20, 20)), RectI8::largest());
    }

    #[test]
    fn add_small_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MIN + 10, i8::MIN + 10), &RectI8::min()), RectI8::min());
        assert_eq!(add(&RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX - 1, i8::MAX - 1), &RectI8::max()), RectI8::max());
    }

    #[test]
    fn add_big_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectI8::largest(), &RectI8::largest()), RectI8::largest());

        let r_large = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1);
        assert_eq!(add(&r_large, &RectI8::largest()), RectI8::largest());
        assert_eq!(add(&r_large, &RectI8::of(i8::MIN, 0, 0, 0)), RectI8::of(i8::MIN, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1));
        assert_eq!(add(&r_large, &RectI8::of(0, i8::MIN, 0, 0)), RectI8::of(i8::MIN + 1, i8::MIN, i8::MAX - 1, i8::MAX - 1));
        assert_eq!(add(&r_large, &RectI8::of(0, 0, i8::MAX, 0)), RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX - 1));
        assert_eq!(add(&r_large, &RectI8::of(0, 0, 0, i8::MAX)), RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX));
    }
}
