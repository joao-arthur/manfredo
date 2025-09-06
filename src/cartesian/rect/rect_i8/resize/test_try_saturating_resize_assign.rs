use super::try_saturating_resize_assign;
use crate::cartesian::rect::rect_i8::RectI8;

#[test]
fn odd() {
    let mut r = RectI8::of(-5, -5, 5, 5);
    assert_eq!(try_saturating_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, RectI8::of(-4, -4, 4, 4));
    assert_eq!(try_saturating_resize_assign(&mut r, 7), Some(()));
    assert_eq!(r, RectI8::of(-3, -3, 3, 3));
    assert_eq!(try_saturating_resize_assign(&mut r, 5), Some(()));
    assert_eq!(r, RectI8::of(-2, -2, 2, 2));
    assert_eq!(try_saturating_resize_assign(&mut r, 3), Some(()));
    assert_eq!(r, RectI8::of(-1, -1, 1, 1));
    assert_eq!(try_saturating_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, RectI8::of(-4, -4, 4, 4));
}

#[test]
fn even() {
    let mut r = RectI8::of(-5, -5, 4, 4);
    assert_eq!(try_saturating_resize_assign(&mut r, 10), Some(()));
    assert_eq!(r, RectI8::of(-5, -5, 4, 4));
    assert_eq!(try_saturating_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, RectI8::of(-4, -4, 3, 3));
    assert_eq!(try_saturating_resize_assign(&mut r, 6), Some(()));
    assert_eq!(r, RectI8::of(-3, -3, 2, 2));
    assert_eq!(try_saturating_resize_assign(&mut r, 4), Some(()));
    assert_eq!(r, RectI8::of(-2, -2, 1, 1));
    assert_eq!(try_saturating_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, RectI8::of(-4, -4, 3, 3));
}

#[test]
fn small_size() {
    let mut r = RectI8::of(10, 10, 20, 20);
    assert_eq!(try_saturating_resize_assign(&mut r, 0), None);
    assert_eq!(try_saturating_resize_assign(&mut r, 1), None);
    assert_eq!(try_saturating_resize_assign(&mut r, 2), None);
    assert_eq!(r, RectI8::of(10, 10, 20, 20));
}

#[test]
fn same_size() {
    let mut r_min_2 = RectI8::of(i8::MIN, i8::MIN, i8::MIN + 2, i8::MIN + 2);
    assert_eq!(try_saturating_resize_assign(&mut r_min_2, 3), Some(()));
    assert_eq!(r_min_2, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 2, i8::MIN + 2));

    let mut r_min_3 = RectI8::of(i8::MIN, i8::MIN, i8::MIN + 3, i8::MIN + 3);
    assert_eq!(try_saturating_resize_assign(&mut r_min_3, 4), Some(()));
    assert_eq!(r_min_3, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 3, i8::MIN + 3));

    let mut r_max_2 = RectI8::of(i8::MAX - 2, i8::MAX - 2, i8::MAX, i8::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max_2, 3), Some(()));
    assert_eq!(r_max_2, RectI8::of(i8::MAX - 2, i8::MAX - 2, i8::MAX, i8::MAX));

    let mut r_max_3 = RectI8::of(i8::MAX - 3, i8::MAX - 3, i8::MAX, i8::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max_3, 4), Some(()));
    assert_eq!(r_max_3, RectI8::of(i8::MAX - 3, i8::MAX - 3, i8::MAX, i8::MAX));
}

#[test]
fn bounds() {
    let mut r_min = RectI8::of(i8::MIN, i8::MIN, i8::MIN + 2, i8::MIN + 2);
    assert_eq!(try_saturating_resize_assign(&mut r_min, 11), Some(()));
    assert_eq!(r_min, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10));

    let mut r_max = RectI8::of(i8::MAX - 2, i8::MAX - 2, i8::MAX, i8::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max, 11), Some(()));
    assert_eq!(r_max, RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX, i8::MAX));
}

#[test]
fn small_rect_limits() {
    let mut r_min = RectI8::of(i8::MIN, i8::MIN, i8::MIN + 2, i8::MIN + 2);
    assert_eq!(try_saturating_resize_assign(&mut r_min, u8::MAX), Some(()));
    assert_eq!(r_min, RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1));

    let mut r_max = RectI8::of(i8::MAX - 2, i8::MAX - 2, i8::MAX, i8::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max, u8::MAX), Some(()));
    assert_eq!(r_max, RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX));
}

#[test]
fn big_rect_limits() {
    let mut r_odd_1 = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1);
    assert_eq!(try_saturating_resize_assign(&mut r_odd_1, u8::MAX), Some(()));
    assert_eq!(r_odd_1, RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1));

    let mut r_odd_1 = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_odd_1, u8::MAX), Some(()));
    assert_eq!(r_odd_1, RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX));

    let mut r_even = RectI8::largest();
    assert_eq!(try_saturating_resize_assign(&mut r_even, u8::MAX), Some(()));
    assert_eq!(r_even, RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1));
}
