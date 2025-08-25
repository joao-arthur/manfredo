use crate::cartesian::{
    point::point_u32::PointU32,
    rect::{rect_i32::RectI32, rect_u32::RectU32},
};

pub fn saturating_add(r: &RectU32, delta: &RectI32) -> RectU32 {
    let min_x = r.min.x.saturating_add_signed(delta.min.x);
    let min_y = r.min.y.saturating_add_signed(delta.min.y);
    let max_x = r.max.x.saturating_add_signed(delta.max.x);
    let max_y = r.max.y.saturating_add_signed(delta.max.y);
    RectU32 { min: PointU32 { x: min_x, y: min_y }, max: PointU32 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use super::saturating_add;
    use crate::cartesian::rect::{rect_i32::RectI32, rect_u32::RectU32};

    #[test]
    fn test_saturating_add() {
        assert_eq!(saturating_add(&RectU32::of(0, 0, 12, 15), &RectI32::of(5, 4, 3, 2)), RectU32::of(5, 4, 15, 17));
        assert_eq!(saturating_add(&RectU32::of(5, 4, 15, 20), &RectI32::of(-4, -3, -2, -1)), RectU32::of(1, 1, 13, 19));
    }

    #[test]
    fn saturating_add_to_bounds() {
        assert_eq!(saturating_add(&RectU32::of(2, 5, u32::MAX - 2, u32::MAX - 5), &RectI32::of(-2, -5, 2, 5)), RectU32::largest());
        assert_eq!(saturating_add(&RectU32::of(2, 5, u32::MAX, u32::MAX), &RectI32::of(-2, -5, 0, 0)), RectU32::largest());
        assert_eq!(saturating_add(&RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5), &RectI32::of(0, 0, 2, 5)), RectU32::largest());
    }

    #[test]
    fn saturating_add_edge_out_of_bounds() {
        let r = RectU32::largest();
        assert_eq!(saturating_add(&r, &RectI32::of(-1, 0, 0, 0)), RectU32::largest());
        assert_eq!(saturating_add(&r, &RectI32::of(0, -1, 0, 0)), RectU32::largest());
        assert_eq!(saturating_add(&r, &RectI32::of(0, 0, 1, 0)), RectU32::largest());
        assert_eq!(saturating_add(&r, &RectI32::of(0, 0, 0, 1)), RectU32::largest());
    }

    #[test]
    fn saturating_add_out_of_bounds() {
        let r = RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 10);
        assert_eq!(saturating_add(&r, &RectI32::of(-20, 0, 0, 0)), RectU32::of(0, 10, u32::MAX - 10, u32::MAX - 10));
        assert_eq!(saturating_add(&r, &RectI32::of(0, -20, 0, 0)), RectU32::of(10, 0, u32::MAX - 10, u32::MAX - 10));
        assert_eq!(saturating_add(&r, &RectI32::of(0, 0, 20, 0)), RectU32::of(10, 10, u32::MAX, u32::MAX - 10));
        assert_eq!(saturating_add(&r, &RectI32::of(0, 0, 0, 20)), RectU32::of(10, 10, u32::MAX - 10, u32::MAX));
    }

    #[test]
    fn saturating_add_limits_out_of_bounds() {
        let r = RectU32::largest();
        assert_eq!(saturating_add(&r, &RectI32::of(i32::MIN, 0, 0, 0)), RectU32::largest());
        assert_eq!(saturating_add(&r, &RectI32::of(0, i32::MIN, 0, 0)), RectU32::largest());
        assert_eq!(saturating_add(&r, &RectI32::of(0, 0, i32::MAX, 0)), RectU32::largest());
        assert_eq!(saturating_add(&r, &RectI32::of(0, 0, 0, i32::MAX)), RectU32::largest());
    }
}
