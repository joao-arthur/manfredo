use crate::matrix::{
    point::point_u8::PointU8,
    rect::{rect_i8::RectI8, rect_u8::RectU8},
};

pub fn saturating_add(r: &RectU8, delta: &RectI8) -> RectU8 {
    let min_row = r.min.row.saturating_add_signed(delta.min.row);
    let min_col = r.min.col.saturating_add_signed(delta.min.col);
    let max_row = r.max.row.saturating_add_signed(delta.max.row);
    let max_col = r.max.col.saturating_add_signed(delta.max.col);
    RectU8 { min: PointU8 { row: min_row, col: min_col }, max: PointU8 { row: max_row, col: max_col } }
}

#[cfg(test)]
mod tests {
    use super::saturating_add;
    use crate::matrix::rect::{rect_i8::RectI8, rect_u8::RectU8};

    #[test]
    fn test_saturating_add() {
        assert_eq!(saturating_add(&RectU8::of(0, 0, 12, 15), &RectI8::of(5, 4, 3, 2)), RectU8::of(5, 4, 15, 17));
        assert_eq!(saturating_add(&RectU8::of(5, 4, 15, 20), &RectI8::of(-4, -3, -2, -1)), RectU8::of(1, 1, 13, 19));
    }

    #[test]
    fn saturating_add_to_bounds() {
        assert_eq!(saturating_add(&RectU8::of(2, 5, u8::MAX - 2, u8::MAX - 5), &RectI8::of(-2, -5, 2, 5)), RectU8::largest());
        assert_eq!(saturating_add(&RectU8::of(2, 5, u8::MAX, u8::MAX), &RectI8::of(-2, -5, 0, 0)), RectU8::largest());
        assert_eq!(saturating_add(&RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5), &RectI8::of(0, 0, 2, 5)), RectU8::largest());
    }

    #[test]
    fn saturating_add_edge_out_of_bounds() {
        let r = RectU8::largest();
        assert_eq!(saturating_add(&r, &RectI8::of(-1, 0, 0, 0)), RectU8::largest());
        assert_eq!(saturating_add(&r, &RectI8::of(0, -1, 0, 0)), RectU8::largest());
        assert_eq!(saturating_add(&r, &RectI8::of(0, 0, 1, 0)), RectU8::largest());
        assert_eq!(saturating_add(&r, &RectI8::of(0, 0, 0, 1)), RectU8::largest());
    }

    #[test]
    fn saturating_add_out_of_bounds() {
        let r = RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10);
        assert_eq!(saturating_add(&r, &RectI8::of(-20, 0, 0, 0)), RectU8::of(0, 10, u8::MAX - 10, u8::MAX - 10));
        assert_eq!(saturating_add(&r, &RectI8::of(0, -20, 0, 0)), RectU8::of(10, 0, u8::MAX - 10, u8::MAX - 10));
        assert_eq!(saturating_add(&r, &RectI8::of(0, 0, 20, 0)), RectU8::of(10, 10, u8::MAX, u8::MAX - 10));
        assert_eq!(saturating_add(&r, &RectI8::of(0, 0, 0, 20)), RectU8::of(10, 10, u8::MAX - 10, u8::MAX));
    }

    #[test]
    fn saturating_add_limits_out_of_bounds() {
        let r = RectU8::largest();
        assert_eq!(saturating_add(&r, &RectI8::of(i8::MIN, 0, 0, 0)), RectU8::largest());
        assert_eq!(saturating_add(&r, &RectI8::of(0, i8::MIN, 0, 0)), RectU8::largest());
        assert_eq!(saturating_add(&r, &RectI8::of(0, 0, i8::MAX, 0)), RectU8::largest());
        assert_eq!(saturating_add(&r, &RectI8::of(0, 0, 0, i8::MAX)), RectU8::largest());
    }
}
