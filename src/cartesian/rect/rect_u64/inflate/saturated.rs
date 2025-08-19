use crate::cartesian::{point::point_u64::PointU64, rect::rect_u64::RectU64};

pub fn try_assign_inflate(r: &mut RectU64) -> Option<()> {
    let is_min_x = r.min.x == 0;
    let is_min_y = r.min.y == 0;
    let is_max_x = r.max.x == u64::MAX;
    let is_max_y = r.max.y == u64::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - u64::from(is_min_x) + u64::from(is_max_x);
    let min_y_modifier = 1 - u64::from(is_min_y) + u64::from(is_max_y);
    let max_x_modifier = 1 + u64::from(is_min_x) - u64::from(is_max_x);
    let max_y_modifier = 1 + u64::from(is_min_y) - u64::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
    Some(())
}

pub fn try_inflate(r: &RectU64) -> Option<RectU64> {
    let is_min_x = r.min.x == 0;
    let is_min_y = r.min.y == 0;
    let is_max_x = r.max.x == u64::MAX;
    let is_max_y = r.max.y == u64::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - u64::from(is_min_x) + u64::from(is_max_x);
    let min_y_modifier = 1 - u64::from(is_min_y) + u64::from(is_max_y);
    let max_x_modifier = 1 + u64::from(is_min_x) - u64::from(is_max_x);
    let max_y_modifier = 1 + u64::from(is_min_y) - u64::from(is_max_y);
    let min_x = r.min.x.saturating_sub(min_x_modifier);
    let min_y = r.min.y.saturating_sub(min_y_modifier);
    let max_x = r.max.x.saturating_add(max_x_modifier);
    let max_y = r.max.y.saturating_add(max_y_modifier);
    Some(RectU64 { min: PointU64 { x: min_x, y: min_y }, max: PointU64 { x: max_x, y: max_y } })
}

pub fn assign_inflate(r: &mut RectU64) {
    try_assign_inflate(r).unwrap()
}

