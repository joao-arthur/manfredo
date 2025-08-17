use crate::cartesian::{point::point_u16::PointU16, rect::rect_u16::RectU16};

pub fn try_assign_inflate(r: &mut RectU16) -> Option<()> {
    let is_min_x = r.min.x == 0;
    let is_min_y = r.min.y == 0;
    let is_max_x = r.max.x == u16::MAX;
    let is_max_y = r.max.y == u16::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - u16::from(is_min_x) + u16::from(is_max_x);
    let min_y_modifier = 1 - u16::from(is_min_y) + u16::from(is_max_y);
    let max_x_modifier = 1 + u16::from(is_min_x) - u16::from(is_max_x);
    let max_y_modifier = 1 + u16::from(is_min_y) - u16::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
    Some(())
}

pub fn try_inflate(r: &RectU16) -> Option<RectU16> {
    let is_min_x = r.min.x == 0;
    let is_min_y = r.min.y == 0;
    let is_max_x = r.max.x == u16::MAX;
    let is_max_y = r.max.y == u16::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - u16::from(is_min_x) + u16::from(is_max_x);
    let min_y_modifier = 1 - u16::from(is_min_y) + u16::from(is_max_y);
    let max_x_modifier = 1 + u16::from(is_min_x) - u16::from(is_max_x);
    let max_y_modifier = 1 + u16::from(is_min_y) - u16::from(is_max_y);
    Some(RectU16 {
        min: PointU16 { x: r.min.x.saturating_sub(min_x_modifier), y: r.min.y.saturating_sub(min_y_modifier) },
        max: PointU16 { x: r.max.x.saturating_add(max_x_modifier), y: r.max.y.saturating_add(max_y_modifier) },
    })
}

pub fn assign_inflate(r: &mut RectU16) {
    try_assign_inflate(r).unwrap()
}

