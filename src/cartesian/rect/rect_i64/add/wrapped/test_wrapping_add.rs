use crate::cartesian::{point::point_i64::PointI64, rect::rect_i64::RectI64};

pub fn wrapping_add(r: &RectI64, delta: &RectI64) -> RectI64 {
    let min_x = r.min.x.wrapping_add(delta.min.x);
    let min_y = r.min.y.wrapping_add(delta.min.y);
    let max_x = r.max.x.wrapping_add(delta.max.x);
    let max_y = r.max.y.wrapping_add(delta.max.y);
    RectI64 { min: PointI64 { x: min_x, y: min_y }, max: PointI64 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use super::wrapping_add;
    use crate::cartesian::rect::rect_i64::RectI64;

    #[test]
    fn test_wrapping_add() {
        assert_eq!(wrapping_add(&RectI64::of(0, 0, 12, 15), &RectI64::of(5, 4, 3, 2)), RectI64::of(5, 4, 15, 17));
        assert_eq!(wrapping_add(&RectI64::of(5, 4, 15, 17), &RectI64::of(-14, -13, -12, -11)), RectI64::of(-9, -9, 3, 6));
    }

    #[test]
    fn wrapping_add_small_rect_to_bounds() {
        assert_eq!(
            wrapping_add(&RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MIN + 12, i64::MIN + 15), &RectI64::of(-2, -5, 9, 7)),
            RectI64::of(i64::MIN, i64::MIN, i64::MIN + 21, i64::MIN + 22)
        );
        assert_eq!(
            wrapping_add(&RectI64::of(i64::MAX - 12, i64::MAX - 15, i64::MAX - 2, i64::MAX - 5), &RectI64::of(-9, -7, 2, 5)),
            RectI64::of(i64::MAX - 21, i64::MAX - 22, i64::MAX, i64::MAX)
        );
    }

    #[test]
    fn wrapping_add_big_rect_to_bounds() {
        assert_eq!(
            wrapping_add(&RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX - 2, i64::MAX - 5), &RectI64::of(-2, -5, 2, 5)),
            RectI64::largest()
        );
        assert_eq!(wrapping_add(&RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX), &RectI64::of(-2, -5, 0, 0)), RectI64::largest());
        assert_eq!(wrapping_add(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5), &RectI64::of(0, 0, 2, 5)), RectI64::largest());
    }

    #[test]
    fn wrapping_add_small_rect_out_of_bounds() {
        assert_eq!(
            wrapping_add(&RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MIN + 20, i64::MIN + 30), &RectI64::of(-20, -20, 0, 0)),
            RectI64::of(i64::MAX - 9, i64::MAX - 14, i64::MIN + 20, i64::MIN + 30)
        );
        assert_eq!(
            wrapping_add(&RectI64::of(i64::MAX - 20, i64::MAX - 30, i64::MAX - 5, i64::MAX - 10), &RectI64::of(0, 0, 20, 20)),
            RectI64::of(i64::MAX - 20, i64::MAX - 30, i64::MIN + 14, i64::MIN + 9)
        );
    }

    #[test]
    fn wrapping_add_big_rect_out_of_bounds() {
        assert_eq!(
            wrapping_add(&RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MAX, i64::MAX), &RectI64::of(-20, -20, 0, 0)),
            RectI64::of(i64::MAX - 9, i64::MAX - 14, i64::MAX, i64::MAX)
        );
        assert_eq!(
            wrapping_add(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 5, i64::MAX - 10), &RectI64::of(0, 0, 20, 20)),
            RectI64::of(i64::MIN, i64::MIN, i64::MIN + 14, i64::MIN + 9)
        );
    }

    #[test]
    fn wrapping_add_small_rect_limits_out_of_bounds() {
        assert_eq!(wrapping_add(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MIN + 10, i64::MIN + 10), &RectI64::min()), RectI64::of(1, 1, 10, 10));
        assert_eq!(
            wrapping_add(&RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX - 1, i64::MAX - 1), &RectI64::max()),
            RectI64::of(-12, -12, -3, -3)
        );
    }

    #[test]
    fn wrapping_add_big_rect_limits_out_of_bounds() {
        assert_eq!(wrapping_add(&RectI64::largest(), &RectI64::min()), RectI64::of(0, 0, -1, -1));
        assert_eq!(wrapping_add(&RectI64::largest(), &RectI64::max()), RectI64::of(-1, -1, -2, -2));
        assert_eq!(wrapping_add(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX), &RectI64::min()), RectI64::of(1, 1, -1, -1));
        assert_eq!(wrapping_add(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1), &RectI64::max()), RectI64::of(-1, -1, -3, -3));
    }
}
