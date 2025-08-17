use crate::cartesian::{
    point::{point_i16::PointI16, point_u16::PointU16},
    rect::rect_u16::{RectU16, delta_x, delta_y},
};

pub fn assign_add(r: &mut RectU16, delta: &PointI16) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i32::from(r.min.x) + i32::from(delta.x);
    let temp_min_y = i32::from(r.min.y) + i32::from(delta.y);
    let clamped_x = temp_min_x.clamp(0, i32::from(u16::MAX) - i32::from(dx));
    let clamped_y = temp_min_y.clamp(0, i32::from(u16::MAX) - i32::from(dy));
    let min_x = clamped_x as u16;
    let min_y = clamped_y as u16;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = min_x + dx;
    r.max.y = min_y + dy;
}

pub fn add(r: &RectU16, delta: &PointI16) -> RectU16 {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i32::from(r.min.x) + i32::from(delta.x);
    let temp_min_y = i32::from(r.min.y) + i32::from(delta.y);
    let clamped_x = temp_min_x.clamp(0, i32::from(u16::MAX) - i32::from(dx));
    let clamped_y = temp_min_y.clamp(0, i32::from(u16::MAX) - i32::from(dy));
    let min_x = clamped_x as u16;
    let min_y = clamped_y as u16;
    let max_x = min_x + dx;
    let max_y = min_y + dy;
    RectU16 { min: PointU16 { x: min_x, y: min_y }, max: PointU16 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{point::point_i16::PointI16, rect::rect_u16::RectU16};

    use super::{add, assign_add};

    #[test]
    fn test_assign_add() {
        let mut r = RectU16::of(0, 0, 12, 15);
        assign_add(&mut r, &PointI16::of(5, 4));
        assert_eq!(r, RectU16::of(5, 4, 17, 19));
        assign_add(&mut r, &PointI16::of(-4, -2));
        assert_eq!(r, RectU16::of(1, 2, 13, 17));
    }

    #[test]
    fn assign_add_small_rect_to_bounds() {
        let mut min_r = RectU16::of(2, 5, 12, 15);
        assign_add(&mut min_r, &PointI16::of(-2, -5));
        assert_eq!(min_r, RectU16::of(0, 0, 10, 10));

        let mut max_r = RectU16::of(u16::MAX - 12, u16::MAX - 15, u16::MAX - 2, u16::MAX - 5);
        assign_add(&mut max_r, &PointI16::of(2, 5));
        assert_eq!(max_r, RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX, u16::MAX));
    }

    #[test]
    fn assign_add_big_rect_to_bounds() {
        let mut min_r = RectU16::of(2, 5, u16::MAX, u16::MAX);
        assign_add(&mut min_r, &PointI16::of(-2, -5));
        assert_eq!(min_r, RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5));

        let mut max_r = RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5);
        assign_add(&mut max_r, &PointI16::of(2, 5));
        assert_eq!(max_r, RectU16::of(2, 5, u16::MAX, u16::MAX));
    }

    #[test]
    fn assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectU16::of(10, 5, 20, 30);
        assign_add(&mut r_min, &PointI16::of(-20, -20));
        assert_eq!(r_min, RectU16::of(0, 0, 10, 25));

        let mut r_max = RectU16::of(u16::MAX - 20, u16::MAX - 30, u16::MAX - 5, u16::MAX - 10);
        assign_add(&mut r_max, &PointI16::of(20, 20));
        assert_eq!(r_max, RectU16::of(u16::MAX - 15, u16::MAX - 20, u16::MAX, u16::MAX));
    }

    #[test]
    fn assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectU16::of(10, 5, u16::MAX, u16::MAX);
        assign_add(&mut r_min, &PointI16::of(-20, -20));
        assert_eq!(r_min, RectU16::of(0, 0, u16::MAX - 10, u16::MAX - 5));

        let mut r_max = RectU16::of(0, 0, u16::MAX - 5, u16::MAX - 10);
        assign_add(&mut r_max, &PointI16::of(20, 20));
        assert_eq!(r_max, RectU16::of(5, 10, u16::MAX, u16::MAX));
    }

    #[test]
    fn assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU16::of(1, 1, 10, 10);
        assign_add(&mut r_min, &PointI16::min());
        assert_eq!(r_min, RectU16::of(0, 0, 9, 9));

        let mut r_max = RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX - 1, u16::MAX - 1);
        assign_add(&mut r_max, &PointI16::max());
        assert_eq!(r_max, RectU16::of(u16::MAX - 9, u16::MAX - 9, u16::MAX, u16::MAX));
    }

    #[test]
    fn assign_add_big_rect_limits_out_of_bounds() {
        let mut r = RectU16::largest();
        assign_add(&mut r, &PointI16::min());
        assert_eq!(r, RectU16::largest());
        assign_add(&mut r, &PointI16::max());
        assert_eq!(r, RectU16::largest());

        let mut r_min = RectU16::of(1, 1, u16::MAX, u16::MAX);
        assign_add(&mut r_min, &PointI16::min());
        assert_eq!(r_min, RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1));

        let mut r_max = RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1);
        assign_add(&mut r_max, &PointI16::max());
        assert_eq!(r_max, RectU16::of(1, 1, u16::MAX, u16::MAX));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&RectU16::of(0, 0, 12, 15), &PointI16::of(5, 4)), RectU16::of(5, 4, 17, 19));
        assert_eq!(add(&RectU16::of(5, 4, 17, 19), &PointI16::of(-4, -2)), RectU16::of(1, 2, 13, 17));
    }

    #[test]
    fn add_small_rect_to_bounds() {
        assert_eq!(add(&RectU16::of(2, 5, 12, 15), &PointI16::of(-2, -5)), RectU16::of(0, 0, 10, 10));
        assert_eq!(
            add(&RectU16::of(u16::MAX - 12, u16::MAX - 15, u16::MAX - 2, u16::MAX - 5), &PointI16::of(2, 5)),
            RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX, u16::MAX)
        );
    }

    #[test]
    fn add_big_rect_to_bounds() {
        assert_eq!(add(&RectU16::of(2, 5, u16::MAX, u16::MAX), &PointI16::of(-2, -5)), RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5));
        assert_eq!(add(&RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5), &PointI16::of(2, 5)), RectU16::of(2, 5, u16::MAX, u16::MAX));
    }

    #[test]
    fn add_small_rect_out_of_bounds() {
        assert_eq!(add(&RectU16::of(10, 5, 20, 30), &PointI16::of(-20, -20)), RectU16::of(0, 0, 10, 25));
        assert_eq!(
            add(&RectU16::of(u16::MAX - 20, u16::MAX - 30, u16::MAX - 5, u16::MAX - 10), &PointI16::of(20, 20)),
            RectU16::of(u16::MAX - 15, u16::MAX - 20, u16::MAX, u16::MAX)
        );
    }

    #[test]
    fn add_big_rect_out_of_bounds() {
        assert_eq!(add(&RectU16::of(10, 5, u16::MAX, u16::MAX), &PointI16::of(-20, -20)), RectU16::of(0, 0, u16::MAX - 10, u16::MAX - 5));
        assert_eq!(add(&RectU16::of(0, 0, u16::MAX - 5, u16::MAX - 10), &PointI16::of(20, 20)), RectU16::of(5, 10, u16::MAX, u16::MAX));
    }

    #[test]
    fn add_small_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectU16::of(1, 1, 10, 10), &PointI16::min()), RectU16::of(0, 0, 9, 9));
        assert_eq!(
            add(&RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX - 1, u16::MAX - 1), &PointI16::max()),
            RectU16::of(u16::MAX - 9, u16::MAX - 9, u16::MAX, u16::MAX)
        );
    }

    #[test]
    fn add_big_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectU16::of(1, 1, u16::MAX, u16::MAX), &PointI16::min()), RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1));
        assert_eq!(add(&RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1), &PointI16::max()), RectU16::of(1, 1, u16::MAX, u16::MAX));
    }
}
