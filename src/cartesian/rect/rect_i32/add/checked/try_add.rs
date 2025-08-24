use crate::cartesian::{point::point_i32::PointI32, rect::rect_i32::RectI32};

pub fn try_add(r: &RectI32, delta: &RectI32) -> Option<RectI32> {
    let min_x = r.min.x.checked_add(delta.min.x)?;
    let min_y = r.min.y.checked_add(delta.min.y)?;
    let max_x = r.max.x.checked_add(delta.max.x)?;
    let max_y = r.max.y.checked_add(delta.max.y)?;
    Some(RectI32 { min: PointI32 { x: min_x, y: min_y }, max: PointI32 { x: max_x, y: max_y } })
}

#[cfg(test)]
mod tests {
    use super::try_add;
    use crate::cartesian::rect::rect_i32::RectI32;

    #[test]
    fn test_try_add() {
        assert_eq!(try_add(&RectI32::of(0, 0, 12, 15), &RectI32::of(5, 4, 3, 2)), Some(RectI32::of(5, 4, 15, 17)));
        assert_eq!(try_add(&RectI32::of(5, 4, 15, 17), &RectI32::of(-14, -13, -12, -11)), Some(RectI32::of(-9, -9, 3, 6)));
    }

    #[test]
    fn try_add_big_rect_to_bounds() {
        assert_eq!(
            try_add(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX - 2, i32::MAX - 5), &RectI32::of(-2, -5, 2, 5)),
            Some(RectI32::largest())
        );
        assert_eq!(try_add(&RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX), &RectI32::of(-2, -5, 0, 0)), Some(RectI32::largest()));
        assert_eq!(try_add(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5), &RectI32::of(0, 0, 2, 5)), Some(RectI32::largest()));
    }

    #[test]
    fn try_add_edge_out_of_bounds() {
        let r = RectI32::largest();
        assert_eq!(try_add(&r, &RectI32::of(-1, 0, 0, 0)), None);
        assert_eq!(try_add(&r, &RectI32::of(0, -1, 0, 0)), None);
        assert_eq!(try_add(&r, &RectI32::of(0, 0, 1, 0)), None);
        assert_eq!(try_add(&r, &RectI32::of(0, 0, 0, 1)), None);
    }

    #[test]
    fn try_add_out_of_bounds() {
        let r = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 10, i32::MAX - 10);
        assert_eq!(try_add(&r, &RectI32::of(-20, 0, 0, 0)), None);
        assert_eq!(try_add(&r, &RectI32::of(0, -20, 0, 0)), None);
        assert_eq!(try_add(&r, &RectI32::of(0, 0, 20, 0)), None);
        assert_eq!(try_add(&r, &RectI32::of(0, 0, 0, 20)), None);
    }

    #[test]
    fn try_add_limits_out_of_bounds() {
        let r = RectI32::largest();
        assert_eq!(try_add(&r, &RectI32::of(i32::MIN, 0, 0, 0)), None);
        assert_eq!(try_add(&r, &RectI32::of(0, i32::MIN, 0, 0)), None);
        assert_eq!(try_add(&r, &RectI32::of(0, 0, i32::MAX, 0)), None);
        assert_eq!(try_add(&r, &RectI32::of(0, 0, 0, i32::MAX)), None);
    }
}
