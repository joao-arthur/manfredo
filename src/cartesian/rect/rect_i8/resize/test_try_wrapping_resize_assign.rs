use super::try_wrapping_resize_assign;
use crate::cartesian::rect::rect_i8::RectI8;

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[test]
fn odd() {
    let mut r = RectI8::of(-5, -5, 5, 5);
    assert_eq!(try_wrapping_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, RectI8::of(-4, -4, 4, 4));
    assert_eq!(try_wrapping_resize_assign(&mut r, 7), Some(()));
    assert_eq!(r, RectI8::of(-3, -3, 3, 3));
    assert_eq!(try_wrapping_resize_assign(&mut r, 5), Some(()));
    assert_eq!(r, RectI8::of(-2, -2, 2, 2));
    assert_eq!(try_wrapping_resize_assign(&mut r, 3), Some(()));
    assert_eq!(r, RectI8::of(-1, -1, 1, 1));
    assert_eq!(try_wrapping_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, RectI8::of(-4, -4, 4, 4));
}

#[test]
fn even() {
    let mut r = RectI8::of(-5, -5, 4, 4);
    assert_eq!(try_wrapping_resize_assign(&mut r, 10), Some(()));
    assert_eq!(r, RectI8::of(-5, -5, 4, 4));
    assert_eq!(try_wrapping_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, RectI8::of(-4, -4, 3, 3));
    assert_eq!(try_wrapping_resize_assign(&mut r, 6), Some(()));
    assert_eq!(r, RectI8::of(-3, -3, 2, 2));
    assert_eq!(try_wrapping_resize_assign(&mut r, 4), Some(()));
    assert_eq!(r, RectI8::of(-2, -2, 1, 1));
    assert_eq!(try_wrapping_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, RectI8::of(-4, -4, 3, 3));
}

#[test]
fn small_size() {
    let mut r = RectI8::of(10, 10, 20, 20);
    assert_eq!(try_wrapping_resize_assign(&mut r, 0), None);
    assert_eq!(try_wrapping_resize_assign(&mut r, 1), None);
    assert_eq!(try_wrapping_resize_assign(&mut r, 2), None);
    assert_eq!(r, RectI8::of(10, 10, 20, 20));
}

#[test]
fn same_size() {
    let mut r_min_2 = RectI8::of(MIN, MIN, MIN + 2, MIN + 2);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_2, 3), Some(()));
    assert_eq!(r_min_2, RectI8::of(MIN, MIN, MIN + 2, MIN + 2));

    let mut r_min_3 = RectI8::of(MIN, MIN, MIN + 3, MIN + 3);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_3, 4), Some(()));
    assert_eq!(r_min_3, RectI8::of(MIN, MIN, MIN + 3, MIN + 3));

    let mut r_max_2 = RectI8::of(MAX - 2, MAX - 2, MAX, MAX);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_2, 3), Some(()));
    assert_eq!(r_max_2, RectI8::of(MAX - 2, MAX - 2, MAX, MAX));

    let mut r_max_3 = RectI8::of(MAX - 3, MAX - 3, MAX, MAX);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_3, 4), Some(()));
    assert_eq!(r_max_3, RectI8::of(MAX - 3, MAX - 3, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r_min_x = RectI8::of(MIN, MIN + 2, MIN + 2, MIN + 4);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_x, 5), Some(()));
    assert_eq!(r_min_x, RectI8::of(MAX, MIN + 1, MIN + 3, MIN + 5));

    let mut r_min_y = RectI8::of(MIN + 2, MIN, MIN + 4, MIN + 2);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_y, 5), Some(()));
    assert_eq!(r_min_y, RectI8::of(MIN + 1, MAX, MIN + 5, MIN + 3));

    let mut r_max_x = RectI8::of(MAX - 2, MAX - 4, MAX, MAX - 2);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_x, 5), Some(()));
    assert_eq!(r_max_x, RectI8::of(MAX - 3, MAX - 5, MIN, MAX - 1));

    let mut r_max_y = RectI8::of(MAX - 4, MAX - 2, MAX - 2, MAX);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_y, 5), Some(()));
    assert_eq!(r_max_y, RectI8::of(MAX - 5, MAX - 3, MAX - 1, MIN));
}

#[test]
fn small_rect_limits_out_of_bounds() {
    let mut r_min_x = RectI8::of(MIN, MIN + 2, MIN + 2, MIN + 4);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_x, u8::MAX), Some(()));
    assert_eq!(r_min_x, RectI8::of(2, 4, 0, 2));

    let mut r_min_y = RectI8::of(MIN + 2, MIN, MIN + 4, MIN + 2);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_y, u8::MAX), Some(()));
    assert_eq!(r_min_y, RectI8::of(4, 2, 2, 0));

    let mut r_max_x = RectI8::of(MAX - 2, MAX - 4, MAX, MAX - 2);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_x, u8::MAX), Some(()));
    assert_eq!(r_max_x, RectI8::of(-1, -3, -3, -5));

    let mut r_max_y = RectI8::of(MAX - 4, MAX - 2, MAX - 2, MAX);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_y, u8::MAX), Some(()));
    assert_eq!(r_max_y, RectI8::of(-3, -1, -5, -3));
}

#[test]
fn big_rect_limits_out_of_bounds() {
    let mut r_odd_1 = RectI8::of(MIN, MIN, MAX - 1, MAX - 1);
    assert_eq!(try_wrapping_resize_assign(&mut r_odd_1, u8::MAX), Some(()));
    assert_eq!(r_odd_1, RectI8::of(MIN, MIN, MAX - 1, MAX - 1));

    let mut r_odd_1 = RectI8::of(MIN + 1, MIN + 1, MAX, MAX);
    assert_eq!(try_wrapping_resize_assign(&mut r_odd_1, u8::MAX), Some(()));
    assert_eq!(r_odd_1, RectI8::of(MIN + 1, MIN + 1, MAX, MAX));

    let mut r_even = RectI8::largest();
    assert_eq!(try_wrapping_resize_assign(&mut r_even, u8::MAX), Some(()));
    assert_eq!(r_even, RectI8::of(MIN, MIN, MAX - 1, MAX - 1));
}
