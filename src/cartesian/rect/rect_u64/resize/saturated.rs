use crate::cartesian::{
    point::point_u64::PointU64,
    rect::rect_u64::{RectU64, delta_x, delta_y},
};

pub fn try_assign_resize(r: &mut RectU64, size: u64) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_x = i128::from(delta_x(r)) + 1 - i128::from(size);
    let diff_y = i128::from(delta_y(r)) + 1 - i128::from(size);
    let temp_min_x = i128::from(r.min.x) + diff_x / 2;
    let temp_min_y = i128::from(r.min.y) + diff_y / 2;
    let min_x = temp_min_x.clamp(0, i128::from(u64::MAX) - i128::from(size) + 1);
    let min_y = temp_min_y.clamp(0, i128::from(u64::MAX) - i128::from(size) + 1);
    r.min.x = min_x as u64;
    r.min.y = min_y as u64;
    r.max.x = (min_x + i128::from(size) - 1) as u64;
    r.max.y = (min_y + i128::from(size) - 1) as u64;
    Some(())
}

pub fn try_resize(r: &RectU64, size: u64) -> Option<RectU64> {
    if size < 3 {
        return None;
    }
    let diff_x = i128::from(delta_x(r)) + 1 - i128::from(size);
    let diff_y = i128::from(delta_y(r)) + 1 - i128::from(size);
    let temp_min_x = i128::from(r.min.x) + diff_x / 2;
    let temp_min_y = i128::from(r.min.y) + diff_y / 2;
    let clamped_min_x = temp_min_x.clamp(0, i128::from(u64::MAX) - i128::from(size) + 1);
    let clamped_min_y = temp_min_y.clamp(0, i128::from(u64::MAX) - i128::from(size) + 1);
    let min_x = clamped_min_x as u64;
    let min_y = clamped_min_y as u64;
    let max_x = (clamped_min_x + i128::from(size) - 1) as u64;
    let max_y = (clamped_min_y + i128::from(size) - 1) as u64;
    Some(RectU64 { min: PointU64 { x: min_x, y: min_y }, max: PointU64 { x: max_x, y: max_y } })
}

pub fn assign_resize(r: &mut RectU64, size: u64) {
    try_assign_resize(r, size).unwrap()
}

