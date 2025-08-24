use crate::matrix::{point::point_i16::PointI16, rect::rect_i16::RectI16};

pub fn add(r: &RectI16, delta: &RectI16) -> RectI16 {
    let min_row = r.min.row.saturating_add(delta.min.row);
    let min_col = r.min.col.saturating_add(delta.min.col);
    let max_row = r.max.row.saturating_add(delta.max.row);
    let max_col = r.max.col.saturating_add(delta.max.col);
    RectI16 { min: PointI16 { row: min_row, col: min_col }, max: PointI16 { row: max_row, col: max_col } }
}

#[cfg(test)]
mod tests {
    use super::add;
    use crate::matrix::rect::rect_i16::RectI16;

    #[test]
    fn test_add() {
        assert_eq!(add(&RectI16::of(0, 0, 12, 15), &RectI16::of(5, 4, 3, 2)), RectI16::of(5, 4, 15, 17));
        assert_eq!(add(&RectI16::of(5, 4, 15, 17), &RectI16::of(-14, -13, -12, -11)), RectI16::of(-9, -9, 3, 6));
    }

    #[test]
    fn add_small_rect_to_bounds() {
        assert_eq!(
            add(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MIN + 12, i16::MIN + 15), &RectI16::of(-2, -5, 9, 7)),
            RectI16::of(i16::MIN, i16::MIN, i16::MIN + 21, i16::MIN + 22)
        );
        assert_eq!(
            add(&RectI16::of(i16::MAX - 12, i16::MAX - 15, i16::MAX - 2, i16::MAX - 5), &RectI16::of(-9, -7, 2, 5)),
            RectI16::of(i16::MAX - 21, i16::MAX - 22, i16::MAX, i16::MAX)
        );
    }

    #[test]
    fn add_big_rect_to_bounds() {
        assert_eq!(add(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX - 2, i16::MAX - 5), &RectI16::of(-2, -5, 2, 5)), RectI16::largest());
        assert_eq!(add(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX), &RectI16::of(-2, -5, 0, 0)), RectI16::largest());
        assert_eq!(add(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5), &RectI16::of(0, 0, 2, 5)), RectI16::largest());
    }

    #[test]
    fn add_small_rect_out_of_bounds() {
        assert_eq!(
            add(&RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30), &RectI16::of(-20, -20, 0, 0)),
            RectI16::of(i16::MIN, i16::MIN, i16::MIN + 20, i16::MIN + 30)
        );
        assert_eq!(
            add(&RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10), &RectI16::of(0, 0, 20, 20)),
            RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX, i16::MAX)
        );
    }

    #[test]
    fn add_big_rect_out_of_bounds() {
        assert_eq!(add(&RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX), &RectI16::of(-20, -20, 0, 0)), RectI16::largest());
        assert_eq!(add(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10), &RectI16::of(0, 0, 20, 20)), RectI16::largest());
    }

    #[test]
    fn add_small_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10), &RectI16::min()), RectI16::min());
        assert_eq!(add(&RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1), &RectI16::max()), RectI16::max());
    }

    #[test]
    fn add_big_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectI16::largest(), &RectI16::largest()), RectI16::largest());

        let r_large = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assert_eq!(add(&r_large, &RectI16::largest()), RectI16::largest());
        assert_eq!(add(&r_large, &RectI16::of(i16::MIN, 0, 0, 0)), RectI16::of(i16::MIN, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1));
        assert_eq!(add(&r_large, &RectI16::of(0, i16::MIN, 0, 0)), RectI16::of(i16::MIN + 1, i16::MIN, i16::MAX - 1, i16::MAX - 1));
        assert_eq!(add(&r_large, &RectI16::of(0, 0, i16::MAX, 0)), RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX - 1));
        assert_eq!(add(&r_large, &RectI16::of(0, 0, 0, i16::MAX)), RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX));
    }
}
