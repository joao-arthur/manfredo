use crate::cartesian::{point::point_i16::PointI16, rect::rect_i16::RectI16};

pub fn try_checked_add(r: &RectI16, delta: &RectI16) -> Option<RectI16> {
    let min_x = r.min.x.checked_add(delta.min.x)?;
    let min_y = r.min.y.checked_add(delta.min.y)?;
    let max_x = r.max.x.checked_add(delta.max.x)?;
    let max_y = r.max.y.checked_add(delta.max.y)?;
    Some(RectI16 { min: PointI16 { x: min_x, y: min_y }, max: PointI16 { x: max_x, y: max_y } })
}

#[cfg(test)]
mod tests {
    use super::try_checked_add;
    use crate::cartesian::rect::rect_i16::RectI16;

    #[test]
    fn test_try_checked_add() {
        assert_eq!(try_checked_add(&RectI16::of(0, 0, 12, 15), &RectI16::of(5, 4, 3, 2)), Some(RectI16::of(5, 4, 15, 17)));
        assert_eq!(try_checked_add(&RectI16::of(5, 4, 15, 17), &RectI16::of(-14, -13, -12, -11)), Some(RectI16::of(-9, -9, 3, 6)));
    }

    #[test]
    fn try_checked_add_to_bounds() {
        assert_eq!(
            try_checked_add(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX - 2, i16::MAX - 5), &RectI16::of(-2, -5, 2, 5)),
            Some(RectI16::largest())
        );
        assert_eq!(
            try_checked_add(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX), &RectI16::of(-2, -5, 0, 0)),
            Some(RectI16::largest())
        );
        assert_eq!(try_checked_add(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5), &RectI16::of(0, 0, 2, 5)), Some(RectI16::largest()));
    }

    #[test]
    fn try_checked_add_edge_out_of_bounds() {
        let r = RectI16::largest();
        assert_eq!(try_checked_add(&r, &RectI16::of(-1, 0, 0, 0)), None);
        assert_eq!(try_checked_add(&r, &RectI16::of(0, -1, 0, 0)), None);
        assert_eq!(try_checked_add(&r, &RectI16::of(0, 0, 1, 0)), None);
        assert_eq!(try_checked_add(&r, &RectI16::of(0, 0, 0, 1)), None);
    }

    #[test]
    fn try_checked_add_out_of_bounds() {
        let r = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10);
        assert_eq!(try_checked_add(&r, &RectI16::of(-20, 0, 0, 0)), None);
        assert_eq!(try_checked_add(&r, &RectI16::of(0, -20, 0, 0)), None);
        assert_eq!(try_checked_add(&r, &RectI16::of(0, 0, 20, 0)), None);
        assert_eq!(try_checked_add(&r, &RectI16::of(0, 0, 0, 20)), None);
    }

    #[test]
    fn try_checked_add_limits_out_of_bounds() {
        let r = RectI16::largest();
        assert_eq!(try_checked_add(&r, &RectI16::of(i16::MIN, 0, 0, 0)), None);
        assert_eq!(try_checked_add(&r, &RectI16::of(0, i16::MIN, 0, 0)), None);
        assert_eq!(try_checked_add(&r, &RectI16::of(0, 0, i16::MAX, 0)), None);
        assert_eq!(try_checked_add(&r, &RectI16::of(0, 0, 0, i16::MAX)), None);
    }
}
