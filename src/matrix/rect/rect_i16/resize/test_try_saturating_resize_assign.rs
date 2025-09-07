use super::try_saturating_resize_assign;
use crate::matrix::rect::rect_i16::RectI16;

const MIN: i16 = i16::MIN;
const MAX: i16 = i16::MAX;

#[test]
fn odd() {
    let mut r = RectI16::of(-5, -5, 5, 5);
    assert_eq!(try_saturating_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, RectI16::of(-4, -4, 4, 4));
    assert_eq!(try_saturating_resize_assign(&mut r, 7), Some(()));
    assert_eq!(r, RectI16::of(-3, -3, 3, 3));
    assert_eq!(try_saturating_resize_assign(&mut r, 5), Some(()));
    assert_eq!(r, RectI16::of(-2, -2, 2, 2));
    assert_eq!(try_saturating_resize_assign(&mut r, 3), Some(()));
    assert_eq!(r, RectI16::of(-1, -1, 1, 1));
    assert_eq!(try_saturating_resize_assign(&mut r, 9), Some(()));
    assert_eq!(r, RectI16::of(-4, -4, 4, 4));
}

#[test]
fn even() {
    let mut r = RectI16::of(-5, -5, 4, 4);
    assert_eq!(try_saturating_resize_assign(&mut r, 10), Some(()));
    assert_eq!(r, RectI16::of(-5, -5, 4, 4));
    assert_eq!(try_saturating_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, RectI16::of(-4, -4, 3, 3));
    assert_eq!(try_saturating_resize_assign(&mut r, 6), Some(()));
    assert_eq!(r, RectI16::of(-3, -3, 2, 2));
    assert_eq!(try_saturating_resize_assign(&mut r, 4), Some(()));
    assert_eq!(r, RectI16::of(-2, -2, 1, 1));
    assert_eq!(try_saturating_resize_assign(&mut r, 8), Some(()));
    assert_eq!(r, RectI16::of(-4, -4, 3, 3));
}

#[test]
fn small_size() {
    let mut r = RectI16::of(10, 10, 20, 20);
    assert_eq!(try_saturating_resize_assign(&mut r, 0), None);
    assert_eq!(try_saturating_resize_assign(&mut r, 1), None);
    assert_eq!(try_saturating_resize_assign(&mut r, 2), None);
    assert_eq!(r, RectI16::of(10, 10, 20, 20));
}

#[test]
fn same_size() {
    let mut r_min_2 = RectI16::of(MIN, MIN, MIN + 2, MIN + 2);
    assert_eq!(try_saturating_resize_assign(&mut r_min_2, 3), Some(()));
    assert_eq!(r_min_2, RectI16::of(MIN, MIN, MIN + 2, MIN + 2));

    let mut r_min_3 = RectI16::of(MIN, MIN, MIN + 3, MIN + 3);
    assert_eq!(try_saturating_resize_assign(&mut r_min_3, 4), Some(()));
    assert_eq!(r_min_3, RectI16::of(MIN, MIN, MIN + 3, MIN + 3));

    let mut r_max_2 = RectI16::of(MAX - 2, MAX - 2, MAX, MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max_2, 3), Some(()));
    assert_eq!(r_max_2, RectI16::of(MAX - 2, MAX - 2, MAX, MAX));

    let mut r_max_3 = RectI16::of(MAX - 3, MAX - 3, MAX, MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max_3, 4), Some(()));
    assert_eq!(r_max_3, RectI16::of(MAX - 3, MAX - 3, MAX, MAX));
}

#[test]
fn bounds() {
    let mut r_min = RectI16::of(MIN, MIN, MIN + 2, MIN + 2);
    assert_eq!(try_saturating_resize_assign(&mut r_min, 11), Some(()));
    assert_eq!(r_min, RectI16::of(MIN, MIN, MIN + 10, MIN + 10));

    let mut r_max = RectI16::of(MAX - 2, MAX - 2, MAX, MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max, 11), Some(()));
    assert_eq!(r_max, RectI16::of(MAX - 10, MAX - 10, MAX, MAX));
}

#[test]
fn small_rect_limits() {
    let mut r_min = RectI16::of(MIN, MIN, MIN + 2, MIN + 2);
    assert_eq!(try_saturating_resize_assign(&mut r_min, u16::MAX), Some(()));
    assert_eq!(r_min, RectI16::of(MIN, MIN, MAX - 1, MAX - 1));

    let mut r_max = RectI16::of(MAX - 2, MAX - 2, MAX, MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_max, u16::MAX), Some(()));
    assert_eq!(r_max, RectI16::of(MIN + 1, MIN + 1, MAX, MAX));
}

#[test]
fn big_rect_limits() {
    let mut r_odd_1 = RectI16::of(MIN, MIN, MAX - 1, MAX - 1);
    assert_eq!(try_saturating_resize_assign(&mut r_odd_1, u16::MAX), Some(()));
    assert_eq!(r_odd_1, RectI16::of(MIN, MIN, MAX - 1, MAX - 1));

    let mut r_odd_1 = RectI16::of(MIN + 1, MIN + 1, MAX, MAX);
    assert_eq!(try_saturating_resize_assign(&mut r_odd_1, u16::MAX), Some(()));
    assert_eq!(r_odd_1, RectI16::of(MIN + 1, MIN + 1, MAX, MAX));

    let mut r_even = RectI16::largest();
    assert_eq!(try_saturating_resize_assign(&mut r_even, u16::MAX), Some(()));
    assert_eq!(r_even, RectI16::of(MIN, MIN, MAX - 1, MAX - 1));
}
