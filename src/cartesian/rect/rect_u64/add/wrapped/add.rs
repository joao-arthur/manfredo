use crate::cartesian::{
    point::point_u64::PointU64,
    rect::{rect_i64::RectI64, rect_u64::RectU64},
};

pub fn add(r: &RectU64, delta: &RectI64) -> RectU64 {
    let min_x = r.min.x.wrapping_add_signed(delta.min.x);
    let min_y = r.min.y.wrapping_add_signed(delta.min.y);
    let max_x = r.max.x.wrapping_add_signed(delta.max.x);
    let max_y = r.max.y.wrapping_add_signed(delta.max.y);
    RectU64 { min: PointU64 { x: min_x, y: min_y }, max: PointU64 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use super::add;
    use crate::cartesian::rect::{rect_i64::RectI64, rect_u64::RectU64};

    #[test]
    fn test_add() {
        assert_eq!(add(&RectU64::of(0, 0, 12, 15), &RectI64::of(5, 4, 3, 2)), RectU64::of(5, 4, 15, 17));
        assert_eq!(add(&RectU64::of(5, 4, 15, 20), &RectI64::of(-4, -3, -2, -1)), RectU64::of(1, 1, 13, 19));
    }

    #[test]
    fn add_small_rect_to_bounds() {
        assert_eq!(add(&RectU64::of(2, 5, 12, 15), &RectI64::of(-2, -5, 9, 7)), RectU64::of(0, 0, 21, 22));
        assert_eq!(
            add(&RectU64::of(u64::MAX - 12, u64::MAX - 15, u64::MAX - 2, u64::MAX - 5), &RectI64::of(-9, -7, 2, 5)),
            RectU64::of(u64::MAX - 21, u64::MAX - 22, u64::MAX, u64::MAX)
        );
    }

    #[test]
    fn add_big_rect_to_bounds() {
        assert_eq!(add(&RectU64::of(2, 5, u64::MAX - 2, u64::MAX - 5), &RectI64::of(-2, -5, 2, 5)), RectU64::largest());
        assert_eq!(add(&RectU64::of(2, 5, u64::MAX, u64::MAX), &RectI64::of(-2, -5, 0, 0)), RectU64::largest());
        assert_eq!(add(&RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5), &RectI64::of(0, 0, 2, 5)), RectU64::largest());
    }

    #[test]
    fn add_small_rect_out_of_bounds() {
        assert_eq!(add(&RectU64::of(10, 5, 20, 30), &RectI64::of(-20, -20, 0, 0)), RectU64::of(u64::MAX - 9, u64::MAX - 14, 20, 30));
        assert_eq!(
            add(&RectU64::of(u64::MAX - 20, u64::MAX - 30, u64::MAX - 5, u64::MAX - 10), &RectI64::of(0, 0, 20, 20)),
            RectU64::of(u64::MAX - 20, u64::MAX - 30, 14, 9)
        );
    }

    #[test]
    fn add_big_rect_out_of_bounds() {
        assert_eq!(
            add(&RectU64::of(10, 5, u64::MAX, u64::MAX), &RectI64::of(-20, -20, 0, 0)),
            RectU64::of(u64::MAX - 9, u64::MAX - 14, u64::MAX, u64::MAX)
        );
        assert_eq!(add(&RectU64::of(0, 0, u64::MAX - 5, u64::MAX - 10), &RectI64::of(0, 0, 20, 20)), RectU64::of(0, 0, 14, 9));
    }

    #[test]
    fn add_small_rect_limits_out_of_bounds() {
        assert_eq!(
            add(&RectU64::of(1, 1, 10, 10), &RectI64::min()),
            RectU64::of((i64::MAX as u64) + 2, (i64::MAX as u64) + 2, (i64::MAX as u64) + 11, (i64::MAX as u64) + 11)
        );
        assert_eq!(
            add(&RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX - 1, u64::MAX - 1), &RectI64::max()),
            RectU64::of((i64::MAX as u64) - 11, (i64::MAX as u64) - 11, (i64::MAX as u64) - 2, (i64::MAX as u64) - 2)
        );
    }

    #[test]
    fn add_big_rect_limits_out_of_bounds() {
        assert_eq!(
            add(&RectU64::largest(), &RectI64::min()),
            RectU64::of((i64::MAX as u64) + 1, (i64::MAX as u64) + 1, i64::MAX as u64, i64::MAX as u64)
        );
        assert_eq!(
            add(&RectU64::largest(), &RectI64::max()),
            RectU64::of(i64::MAX as u64, i64::MAX as u64, (i64::MAX as u64) - 1, (i64::MAX as u64) - 1)
        );
        assert_eq!(
            add(&RectU64::of(1, 1, u64::MAX, u64::MAX), &RectI64::min()),
            RectU64::of((i64::MAX as u64) + 2, (i64::MAX as u64) + 2, i64::MAX as u64, i64::MAX as u64)
        );
        assert_eq!(
            add(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1), &RectI64::max()),
            RectU64::of(i64::MAX as u64, i64::MAX as u64, (i64::MAX as u64) - 2, (i64::MAX as u64) - 2)
        );
    }
}
