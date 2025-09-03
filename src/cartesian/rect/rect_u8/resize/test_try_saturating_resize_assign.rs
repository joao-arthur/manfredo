use super::try_saturating_resize_assign;
use crate::cartesian::rect::rect_u8::RectU8;

#[test]
fn try_saturating_resize_assign_odd() {
    let mut r = RectU8::of(5, 5, 15, 15);
    assert_eq!(try_saturating_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, RectU8::of(6, 6, 14, 14));
    assert_eq!(try_saturating_resize_assign(&mut r, 7), Some(()));
    assert_eq!(r, RectU8::of(7, 7, 13, 13));
    assert_eq!(try_saturating_resize_assign(&mut r, 5), Some(()));
    assert_eq!(r, RectU8::of(8, 8, 12, 12));
    assert_eq!(try_saturating_resize_assign(&mut r, 3), Some(()));
    assert_eq!(r, RectU8::of(9, 9, 11, 11));
    assert_eq!(try_saturating_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, RectU8::of(6, 6, 14, 14));
}

#[test]
fn try_saturating_resize_assign_even() {
    let mut r = RectU8::of(5, 5, 14, 14);
    assert_eq!(try_saturating_resize_assign(&mut r, 10), Some(()));
    assert_eq!(r, RectU8::of(5, 5, 14, 14));
    assert_eq!(try_saturating_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, RectU8::of(6, 6, 13, 13));
    assert_eq!(try_saturating_resize_assign(&mut r, 6), Some(()));
    assert_eq!(r, RectU8::of(7, 7, 12, 12));
    assert_eq!(try_saturating_resize_assign(&mut r, 4), Some(()));
    assert_eq!(r, RectU8::of(8, 8, 11, 11));
    assert_eq!(try_saturating_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, RectU8::of(6, 6, 13, 13));
}

#[test]
fn try_saturating_resize_assign_small_size() {
    let mut r = RectU8::of(10, 10, 20, 20);
    assert_eq!(try_saturating_resize_assign(&mut r, 0), None);
    assert_eq!(try_saturating_resize_assign(&mut r, 1), None);
    assert_eq!(try_saturating_resize_assign(&mut r, 2), None);
    assert_eq!(r, RectU8::of(10, 10, 20, 20));
}

#[test]
fn try_saturating_resize_assign_same_size() {
    let mut r_min_2 = RectU8::of(0, 0, 2, 2);
    assert_eq!(try_saturating_resize_assign(&mut r_min_2, 3), Some(()));
    assert_eq!(r_min_2, RectU8::of(0, 0, 2, 2));

    let mut r_min_3 = RectU8::of(0, 0, 3, 3);
    assert_eq!(try_saturating_resize_assign(&mut r_min_3, 4), Some(()));
    assert_eq!(r_min_3, RectU8::of(0, 0, 3, 3));

    let mut r_max_2 = RectU8::of(u8::MAX - 2, u8::MAX - 2, u8::MAX, u8::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max_2, 3), Some(()));
    assert_eq!(r_max_2, RectU8::of(u8::MAX - 2, u8::MAX - 2, u8::MAX, u8::MAX));

    let mut r_max_3 = RectU8::of(u8::MAX - 3, u8::MAX - 3, u8::MAX, u8::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max_3, 4), Some(()));
    assert_eq!(r_max_3, RectU8::of(u8::MAX - 3, u8::MAX - 3, u8::MAX, u8::MAX));
}

#[test]
fn try_saturating_resize_assign_bounds() {
    let mut r_min = RectU8::of(0, 0, 2, 2);
    assert_eq!(try_saturating_resize_assign(&mut r_min, 11), Some(()));
    assert_eq!(r_min, RectU8::of(0, 0, 10, 10));

    let mut r_max = RectU8::of(u8::MAX - 2, u8::MAX - 2, u8::MAX, u8::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max, 11), Some(()));
    assert_eq!(r_max, RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX, u8::MAX));
}

#[test]
fn try_saturating_resize_assign_small_rect_limits() {
    let mut r_min = RectU8::of(0, 0, 2, 2);
    assert_eq!(try_saturating_resize_assign(&mut r_min, u8::MAX), Some(()));
    assert_eq!(r_min, RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1));

    let mut r_max = RectU8::of(u8::MAX - 2, u8::MAX - 2, u8::MAX, u8::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max, u8::MAX), Some(()));
    assert_eq!(r_max, RectU8::of(1, 1, u8::MAX, u8::MAX));
}

#[test]
fn try_saturating_resize_assign_big_rect_limits() {
    let mut r_odd_1 = RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1);
    assert_eq!(try_saturating_resize_assign(&mut r_odd_1, u8::MAX), Some(()));
    assert_eq!(r_odd_1, RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1));

    let mut r_odd_1 = RectU8::of(1, 1, u8::MAX, u8::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_odd_1, u8::MAX), Some(()));
    assert_eq!(r_odd_1, RectU8::of(1, 1, u8::MAX, u8::MAX));

    let mut r_even = RectU8::largest();
    assert_eq!(try_saturating_resize_assign(&mut r_even, u8::MAX), Some(()));
    assert_eq!(r_even, RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1));
}
