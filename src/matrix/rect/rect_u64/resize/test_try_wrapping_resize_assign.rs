use super::try_wrapping_resize_assign;
use crate::matrix::rect::rect_u64::RectU64;

#[test]
fn try_wrapping_resize_assign_odd() {
    let mut r = RectU64::of(5, 5, 15, 15);
    assert_eq!(try_wrapping_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, RectU64::of(6, 6, 14, 14));
    assert_eq!(try_wrapping_resize_assign(&mut r, 7), Some(()));
    assert_eq!(r, RectU64::of(7, 7, 13, 13));
    assert_eq!(try_wrapping_resize_assign(&mut r, 5), Some(()));
    assert_eq!(r, RectU64::of(8, 8, 12, 12));
    assert_eq!(try_wrapping_resize_assign(&mut r, 3), Some(()));
    assert_eq!(r, RectU64::of(9, 9, 11, 11));
    assert_eq!(try_wrapping_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, RectU64::of(6, 6, 14, 14));
}

#[test]
fn try_wrapping_resize_assign_even() {
    let mut r = RectU64::of(5, 5, 14, 14);
    assert_eq!(try_wrapping_resize_assign(&mut r, 10), Some(()));
    assert_eq!(r, RectU64::of(5, 5, 14, 14));
    assert_eq!(try_wrapping_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, RectU64::of(6, 6, 13, 13));
    assert_eq!(try_wrapping_resize_assign(&mut r, 6), Some(()));
    assert_eq!(r, RectU64::of(7, 7, 12, 12));
    assert_eq!(try_wrapping_resize_assign(&mut r, 4), Some(()));
    assert_eq!(r, RectU64::of(8, 8, 11, 11));
    assert_eq!(try_wrapping_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, RectU64::of(6, 6, 13, 13));
}

#[test]
fn try_wrapping_resize_assign_small_size() {
    let mut r = RectU64::of(10, 10, 20, 20);
    assert_eq!(try_wrapping_resize_assign(&mut r, 0), None);
    assert_eq!(try_wrapping_resize_assign(&mut r, 1), None);
    assert_eq!(try_wrapping_resize_assign(&mut r, 2), None);
    assert_eq!(r, RectU64::of(10, 10, 20, 20));
}

#[test]
fn try_wrapping_resize_assign_same_size() {
    let mut r_min_2 = RectU64::of(0, 0, 2, 2);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_2, 3), Some(()));
    assert_eq!(r_min_2, RectU64::of(0, 0, 2, 2));

    let mut r_min_3 = RectU64::of(0, 0, 3, 3);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_3, 4), Some(()));
    assert_eq!(r_min_3, RectU64::of(0, 0, 3, 3));

    let mut r_max_2 = RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_2, 3), Some(()));
    assert_eq!(r_max_2, RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX));

    let mut r_max_3 = RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_3, 4), Some(()));
    assert_eq!(r_max_3, RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX));
}

#[test]
fn try_wrapping_resize_assign_small_rect_out_of_bounds() {
    let mut r_min_x = RectU64::of(0, 2, 2, 4);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_x, 5), Some(()));
    assert_eq!(r_min_x, RectU64::of(u64::MAX, 1, 3, 5));

    let mut r_min_y = RectU64::of(2, 0, 4, 2);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_y, 5), Some(()));
    assert_eq!(r_min_y, RectU64::of(1, u64::MAX, 5, 3));

    let mut r_max_x = RectU64::of(u64::MAX - 2, u64::MAX - 4, u64::MAX, u64::MAX - 2);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_x, 5), Some(()));
    assert_eq!(r_max_x, RectU64::of(u64::MAX - 3, u64::MAX - 5, 0, u64::MAX - 1));

    let mut r_max_y = RectU64::of(u64::MAX - 4, u64::MAX - 2, u64::MAX - 2, u64::MAX);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_y, 5), Some(()));
    assert_eq!(r_max_y, RectU64::of(u64::MAX - 5, u64::MAX - 3, u64::MAX - 1, 0));
}

#[test]
fn try_wrapping_resize_assign_small_rect_limits_out_of_bounds() {
    let mut r_min_x = RectU64::of(0, 2, 2, 4);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_x, u64::MAX), Some(()));
    assert_eq!(r_min_x, RectU64::of(u64::MAX / 2 + 3, u64::MAX / 2 + 5, u64::MAX / 2 + 1, u64::MAX / 2 + 3));

    let mut r_min_y = RectU64::of(2, 0, 4, 2);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_y, u64::MAX), Some(()));
    assert_eq!(r_min_y, RectU64::of(u64::MAX / 2 + 5, u64::MAX / 2 + 3, u64::MAX / 2 + 3, u64::MAX / 2 + 1));

    let mut r_max_x = RectU64::of(u64::MAX - 2, u64::MAX - 4, u64::MAX, u64::MAX - 2);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_x, u64::MAX), Some(()));
    assert_eq!(r_max_x, RectU64::of(u64::MAX / 2, u64::MAX / 2 - 2, u64::MAX / 2 - 2, u64::MAX / 2 - 4));

    let mut r_max_y = RectU64::of(u64::MAX - 4, u64::MAX - 2, u64::MAX - 2, u64::MAX);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_y, u64::MAX), Some(()));
    assert_eq!(r_max_y, RectU64::of(u64::MAX / 2 - 2, u64::MAX / 2, u64::MAX / 2 - 4, u64::MAX / 2 - 2));
}

#[test]
fn try_wrapping_resize_assign_big_rect_limits_out_of_bounds() {
    let mut r_odd_1 = RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1);
    assert_eq!(try_wrapping_resize_assign(&mut r_odd_1, u64::MAX), Some(()));
    assert_eq!(r_odd_1, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));

    let mut r_odd_1 = RectU64::of(1, 1, u64::MAX, u64::MAX);
    assert_eq!(try_wrapping_resize_assign(&mut r_odd_1, u64::MAX), Some(()));
    assert_eq!(r_odd_1, RectU64::of(1, 1, u64::MAX, u64::MAX));

    let mut r_even = RectU64::largest();
    assert_eq!(try_wrapping_resize_assign(&mut r_even, u64::MAX), Some(()));
    assert_eq!(r_even, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));
}
