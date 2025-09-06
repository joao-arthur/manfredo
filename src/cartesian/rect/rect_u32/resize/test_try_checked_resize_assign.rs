use super::try_checked_resize_assign;
use crate::cartesian::rect::rect_u32::RectU32;

#[test]
fn odd() {
    let mut r = RectU32::of(5, 5, 15, 15);
    assert_eq!(try_checked_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, RectU32::of(6, 6, 14, 14));
    assert_eq!(try_checked_resize_assign(&mut r, 7), Some(()));
    assert_eq!(r, RectU32::of(7, 7, 13, 13));
    assert_eq!(try_checked_resize_assign(&mut r, 5), Some(()));
    assert_eq!(r, RectU32::of(8, 8, 12, 12));
    assert_eq!(try_checked_resize_assign(&mut r, 3), Some(()));
    assert_eq!(r, RectU32::of(9, 9, 11, 11));
    assert_eq!(try_checked_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, RectU32::of(6, 6, 14, 14));
}

#[test]
fn even() {
    let mut r = RectU32::of(5, 5, 14, 14);
    assert_eq!(try_checked_resize_assign(&mut r, 10), Some(()));
    assert_eq!(r, RectU32::of(5, 5, 14, 14));
    assert_eq!(try_checked_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, RectU32::of(6, 6, 13, 13));
    assert_eq!(try_checked_resize_assign(&mut r, 6), Some(()));
    assert_eq!(r, RectU32::of(7, 7, 12, 12));
    assert_eq!(try_checked_resize_assign(&mut r, 4), Some(()));
    assert_eq!(r, RectU32::of(8, 8, 11, 11));
    assert_eq!(try_checked_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, RectU32::of(6, 6, 13, 13));
}

#[test]
fn small_size() {
    let mut r = RectU32::of(10, 10, 20, 20);
    assert_eq!(try_checked_resize_assign(&mut r, 0), None);
    assert_eq!(try_checked_resize_assign(&mut r, 1), None);
    assert_eq!(try_checked_resize_assign(&mut r, 2), None);
    assert_eq!(r, RectU32::of(10, 10, 20, 20));
}

#[test]
fn same_size() {
    let mut r_min_2 = RectU32::of(0, 0, 2, 2);
    assert_eq!(try_checked_resize_assign(&mut r_min_2, 3), Some(()));
    assert_eq!(r_min_2, RectU32::of(0, 0, 2, 2));

    let mut r_min_3 = RectU32::of(0, 0, 3, 3);
    assert_eq!(try_checked_resize_assign(&mut r_min_3, 4), Some(()));
    assert_eq!(r_min_3, RectU32::of(0, 0, 3, 3));

    let mut r_max_2 = RectU32::of(u32::MAX - 2, u32::MAX - 2, u32::MAX, u32::MAX);
    assert_eq!(try_checked_resize_assign(&mut r_max_2, 3), Some(()));
    assert_eq!(r_max_2, RectU32::of(u32::MAX - 2, u32::MAX - 2, u32::MAX, u32::MAX));

    let mut r_max_3 = RectU32::of(u32::MAX - 3, u32::MAX - 3, u32::MAX, u32::MAX);
    assert_eq!(try_checked_resize_assign(&mut r_max_3, 4), Some(()));
    assert_eq!(r_max_3, RectU32::of(u32::MAX - 3, u32::MAX - 3, u32::MAX, u32::MAX));
}

#[test]
fn small_rect_out_of_bounds() {
    let mut r_min_x = RectU32::of(0, 2, 2, 4);
    assert_eq!(try_checked_resize_assign(&mut r_min_x, 5), None);
    assert_eq!(r_min_x, RectU32::of(0, 2, 2, 4));

    let mut r_min_y = RectU32::of(2, 0, 4, 2);
    assert_eq!(try_checked_resize_assign(&mut r_min_y, 5), None);
    assert_eq!(r_min_y, RectU32::of(2, 0, 4, 2));

    let mut r_max_x = RectU32::of(u32::MAX - 2, u32::MAX - 4, u32::MAX, u32::MAX - 2);
    assert_eq!(try_checked_resize_assign(&mut r_max_x, 5), None);
    assert_eq!(r_max_x, RectU32::of(u32::MAX - 2, u32::MAX - 4, u32::MAX, u32::MAX - 2));

    let mut r_max_y = RectU32::of(u32::MAX - 4, u32::MAX - 2, u32::MAX - 2, u32::MAX);
    assert_eq!(try_checked_resize_assign(&mut r_max_y, 5), None);
    assert_eq!(r_max_y, RectU32::of(u32::MAX - 4, u32::MAX - 2, u32::MAX - 2, u32::MAX));
}

#[test]
fn small_rect_limits_out_of_bounds() {
    let mut r_min_x = RectU32::of(0, 2, 2, 4);
    assert_eq!(try_checked_resize_assign(&mut r_min_x, u32::MAX), None);
    assert_eq!(r_min_x, RectU32::of(0, 2, 2, 4));

    let mut r_min_y = RectU32::of(2, 0, 4, 2);
    assert_eq!(try_checked_resize_assign(&mut r_min_y, u32::MAX), None);
    assert_eq!(r_min_y, RectU32::of(2, 0, 4, 2));

    let mut r_max_x = RectU32::of(u32::MAX - 2, u32::MAX - 4, u32::MAX, u32::MAX - 2);
    assert_eq!(try_checked_resize_assign(&mut r_max_x, u32::MAX), None);
    assert_eq!(r_max_x, RectU32::of(u32::MAX - 2, u32::MAX - 4, u32::MAX, u32::MAX - 2));

    let mut r_max_y = RectU32::of(u32::MAX - 4, u32::MAX - 2, u32::MAX - 2, u32::MAX);
    assert_eq!(try_checked_resize_assign(&mut r_max_y, u32::MAX), None);
    assert_eq!(r_max_y, RectU32::of(u32::MAX - 4, u32::MAX - 2, u32::MAX - 2, u32::MAX));
}

#[test]
fn big_rect_limits_out_of_bounds() {
    let mut r_odd_1 = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
    assert_eq!(try_checked_resize_assign(&mut r_odd_1, u32::MAX), Some(()));
    assert_eq!(r_odd_1, RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1));

    let mut r_odd_1 = RectU32::of(1, 1, u32::MAX, u32::MAX);
    assert_eq!(try_checked_resize_assign(&mut r_odd_1, u32::MAX), Some(()));
    assert_eq!(r_odd_1, RectU32::of(1, 1, u32::MAX, u32::MAX));

    let mut r_even = RectU32::largest();
    assert_eq!(try_checked_resize_assign(&mut r_even, u32::MAX), Some(()));
    assert_eq!(r_even, RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1));
}
