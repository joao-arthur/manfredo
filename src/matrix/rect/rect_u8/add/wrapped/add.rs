use crate::matrix::{
    point::point_u8::PointU8,
    rect::{rect_i8::RectI8, rect_u8::RectU8},
};

pub fn add(r: &RectU8, delta: &RectI8) -> RectU8 {
    let min_row = r.min.row.wrapping_add_signed(delta.min.row);
    let min_col = r.min.col.wrapping_add_signed(delta.min.col);
    let max_row = r.max.row.wrapping_add_signed(delta.max.row);
    let max_col = r.max.col.wrapping_add_signed(delta.max.col);
    RectU8 { min: PointU8 { row: min_row, col: min_col }, max: PointU8 { row: max_row, col: max_col } }
}

#[cfg(test)]
mod tests {
    use super::add;
    use crate::matrix::rect::{rect_i8::RectI8, rect_u8::RectU8};

    #[test]
    fn test_add() {
        assert_eq!(add(&RectU8::of(0, 0, 12, 15), &RectI8::of(5, 4, 3, 2)), RectU8::of(5, 4, 15, 17));
        assert_eq!(add(&RectU8::of(5, 4, 15, 20), &RectI8::of(-4, -3, -2, -1)), RectU8::of(1, 1, 13, 19));
    }

    #[test]
    fn add_small_rect_to_bounds() {
        assert_eq!(add(&RectU8::of(2, 5, 12, 15), &RectI8::of(-2, -5, 9, 7)), RectU8::of(0, 0, 21, 22));
        assert_eq!(
            add(&RectU8::of(u8::MAX - 12, u8::MAX - 15, u8::MAX - 2, u8::MAX - 5), &RectI8::of(-9, -7, 2, 5)),
            RectU8::of(u8::MAX - 21, u8::MAX - 22, u8::MAX, u8::MAX)
        );
    }

    #[test]
    fn add_big_rect_to_bounds() {
        assert_eq!(add(&RectU8::of(2, 5, u8::MAX - 2, u8::MAX - 5), &RectI8::of(-2, -5, 2, 5)), RectU8::largest());
        assert_eq!(add(&RectU8::of(2, 5, u8::MAX, u8::MAX), &RectI8::of(-2, -5, 0, 0)), RectU8::largest());
        assert_eq!(add(&RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5), &RectI8::of(0, 0, 2, 5)), RectU8::largest());
    }

    #[test]
    fn add_small_rect_out_of_bounds() {
        assert_eq!(add(&RectU8::of(10, 5, 20, 30), &RectI8::of(-20, -20, 0, 0)), RectU8::of(u8::MAX - 9, u8::MAX - 14, 20, 30));
        assert_eq!(
            add(&RectU8::of(u8::MAX - 20, u8::MAX - 30, u8::MAX - 5, u8::MAX - 10), &RectI8::of(0, 0, 20, 20)),
            RectU8::of(u8::MAX - 20, u8::MAX - 30, 14, 9)
        );
    }

    #[test]
    fn add_big_rect_out_of_bounds() {
        assert_eq!(add(&RectU8::of(10, 5, u8::MAX, u8::MAX), &RectI8::of(-20, -20, 0, 0)), RectU8::of(u8::MAX - 9, u8::MAX - 14, u8::MAX, u8::MAX));
        assert_eq!(add(&RectU8::of(0, 0, u8::MAX - 5, u8::MAX - 10), &RectI8::of(0, 0, 20, 20)), RectU8::of(0, 0, 14, 9));
    }

    #[test]
    fn add_small_rect_limits_out_of_bounds() {
        assert_eq!(
            add(&RectU8::of(1, 1, 10, 10), &RectI8::min()),
            RectU8::of((i8::MAX as u8) + 2, (i8::MAX as u8) + 2, (i8::MAX as u8) + 11, (i8::MAX as u8) + 11)
        );
        assert_eq!(
            add(&RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX - 1, u8::MAX - 1), &RectI8::max()),
            RectU8::of((i8::MAX as u8) - 11, (i8::MAX as u8) - 11, (i8::MAX as u8) - 2, (i8::MAX as u8) - 2)
        );
    }

    #[test]
    fn add_big_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectU8::largest(), &RectI8::min()), RectU8::of((i8::MAX as u8) + 1, (i8::MAX as u8) + 1, i8::MAX as u8, i8::MAX as u8));
        assert_eq!(add(&RectU8::largest(), &RectI8::max()), RectU8::of(i8::MAX as u8, i8::MAX as u8, (i8::MAX as u8) - 1, (i8::MAX as u8) - 1));
        assert_eq!(
            add(&RectU8::of(1, 1, u8::MAX, u8::MAX), &RectI8::min()),
            RectU8::of((i8::MAX as u8) + 2, (i8::MAX as u8) + 2, i8::MAX as u8, i8::MAX as u8)
        );
        assert_eq!(
            add(&RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1), &RectI8::max()),
            RectU8::of(i8::MAX as u8, i8::MAX as u8, (i8::MAX as u8) - 2, (i8::MAX as u8) - 2)
        );
    }
}
