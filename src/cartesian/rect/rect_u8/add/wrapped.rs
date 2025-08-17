use crate::cartesian::{
    point::point_u8::PointU8,
    rect::{rect_i8::RectI8, rect_u8::RectU8},
};

pub fn assign_add(r: &mut RectU8, delta: &RectI8) {
    let min_x = r.min.x.wrapping_add(delta.min.x as u8);
    let min_y = r.min.y.wrapping_add(delta.min.y as u8);
    let max_x = r.max.x.wrapping_add(delta.max.x as u8);
    let max_y = r.max.y.wrapping_add(delta.max.y as u8);
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
}

pub fn add(r: &RectU8, delta: &RectI8) -> RectU8 {
    let min_x = r.min.x.wrapping_add(delta.min.x as u8);
    let min_y = r.min.y.wrapping_add(delta.min.y as u8);
    let max_x = r.max.x.wrapping_add(delta.max.x as u8);
    let max_y = r.max.y.wrapping_add(delta.max.y as u8);
    RectU8 { min: PointU8 { x: min_x, y: min_y }, max: PointU8 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::{rect_i8::RectI8, rect_u8::RectU8};

    use super::{add, assign_add};

    #[test]
    fn test_assign_add() {
        let mut r = RectU8::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI8::of(5, 4, 3, 2));
        assert_eq!(r, RectU8::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI8::of(-4, -3, -2, -1));
        assert_eq!(r, RectU8::of(1, 1, 13, 16));
    }

    #[test]
    fn assign_add_small_rect_to_bounds() {
        let mut min_r = RectU8::of(2, 5, 12, 15);
        assign_add(&mut min_r, &RectI8::of(-2, -5, 9, 7));
        assert_eq!(min_r, RectU8::of(0, 0, 21, 22));

        let mut max_r = RectU8::of(u8::MAX - 12, u8::MAX - 15, u8::MAX - 2, u8::MAX - 5);
        assign_add(&mut max_r, &RectI8::of(-9, -7, 2, 5));
        assert_eq!(max_r, RectU8::of(u8::MAX - 21, u8::MAX - 22, u8::MAX, u8::MAX));
    }

    #[test]
    fn assign_add_big_rect_to_bounds() {
        let mut min_r = RectU8::of(2, 5, u8::MAX - 2, u8::MAX - 5);
        assign_add(&mut min_r, &RectI8::of(-2, -5, 2, 5));
        assert_eq!(min_r, RectU8::largest());

        let mut min_r = RectU8::of(2, 5, u8::MAX, u8::MAX);
        assign_add(&mut min_r, &RectI8::of(-2, -5, 0, 0));
        assert_eq!(min_r, RectU8::largest());

        let mut max_r = RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5);
        assign_add(&mut max_r, &RectI8::of(0, 0, 2, 5));
        assert_eq!(max_r, RectU8::largest());
    }

    #[test]
    fn assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectU8::of(10, 5, 20, 30);
        assign_add(&mut r_min, &RectI8::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectU8::of(u8::MAX - 9, u8::MAX - 14, 20, 30));

        let mut r_max = RectU8::of(u8::MAX - 20, u8::MAX - 30, u8::MAX - 5, u8::MAX - 10);
        assign_add(&mut r_max, &RectI8::of(0, 0, 20, 20));
        assert_eq!(r_max, RectU8::of(u8::MAX - 20, u8::MAX - 30, 14, 9));
    }

    #[test]
    fn assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectU8::of(10, 5, u8::MAX, u8::MAX);
        assign_add(&mut r_min, &RectI8::of(-20, -20, 0, 0));
        assert_eq!(r_min, RectU8::of(u8::MAX - 9, u8::MAX - 14, u8::MAX, u8::MAX));

        let mut r_max = RectU8::of(0, 0, u8::MAX - 5, u8::MAX - 10);
        assign_add(&mut r_max, &RectI8::of(0, 0, 20, 20));
        assert_eq!(r_max, RectU8::of(0, 0, 14, 9));
    }

    #[test]
    fn assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU8::of(1, 1, 10, 10);
        assign_add(&mut r_min, &RectI8::min());
        assert_eq!(r_min, RectU8::of((i8::MAX as u8) + 2, (i8::MAX as u8) + 2, (i8::MAX as u8) + 11, (i8::MAX as u8) + 11));

        let mut r_max = RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX - 1, u8::MAX - 1);
        assign_add(&mut r_max, &RectI8::max());
        assert_eq!(r_max, RectU8::of((i8::MAX as u8) - 11, (i8::MAX as u8) - 11, (i8::MAX as u8) - 2, (i8::MAX as u8) - 2));
    }

    #[test]
    fn assign_add_big_rect_limits_out_of_bounds() {
        let mut r1 = RectU8::largest();
        assign_add(&mut r1, &RectI8::min());
        assert_eq!(r1, RectU8::of((i8::MAX as u8) + 1, (i8::MAX as u8) + 1, i8::MAX as u8, i8::MAX as u8));

        let mut r2 = RectU8::largest();
        assign_add(&mut r2, &RectI8::max());
        assert_eq!(r2, RectU8::of(i8::MAX as u8, i8::MAX as u8, (i8::MAX as u8) - 1, (i8::MAX as u8) - 1));

        let mut r_min = RectU8::of(1, 1, u8::MAX, u8::MAX);
        assign_add(&mut r_min, &RectI8::min());
        assert_eq!(r_min, RectU8::of((i8::MAX as u8) + 2, (i8::MAX as u8) + 2, i8::MAX as u8, i8::MAX as u8));

        let mut r_max = RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1);
        assign_add(&mut r_max, &RectI8::max());
        assert_eq!(r_max, RectU8::of(i8::MAX as u8, i8::MAX as u8, (i8::MAX as u8) - 2, (i8::MAX as u8) - 2));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&RectU8::of(0, 0, 12, 15), &RectI8::of(5, 4, 3, 2)), RectU8::of(5, 4, 15, 17));
        assert_eq!(add(&RectU8::of(5, 4, 15, 17), &RectI8::of(-4, -3, -2, -1)), RectU8::of(1, 1, 13, 16));
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
