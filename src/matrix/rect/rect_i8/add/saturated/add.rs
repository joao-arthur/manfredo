use crate::matrix::{point::point_i8::PointI8, rect::rect_i8::RectI8};

pub fn add(r: &RectI8, delta: &RectI8) -> RectI8 {
    let min_row = r.min.row.saturating_add(delta.min.row);
    let min_col = r.min.col.saturating_add(delta.min.col);
    let max_row = r.max.row.saturating_add(delta.max.row);
    let max_col = r.max.col.saturating_add(delta.max.col);
    RectI8 { min: PointI8 { row: min_row, col: min_col }, max: PointI8 { row: max_row, col: max_col } }
}

#[cfg(test)]
mod tests {
    use super::add;
    use crate::matrix::rect::rect_i8::RectI8;

    #[test]
    fn test_add() {
        assert_eq!(add(&RectI8::of(0, 0, 12, 15), &RectI8::of(5, 4, 3, 2)), RectI8::of(5, 4, 15, 17));
        assert_eq!(add(&RectI8::of(5, 4, 15, 17), &RectI8::of(-14, -13, -12, -11)), RectI8::of(-9, -9, 3, 6));
    }

    #[test]
    fn add_small_rect_to_bounds() {
        assert_eq!(
            add(&RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MIN + 12, i8::MIN + 15), &RectI8::of(-2, -5, 9, 7)),
            RectI8::of(i8::MIN, i8::MIN, i8::MIN + 21, i8::MIN + 22)
        );
        assert_eq!(
            add(&RectI8::of(i8::MAX - 12, i8::MAX - 15, i8::MAX - 2, i8::MAX - 5), &RectI8::of(-9, -7, 2, 5)),
            RectI8::of(i8::MAX - 21, i8::MAX - 22, i8::MAX, i8::MAX)
        );
    }

    #[test]
    fn add_big_rect_to_bounds() {
        assert_eq!(add(&RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX - 2, i8::MAX - 5), &RectI8::of(-2, -5, 2, 5)), RectI8::largest());
        assert_eq!(add(&RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX), &RectI8::of(-2, -5, 0, 0)), RectI8::largest());
        assert_eq!(add(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5), &RectI8::of(0, 0, 2, 5)), RectI8::largest());
    }

    #[test]
    fn add_small_rect_out_of_bounds() {
        assert_eq!(
            add(&RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MIN + 20, i8::MIN + 30), &RectI8::of(-20, -20, 0, 0)),
            RectI8::of(i8::MIN, i8::MIN, i8::MIN + 20, i8::MIN + 30)
        );
        assert_eq!(
            add(&RectI8::of(i8::MAX - 20, i8::MAX - 30, i8::MAX - 5, i8::MAX - 10), &RectI8::of(0, 0, 20, 20)),
            RectI8::of(i8::MAX - 20, i8::MAX - 30, i8::MAX, i8::MAX)
        );
    }

    #[test]
    fn add_big_rect_out_of_bounds() {
        assert_eq!(add(&RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MAX, i8::MAX), &RectI8::of(-20, -20, 0, 0)), RectI8::largest());
        assert_eq!(add(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 5, i8::MAX - 10), &RectI8::of(0, 0, 20, 20)), RectI8::largest());
    }

    #[test]
    fn add_small_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MIN + 10, i8::MIN + 10), &RectI8::min()), RectI8::min());
        assert_eq!(add(&RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX - 1, i8::MAX - 1), &RectI8::max()), RectI8::max());
    }

    #[test]
    fn add_big_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectI8::largest(), &RectI8::largest()), RectI8::largest());

        let r_large = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1);
        assert_eq!(add(&r_large, &RectI8::largest()), RectI8::largest());
        assert_eq!(add(&r_large, &RectI8::of(i8::MIN, 0, 0, 0)), RectI8::of(i8::MIN, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1));
        assert_eq!(add(&r_large, &RectI8::of(0, i8::MIN, 0, 0)), RectI8::of(i8::MIN + 1, i8::MIN, i8::MAX - 1, i8::MAX - 1));
        assert_eq!(add(&r_large, &RectI8::of(0, 0, i8::MAX, 0)), RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX - 1));
        assert_eq!(add(&r_large, &RectI8::of(0, 0, 0, i8::MAX)), RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX));
    }
}
