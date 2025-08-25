use super::try_saturating_resize_assign;
use crate::cartesian::rect::rect_u64::RectU64;

#[test]
fn try_saturating_resize_assign_odd() {
    let mut r = RectU64::of(5, 5, 15, 15);
    assert_eq!(try_saturating_resize_assign(&mut r, 11), Some(()));
    assert_eq!(r, RectU64::of(5, 5, 15, 15));
    assert_eq!(try_saturating_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, RectU64::of(6, 6, 14, 14));
    assert_eq!(try_saturating_resize_assign(&mut r, 7), Some(()));
    assert_eq!(r, RectU64::of(7, 7, 13, 13));
    assert_eq!(try_saturating_resize_assign(&mut r, 5), Some(()));
    assert_eq!(r, RectU64::of(8, 8, 12, 12));
    assert_eq!(try_saturating_resize_assign(&mut r, 3), Some(()));
    assert_eq!(r, RectU64::of(9, 9, 11, 11));
    assert_eq!(try_saturating_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, RectU64::of(6, 6, 14, 14));
}

#[test]
fn try_saturating_resize_assign_even() {
    let mut r = RectU64::of(5, 5, 14, 14);
    assert_eq!(try_saturating_resize_assign(&mut r, 10), Some(()));
    assert_eq!(r, RectU64::of(5, 5, 14, 14));
    assert_eq!(try_saturating_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, RectU64::of(6, 6, 13, 13));
    assert_eq!(try_saturating_resize_assign(&mut r, 6), Some(()));
    assert_eq!(r, RectU64::of(7, 7, 12, 12));
    assert_eq!(try_saturating_resize_assign(&mut r, 4), Some(()));
    assert_eq!(r, RectU64::of(8, 8, 11, 11));
    assert_eq!(try_saturating_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, RectU64::of(6, 6, 13, 13));
}

#[test]
fn try_saturating_resize_assign_small_size() {
    let mut r = RectU64::of(10, 10, 20, 20);
    assert_eq!(try_saturating_resize_assign(&mut r, 0), None);
    assert_eq!(try_saturating_resize_assign(&mut r, 1), None);
    assert_eq!(try_saturating_resize_assign(&mut r, 2), None);
    assert_eq!(r, RectU64::of(10, 10, 20, 20));
}

#[test]
fn try_saturating_resize_assign_same_size() {
    let mut r_11 = RectU64::of(10, 10, 20, 20);
    assert_eq!(try_saturating_resize_assign(&mut r_11, 11), Some(()));
    assert_eq!(r_11, RectU64::of(10, 10, 20, 20));

    let mut r_12 = RectU64::of(10, 10, 21, 21);
    assert_eq!(try_saturating_resize_assign(&mut r_12, 12), Some(()));
    assert_eq!(r_12, RectU64::of(10, 10, 21, 21));

    let mut r_13 = RectU64::of(9, 9, 21, 21);
    assert_eq!(try_saturating_resize_assign(&mut r_13, 13), Some(()));
    assert_eq!(r_13, RectU64::of(9, 9, 21, 21));
}

#[test]
fn try_saturating_resize_assign_odd_small_rect_out_of_bounds() {
    let mut r_min = RectU64::of(0, 0, 2, 2);
    assert_eq!(try_saturating_resize_assign(&mut r_min, 11), Some(()));
    assert_eq!(r_min, RectU64::of(0, 0, 10, 10));

    let mut r_max = RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max, 11), Some(()));
    assert_eq!(r_max, RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX));
}

#[test]
fn try_saturating_resize_assign_even_small_rect_out_of_bounds() {
    let mut r = RectU64::of(0, 0, 3, 3);
    assert_eq!(try_saturating_resize_assign(&mut r, 11), Some(()));
    assert_eq!(r, RectU64::of(0, 0, 10, 10));

    let mut r = RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r, 11), Some(()));
    assert_eq!(r, RectU64::of(u64::MAX - 10, u64::MAX - 10, u64::MAX, u64::MAX));
}

#[test]
fn try_saturating_resize_assign_odd_small_rect_limits_out_of_bounds() {
    let mut r_min = RectU64::of(0, 0, 2, 2);
    assert_eq!(try_saturating_resize_assign(&mut r_min, u64::MAX), Some(()));
    assert_eq!(r_min, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));

    let mut r_max = RectU64::of(u64::MAX - 2, u64::MAX - 2, u64::MAX, u64::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max, u64::MAX), Some(()));
    assert_eq!(r_max, RectU64::of(1, 1, u64::MAX, u64::MAX));
}

#[test]
fn try_saturating_resize_assign_even_small_rect_limits_out_of_bounds() {
    let mut r = RectU64::of(0, 0, 3, 3);
    assert_eq!(try_saturating_resize_assign(&mut r, u64::MAX - 1), Some(()));
    assert_eq!(r, RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 2));

    let mut r = RectU64::of(u64::MAX - 3, u64::MAX - 3, u64::MAX, u64::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r, u64::MAX - 1), Some(()));
    assert_eq!(r, RectU64::of(2, 2, u64::MAX, u64::MAX));
}

#[test]
fn try_saturating_resize_assign_big_rect_limits_out_of_bounds() {
    let mut r_odd_1 = RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1);
    assert_eq!(try_saturating_resize_assign(&mut r_odd_1, u64::MAX), Some(()));
    assert_eq!(r_odd_1, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));

    let mut r_odd_1 = RectU64::of(1, 1, u64::MAX, u64::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_odd_1, u64::MAX), Some(()));
    assert_eq!(r_odd_1, RectU64::of(1, 1, u64::MAX, u64::MAX));

    let mut r_even = RectU64::largest();
    assert_eq!(try_saturating_resize_assign(&mut r_even, u64::MAX), Some(()));
    assert_eq!(r_even, RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1));
}
