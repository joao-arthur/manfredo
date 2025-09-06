use super::try_wrapping_resize_assign;
use crate::matrix::rect::rect_i32::RectI32;

#[test]
fn odd() {
    let mut r = RectI32::of(-5, -5, 5, 5);
    assert_eq!(try_wrapping_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, RectI32::of(-4, -4, 4, 4));
    assert_eq!(try_wrapping_resize_assign(&mut r, 7), Some(()));
    assert_eq!(r, RectI32::of(-3, -3, 3, 3));
    assert_eq!(try_wrapping_resize_assign(&mut r, 5), Some(()));
    assert_eq!(r, RectI32::of(-2, -2, 2, 2));
    assert_eq!(try_wrapping_resize_assign(&mut r, 3), Some(()));
    assert_eq!(r, RectI32::of(-1, -1, 1, 1));
    assert_eq!(try_wrapping_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, RectI32::of(-4, -4, 4, 4));
}

#[test]
fn even() {
    let mut r = RectI32::of(-5, -5, 4, 4);
    assert_eq!(try_wrapping_resize_assign(&mut r, 10), Some(()));
    assert_eq!(r, RectI32::of(-5, -5, 4, 4));
    assert_eq!(try_wrapping_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, RectI32::of(-4, -4, 3, 3));
    assert_eq!(try_wrapping_resize_assign(&mut r, 6), Some(()));
    assert_eq!(r, RectI32::of(-3, -3, 2, 2));
    assert_eq!(try_wrapping_resize_assign(&mut r, 4), Some(()));
    assert_eq!(r, RectI32::of(-2, -2, 1, 1));
    assert_eq!(try_wrapping_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, RectI32::of(-4, -4, 3, 3));
}

#[test]
fn small_size() {
    let mut r = RectI32::of(10, 10, 20, 20);
    assert_eq!(try_wrapping_resize_assign(&mut r, 0), None);
    assert_eq!(try_wrapping_resize_assign(&mut r, 1), None);
    assert_eq!(try_wrapping_resize_assign(&mut r, 2), None);
    assert_eq!(r, RectI32::of(10, 10, 20, 20));
}

#[test]
fn same_size() {
    let mut r_min_2 = RectI32::of(i32::MIN, i32::MIN, i32::MIN + 2, i32::MIN + 2);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_2, 3), Some(()));
    assert_eq!(r_min_2, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 2, i32::MIN + 2));

    let mut r_min_3 = RectI32::of(i32::MIN, i32::MIN, i32::MIN + 3, i32::MIN + 3);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_3, 4), Some(()));
    assert_eq!(r_min_3, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 3, i32::MIN + 3));

    let mut r_max_2 = RectI32::of(i32::MAX - 2, i32::MAX - 2, i32::MAX, i32::MAX);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_2, 3), Some(()));
    assert_eq!(r_max_2, RectI32::of(i32::MAX - 2, i32::MAX - 2, i32::MAX, i32::MAX));

    let mut r_max_3 = RectI32::of(i32::MAX - 3, i32::MAX - 3, i32::MAX, i32::MAX);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_3, 4), Some(()));
    assert_eq!(r_max_3, RectI32::of(i32::MAX - 3, i32::MAX - 3, i32::MAX, i32::MAX));
}

#[test]
fn out_of_bounds() {
    let mut r_min_row = RectI32::of(i32::MIN, i32::MIN + 2, i32::MIN + 2, i32::MIN + 4);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_row, 5), Some(()));
    assert_eq!(r_min_row, RectI32::of(i32::MAX, i32::MIN + 1, i32::MIN + 3, i32::MIN + 5));

    let mut r_min_y = RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 4, i32::MIN + 2);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_y, 5), Some(()));
    assert_eq!(r_min_y, RectI32::of(i32::MIN + 1, i32::MAX, i32::MIN + 5, i32::MIN + 3));

    let mut r_max_x = RectI32::of(i32::MAX - 2, i32::MAX - 4, i32::MAX, i32::MAX - 2);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_x, 5), Some(()));
    assert_eq!(r_max_x, RectI32::of(i32::MAX - 3, i32::MAX - 5, i32::MIN, i32::MAX - 1));

    let mut r_max_y = RectI32::of(i32::MAX - 4, i32::MAX - 2, i32::MAX - 2, i32::MAX);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_y, 5), Some(()));
    assert_eq!(r_max_y, RectI32::of(i32::MAX - 5, i32::MAX - 3, i32::MAX - 1, i32::MIN));
}

#[test]
fn small_rect_limits_out_of_bounds() {
    let mut r_min_row = RectI32::of(i32::MIN, i32::MIN + 2, i32::MIN + 2, i32::MIN + 4);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_row, u32::MAX), Some(()));
    assert_eq!(r_min_row, RectI32::of(2, 4, 0, 2));

    let mut r_min_y = RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 4, i32::MIN + 2);
    assert_eq!(try_wrapping_resize_assign(&mut r_min_y, u32::MAX), Some(()));
    assert_eq!(r_min_y, RectI32::of(4, 2, 2, 0));

    let mut r_max_x = RectI32::of(i32::MAX - 2, i32::MAX - 4, i32::MAX, i32::MAX - 2);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_x, u32::MAX), Some(()));
    assert_eq!(r_max_x, RectI32::of(-1, -3, -3, -5));

    let mut r_max_y = RectI32::of(i32::MAX - 4, i32::MAX - 2, i32::MAX - 2, i32::MAX);
    assert_eq!(try_wrapping_resize_assign(&mut r_max_y, u32::MAX), Some(()));
    assert_eq!(r_max_y, RectI32::of(-3, -1, -5, -3));
}

#[test]
fn big_rect_limits_out_of_bounds() {
    let mut r_odd_1 = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1);
    assert_eq!(try_wrapping_resize_assign(&mut r_odd_1, u32::MAX), Some(()));
    assert_eq!(r_odd_1, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1));

    let mut r_odd_1 = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX);
    assert_eq!(try_wrapping_resize_assign(&mut r_odd_1, u32::MAX), Some(()));
    assert_eq!(r_odd_1, RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX));

    let mut r_even = RectI32::largest();
    assert_eq!(try_wrapping_resize_assign(&mut r_even, u32::MAX), Some(()));
    assert_eq!(r_even, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1));
}
