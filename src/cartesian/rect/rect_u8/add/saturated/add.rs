use crate::cartesian::{
    point::point_u8::PointU8,
    rect::{rect_i8::RectI8, rect_u8::RectU8},
};

pub fn add(r: &RectU8, delta: &RectI8) -> RectU8 {
    let min_x = r.min.x.saturating_add_signed(delta.min.x);
    let min_y = r.min.y.saturating_add_signed(delta.min.y);
    let max_x = r.max.x.saturating_add_signed(delta.max.x);
    let max_y = r.max.y.saturating_add_signed(delta.max.y);
    RectU8 { min: PointU8 { x: min_x, y: min_y }, max: PointU8 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use super::add;
    use crate::cartesian::rect::{rect_i8::RectI8, rect_u8::RectU8};

    #[test]
    fn test_add() {
        assert_eq!(add(&RectU8::of(0, 0, 12, 15), &RectI8::of(5, 4, 3, 2)), RectU8::of(5, 4, 15, 17));
        assert_eq!(add(&RectU8::of(5, 4, 15, 20), &RectI8::of(-4, -3, -2, -1)), RectU8::of(1, 1, 13, 19));
    }

    #[test]
    fn add_to_bounds() {
        assert_eq!(add(&RectU8::of(2, 5, u8::MAX - 2, u8::MAX - 5), &RectI8::of(-2, -5, 2, 5)), RectU8::largest());
        assert_eq!(add(&RectU8::of(2, 5, u8::MAX, u8::MAX), &RectI8::of(-2, -5, 0, 0)), RectU8::largest());
        assert_eq!(add(&RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5), &RectI8::of(0, 0, 2, 5)), RectU8::largest());
    }

    #[test]
    fn add_edge_out_of_bounds() {
        let r = RectU8::largest();
        assert_eq!(add(&r, &RectI8::of(-1, 0, 0, 0)), RectU8::largest());
        assert_eq!(add(&r, &RectI8::of(0, -1, 0, 0)), RectU8::largest());
        assert_eq!(add(&r, &RectI8::of(0, 0, 1, 0)), RectU8::largest());
        assert_eq!(add(&r, &RectI8::of(0, 0, 0, 1)), RectU8::largest());
    }

    #[test]
    fn add_out_of_bounds() {
        let r = RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10);
        assert_eq!(add(&r, &RectI8::of(-20, 0, 0, 0)), RectU8::of(0, 10, u8::MAX - 10, u8::MAX - 10));
        assert_eq!(add(&r, &RectI8::of(0, -20, 0, 0)), RectU8::of(10, 0, u8::MAX - 10, u8::MAX - 10));
        assert_eq!(add(&r, &RectI8::of(0, 0, 20, 0)), RectU8::of(10, 10, u8::MAX, u8::MAX - 10));
        assert_eq!(add(&r, &RectI8::of(0, 0, 0, 20)), RectU8::of(10, 10, u8::MAX - 10, u8::MAX));
    }

    #[test]
    fn add_limits_out_of_bounds() {
        let r = RectU8::largest();
        assert_eq!(add(&r, &RectI8::of(i8::MIN, 0, 0, 0)), RectU8::largest());
        assert_eq!(add(&r, &RectI8::of(0, i8::MIN, 0, 0)), RectU8::largest());
        assert_eq!(add(&r, &RectI8::of(0, 0, i8::MAX, 0)), RectU8::largest());
        assert_eq!(add(&r, &RectI8::of(0, 0, 0, i8::MAX)), RectU8::largest());
    }
}