pub fn inflate(r: &RectU64) -> RectU64 {
    try_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{RectU64, assign_inflate, inflate, try_assign_inflate, try_inflate};

    #[test]
    fn try_assign_inflate_min_bounds() {
        let mut r = RectU64::of(7, 2, 4, 13);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(6, 1, 5, 14));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(5, 0, 6, 15));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(4, 0, 7, 17));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(3, 0, 8, 19));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(2, 0, 9, 21));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(1, 0, 10, 23));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(0, 0, 11, 25));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(0, 0, 13, 27));
    }

    #[test]
    fn try_assign_inflate_max_bounds() {
        let mut r = RectU64::of(200, 230, u64::MAX - 5, u64::MAX - 3);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(199, 229, u64::MAX - 4, u64::MAX - 2));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(198, 228, u64::MAX - 3, u64::MAX - 1));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(197, 227, u64::MAX - 2, u64::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(196, 225, u64::MAX - 1, u64::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(195, 223, u64::MAX, u64::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(193, 221, u64::MAX, u64::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(191, 219, u64::MAX, u64::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(189, 217, u64::MAX, u64::MAX));
    }

    #[test]
    fn try_assign_inflate_to_bounds() {
        let mut r_odd = RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_odd), Some(()));
        assert_eq!(r_odd, RectU64::largest());

        let mut r_even_1 = RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_even_1), Some(()));
        assert_eq!(r_even_1, RectU64::largest());

        let mut r_even_2 = RectU64::of(1, 1, u64::MAX, u64::MAX);
        assert_eq!(try_assign_inflate(&mut r_even_2), Some(()));
        assert_eq!(r_even_2, RectU64::largest());
    }

    #[test]
    fn try_assign_inflate_width_to_bounds() {
        let mut r_min = RectU64::of(1, 10, 100, 100);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectU64::of(0, 9, 101, 101));

        let mut r_max = RectU64::of(10, 10, u64::MAX - 1, 100);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectU64::of(9, 9, u64::MAX, 101));
    }

    #[test]
    fn try_assign_inflate_height_to_bounds() {
        let mut r_min = RectU64::of(10, 1, 100, 100);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectU64::of(9, 0, 101, 101));

        let mut r_max = RectU64::of(10, 10, 100, u64::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectU64::of(9, 9, 101, u64::MAX));
    }

    #[test]
    fn try_assign_inflate_out_of_bounds() {
        let mut r = RectU64::largest();
        assert_eq!(try_assign_inflate(&mut r), None);
        assert_eq!(r, RectU64::largest());

        let mut r_x = RectU64::of(0, 10, u64::MAX, 100);
        assert_eq!(try_assign_inflate(&mut r_x), None);
        assert_eq!(r_x, RectU64::of(0, 10, u64::MAX, 100));

        let mut r_y = RectU64::of(10, 0, 100, u64::MAX);
        assert_eq!(try_assign_inflate(&mut r_y), None);
        assert_eq!(r_y, RectU64::of(10, 0, 100, u64::MAX));
    }

    #[test]
    fn try_inflate_min_bounds() {
        assert_eq!(try_inflate(&RectU64::of(7, 2, 4, 13)), Some(RectU64::of(6, 1, 5, 14)));
        assert_eq!(try_inflate(&RectU64::of(6, 1, 5, 14)), Some(RectU64::of(5, 0, 6, 15)));
        assert_eq!(try_inflate(&RectU64::of(5, 0, 6, 15)), Some(RectU64::of(4, 0, 7, 17)));
        assert_eq!(try_inflate(&RectU64::of(4, 0, 7, 17)), Some(RectU64::of(3, 0, 8, 19)));
        assert_eq!(try_inflate(&RectU64::of(3, 0, 8, 19)), Some(RectU64::of(2, 0, 9, 21)));
        assert_eq!(try_inflate(&RectU64::of(2, 0, 9, 21)), Some(RectU64::of(1, 0, 10, 23)));
        assert_eq!(try_inflate(&RectU64::of(1, 0, 10, 23)), Some(RectU64::of(0, 0, 11, 25)));
        assert_eq!(try_inflate(&RectU64::of(0, 0, 11, 25)), Some(RectU64::of(0, 0, 13, 27)));
    }

    #[test]
    fn try_inflate_max_bounds() {
        assert_eq!(try_inflate(&RectU64::of(200, 230, u64::MAX - 5, u64::MAX - 3)), Some(RectU64::of(199, 229, u64::MAX - 4, u64::MAX - 2)));
        assert_eq!(try_inflate(&RectU64::of(199, 229, u64::MAX - 4, u64::MAX - 2)), Some(RectU64::of(198, 228, u64::MAX - 3, u64::MAX - 1)));
        assert_eq!(try_inflate(&RectU64::of(198, 228, u64::MAX - 3, u64::MAX - 1)), Some(RectU64::of(197, 227, u64::MAX - 2, u64::MAX)));
        assert_eq!(try_inflate(&RectU64::of(197, 227, u64::MAX - 2, u64::MAX)), Some(RectU64::of(196, 225, u64::MAX - 1, u64::MAX)));
        assert_eq!(try_inflate(&RectU64::of(196, 225, u64::MAX - 1, u64::MAX)), Some(RectU64::of(195, 223, u64::MAX, u64::MAX)));
        assert_eq!(try_inflate(&RectU64::of(195, 223, u64::MAX, u64::MAX)), Some(RectU64::of(193, 221, u64::MAX, u64::MAX)));
        assert_eq!(try_inflate(&RectU64::of(193, 221, u64::MAX, u64::MAX)), Some(RectU64::of(191, 219, u64::MAX, u64::MAX)));
        assert_eq!(try_inflate(&RectU64::of(191, 219, u64::MAX, u64::MAX)), Some(RectU64::of(189, 217, u64::MAX, u64::MAX)));
    }

    #[test]
    fn try_inflate_to_bounds() {
        assert_eq!(try_inflate(&RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1)), Some(RectU64::largest()));
        assert_eq!(try_inflate(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1)), Some(RectU64::largest()));
        assert_eq!(try_inflate(&RectU64::of(1, 1, u64::MAX, u64::MAX)), Some(RectU64::largest()));
    }

    #[test]
    fn try_inflate_width_to_bounds() {
        assert_eq!(try_inflate(&RectU64::of(1, 10, 100, 100)), Some(RectU64::of(0, 9, 101, 101)));
        assert_eq!(try_inflate(&RectU64::of(10, 10, u64::MAX - 1, 100)), Some(RectU64::of(9, 9, u64::MAX, 101)));
    }

    #[test]
    fn try_inflate_height_to_bounds() {
        assert_eq!(try_inflate(&RectU64::of(10, 1, 100, 100)), Some(RectU64::of(9, 0, 101, 101)));
        assert_eq!(try_inflate(&RectU64::of(10, 10, 100, u64::MAX - 1)), Some(RectU64::of(9, 9, 101, u64::MAX)));
    }

    #[test]
    fn try_inflate_out_of_bounds() {
        assert_eq!(try_inflate(&RectU64::largest()), None);
        assert_eq!(try_inflate(&RectU64::of(0, 10, u64::MAX, 100)), None);
        assert_eq!(try_inflate(&RectU64::of(10, 0, 100, u64::MAX)), None);
    }

    #[test]
    fn assign_inflate_min_bounds() {
        let mut r = RectU64::of(7, 2, 4, 13);
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(6, 1, 5, 14));
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(5, 0, 6, 15));
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(4, 0, 7, 17));
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(3, 0, 8, 19));
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(2, 0, 9, 21));
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(1, 0, 10, 23));
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(0, 0, 11, 25));
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(0, 0, 13, 27));
    }

    #[test]
    fn assign_inflate_max_bounds() {
        let mut r = RectU64::of(200, 230, u64::MAX - 5, u64::MAX - 3);
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(199, 229, u64::MAX - 4, u64::MAX - 2));
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(198, 228, u64::MAX - 3, u64::MAX - 1));
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(197, 227, u64::MAX - 2, u64::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(196, 225, u64::MAX - 1, u64::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(195, 223, u64::MAX, u64::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(193, 221, u64::MAX, u64::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(191, 219, u64::MAX, u64::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(189, 217, u64::MAX, u64::MAX));
    }

    #[test]
    fn inflate_min_bounds() {
        assert_eq!(inflate(&RectU64::of(7, 2, 4, 13)), RectU64::of(6, 1, 5, 14));
        assert_eq!(inflate(&RectU64::of(6, 1, 5, 14)), RectU64::of(5, 0, 6, 15));
        assert_eq!(inflate(&RectU64::of(5, 0, 6, 15)), RectU64::of(4, 0, 7, 17));
        assert_eq!(inflate(&RectU64::of(4, 0, 7, 17)), RectU64::of(3, 0, 8, 19));
        assert_eq!(inflate(&RectU64::of(3, 0, 8, 19)), RectU64::of(2, 0, 9, 21));
        assert_eq!(inflate(&RectU64::of(2, 0, 9, 21)), RectU64::of(1, 0, 10, 23));
        assert_eq!(inflate(&RectU64::of(1, 0, 10, 23)), RectU64::of(0, 0, 11, 25));
        assert_eq!(inflate(&RectU64::of(0, 0, 11, 25)), RectU64::of(0, 0, 13, 27));
    }

    #[test]
    fn inflate_max_bounds() {
        assert_eq!(inflate(&RectU64::of(200, 230, u64::MAX - 5, u64::MAX - 3)), RectU64::of(199, 229, u64::MAX - 4, u64::MAX - 2));
        assert_eq!(inflate(&RectU64::of(199, 229, u64::MAX - 4, u64::MAX - 2)), RectU64::of(198, 228, u64::MAX - 3, u64::MAX - 1));
        assert_eq!(inflate(&RectU64::of(198, 228, u64::MAX - 3, u64::MAX - 1)), RectU64::of(197, 227, u64::MAX - 2, u64::MAX));
        assert_eq!(inflate(&RectU64::of(197, 227, u64::MAX - 2, u64::MAX)), RectU64::of(196, 225, u64::MAX - 1, u64::MAX));
        assert_eq!(inflate(&RectU64::of(196, 225, u64::MAX - 1, u64::MAX)), RectU64::of(195, 223, u64::MAX, u64::MAX));
        assert_eq!(inflate(&RectU64::of(195, 223, u64::MAX, u64::MAX)), RectU64::of(193, 221, u64::MAX, u64::MAX));
        assert_eq!(inflate(&RectU64::of(193, 221, u64::MAX, u64::MAX)), RectU64::of(191, 219, u64::MAX, u64::MAX));
        assert_eq!(inflate(&RectU64::of(191, 219, u64::MAX, u64::MAX)), RectU64::of(189, 217, u64::MAX, u64::MAX));
    }
}
