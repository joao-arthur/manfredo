use crate::matrix::{point::point_i64::PointI64, rect::rect_i64::RectI64};

pub fn add(r: &RectI64, delta: &RectI64) -> RectI64 {
    let min_row = r.min.row.saturating_add(delta.min.row);
    let min_col = r.min.col.saturating_add(delta.min.col);
    let max_row = r.max.row.saturating_add(delta.max.row);
    let max_col = r.max.col.saturating_add(delta.max.col);
    RectI64 { min: PointI64 { row: min_row, col: min_col }, max: PointI64 { row: max_row, col: max_col } }
}

#[cfg(test)]
mod tests {
    use super::add;
    use crate::matrix::rect::rect_i64::RectI64;

    #[test]
    fn test_add() {
        assert_eq!(add(&RectI64::of(0, 0, 12, 15), &RectI64::of(5, 4, 3, 2)), RectI64::of(5, 4, 15, 17));
        assert_eq!(add(&RectI64::of(5, 4, 15, 17), &RectI64::of(-14, -13, -12, -11)), RectI64::of(-9, -9, 3, 6));
    }

    #[test]
    fn add_to_bounds() {
        assert_eq!(add(&RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX - 2, i64::MAX - 5), &RectI64::of(-2, -5, 2, 5)), RectI64::largest());
        assert_eq!(add(&RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX), &RectI64::of(-2, -5, 0, 0)), RectI64::largest());
        assert_eq!(add(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5), &RectI64::of(0, 0, 2, 5)), RectI64::largest());
    }

    #[test]
    fn add_edge_out_of_bounds() {
        let r = RectI64::largest();
        assert_eq!(add(&r, &RectI64::of(-1, 0, 0, 0)), RectI64::largest());
        assert_eq!(add(&r, &RectI64::of(0, -1, 0, 0)), RectI64::largest());
        assert_eq!(add(&r, &RectI64::of(0, 0, 1, 0)), RectI64::largest());
        assert_eq!(add(&r, &RectI64::of(0, 0, 0, 1)), RectI64::largest());
    }

    #[test]
    fn add_out_of_bounds() {
        let r = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10);
        assert_eq!(add(&r, &RectI64::of(-20, 0, 0, 0)), RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10));
        assert_eq!(add(&r, &RectI64::of(0, -20, 0, 0)), RectI64::of(i64::MIN + 10, i64::MIN, i64::MAX - 10, i64::MAX - 10));
        assert_eq!(add(&r, &RectI64::of(0, 0, 20, 0)), RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX, i64::MAX - 10));
        assert_eq!(add(&r, &RectI64::of(0, 0, 0, 20)), RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX));
    }

    #[test]
    fn add_limits_out_of_bounds() {
        let r = RectI64::largest();
        assert_eq!(add(&r, &RectI64::of(i64::MIN, 0, 0, 0)), RectI64::largest());
        assert_eq!(add(&r, &RectI64::of(0, i64::MIN, 0, 0)), RectI64::largest());
        assert_eq!(add(&r, &RectI64::of(0, 0, i64::MAX, 0)), RectI64::largest());
        assert_eq!(add(&r, &RectI64::of(0, 0, 0, i64::MAX)), RectI64::largest());
    }
}
