use crate::matrix::{
    point::point_u64::PointU64,
    rect::{rect_i64::RectI64, rect_u64::RectU64},
};

pub fn saturating_add(r: &RectU64, delta: &RectI64) -> RectU64 {
    let min_row = r.min.row.saturating_add_signed(delta.min.row);
    let min_col = r.min.col.saturating_add_signed(delta.min.col);
    let max_row = r.max.row.saturating_add_signed(delta.max.row);
    let max_col = r.max.col.saturating_add_signed(delta.max.col);
    RectU64 { min: PointU64 { row: min_row, col: min_col }, max: PointU64 { row: max_row, col: max_col } }
}

#[cfg(test)]
mod tests {
    use super::saturating_add;
    use crate::matrix::rect::{rect_i64::RectI64, rect_u64::RectU64};

    #[test]
    fn test_saturating_add() {
        assert_eq!(saturating_add(&RectU64::of(0, 0, 12, 15), &RectI64::of(5, 4, 3, 2)), RectU64::of(5, 4, 15, 17));
        assert_eq!(saturating_add(&RectU64::of(5, 4, 15, 20), &RectI64::of(-4, -3, -2, -1)), RectU64::of(1, 1, 13, 19));
    }

    #[test]
    fn saturating_add_to_bounds() {
        assert_eq!(saturating_add(&RectU64::of(2, 5, u64::MAX - 2, u64::MAX - 5), &RectI64::of(-2, -5, 2, 5)), RectU64::largest());
        assert_eq!(saturating_add(&RectU64::of(2, 5, u64::MAX, u64::MAX), &RectI64::of(-2, -5, 0, 0)), RectU64::largest());
        assert_eq!(saturating_add(&RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5), &RectI64::of(0, 0, 2, 5)), RectU64::largest());
    }

    #[test]
    fn saturating_add_edge_out_of_bounds() {
        let r = RectU64::largest();
        assert_eq!(saturating_add(&r, &RectI64::of(-1, 0, 0, 0)), RectU64::largest());
        assert_eq!(saturating_add(&r, &RectI64::of(0, -1, 0, 0)), RectU64::largest());
        assert_eq!(saturating_add(&r, &RectI64::of(0, 0, 1, 0)), RectU64::largest());
        assert_eq!(saturating_add(&r, &RectI64::of(0, 0, 0, 1)), RectU64::largest());
    }

    #[test]
    fn saturating_add_out_of_bounds() {
        let r = RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10);
        assert_eq!(saturating_add(&r, &RectI64::of(-20, 0, 0, 0)), RectU64::of(0, 10, u64::MAX - 10, u64::MAX - 10));
        assert_eq!(saturating_add(&r, &RectI64::of(0, -20, 0, 0)), RectU64::of(10, 0, u64::MAX - 10, u64::MAX - 10));
        assert_eq!(saturating_add(&r, &RectI64::of(0, 0, 20, 0)), RectU64::of(10, 10, u64::MAX, u64::MAX - 10));
        assert_eq!(saturating_add(&r, &RectI64::of(0, 0, 0, 20)), RectU64::of(10, 10, u64::MAX - 10, u64::MAX));
    }

    #[test]
    fn saturating_add_limits_out_of_bounds() {
        let r = RectU64::largest();
        assert_eq!(saturating_add(&r, &RectI64::of(i64::MIN, 0, 0, 0)), RectU64::largest());
        assert_eq!(saturating_add(&r, &RectI64::of(0, i64::MIN, 0, 0)), RectU64::largest());
        assert_eq!(saturating_add(&r, &RectI64::of(0, 0, i64::MAX, 0)), RectU64::largest());
        assert_eq!(saturating_add(&r, &RectI64::of(0, 0, 0, i64::MAX)), RectU64::largest());
    }
}
