use crate::cartesian::{
    point::point_u64::PointU64,
    rect::{rect_i64::RectI64, rect_u64::RectU64},
};

pub fn try_add(r: &RectU64, delta: &RectI64) -> Option<RectU64> {
    let min_x = r.min.x.checked_add_signed(delta.min.x)?;
    let min_y = r.min.y.checked_add_signed(delta.min.y)?;
    let max_x = r.max.x.checked_add_signed(delta.max.x)?;
    let max_y = r.max.y.checked_add_signed(delta.max.y)?;
    Some(RectU64 { min: PointU64 { x: min_x, y: min_y }, max: PointU64 { x: max_x, y: max_y } })
}

#[cfg(test)]
mod tests {
    use super::try_add;
    use crate::cartesian::rect::{rect_i64::RectI64, rect_u64::RectU64};

    #[test]
    fn test_try_add() {
        assert_eq!(try_add(&RectU64::of(0, 0, 12, 15), &RectI64::of(5, 4, 3, 2)), Some(RectU64::of(5, 4, 15, 17)));
        assert_eq!(try_add(&RectU64::of(5, 4, 15, 20), &RectI64::of(-4, -3, -2, -1)), Some(RectU64::of(1, 1, 13, 19)));
    }

    #[test]
    fn try_add_big_rect_to_bounds() {
        assert_eq!(try_add(&RectU64::of(2, 5, u64::MAX - 2, u64::MAX - 5), &RectI64::of(-2, -5, 2, 5)), Some(RectU64::largest()));
        assert_eq!(try_add(&RectU64::of(2, 5, u64::MAX, u64::MAX), &RectI64::of(-2, -5, 0, 0)), Some(RectU64::largest()));
        assert_eq!(try_add(&RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5), &RectI64::of(0, 0, 2, 5)), Some(RectU64::largest()));
    }

    #[test]
    fn try_add_edge_out_of_bounds() {
        let r = RectU64::largest();
        assert_eq!(try_add(&r, &RectI64::of(-1, 0, 0, 0)), None);
        assert_eq!(try_add(&r, &RectI64::of(0, -1, 0, 0)), None);
        assert_eq!(try_add(&r, &RectI64::of(0, 0, 1, 0)), None);
        assert_eq!(try_add(&r, &RectI64::of(0, 0, 0, 1)), None);
    }

    #[test]
    fn try_add_out_of_bounds() {
        let r = RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10);
        assert_eq!(try_add(&r, &RectI64::of(-20, 0, 0, 0)), None);
        assert_eq!(try_add(&r, &RectI64::of(0, -20, 0, 0)), None);
        assert_eq!(try_add(&r, &RectI64::of(0, 0, 20, 0)), None);
        assert_eq!(try_add(&r, &RectI64::of(0, 0, 0, 20)), None);
    }

    #[test]
    fn try_add_limits_out_of_bounds() {
        let r = RectU64::largest();
        assert_eq!(try_add(&r, &RectI64::of(i64::MIN, 0, 0, 0)), None);
        assert_eq!(try_add(&r, &RectI64::of(0, i64::MIN, 0, 0)), None);
        assert_eq!(try_add(&r, &RectI64::of(0, 0, i64::MAX, 0)), None);
        assert_eq!(try_add(&r, &RectI64::of(0, 0, 0, i64::MAX)), None);
    }
}
