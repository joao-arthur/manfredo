use super::try_saturating_resize_assign;
use crate::cartesian::rect::rect_i64::RectI64;

#[test]
fn try_saturating_resize_assign_odd() {
    let mut r = RectI64::of(-5, -5, 5, 5);
    assert_eq!(try_saturating_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, RectI64::of(-4, -4, 4, 4));
    assert_eq!(try_saturating_resize_assign(&mut r, 7), Some(()));
    assert_eq!(r, RectI64::of(-3, -3, 3, 3));
    assert_eq!(try_saturating_resize_assign(&mut r, 5), Some(()));
    assert_eq!(r, RectI64::of(-2, -2, 2, 2));
    assert_eq!(try_saturating_resize_assign(&mut r, 3), Some(()));
    assert_eq!(r, RectI64::of(-1, -1, 1, 1));
    assert_eq!(try_saturating_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, RectI64::of(-4, -4, 4, 4));
}

#[test]
fn try_saturating_resize_assign_even() {
    let mut r = RectI64::of(-5, -5, 4, 4);
    assert_eq!(try_saturating_resize_assign(&mut r, 10), Some(()));
    assert_eq!(r, RectI64::of(-5, -5, 4, 4));
    assert_eq!(try_saturating_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, RectI64::of(-4, -4, 3, 3));
    assert_eq!(try_saturating_resize_assign(&mut r, 6), Some(()));
    assert_eq!(r, RectI64::of(-3, -3, 2, 2));
    assert_eq!(try_saturating_resize_assign(&mut r, 4), Some(()));
    assert_eq!(r, RectI64::of(-2, -2, 1, 1));
    assert_eq!(try_saturating_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, RectI64::of(-4, -4, 3, 3));
}

#[test]
fn try_saturating_resize_assign_small_size() {
    let mut r = RectI64::of(10, 10, 20, 20);
    assert_eq!(try_saturating_resize_assign(&mut r, 0), None);
    assert_eq!(try_saturating_resize_assign(&mut r, 1), None);
    assert_eq!(try_saturating_resize_assign(&mut r, 2), None);
    assert_eq!(r, RectI64::of(10, 10, 20, 20));
}

#[test]
fn try_saturating_resize_assign_same_size() {
    let mut r_min_2 = RectI64::of(i64::MIN, i64::MIN, i64::MIN + 2, i64::MIN + 2);
    assert_eq!(try_saturating_resize_assign(&mut r_min_2, 3), Some(()));
    assert_eq!(r_min_2, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 2, i64::MIN + 2));

    let mut r_min_3 = RectI64::of(i64::MIN, i64::MIN, i64::MIN + 3, i64::MIN + 3);
    assert_eq!(try_saturating_resize_assign(&mut r_min_3, 4), Some(()));
    assert_eq!(r_min_3, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 3, i64::MIN + 3));

    let mut r_max_2 = RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max_2, 3), Some(()));
    assert_eq!(r_max_2, RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX));

    let mut r_max_3 = RectI64::of(i64::MAX - 3, i64::MAX - 3, i64::MAX, i64::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max_3, 4), Some(()));
    assert_eq!(r_max_3, RectI64::of(i64::MAX - 3, i64::MAX - 3, i64::MAX, i64::MAX));
}

#[test]
fn try_saturating_resize_assign_bounds() {
    let mut r_min = RectI64::of(i64::MIN, i64::MIN, i64::MIN + 2, i64::MIN + 2);
    assert_eq!(try_saturating_resize_assign(&mut r_min, 11), Some(()));
    assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10));

    let mut r_max = RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max, 11), Some(()));
    assert_eq!(r_max, RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX, i64::MAX));
}

#[test]
fn try_saturating_resize_assign_small_rect_limits() {
    let mut r_min = RectI64::of(i64::MIN, i64::MIN, i64::MIN + 2, i64::MIN + 2);
    assert_eq!(try_saturating_resize_assign(&mut r_min, u64::MAX), Some(()));
    assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1));

    let mut r_max = RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max, u64::MAX), Some(()));
    assert_eq!(r_max, RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX));
}

#[test]
fn try_saturating_resize_assign_big_rect_limits() {
    let mut r_odd_1 = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1);
    assert_eq!(try_saturating_resize_assign(&mut r_odd_1, u64::MAX), Some(()));
    assert_eq!(r_odd_1, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1));

    let mut r_odd_1 = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_odd_1, u64::MAX), Some(()));
    assert_eq!(r_odd_1, RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX));

    let mut r_even = RectI64::largest();
    assert_eq!(try_saturating_resize_assign(&mut r_even, u64::MAX), Some(()));
    assert_eq!(r_even, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1));
}
