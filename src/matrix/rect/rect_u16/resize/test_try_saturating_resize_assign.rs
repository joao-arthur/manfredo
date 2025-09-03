use super::try_saturating_resize_assign;
use crate::matrix::rect::rect_u16::RectU16;

#[test]
fn try_saturating_resize_assign_odd() {
    let mut r = RectU16::of(5, 5, 15, 15);
    assert_eq!(try_saturating_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, RectU16::of(6, 6, 14, 14));
    assert_eq!(try_saturating_resize_assign(&mut r, 7), Some(()));
    assert_eq!(r, RectU16::of(7, 7, 13, 13));
    assert_eq!(try_saturating_resize_assign(&mut r, 5), Some(()));
    assert_eq!(r, RectU16::of(8, 8, 12, 12));
    assert_eq!(try_saturating_resize_assign(&mut r, 3), Some(()));
    assert_eq!(r, RectU16::of(9, 9, 11, 11));
    assert_eq!(try_saturating_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, RectU16::of(6, 6, 14, 14));
}

#[test]
fn try_saturating_resize_assign_even() {
    let mut r = RectU16::of(5, 5, 14, 14);
    assert_eq!(try_saturating_resize_assign(&mut r, 10), Some(()));
    assert_eq!(r, RectU16::of(5, 5, 14, 14));
    assert_eq!(try_saturating_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, RectU16::of(6, 6, 13, 13));
    assert_eq!(try_saturating_resize_assign(&mut r, 6), Some(()));
    assert_eq!(r, RectU16::of(7, 7, 12, 12));
    assert_eq!(try_saturating_resize_assign(&mut r, 4), Some(()));
    assert_eq!(r, RectU16::of(8, 8, 11, 11));
    assert_eq!(try_saturating_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, RectU16::of(6, 6, 13, 13));
}

#[test]
fn try_saturating_resize_assign_small_size() {
    let mut r = RectU16::of(10, 10, 20, 20);
    assert_eq!(try_saturating_resize_assign(&mut r, 0), None);
    assert_eq!(try_saturating_resize_assign(&mut r, 1), None);
    assert_eq!(try_saturating_resize_assign(&mut r, 2), None);
    assert_eq!(r, RectU16::of(10, 10, 20, 20));
}

#[test]
fn try_saturating_resize_assign_same_size() {
    let mut r_min_2 = RectU16::of(0, 0, 2, 2);
    assert_eq!(try_saturating_resize_assign(&mut r_min_2, 3), Some(()));
    assert_eq!(r_min_2, RectU16::of(0, 0, 2, 2));

    let mut r_min_3 = RectU16::of(0, 0, 3, 3);
    assert_eq!(try_saturating_resize_assign(&mut r_min_3, 4), Some(()));
    assert_eq!(r_min_3, RectU16::of(0, 0, 3, 3));

    let mut r_max_2 = RectU16::of(u16::MAX - 2, u16::MAX - 2, u16::MAX, u16::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max_2, 3), Some(()));
    assert_eq!(r_max_2, RectU16::of(u16::MAX - 2, u16::MAX - 2, u16::MAX, u16::MAX));

    let mut r_max_3 = RectU16::of(u16::MAX - 3, u16::MAX - 3, u16::MAX, u16::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max_3, 4), Some(()));
    assert_eq!(r_max_3, RectU16::of(u16::MAX - 3, u16::MAX - 3, u16::MAX, u16::MAX));
}

#[test]
fn try_saturating_resize_assign_bounds() {
    let mut r_min = RectU16::of(0, 0, 2, 2);
    assert_eq!(try_saturating_resize_assign(&mut r_min, 11), Some(()));
    assert_eq!(r_min, RectU16::of(0, 0, 10, 10));

    let mut r_max = RectU16::of(u16::MAX - 2, u16::MAX - 2, u16::MAX, u16::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max, 11), Some(()));
    assert_eq!(r_max, RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX, u16::MAX));
}

#[test]
fn try_saturating_resize_assign_small_rect_limits() {
    let mut r_min = RectU16::of(0, 0, 2, 2);
    assert_eq!(try_saturating_resize_assign(&mut r_min, u16::MAX), Some(()));
    assert_eq!(r_min, RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1));

    let mut r_max = RectU16::of(u16::MAX - 2, u16::MAX - 2, u16::MAX, u16::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max, u16::MAX), Some(()));
    assert_eq!(r_max, RectU16::of(1, 1, u16::MAX, u16::MAX));
}

#[test]
fn try_saturating_resize_assign_big_rect_limits() {
    let mut r_odd_1 = RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1);
    assert_eq!(try_saturating_resize_assign(&mut r_odd_1, u16::MAX), Some(()));
    assert_eq!(r_odd_1, RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1));

    let mut r_odd_1 = RectU16::of(1, 1, u16::MAX, u16::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_odd_1, u16::MAX), Some(()));
    assert_eq!(r_odd_1, RectU16::of(1, 1, u16::MAX, u16::MAX));

    let mut r_even = RectU16::largest();
    assert_eq!(try_saturating_resize_assign(&mut r_even, u16::MAX), Some(()));
    assert_eq!(r_even, RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1));
}
