use crate::cartesian::{
    point::point_u32::PointU32,
    rect::rect_u32::{RectU32, delta_x, delta_y},
};

pub fn try_saturating_resize_assign(r: &mut RectU32, size: u32) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_x = i64::from(delta_x(r)) + 1 - i64::from(size);
    let diff_y = i64::from(delta_y(r)) + 1 - i64::from(size);
    let temp_min_x = i64::from(r.min.x) + diff_x / 2;
    let temp_min_y = i64::from(r.min.y) + diff_y / 2;
    let min_x = temp_min_x.clamp(0, i64::from(u32::MAX) - i64::from(size) + 1);
    let min_y = temp_min_y.clamp(0, i64::from(u32::MAX) - i64::from(size) + 1);
    r.min.x = min_x as u32;
    r.min.y = min_y as u32;
    r.max.x = (min_x + i64::from(size) - 1) as u32;
    r.max.y = (min_y + i64::from(size) - 1) as u32;
    Some(())
}

pub fn try_saturating_resize(r: &RectU32, size: u32) -> Option<RectU32> {
    if size < 3 {
        return None;
    }
    let diff_x = i64::from(delta_x(r)) + 1 - i64::from(size);
    let diff_y = i64::from(delta_y(r)) + 1 - i64::from(size);
    let temp_min_x = i64::from(r.min.x) + diff_x / 2;
    let temp_min_y = i64::from(r.min.y) + diff_y / 2;
    let clamped_min_x = temp_min_x.clamp(0, i64::from(u32::MAX) - i64::from(size) + 1);
    let clamped_min_y = temp_min_y.clamp(0, i64::from(u32::MAX) - i64::from(size) + 1);
    let min_x = clamped_min_x as u32;
    let min_y = clamped_min_y as u32;
    let max_x = (clamped_min_x + i64::from(size) - 1) as u32;
    let max_y = (clamped_min_y + i64::from(size) - 1) as u32;
    Some(RectU32 { min: PointU32 { x: min_x, y: min_y }, max: PointU32 { x: max_x, y: max_y } })
}

pub fn saturating_resize_assign(r: &mut RectU32, size: u32) {
    try_saturating_resize_assign(r, size).unwrap()
}

