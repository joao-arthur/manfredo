use crate::cartesian::{
    point::point_u16::PointU16,
    rect::{rect_i16::RectI16, rect_u16::RectU16},
};

pub fn try_add(r: &RectU16, delta: &RectI16) -> Option<RectU16> {
    let min_x = r.min.x.checked_add_signed(delta.min.x)?;
    let min_y = r.min.y.checked_add_signed(delta.min.y)?;
    let max_x = r.max.x.checked_add_signed(delta.max.x)?;
    let max_y = r.max.y.checked_add_signed(delta.max.y)?;
    Some(RectU16 { min: PointU16 { x: min_x, y: min_y }, max: PointU16 { x: max_x, y: max_y } })
}

#[cfg(test)]
mod tests {
    use super::try_add;
    use crate::cartesian::rect::{rect_i16::RectI16, rect_u16::RectU16};

    #[test]
    fn test_try_add() {
        assert_eq!(try_add(&RectU16::of(0, 0, 12, 15), &RectI16::of(5, 4, 3, 2)), Some(RectU16::of(5, 4, 15, 17)));
        assert_eq!(try_add(&RectU16::of(5, 4, 15, 20), &RectI16::of(-4, -3, -2, -1)), Some(RectU16::of(1, 1, 13, 19)));
    }

    #[test]
    fn try_add_to_bounds() {
        assert_eq!(try_add(&RectU16::of(2, 5, u16::MAX - 2, u16::MAX - 5), &RectI16::of(-2, -5, 2, 5)), Some(RectU16::largest()));
        assert_eq!(try_add(&RectU16::of(2, 5, u16::MAX, u16::MAX), &RectI16::of(-2, -5, 0, 0)), Some(RectU16::largest()));
        assert_eq!(try_add(&RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5), &RectI16::of(0, 0, 2, 5)), Some(RectU16::largest()));
    }

    #[test]
    fn try_add_edge_out_of_bounds() {
        let r = RectU16::largest();
        assert_eq!(try_add(&r, &RectI16::of(-1, 0, 0, 0)), None);
        assert_eq!(try_add(&r, &RectI16::of(0, -1, 0, 0)), None);
        assert_eq!(try_add(&r, &RectI16::of(0, 0, 1, 0)), None);
        assert_eq!(try_add(&r, &RectI16::of(0, 0, 0, 1)), None);
    }

    #[test]
    fn try_add_out_of_bounds() {
        let r = RectU16::of(10, 10, u16::MAX - 10, u16::MAX - 10);
        assert_eq!(try_add(&r, &RectI16::of(-20, 0, 0, 0)), None);
        assert_eq!(try_add(&r, &RectI16::of(0, -20, 0, 0)), None);
        assert_eq!(try_add(&r, &RectI16::of(0, 0, 20, 0)), None);
        assert_eq!(try_add(&r, &RectI16::of(0, 0, 0, 20)), None);
    }

    #[test]
    fn try_add_limits_out_of_bounds() {
        let r = RectU16::largest();
        assert_eq!(try_add(&r, &RectI16::of(i16::MIN, 0, 0, 0)), None);
        assert_eq!(try_add(&r, &RectI16::of(0, i16::MIN, 0, 0)), None);
        assert_eq!(try_add(&r, &RectI16::of(0, 0, i16::MAX, 0)), None);
        assert_eq!(try_add(&r, &RectI16::of(0, 0, 0, i16::MAX)), None);
    }
}
