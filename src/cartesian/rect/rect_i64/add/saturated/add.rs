use crate::cartesian::{point::point_i64::PointI64, rect::rect_i64::RectI64};

pub fn saturating_add(r: &RectI64, delta: &RectI64) -> RectI64 {
    let min_x = r.min.x.saturating_add(delta.min.x);
    let min_y = r.min.y.saturating_add(delta.min.y);
    let max_x = r.max.x.saturating_add(delta.max.x);
    let max_y = r.max.y.saturating_add(delta.max.y);
    RectI64 { min: PointI64 { x: min_x, y: min_y }, max: PointI64 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use super::saturating_add;
    use crate::cartesian::rect::rect_i64::RectI64;

    #[test]
    fn test_saturating_add() {
        assert_eq!(saturating_add(&RectI64::of(0, 0, 12, 15), &RectI64::of(5, 4, 3, 2)), RectI64::of(5, 4, 15, 17));
        assert_eq!(saturating_add(&RectI64::of(5, 4, 15, 17), &RectI64::of(-14, -13, -12, -11)), RectI64::of(-9, -9, 3, 6));
    }

    #[test]
    fn saturating_add_to_bounds() {
        assert_eq!(
            saturating_add(&RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX - 2, i64::MAX - 5), &RectI64::of(-2, -5, 2, 5)),
            RectI64::largest()
        );
        assert_eq!(saturating_add(&RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX), &RectI64::of(-2, -5, 0, 0)), RectI64::largest());
        assert_eq!(saturating_add(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5), &RectI64::of(0, 0, 2, 5)), RectI64::largest());
    }

    #[test]
    fn saturating_add_edge_out_of_bounds() {
        let r = RectI64::largest();
        assert_eq!(saturating_add(&r, &RectI64::of(-1, 0, 0, 0)), RectI64::largest());
        assert_eq!(saturating_add(&r, &RectI64::of(0, -1, 0, 0)), RectI64::largest());
        assert_eq!(saturating_add(&r, &RectI64::of(0, 0, 1, 0)), RectI64::largest());
        assert_eq!(saturating_add(&r, &RectI64::of(0, 0, 0, 1)), RectI64::largest());
    }

    #[test]
    fn saturating_add_out_of_bounds() {
        let r = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10);
        assert_eq!(saturating_add(&r, &RectI64::of(-20, 0, 0, 0)), RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10));
        assert_eq!(saturating_add(&r, &RectI64::of(0, -20, 0, 0)), RectI64::of(i64::MIN + 10, i64::MIN, i64::MAX - 10, i64::MAX - 10));
        assert_eq!(saturating_add(&r, &RectI64::of(0, 0, 20, 0)), RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX, i64::MAX - 10));
        assert_eq!(saturating_add(&r, &RectI64::of(0, 0, 0, 20)), RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX));
    }

    #[test]
    fn saturating_add_limits_out_of_bounds() {
        let r = RectI64::largest();
        assert_eq!(saturating_add(&r, &RectI64::of(i64::MIN, 0, 0, 0)), RectI64::largest());
        assert_eq!(saturating_add(&r, &RectI64::of(0, i64::MIN, 0, 0)), RectI64::largest());
        assert_eq!(saturating_add(&r, &RectI64::of(0, 0, i64::MAX, 0)), RectI64::largest());
        assert_eq!(saturating_add(&r, &RectI64::of(0, 0, 0, i64::MAX)), RectI64::largest());
    }
}
