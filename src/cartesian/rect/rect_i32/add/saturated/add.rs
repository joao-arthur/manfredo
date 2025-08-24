use crate::cartesian::{point::point_i32::PointI32, rect::rect_i32::RectI32};

pub fn add(r: &RectI32, delta: &RectI32) -> RectI32 {
    let min_x = r.min.x.saturating_add(delta.min.x);
    let min_y = r.min.y.saturating_add(delta.min.y);
    let max_x = r.max.x.saturating_add(delta.max.x);
    let max_y = r.max.y.saturating_add(delta.max.y);
    RectI32 { min: PointI32 { x: min_x, y: min_y }, max: PointI32 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use super::add;
    use crate::cartesian::rect::rect_i32::RectI32;

    #[test]
    fn test_add() {
        assert_eq!(add(&RectI32::of(0, 0, 12, 15), &RectI32::of(5, 4, 3, 2)), RectI32::of(5, 4, 15, 17));
        assert_eq!(add(&RectI32::of(5, 4, 15, 17), &RectI32::of(-14, -13, -12, -11)), RectI32::of(-9, -9, 3, 6));
    }

    #[test]
    fn add_small_rect_to_bounds() {
        assert_eq!(
            add(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MIN + 12, i32::MIN + 15), &RectI32::of(-2, -5, 9, 7)),
            RectI32::of(i32::MIN, i32::MIN, i32::MIN + 21, i32::MIN + 22)
        );
        assert_eq!(
            add(&RectI32::of(i32::MAX - 12, i32::MAX - 15, i32::MAX - 2, i32::MAX - 5), &RectI32::of(-9, -7, 2, 5)),
            RectI32::of(i32::MAX - 21, i32::MAX - 22, i32::MAX, i32::MAX)
        );
    }

    #[test]
    fn add_big_rect_to_bounds() {
        assert_eq!(add(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX - 2, i32::MAX - 5), &RectI32::of(-2, -5, 2, 5)), RectI32::largest());
        assert_eq!(add(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX), &RectI32::of(-2, -5, 0, 0)), RectI32::largest());
        assert_eq!(add(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5), &RectI32::of(0, 0, 2, 5)), RectI32::largest());
    }

    #[test]
    fn add_small_rect_out_of_bounds() {
        assert_eq!(
            add(&RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MIN + 20, i32::MIN + 30), &RectI32::of(-20, -20, 0, 0)),
            RectI32::of(i32::MIN, i32::MIN, i32::MIN + 20, i32::MIN + 30)
        );
        assert_eq!(
            add(&RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX - 5, i32::MAX - 10), &RectI32::of(0, 0, 20, 20)),
            RectI32::of(i32::MAX - 20, i32::MAX - 30, i32::MAX, i32::MAX)
        );
    }

    #[test]
    fn add_big_rect_out_of_bounds() {
        assert_eq!(add(&RectI32::of(i32::MIN + 10, i32::MIN + 5, i32::MAX, i32::MAX), &RectI32::of(-20, -20, 0, 0)), RectI32::largest());
        assert_eq!(add(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 5, i32::MAX - 10), &RectI32::of(0, 0, 20, 20)), RectI32::largest());
    }

    #[test]
    fn add_small_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MIN + 10, i32::MIN + 10), &RectI32::min()), RectI32::min());
        assert_eq!(add(&RectI32::of(i32::MAX - 10, i32::MAX - 10, i32::MAX - 1, i32::MAX - 1), &RectI32::max()), RectI32::max());
    }

    #[test]
    fn add_big_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectI32::largest(), &RectI32::largest()), RectI32::largest());

        let r_large = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1);
        assert_eq!(add(&r_large, &RectI32::largest()), RectI32::largest());
        assert_eq!(add(&r_large, &RectI32::of(i32::MIN, 0, 0, 0)), RectI32::of(i32::MIN, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1));
        assert_eq!(add(&r_large, &RectI32::of(0, i32::MIN, 0, 0)), RectI32::of(i32::MIN + 1, i32::MIN, i32::MAX - 1, i32::MAX - 1));
        assert_eq!(add(&r_large, &RectI32::of(0, 0, i32::MAX, 0)), RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX - 1));
        assert_eq!(add(&r_large, &RectI32::of(0, 0, 0, i32::MAX)), RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX));
    }
}
