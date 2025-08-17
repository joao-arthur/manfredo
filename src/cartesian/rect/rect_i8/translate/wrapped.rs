use crate::cartesian::{
    point::point_i8::PointI8,
    rect::rect_i8::{RectI8, delta_x, delta_y},
};

pub fn assign_add(r: &mut RectI8, delta: &PointI8) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let min_x = r.min.x.wrapping_add(delta.x);
    let min_y = r.min.y.wrapping_add(delta.y);
    let max_x = min_x.wrapping_add(dx as i8);
    let max_y = min_y.wrapping_add(dy as i8);
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
}

pub fn add(r: &RectI8, delta: &PointI8) -> RectI8 {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let min_x = r.min.x.wrapping_add(delta.x);
    let min_y = r.min.y.wrapping_add(delta.y);
    let max_x = min_x.wrapping_add(dx as i8);
    let max_y = min_y.wrapping_add(dy as i8);
    RectI8 { min: PointI8 { x: min_x, y: min_y }, max: PointI8 { x: max_x, y: max_y } }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{point::point_i8::PointI8, rect::rect_i8::RectI8};

    use super::{add, assign_add};

    #[test]
    fn test_assign_add() {
        let mut r = RectI8::of(0, 0, 12, 15);
        assign_add(&mut r, &PointI8::of(5, 4));
        assert_eq!(r, RectI8::of(5, 4, 17, 19));
        assign_add(&mut r, &PointI8::of(-4, -2));
        assert_eq!(r, RectI8::of(1, 2, 13, 17));
    }

    #[test]
    fn assign_add_small_rect_to_bounds() {
        let mut min_r = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MIN + 12, i8::MIN + 15);
        assign_add(&mut min_r, &PointI8::of(-2, -5));
        assert_eq!(min_r, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10));

        let mut max_r = RectI8::of(i8::MAX - 12, i8::MAX - 15, i8::MAX - 2, i8::MAX - 5);
        assign_add(&mut max_r, &PointI8::of(2, 5));
        assert_eq!(max_r, RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX, i8::MAX));
    }

    #[test]
    fn assign_add_big_rect_to_bounds() {
        let mut min_r = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX);
        assign_add(&mut min_r, &PointI8::of(-2, -5));
        assert_eq!(min_r, RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5));

        let mut max_r = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5);
        assign_add(&mut max_r, &PointI8::of(2, 5));
        assert_eq!(max_r, RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX));
    }

    #[test]
    fn assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MIN + 20, i8::MIN + 30);
        assign_add(&mut r_min, &PointI8::of(-20, -20));
        assert_eq!(r_min, RectI8::of(i8::MAX - 9, i8::MAX - 14, i8::MIN, i8::MIN + 10));

        let mut r_max = RectI8::of(i8::MAX - 20, i8::MAX - 30, i8::MAX - 5, i8::MAX - 10);
        assign_add(&mut r_max, &PointI8::of(20, 20));
        assert_eq!(r_max, RectI8::of(i8::MAX, i8::MAX - 10, i8::MIN + 14, i8::MIN + 9));
    }

    #[test]
    fn assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MAX, i8::MAX);
        assign_add(&mut r_min, &PointI8::of(-20, -20));
        assert_eq!(r_min, RectI8::of(i8::MAX - 9, i8::MAX - 14, i8::MAX - 20, i8::MAX - 20));

        let mut r_max = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 5, i8::MAX - 10);
        assign_add(&mut r_max, &PointI8::of(20, 20));
        assert_eq!(r_max, RectI8::of(i8::MIN + 20, i8::MIN + 20, i8::MIN + 14, i8::MIN + 9));
    }

    #[test]
    fn assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MIN + 10, i8::MIN + 10);
        assign_add(&mut r_min, &PointI8::min());
        assert_eq!(r_min, RectI8::of(1, 1, 10, 10));

        let mut r_max = RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX - 1, i8::MAX - 1);
        assign_add(&mut r_max, &PointI8::max());
        assert_eq!(r_max, RectI8::of(-12, -12, -3, -3));
    }

    #[test]
    fn assign_add_big_rect_limits_out_of_bounds() {
        let mut r1 = RectI8::largest();
        assign_add(&mut r1, &PointI8::min());
        assert_eq!(r1, RectI8::of(0, 0, -1, -1));

        let mut r2 = RectI8::largest();
        assign_add(&mut r2, &PointI8::max());
        assert_eq!(r2, RectI8::of(-1, -1, -2, -2));

        let mut r_min = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX);
        assign_add(&mut r_min, &PointI8::min());
        assert_eq!(r_min, RectI8::of(1, 1, -1, -1));

        let mut r_max = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1);
        assign_add(&mut r_max, &PointI8::max());
        assert_eq!(r_max, RectI8::of(-1, -1, -3, -3));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&RectI8::of(0, 0, 12, 15), &PointI8::of(5, 4)), RectI8::of(5, 4, 17, 19));
        assert_eq!(add(&RectI8::of(5, 4, 17, 19), &PointI8::of(-4, -2)), RectI8::of(1, 2, 13, 17));
    }

    #[test]
    fn add_small_rect_to_bounds() {
        assert_eq!(
            add(&RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MIN + 12, i8::MIN + 15), &PointI8::of(-2, -5)),
            RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10)
        );
        assert_eq!(
            add(&RectI8::of(i8::MAX - 12, i8::MAX - 15, i8::MAX - 2, i8::MAX - 5), &PointI8::of(2, 5)),
            RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX, i8::MAX)
        );
    }

    #[test]
    fn add_big_rect_to_bounds() {
        assert_eq!(
            add(&RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX), &PointI8::of(-2, -5)),
            RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5)
        );
        assert_eq!(
            add(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5), &PointI8::of(2, 5)),
            RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX)
        );
    }

    #[test]
    fn add_small_rect_out_of_bounds() {
        assert_eq!(
            add(&RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MIN + 20, i8::MIN + 30), &PointI8::of(-20, -20)),
            RectI8::of(i8::MAX - 9, i8::MAX - 14, i8::MIN, i8::MIN + 10)
        );
        assert_eq!(
            add(&RectI8::of(i8::MAX - 20, i8::MAX - 30, i8::MAX - 5, i8::MAX - 10), &PointI8::of(20, 20)),
            RectI8::of(i8::MAX, i8::MAX - 10, i8::MIN + 14, i8::MIN + 9)
        );
    }

    #[test]
    fn add_big_rect_out_of_bounds() {
        assert_eq!(
            add(&RectI8::of(i8::MIN + 10, i8::MIN + 5, i8::MAX, i8::MAX), &PointI8::of(-20, -20)),
            RectI8::of(i8::MAX - 9, i8::MAX - 14, i8::MAX - 20, i8::MAX - 20)
        );
        assert_eq!(
            add(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 5, i8::MAX - 10), &PointI8::of(20, 20)),
            RectI8::of(i8::MIN + 20, i8::MIN + 20, i8::MIN + 14, i8::MIN + 9)
        );
    }

    #[test]
    fn add_small_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MIN + 10, i8::MIN + 10), &PointI8::min()), RectI8::of(1, 1, 10, 10));
        assert_eq!(add(&RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX - 1, i8::MAX - 1), &PointI8::max()), RectI8::of(-12, -12, -3, -3));
    }

    #[test]
    fn add_big_rect_limits_out_of_bounds() {
        assert_eq!(add(&RectI8::largest(), &PointI8::min()), RectI8::of(0, 0, -1, -1));
        assert_eq!(add(&RectI8::largest(), &PointI8::max()), RectI8::of(-1, -1, -2, -2));
        assert_eq!(add(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX), &PointI8::min()), RectI8::of(1, 1, -1, -1));
        assert_eq!(add(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1), &PointI8::max()), RectI8::of(-1, -1, -3, -3));
    }
}