pub fn resize(r: &RectU64, size: u64) -> RectU64 {
    try_resize(r, size).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_u64::RectU64;

    use super::{ assign_resize, resize, try_assign_resize, try_resize};

    #[test]
    fn try_assign_resize_odd() {
        let mut r = RectU64::of(5, 5, 15, 15);
        assert_eq!(try_assign_resize(&mut r, 11), Some(()));
        assert_eq!(r, RectU64::of(5, 5, 15, 15));
        assert_eq!(try_assign_resize(&mut r, 9), Some(()));
        assert_eq!(r, RectU64::of(6, 6, 14, 14));
        assert_eq!(try_assign_resize(&mut r, 7), Some(()));
        assert_eq!(r, RectU64::of(7, 7, 13, 13));
        assert_eq!(try_assign_resize(&mut r, 5), Some(()));
        assert_eq!(r, RectU64::of(8, 8, 12, 12));
        assert_eq!(try_assign_resize(&mut r, 3), Some(()));
        assert_eq!(r, RectU64::of(9, 9, 11, 11));
        assert_eq!(try_assign_resize(&mut r, 9), Some(()));
        assert_eq!(r, RectU64::of(6, 6, 14, 14));
    }

    #[test]
    fn try_assign_resize_even() {
        let mut r = RectU64::of(5, 5, 14, 14);
        assert_eq!(try_assign_resize(&mut r, 10), Some(()));
        assert_eq!(r, RectU64::of(5, 5, 14, 14));
        assert_eq!(try_assign_resize(&mut r, 8), Some(()));
        assert_eq!(r, RectU64::of(6, 6, 13, 13));
        assert_eq!(try_assign_resize(&mut r, 6), Some(()));
        assert_eq!(r, RectU64::of(7, 7, 12, 12));
        assert_eq!(try_assign_resize(&mut r, 4), Some(()));
        assert_eq!(r, RectU64::of(8, 8, 11, 11));
        assert_eq!(try_assign_resize(&mut r, 8), Some(()));
        assert_eq!(r, RectU64::of(6, 6, 13, 13));
    }

    #[test]
    fn try_assign_resize_small_size() {
        let mut r = RectU64::of(10, 10, 20, 20);
        assert_eq!(try_assign_resize(&mut r, 0), None);
        assert_eq!(try_assign_resize(&mut r, 1), None);
        assert_eq!(try_assign_resize(&mut r, 2), None);
        assert_eq!(r, RectU64::of(10, 10, 20, 20));
    }

    #[test]
    fn try_assign_resize_same_size() {
        let mut r_11 = RectU64::of(10, 10, 20, 20);
        assert_eq!(try_assign_resize(&mut r_11, 11), Some(()));
        assert_eq!(r_11, RectU64::of(10, 10, 20, 20));

        let mut r_12 = RectU64::of(10, 10, 21, 21);
        assert_eq!(try_assign_resize(&mut r_12, 12), Some(()));
        assert_eq!(r_12, RectU64::of(10, 10, 21, 21));

        let mut r_13 = RectU64::of(9, 9, 21, 21);
        assert_eq!(try_assign_resize(&mut r_13, 13), Some(()));
        assert_eq!(r_13, RectU64::of(9, 9, 21, 21));
    }

    #[test]
    fn try_assign_resize_odd_small_rect_out_of_bounds() {
        let mut r_min = RectU64::of(0, 0, 2, 2);
        assert_eq!(try_assign_resize(&mut r_min, 11), Some(()));
        assert_eq!(r_min, RectU64::of(0, 0, 10, 10));

        let mut r_max = RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX);
        assert_eq!(try_assign_resize(&mut r_max, 11), Some(()));
        assert_eq!(r_max, RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX));
    }

    #[test]
    fn try_assign_resize_even_small_rect_out_of_bounds() {
        let mut r = RectU64::of(0, 0, 3, 3);
        assert_eq!(try_assign_resize(&mut r, 11), Some(()));
        assert_eq!(r, RectU64::of(0, 0, 10, 10));

        let mut r = RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX);
        assert_eq!(try_assign_resize(&mut r, 11), Some(()));
        assert_eq!(r, RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX));
    }

    #[test]
    fn try_assign_resize_odd_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU64::of(0, 0, 2, 2);
        assert_eq!(try_assign_resize(&mut r_min, u64::MAX), Some(()));
        assert_eq!(r_min, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));

        let mut r_max = RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX);
        assert_eq!(try_assign_resize(&mut r_max, u64::MAX), Some(()));
        assert_eq!(r_max, RectU64::of(1, 1, u64::MAX, u64::MAX));
    }

    #[test]
    fn try_assign_resize_even_small_rect_limits_out_of_bounds() {
        let mut r = RectU64::of(0, 0, 3, 3);
        assert_eq!(try_assign_resize(&mut r, u64::MAX - 1), Some(()));
        assert_eq!(r, RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 2));

        let mut r = RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX);
        assert_eq!(try_assign_resize(&mut r, u64::MAX - 1), Some(()));
        assert_eq!(r, RectU64::of(2, 2, u64::MAX, u64::MAX));
    }

    #[test]
    fn try_assign_resize_big_rect_limits_out_of_bounds() {
        let mut r_odd_1 = RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1);
        assert_eq!(try_assign_resize(&mut r_odd_1, u64::MAX), Some(()));
        assert_eq!(r_odd_1, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));

        let mut r_odd_1 = RectU64::of(1, 1, u64::MAX, u64::MAX);
        assert_eq!(try_assign_resize(&mut r_odd_1, u64::MAX), Some(()));
        assert_eq!(r_odd_1, RectU64::of(1, 1, u64::MAX, u64::MAX));

        let mut r_even = RectU64::largest();
        assert_eq!(try_assign_resize(&mut r_even, u64::MAX), Some(()));
        assert_eq!(r_even, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));
    }

    #[test]
    fn try_resize_odd() {
        assert_eq!(try_resize(&RectU64::of(5, 5, 15, 15), 11), Some(RectU64::of(5, 5, 15, 15)));
        assert_eq!(try_resize(&RectU64::of(5, 5, 15, 15), 9), Some(RectU64::of(6, 6, 14, 14)));
        assert_eq!(try_resize(&RectU64::of(6, 6, 14, 14), 7), Some(RectU64::of(7, 7, 13, 13)));
        assert_eq!(try_resize(&RectU64::of(7, 7, 13, 13), 5), Some(RectU64::of(8, 8, 12, 12)));
        assert_eq!(try_resize(&RectU64::of(8, 8, 12, 12), 3), Some(RectU64::of(9, 9, 11, 11)));
        assert_eq!(try_resize(&RectU64::of(9, 9, 11, 11), 9), Some(RectU64::of(6, 6, 14, 14)));
    }

    #[test]
    fn try_resize_even() {
        assert_eq!(try_resize(&RectU64::of(5, 5, 14, 14), 10), Some(RectU64::of(5, 5, 14, 14)));
        assert_eq!(try_resize(&RectU64::of(5, 5, 14, 14), 8), Some(RectU64::of(6, 6, 13, 13)));
        assert_eq!(try_resize(&RectU64::of(6, 6, 13, 13), 6), Some(RectU64::of(7, 7, 12, 12)));
        assert_eq!(try_resize(&RectU64::of(7, 7, 12, 12), 4), Some(RectU64::of(8, 8, 11, 11)));
        assert_eq!(try_resize(&RectU64::of(8, 8, 11, 11), 8), Some(RectU64::of(6, 6, 13, 13)));
    }

    #[test]
    fn try_resize_small_size() {
        let r = RectU64::of(10, 10, 20, 20);
        assert_eq!(try_resize(&r, 0), None);
        assert_eq!(try_resize(&r, 1), None);
        assert_eq!(try_resize(&r, 2), None);
    }

    #[test]
    fn try_resize_same_size() {
        assert_eq!(try_resize(&RectU64::of(10, 10, 20, 20), 11), Some(RectU64::of(10, 10, 20, 20)));
        assert_eq!(try_resize(&RectU64::of(10, 10, 21, 21), 12), Some(RectU64::of(10, 10, 21, 21)));
        assert_eq!(try_resize(&RectU64::of(9, 9, 21, 21), 13), Some(RectU64::of(9, 9, 21, 21)));
    }

    #[test]
    fn try_resize_odd_small_rect_out_of_bounds() {
        assert_eq!(try_resize(&RectU64::of(0, 0, 2, 2), 11), Some(RectU64::of(0, 0, 10, 10)));
        assert_eq!(
            try_resize(&RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX), 11),
            Some(RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX))
        );
    }

    #[test]
    fn try_resize_even_small_rect_out_of_bounds() {
        assert_eq!(try_resize(&RectU64::of(0, 0, 3, 3), 11), Some(RectU64::of(0, 0, 10, 10)));
        assert_eq!(
            try_resize(&RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX), 11),
            Some(RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX))
        );
    }

    #[test]
    fn try_resize_odd_small_rect_limits_out_of_bounds() {
        assert_eq!(try_resize(&RectU64::of(0, 0, 2, 2), u64::MAX), Some(RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1)));
        assert_eq!(try_resize(&RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX), u64::MAX), Some(RectU64::of(1, 1, u64::MAX, u64::MAX)));
    }

    #[test]
    fn try_resize_even_small_rect_limits_out_of_bounds() {
        assert_eq!(try_resize(&RectU64::of(0, 0, 3, 3), u64::MAX - 1), Some(RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 2)));
        assert_eq!(
            try_resize(&RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX), u64::MAX - 1),
            Some(RectU64::of(2, 2, u64::MAX, u64::MAX))
        );
    }

    #[test]
    fn try_resize_big_rect_limits_out_of_bounds() {
        assert_eq!(try_resize(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1), u64::MAX), Some(RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1)));
        assert_eq!(try_resize(&RectU64::of(1, 1, u64::MAX, u64::MAX), u64::MAX), Some(RectU64::of(1, 1, u64::MAX, u64::MAX)));
        assert_eq!(try_resize(&RectU64::largest(), u64::MAX), Some(RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1)));
    }

    #[test]
    fn assign_resize_odd() {
        let mut r = RectU64::of(5, 5, 15, 15);
        assign_resize(&mut r, 11);
        assert_eq!(r, RectU64::of(5, 5, 15, 15));
        assign_resize(&mut r, 9);
        assert_eq!(r, RectU64::of(6, 6, 14, 14));
        assign_resize(&mut r, 7);
        assert_eq!(r, RectU64::of(7, 7, 13, 13));
        assign_resize(&mut r, 5);
        assert_eq!(r, RectU64::of(8, 8, 12, 12));
        assign_resize(&mut r, 3);
        assert_eq!(r, RectU64::of(9, 9, 11, 11));
        assign_resize(&mut r, 9);
        assert_eq!(r, RectU64::of(6, 6, 14, 14));
    }

    #[test]
    fn assign_resize_even() {
        let mut r = RectU64::of(5, 5, 14, 14);
        assign_resize(&mut r, 10);
        assert_eq!(r, RectU64::of(5, 5, 14, 14));
        assign_resize(&mut r, 8);
        assert_eq!(r, RectU64::of(6, 6, 13, 13));
        assign_resize(&mut r, 6);
        assert_eq!(r, RectU64::of(7, 7, 12, 12));
        assign_resize(&mut r, 4);
        assert_eq!(r, RectU64::of(8, 8, 11, 11));
        assign_resize(&mut r, 8);
        assert_eq!(r, RectU64::of(6, 6, 13, 13));
    }

    #[test]
    fn resize_odd() {
        assert_eq!(resize(&RectU64::of(5, 5, 15, 15), 11), RectU64::of(5, 5, 15, 15));
        assert_eq!(resize(&RectU64::of(5, 5, 15, 15), 9), RectU64::of(6, 6, 14, 14));
        assert_eq!(resize(&RectU64::of(6, 6, 14, 14), 7), RectU64::of(7, 7, 13, 13));
        assert_eq!(resize(&RectU64::of(7, 7, 13, 13), 5), RectU64::of(8, 8, 12, 12));
        assert_eq!(resize(&RectU64::of(8, 8, 12, 12), 3), RectU64::of(9, 9, 11, 11));
        assert_eq!(resize(&RectU64::of(9, 9, 11, 11), 9), RectU64::of(6, 6, 14, 14));
    }

    #[test]
    fn resize_even() {
        assert_eq!(resize(&RectU64::of(5, 5, 14, 14), 10), RectU64::of(5, 5, 14, 14));
        assert_eq!(resize(&RectU64::of(5, 5, 14, 14), 8), RectU64::of(6, 6, 13, 13));
        assert_eq!(resize(&RectU64::of(6, 6, 13, 13), 6), RectU64::of(7, 7, 12, 12));
        assert_eq!(resize(&RectU64::of(7, 7, 12, 12), 4), RectU64::of(8, 8, 11, 11));
        assert_eq!(resize(&RectU64::of(8, 8, 11, 11), 8), RectU64::of(6, 6, 13, 13));
    }
}