pub fn inflate(r: &RectU16) -> RectU16 {
    try_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{RectU16, assign_inflate, inflate, try_assign_inflate, try_inflate};

    #[test]
    fn try_assign_inflate_min_bounds() {
        let mut r = RectU16::of(7, 2, 4, 13);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(6, 1, 5, 14));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(5, 0, 6, 15));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(4, 0, 7, 17));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(3, 0, 8, 19));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(2, 0, 9, 21));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(1, 0, 10, 23));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(0, 0, 11, 25));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(0, 0, 13, 27));
    }

    #[test]
    fn try_assign_inflate_max_bounds() {
        let mut r = RectU16::of(200, 230, u16::MAX - 5, u16::MAX - 3);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(199, 229, u16::MAX - 4, u16::MAX - 2));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(198, 228, u16::MAX - 3, u16::MAX - 1));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(197, 227, u16::MAX - 2, u16::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(196, 225, u16::MAX - 1, u16::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(195, 223, u16::MAX, u16::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(193, 221, u16::MAX, u16::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(191, 219, u16::MAX, u16::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(189, 217, u16::MAX, u16::MAX));
    }

    #[test]
    fn try_assign_inflate_to_bounds() {
        let mut r_odd = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_odd), Some(()));
        assert_eq!(r_odd, RectU16::of(0, 0, u16::MAX, u16::MAX));

        let mut r_even_1 = RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_even_1), Some(()));
        assert_eq!(r_even_1, RectU16::of(0, 0, u16::MAX, u16::MAX));

        let mut r_even_2 = RectU16::of(1, 1, u16::MAX, u16::MAX);
        assert_eq!(try_assign_inflate(&mut r_even_2), Some(()));
        assert_eq!(r_even_2, RectU16::of(0, 0, u16::MAX, u16::MAX));
    }

    #[test]
    fn try_assign_inflate_width_to_bounds() {
        let mut r_min = RectU16::of(1, 10, 100, 100);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectU16::of(0, 9, 101, 101));

        let mut r_max = RectU16::of(10, 10, u16::MAX - 1, 100);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectU16::of(9, 9, u16::MAX, 101));
    }

    #[test]
    fn try_assign_inflate_height_to_bounds() {
        let mut r_min = RectU16::of(10, 1, 100, 100);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectU16::of(9, 0, 101, 101));

        let mut r_max = RectU16::of(10, 10, 100, u16::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectU16::of(9, 9, 101, u16::MAX));
    }

    #[test]
    fn try_assign_inflate_out_of_bounds() {
        let mut r = RectU16::of(0, 0, u16::MAX, u16::MAX);
        assert_eq!(try_assign_inflate(&mut r), None);
        assert_eq!(r, RectU16::of(0, 0, u16::MAX, u16::MAX));

        let mut r_x = RectU16::of(0, 10, u16::MAX, 100);
        assert_eq!(try_assign_inflate(&mut r_x), None);
        assert_eq!(r_x, RectU16::of(0, 10, u16::MAX, 100));

        let mut r_y = RectU16::of(10, 0, 100, u16::MAX);
        assert_eq!(try_assign_inflate(&mut r_y), None);
        assert_eq!(r_y, RectU16::of(10, 0, 100, u16::MAX));
    }

    #[test]
    fn try_inflate_min_bounds() {
        assert_eq!(try_inflate(&RectU16::of(7, 2, 4, 13)), Some(RectU16::of(6, 1, 5, 14)));
        assert_eq!(try_inflate(&RectU16::of(6, 1, 5, 14)), Some(RectU16::of(5, 0, 6, 15)));
        assert_eq!(try_inflate(&RectU16::of(5, 0, 6, 15)), Some(RectU16::of(4, 0, 7, 17)));
        assert_eq!(try_inflate(&RectU16::of(4, 0, 7, 17)), Some(RectU16::of(3, 0, 8, 19)));
        assert_eq!(try_inflate(&RectU16::of(3, 0, 8, 19)), Some(RectU16::of(2, 0, 9, 21)));
        assert_eq!(try_inflate(&RectU16::of(2, 0, 9, 21)), Some(RectU16::of(1, 0, 10, 23)));
        assert_eq!(try_inflate(&RectU16::of(1, 0, 10, 23)), Some(RectU16::of(0, 0, 11, 25)));
        assert_eq!(try_inflate(&RectU16::of(0, 0, 11, 25)), Some(RectU16::of(0, 0, 13, 27)));
    }

    #[test]
    fn try_inflate_max_bounds() {
        assert_eq!(try_inflate(&RectU16::of(200, 230, u16::MAX - 5, u16::MAX - 3)), Some(RectU16::of(199, 229, u16::MAX - 4, u16::MAX - 2)));
        assert_eq!(try_inflate(&RectU16::of(199, 229, u16::MAX - 4, u16::MAX - 2)), Some(RectU16::of(198, 228, u16::MAX - 3, u16::MAX - 1)));
        assert_eq!(try_inflate(&RectU16::of(198, 228, u16::MAX - 3, u16::MAX - 1)), Some(RectU16::of(197, 227, u16::MAX - 2, u16::MAX)));
        assert_eq!(try_inflate(&RectU16::of(197, 227, u16::MAX - 2, u16::MAX)), Some(RectU16::of(196, 225, u16::MAX - 1, u16::MAX)));
        assert_eq!(try_inflate(&RectU16::of(196, 225, u16::MAX - 1, u16::MAX)), Some(RectU16::of(195, 223, u16::MAX, u16::MAX)));
        assert_eq!(try_inflate(&RectU16::of(195, 223, u16::MAX, u16::MAX)), Some(RectU16::of(193, 221, u16::MAX, u16::MAX)));
        assert_eq!(try_inflate(&RectU16::of(193, 221, u16::MAX, u16::MAX)), Some(RectU16::of(191, 219, u16::MAX, u16::MAX)));
        assert_eq!(try_inflate(&RectU16::of(191, 219, u16::MAX, u16::MAX)), Some(RectU16::of(189, 217, u16::MAX, u16::MAX)));
    }

    #[test]
    fn try_inflate_to_bounds() {
        assert_eq!(try_inflate(&RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1)), Some(RectU16::of(0, 0, u16::MAX, u16::MAX)));
        assert_eq!(try_inflate(&RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1)), Some(RectU16::of(0, 0, u16::MAX, u16::MAX)));
        assert_eq!(try_inflate(&RectU16::of(1, 1, u16::MAX, u16::MAX)), Some(RectU16::of(0, 0, u16::MAX, u16::MAX)));
    }

    #[test]
    fn try_inflate_width_to_bounds() {
        assert_eq!(try_inflate(&RectU16::of(1, 10, 100, 100)), Some(RectU16::of(0, 9, 101, 101)));
        assert_eq!(try_inflate(&RectU16::of(10, 10, u16::MAX - 1, 100)), Some(RectU16::of(9, 9, u16::MAX, 101)));
    }

    #[test]
    fn try_inflate_height_to_bounds() {
        assert_eq!(try_inflate(&RectU16::of(10, 1, 100, 100)), Some(RectU16::of(9, 0, 101, 101)));
        assert_eq!(try_inflate(&RectU16::of(10, 10, 100, u16::MAX - 1)), Some(RectU16::of(9, 9, 101, u16::MAX)));
    }

    #[test]
    fn try_inflate_out_of_bounds() {
        assert_eq!(try_inflate(&RectU16::of(0, 0, u16::MAX, u16::MAX)), None);
        assert_eq!(try_inflate(&RectU16::of(0, 10, u16::MAX, 100)), None);
        assert_eq!(try_inflate(&RectU16::of(10, 0, 100, u16::MAX)), None);
    }

    #[test]
    fn assign_inflate_min_bounds() {
        let mut r = RectU16::of(7, 2, 4, 13);
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(6, 1, 5, 14));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(5, 0, 6, 15));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(4, 0, 7, 17));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(3, 0, 8, 19));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(2, 0, 9, 21));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(1, 0, 10, 23));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(0, 0, 11, 25));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(0, 0, 13, 27));
    }

    #[test]
    fn assign_inflate_max_bounds() {
        let mut r = RectU16::of(200, 230, u16::MAX - 5, u16::MAX - 3);
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(199, 229, u16::MAX - 4, u16::MAX - 2));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(198, 228, u16::MAX - 3, u16::MAX - 1));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(197, 227, u16::MAX - 2, u16::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(196, 225, u16::MAX - 1, u16::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(195, 223, u16::MAX, u16::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(193, 221, u16::MAX, u16::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(191, 219, u16::MAX, u16::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(189, 217, u16::MAX, u16::MAX));
    }

    #[test]
    fn inflate_min_bounds() {
        assert_eq!(inflate(&RectU16::of(7, 2, 4, 13)), RectU16::of(6, 1, 5, 14));
        assert_eq!(inflate(&RectU16::of(6, 1, 5, 14)), RectU16::of(5, 0, 6, 15));
        assert_eq!(inflate(&RectU16::of(5, 0, 6, 15)), RectU16::of(4, 0, 7, 17));
        assert_eq!(inflate(&RectU16::of(4, 0, 7, 17)), RectU16::of(3, 0, 8, 19));
        assert_eq!(inflate(&RectU16::of(3, 0, 8, 19)), RectU16::of(2, 0, 9, 21));
        assert_eq!(inflate(&RectU16::of(2, 0, 9, 21)), RectU16::of(1, 0, 10, 23));
        assert_eq!(inflate(&RectU16::of(1, 0, 10, 23)), RectU16::of(0, 0, 11, 25));
        assert_eq!(inflate(&RectU16::of(0, 0, 11, 25)), RectU16::of(0, 0, 13, 27));
    }

    #[test]
    fn inflate_max_bounds() {
        assert_eq!(inflate(&RectU16::of(200, 230, u16::MAX - 5, u16::MAX - 3)), RectU16::of(199, 229, u16::MAX - 4, u16::MAX - 2));
        assert_eq!(inflate(&RectU16::of(199, 229, u16::MAX - 4, u16::MAX - 2)), RectU16::of(198, 228, u16::MAX - 3, u16::MAX - 1));
        assert_eq!(inflate(&RectU16::of(198, 228, u16::MAX - 3, u16::MAX - 1)), RectU16::of(197, 227, u16::MAX - 2, u16::MAX));
        assert_eq!(inflate(&RectU16::of(197, 227, u16::MAX - 2, u16::MAX)), RectU16::of(196, 225, u16::MAX - 1, u16::MAX));
        assert_eq!(inflate(&RectU16::of(196, 225, u16::MAX - 1, u16::MAX)), RectU16::of(195, 223, u16::MAX, u16::MAX));
        assert_eq!(inflate(&RectU16::of(195, 223, u16::MAX, u16::MAX)), RectU16::of(193, 221, u16::MAX, u16::MAX));
        assert_eq!(inflate(&RectU16::of(193, 221, u16::MAX, u16::MAX)), RectU16::of(191, 219, u16::MAX, u16::MAX));
        assert_eq!(inflate(&RectU16::of(191, 219, u16::MAX, u16::MAX)), RectU16::of(189, 217, u16::MAX, u16::MAX));
    }
}
