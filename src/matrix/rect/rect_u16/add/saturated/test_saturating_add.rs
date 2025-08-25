use crate::matrix::{
    point::point_u16::PointU16,
    rect::{rect_i16::RectI16, rect_u16::RectU16},
};

pub fn saturating_add(r: &RectU16, delta: &RectI16) -> RectU16 {
    let min_row = r.min.row.saturating_add_signed(delta.min.row);
    let min_col = r.min.col.saturating_add_signed(delta.min.col);
    let max_row = r.max.row.saturating_add_signed(delta.max.row);
    let max_col = r.max.col.saturating_add_signed(delta.max.col);
    RectU16 { min: PointU16 { row: min_row, col: min_col }, max: PointU16 { row: max_row, col: max_col } }
}

#[cfg(test)]
mod tests {
    use super::saturating_add;
    use crate::matrix::rect::{rect_i16::RectI16, rect_u16::RectU16};

    #[test]
    fn test_saturating_add() {
        assert_eq!(saturating_add(&RectU16::of(0, 0, 12, 15), &RectI16::of(5, 4, 3, 2)), RectU16::of(5, 4, 15, 17));
        assert_eq!(saturating_add(&RectU16::of(5, 4, 15, 20), &RectI16::of(-4, -3, -2, -1)), RectU16::of(1, 1, 13, 19));
    }

    #[test]
    fn saturating_add_to_bounds() {
        assert_eq!(saturating_add(&RectU16::of(2, 5, u16::MAX - 2, u16::MAX - 5), &RectI16::of(-2, -5, 2, 5)), RectU16::largest());
        assert_eq!(saturating_add(&RectU16::of(2, 5, u16::MAX, u16::MAX), &RectI16::of(-2, -5, 0, 0)), RectU16::largest());
        assert_eq!(saturating_add(&RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5), &RectI16::of(0, 0, 2, 5)), RectU16::largest());
    }

    #[test]
    fn saturating_add_edge_out_of_bounds() {
        let r = RectU16::largest();
        assert_eq!(saturating_add(&r, &RectI16::of(-1, 0, 0, 0)), RectU16::largest());
        assert_eq!(saturating_add(&r, &RectI16::of(0, -1, 0, 0)), RectU16::largest());
        assert_eq!(saturating_add(&r, &RectI16::of(0, 0, 1, 0)), RectU16::largest());
        assert_eq!(saturating_add(&r, &RectI16::of(0, 0, 0, 1)), RectU16::largest());
    }

    #[test]
    fn saturating_add_out_of_bounds() {
        let r = RectU16::of(10, 10, u16::MAX - 10, u16::MAX - 10);
        assert_eq!(saturating_add(&r, &RectI16::of(-20, 0, 0, 0)), RectU16::of(0, 10, u16::MAX - 10, u16::MAX - 10));
        assert_eq!(saturating_add(&r, &RectI16::of(0, -20, 0, 0)), RectU16::of(10, 0, u16::MAX - 10, u16::MAX - 10));
        assert_eq!(saturating_add(&r, &RectI16::of(0, 0, 20, 0)), RectU16::of(10, 10, u16::MAX, u16::MAX - 10));
        assert_eq!(saturating_add(&r, &RectI16::of(0, 0, 0, 20)), RectU16::of(10, 10, u16::MAX - 10, u16::MAX));
    }

    #[test]
    fn saturating_add_limits_out_of_bounds() {
        let r = RectU16::largest();
        assert_eq!(saturating_add(&r, &RectI16::of(i16::MIN, 0, 0, 0)), RectU16::largest());
        assert_eq!(saturating_add(&r, &RectI16::of(0, i16::MIN, 0, 0)), RectU16::largest());
        assert_eq!(saturating_add(&r, &RectI16::of(0, 0, i16::MAX, 0)), RectU16::largest());
        assert_eq!(saturating_add(&r, &RectI16::of(0, 0, 0, i16::MAX)), RectU16::largest());
    }
}
