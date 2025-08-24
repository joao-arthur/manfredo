use crate::matrix::{point::point_i8::PointI8, rect::rect_i8::RectI8};

pub fn saturating_add(r: &RectI8, delta: &RectI8) -> RectI8 {
    let min_row = r.min.row.saturating_add(delta.min.row);
    let min_col = r.min.col.saturating_add(delta.min.col);
    let max_row = r.max.row.saturating_add(delta.max.row);
    let max_col = r.max.col.saturating_add(delta.max.col);
    RectI8 { min: PointI8 { row: min_row, col: min_col }, max: PointI8 { row: max_row, col: max_col } }
}

#[cfg(test)]
mod tests {
    use super::saturating_add;
    use crate::matrix::rect::rect_i8::RectI8;

    #[test]
    fn test_saturating_add() {
        assert_eq!(saturating_add(&RectI8::of(0, 0, 12, 15), &RectI8::of(5, 4, 3, 2)), RectI8::of(5, 4, 15, 17));
        assert_eq!(saturating_add(&RectI8::of(5, 4, 15, 17), &RectI8::of(-14, -13, -12, -11)), RectI8::of(-9, -9, 3, 6));
    }

    #[test]
    fn saturating_add_to_bounds() {
        assert_eq!(saturating_add(&RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX - 2, i8::MAX - 5), &RectI8::of(-2, -5, 2, 5)), RectI8::largest());
        assert_eq!(saturating_add(&RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX), &RectI8::of(-2, -5, 0, 0)), RectI8::largest());
        assert_eq!(saturating_add(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5), &RectI8::of(0, 0, 2, 5)), RectI8::largest());
    }

    #[test]
    fn saturating_add_edge_out_of_bounds() {
        let r = RectI8::largest();
        assert_eq!(saturating_add(&r, &RectI8::of(-1, 0, 0, 0)), RectI8::largest());
        assert_eq!(saturating_add(&r, &RectI8::of(0, -1, 0, 0)), RectI8::largest());
        assert_eq!(saturating_add(&r, &RectI8::of(0, 0, 1, 0)), RectI8::largest());
        assert_eq!(saturating_add(&r, &RectI8::of(0, 0, 0, 1)), RectI8::largest());
    }

    #[test]
    fn saturating_add_out_of_bounds() {
        let r = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10);
        assert_eq!(saturating_add(&r, &RectI8::of(-20, 0, 0, 0)), RectI8::of(i8::MIN, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10));
        assert_eq!(saturating_add(&r, &RectI8::of(0, -20, 0, 0)), RectI8::of(i8::MIN + 10, i8::MIN, i8::MAX - 10, i8::MAX - 10));
        assert_eq!(saturating_add(&r, &RectI8::of(0, 0, 20, 0)), RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX, i8::MAX - 10));
        assert_eq!(saturating_add(&r, &RectI8::of(0, 0, 0, 20)), RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX));
    }

    #[test]
    fn saturating_add_limits_out_of_bounds() {
        let r = RectI8::largest();
        assert_eq!(saturating_add(&r, &RectI8::of(i8::MIN, 0, 0, 0)), RectI8::largest());
        assert_eq!(saturating_add(&r, &RectI8::of(0, i8::MIN, 0, 0)), RectI8::largest());
        assert_eq!(saturating_add(&r, &RectI8::of(0, 0, i8::MAX, 0)), RectI8::largest());
        assert_eq!(saturating_add(&r, &RectI8::of(0, 0, 0, i8::MAX)), RectI8::largest());
    }
}