pub fn saturating_resize(r: &RectU32, size: u32) -> RectU32 {
    try_saturating_resize(r, size).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{saturating_resize, saturating_resize_assign, try_saturating_resize, try_saturating_resize_assign};
    use crate::cartesian::rect::rect_u32::RectU32;

    #[test]
    fn try_saturating_resize_assign_odd() {
        let mut r = RectU32::of(5, 5, 15, 15);
        assert_eq!(try_saturating_resize_assign(&mut r, 11), Some(()));
        assert_eq!(r, RectU32::of(5, 5, 15, 15));
        assert_eq!(try_saturating_resize_assign(&mut r, 9), Some(()));
        assert_eq!(r, RectU32::of(6, 6, 14, 14));
        assert_eq!(try_saturating_resize_assign(&mut r, 7), Some(()));
        assert_eq!(r, RectU32::of(7, 7, 13, 13));
        assert_eq!(try_saturating_resize_assign(&mut r, 5), Some(()));
        assert_eq!(r, RectU32::of(8, 8, 12, 12));
        assert_eq!(try_saturating_resize_assign(&mut r, 3), Some(()));
        assert_eq!(r, RectU32::of(9, 9, 11, 11));
        assert_eq!(try_saturating_resize_assign(&mut r, 9), Some(()));
        assert_eq!(r, RectU32::of(6, 6, 14, 14));
    }

    #[test]
    fn try_saturating_resize_assign_even() {
        let mut r = RectU32::of(5, 5, 14, 14);
        assert_eq!(try_saturating_resize_assign(&mut r, 10), Some(()));
        assert_eq!(r, RectU32::of(5, 5, 14, 14));
        assert_eq!(try_saturating_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectU32::of(6, 6, 13, 13));
        assert_eq!(try_saturating_resize_assign(&mut r, 6), Some(()));
        assert_eq!(r, RectU32::of(7, 7, 12, 12));
        assert_eq!(try_saturating_resize_assign(&mut r, 4), Some(()));
        assert_eq!(r, RectU32::of(8, 8, 11, 11));
        assert_eq!(try_saturating_resize_assign(&mut r, 8), Some(()));
        assert_eq!(r, RectU32::of(6, 6, 13, 13));
    }

    #[test]
    fn try_saturating_resize_assign_small_size() {
        let mut r = RectU32::of(10, 10, 20, 20);
        assert_eq!(try_saturating_resize_assign(&mut r, 0), None);
        assert_eq!(try_saturating_resize_assign(&mut r, 1), None);
        assert_eq!(try_saturating_resize_assign(&mut r, 2), None);
        assert_eq!(r, RectU32::of(10, 10, 20, 20));
    }

    #[test]
    fn try_saturating_resize_assign_same_size() {
        let mut r_11 = RectU32::of(10, 10, 20, 20);
        assert_eq!(try_saturating_resize_assign(&mut r_11, 11), Some(()));
        assert_eq!(r_11, RectU32::of(10, 10, 20, 20));

        let mut r_12 = RectU32::of(10, 10, 21, 21);
        assert_eq!(try_saturating_resize_assign(&mut r_12, 12), Some(()));
        assert_eq!(r_12, RectU32::of(10, 10, 21, 21));

        let mut r_13 = RectU32::of(9, 9, 21, 21);
        assert_eq!(try_saturating_resize_assign(&mut r_13, 13), Some(()));
        assert_eq!(r_13, RectU32::of(9, 9, 21, 21));
    }

    #[test]
    fn try_saturating_resize_assign_odd_small_rect_out_of_bounds() {
        let mut r_min = RectU32::of(0, 0, 2, 2);
        assert_eq!(try_saturating_resize_assign(&mut r_min, 11), Some(()));
        assert_eq!(r_min, RectU32::of(0, 0, 10, 10));

        let mut r_max = RectU32::of(u32::MAX - 2, u32::MAX - 2, u32::MAX, u32::MAX);
        assert_eq!(try_saturating_resize_assign(&mut r_max, 11), Some(()));
        assert_eq!(r_max, RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX, u32::MAX));
    }

    #[test]
    fn try_saturating_resize_assign_even_small_rect_out_of_bounds() {
        let mut r = RectU32::of(0, 0, 3, 3);
        assert_eq!(try_saturating_resize_assign(&mut r, 11), Some(()));
        assert_eq!(r, RectU32::of(0, 0, 10, 10));

        let mut r = RectU32::of(u32::MAX - 3, u32::MAX - 3, u32::MAX, u32::MAX);
        assert_eq!(try_saturating_resize_assign(&mut r, 11), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX, u32::MAX));
    }

    #[test]
    fn try_saturating_resize_assign_odd_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU32::of(0, 0, 2, 2);
        assert_eq!(try_saturating_resize_assign(&mut r_min, u32::MAX), Some(()));
        assert_eq!(r_min, RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1));

        let mut r_max = RectU32::of(u32::MAX - 2, u32::MAX - 2, u32::MAX, u32::MAX);
        assert_eq!(try_saturating_resize_assign(&mut r_max, u32::MAX), Some(()));
        assert_eq!(r_max, RectU32::of(1, 1, u32::MAX, u32::MAX));
    }

    #[test]
    fn try_saturating_resize_assign_even_small_rect_limits_out_of_bounds() {
        let mut r = RectU32::of(0, 0, 3, 3);
        assert_eq!(try_saturating_resize_assign(&mut r, u32::MAX - 1), Some(()));
        assert_eq!(r, RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 2));

        let mut r = RectU32::of(u32::MAX - 3, u32::MAX - 3, u32::MAX, u32::MAX);
        assert_eq!(try_saturating_resize_assign(&mut r, u32::MAX - 1), Some(()));
        assert_eq!(r, RectU32::of(2, 2, u32::MAX, u32::MAX));
    }

    #[test]
    fn try_saturating_resize_assign_big_rect_limits_out_of_bounds() {
        let mut r_odd_1 = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_saturating_resize_assign(&mut r_odd_1, u32::MAX), Some(()));
        assert_eq!(r_odd_1, RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1));

        let mut r_odd_1 = RectU32::of(1, 1, u32::MAX, u32::MAX);
        assert_eq!(try_saturating_resize_assign(&mut r_odd_1, u32::MAX), Some(()));
        assert_eq!(r_odd_1, RectU32::of(1, 1, u32::MAX, u32::MAX));

        let mut r_even = RectU32::largest();
        assert_eq!(try_saturating_resize_assign(&mut r_even, u32::MAX), Some(()));
        assert_eq!(r_even, RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1));
    }

    #[test]
    fn try_saturating_resize_odd() {
        assert_eq!(try_saturating_resize(&RectU32::of(5, 5, 15, 15), 11), Some(RectU32::of(5, 5, 15, 15)));
        assert_eq!(try_saturating_resize(&RectU32::of(5, 5, 15, 15), 9), Some(RectU32::of(6, 6, 14, 14)));
        assert_eq!(try_saturating_resize(&RectU32::of(6, 6, 14, 14), 7), Some(RectU32::of(7, 7, 13, 13)));
        assert_eq!(try_saturating_resize(&RectU32::of(7, 7, 13, 13), 5), Some(RectU32::of(8, 8, 12, 12)));
        assert_eq!(try_saturating_resize(&RectU32::of(8, 8, 12, 12), 3), Some(RectU32::of(9, 9, 11, 11)));
        assert_eq!(try_saturating_resize(&RectU32::of(9, 9, 11, 11), 9), Some(RectU32::of(6, 6, 14, 14)));
    }

    #[test]
    fn try_saturating_resize_even() {
        assert_eq!(try_saturating_resize(&RectU32::of(5, 5, 14, 14), 10), Some(RectU32::of(5, 5, 14, 14)));
        assert_eq!(try_saturating_resize(&RectU32::of(5, 5, 14, 14), 8), Some(RectU32::of(6, 6, 13, 13)));
        assert_eq!(try_saturating_resize(&RectU32::of(6, 6, 13, 13), 6), Some(RectU32::of(7, 7, 12, 12)));
        assert_eq!(try_saturating_resize(&RectU32::of(7, 7, 12, 12), 4), Some(RectU32::of(8, 8, 11, 11)));
        assert_eq!(try_saturating_resize(&RectU32::of(8, 8, 11, 11), 8), Some(RectU32::of(6, 6, 13, 13)));
    }

    #[test]
    fn try_saturating_resize_small_size() {
        let r = RectU32::of(10, 10, 20, 20);
        assert_eq!(try_saturating_resize(&r, 0), None);
        assert_eq!(try_saturating_resize(&r, 1), None);
        assert_eq!(try_saturating_resize(&r, 2), None);
    }

    #[test]
    fn try_saturating_resize_same_size() {
        assert_eq!(try_saturating_resize(&RectU32::of(10, 10, 20, 20), 11), Some(RectU32::of(10, 10, 20, 20)));
        assert_eq!(try_saturating_resize(&RectU32::of(10, 10, 21, 21), 12), Some(RectU32::of(10, 10, 21, 21)));
        assert_eq!(try_saturating_resize(&RectU32::of(9, 9, 21, 21), 13), Some(RectU32::of(9, 9, 21, 21)));
    }

    #[test]
    fn try_saturating_resize_odd_small_rect_out_of_bounds() {
        assert_eq!(try_saturating_resize(&RectU32::of(0, 0, 2, 2), 11), Some(RectU32::of(0, 0, 10, 10)));
        assert_eq!(
            try_saturating_resize(&RectU32::of(u32::MAX - 2, u32::MAX - 2, u32::MAX, u32::MAX), 11),
            Some(RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX, u32::MAX))
        );
    }

    #[test]
    fn try_saturating_resize_even_small_rect_out_of_bounds() {
        assert_eq!(try_saturating_resize(&RectU32::of(0, 0, 3, 3), 11), Some(RectU32::of(0, 0, 10, 10)));
        assert_eq!(
            try_saturating_resize(&RectU32::of(u32::MAX - 3, u32::MAX - 3, u32::MAX, u32::MAX), 11),
            Some(RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX, u32::MAX))
        );
    }

    #[test]
    fn try_saturating_resize_odd_small_rect_limits_out_of_bounds() {
        assert_eq!(try_saturating_resize(&RectU32::of(0, 0, 2, 2), u32::MAX), Some(RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1)));
        assert_eq!(
            try_saturating_resize(&RectU32::of(u32::MAX - 2, u32::MAX - 2, u32::MAX, u32::MAX), u32::MAX),
            Some(RectU32::of(1, 1, u32::MAX, u32::MAX))
        );
    }

    #[test]
    fn try_saturating_resize_even_small_rect_limits_out_of_bounds() {
        assert_eq!(try_saturating_resize(&RectU32::of(0, 0, 3, 3), u32::MAX - 1), Some(RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 2)));
        assert_eq!(
            try_saturating_resize(&RectU32::of(u32::MAX - 3, u32::MAX - 3, u32::MAX, u32::MAX), u32::MAX - 1),
            Some(RectU32::of(2, 2, u32::MAX, u32::MAX))
        );
    }

    #[test]
    fn try_saturating_resize_big_rect_limits_out_of_bounds() {
        assert_eq!(
            try_saturating_resize(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1), u32::MAX),
            Some(RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1))
        );
        assert_eq!(try_saturating_resize(&RectU32::of(1, 1, u32::MAX, u32::MAX), u32::MAX), Some(RectU32::of(1, 1, u32::MAX, u32::MAX)));
        assert_eq!(try_saturating_resize(&RectU32::largest(), u32::MAX), Some(RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1)));
    }

    #[test]
    fn saturating_resize_assign_odd() {
        let mut r = RectU32::of(5, 5, 15, 15);
        saturating_resize_assign(&mut r, 11);
        assert_eq!(r, RectU32::of(5, 5, 15, 15));
        saturating_resize_assign(&mut r, 9);
        assert_eq!(r, RectU32::of(6, 6, 14, 14));
        saturating_resize_assign(&mut r, 7);
        assert_eq!(r, RectU32::of(7, 7, 13, 13));
        saturating_resize_assign(&mut r, 5);
        assert_eq!(r, RectU32::of(8, 8, 12, 12));
        saturating_resize_assign(&mut r, 3);
        assert_eq!(r, RectU32::of(9, 9, 11, 11));
        saturating_resize_assign(&mut r, 9);
        assert_eq!(r, RectU32::of(6, 6, 14, 14));
    }

    #[test]
    fn saturating_resize_assign_even() {
        let mut r = RectU32::of(5, 5, 14, 14);
        saturating_resize_assign(&mut r, 10);
        assert_eq!(r, RectU32::of(5, 5, 14, 14));
        saturating_resize_assign(&mut r, 8);
        assert_eq!(r, RectU32::of(6, 6, 13, 13));
        saturating_resize_assign(&mut r, 6);
        assert_eq!(r, RectU32::of(7, 7, 12, 12));
        saturating_resize_assign(&mut r, 4);
        assert_eq!(r, RectU32::of(8, 8, 11, 11));
        saturating_resize_assign(&mut r, 8);
        assert_eq!(r, RectU32::of(6, 6, 13, 13));
    }

    #[test]
    fn saturating_resize_odd() {
        assert_eq!(saturating_resize(&RectU32::of(5, 5, 15, 15), 11), RectU32::of(5, 5, 15, 15));
        assert_eq!(saturating_resize(&RectU32::of(5, 5, 15, 15), 9), RectU32::of(6, 6, 14, 14));
        assert_eq!(saturating_resize(&RectU32::of(6, 6, 14, 14), 7), RectU32::of(7, 7, 13, 13));
        assert_eq!(saturating_resize(&RectU32::of(7, 7, 13, 13), 5), RectU32::of(8, 8, 12, 12));
        assert_eq!(saturating_resize(&RectU32::of(8, 8, 12, 12), 3), RectU32::of(9, 9, 11, 11));
        assert_eq!(saturating_resize(&RectU32::of(9, 9, 11, 11), 9), RectU32::of(6, 6, 14, 14));
    }

    #[test]
    fn saturating_resize_even() {
        assert_eq!(saturating_resize(&RectU32::of(5, 5, 14, 14), 10), RectU32::of(5, 5, 14, 14));
        assert_eq!(saturating_resize(&RectU32::of(5, 5, 14, 14), 8), RectU32::of(6, 6, 13, 13));
        assert_eq!(saturating_resize(&RectU32::of(6, 6, 13, 13), 6), RectU32::of(7, 7, 12, 12));
        assert_eq!(saturating_resize(&RectU32::of(7, 7, 12, 12), 4), RectU32::of(8, 8, 11, 11));
        assert_eq!(saturating_resize(&RectU32::of(8, 8, 11, 11), 8), RectU32::of(6, 6, 13, 13));
    }
}
