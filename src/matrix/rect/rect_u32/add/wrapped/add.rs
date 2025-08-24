use crate::matrix::{
    point::point_u32::PointU32,
    rect::{rect_i32::RectI32, rect_u32::RectU32},
};

pub fn add(r: &RectU32, delta: &RectI32) -> RectU32 {
    let min_row = r.min.row.wrapping_add_signed(delta.min.row);
    let min_col = r.min.col.wrapping_add_signed(delta.min.col);
    let max_row = r.max.row.wrapping_add_signed(delta.max.row);
    let max_col = r.max.col.wrapping_add_signed(delta.max.col);
    RectU32 { min: PointU32 { row: min_row, col: min_col }, max: PointU32 { row: max_row, col: max_col } }
}

#[cfg(test)]
mod tests {
    use super::add;
    use crate::matrix::rect::{rect_i32::RectI32, rect_u32::RectU32};

    #[test]
    fn test_add() {
        assert_eq!(add(&RectU32::of(0, 0, 12, 15), &RectI32::of(5, 4, 3, 2)), RectU32::of(5, 4, 15, 17));
        assert_eq!(add(&RectU32::of(5, 4, 15, 20), &RectI32::of(-4, -3, -2, -1)), RectU32::of(1, 1, 13, 19));
    }

    #[test]
    fn add_small_rect_to_bounds() {
        assert_eq!(add(&RectU32::of(2, 5, 12, 15), &RectI32::of(-2, -5, 9, 7)), RectU32::of(0, 0, 21, 22));
        assert_eq!(
            add(&RectU32::of(u32::MAX - 12, u32::MAX - 15, u32::MAX - 2, u32::MAX - 5), &RectI32::of(-9, -7, 2, 5)),
            RectU32::of(u32::MAX - 21, u32::MAX - 22, u32::MAX, u32::MAX)
        );
    }

    #[test]
    fn add_big_rect_to_bounds() {
        assert_eq!(add(&RectU32::of(2, 5, u32::MAX - 2, u32::MAX - 5), &RectI32::of(-2, -5, 2, 5)), RectU32::largest());
        assert_eq!(add(&RectU32::of(2, 5, u32::MAX, u32::MAX), &RectI32::of(-2, -5, 0, 0)), RectU32::largest());
        assert_eq!(add(&RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5), &RectI32::of(0, 0, 2, 5)), RectU32::largest());
    }

    #[test]
    fn add_small_rect_out_of_bounds() {
        assert_eq!(add(&RectU32::of(10, 5, 20, 30), &RectI32::of(-20, -20, 0, 0)), RectU32::of(u32::MAX - 9, u32::MAX - 14, 20, 30));
        assert_eq!(
            add(&RectU32::of(u32::MAX - 20, u32::MAX - 30, u32::MAX - 5, u32::MAX - 10), &RectI32::of(0, 0, 20, 20)),
            RectU32::of(u32::MAX - 20, u32::MAX - 30, 14, 9)
        );
    }

    #[test]
    fn add_big_rect_out_of_bounds() {
        assert_eq!(
            add(&RectU32::of(10, 5, u32::MAX, u32::MAX), &RectI32::of(-20, -20, 0, 0)),
            RectU32::of(u32::MAX - 9, u32::MAX - 14, u32::MAX, u32::MAX)
        );
        assert_eq!(add(&RectU32::of(0, 0, u32::MAX - 5, u32::MAX - 10), &RectI32::of(0, 0, 20, 20)), RectU32::of(0, 0, 14, 9));
    }

    #[test]
    fn add_small_rect_limits_out_of_bounds() {
        assert_eq!(
            add(&RectU32::of(1, 1, 10, 10), &RectI32::min()),
            RectU32::of((i32::MAX as u32) + 2, (i32::MAX as u32) + 2, (i32::MAX as u32) + 11, (i32::MAX as u32) + 11)
        );
        assert_eq!(
            add(&RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1), &RectI32::max()),
            RectU32::of((i32::MAX as u32) - 11, (i32::MAX as u32) - 11, (i32::MAX as u32) - 2, (i32::MAX as u32) - 2)
        );
    }

    #[test]
    fn add_big_rect_limits_out_of_bounds() {
        assert_eq!(
            add(&RectU32::largest(), &RectI32::min()),
            RectU32::of((i32::MAX as u32) + 1, (i32::MAX as u32) + 1, i32::MAX as u32, i32::MAX as u32)
        );
        assert_eq!(
            add(&RectU32::largest(), &RectI32::max()),
            RectU32::of(i32::MAX as u32, i32::MAX as u32, (i32::MAX as u32) - 1, (i32::MAX as u32) - 1)
        );
        assert_eq!(
            add(&RectU32::of(1, 1, u32::MAX, u32::MAX), &RectI32::min()),
            RectU32::of((i32::MAX as u32) + 2, (i32::MAX as u32) + 2, i32::MAX as u32, i32::MAX as u32)
        );
        assert_eq!(
            add(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1), &RectI32::max()),
            RectU32::of(i32::MAX as u32, i32::MAX as u32, (i32::MAX as u32) - 2, (i32::MAX as u32) - 2)
        );
    }
}
