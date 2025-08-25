use super::wrapping_add_assign;
use crate::cartesian::rect::{rect_i16::RectI16, rect_u16::RectU16};

#[test]
fn test_wrapping_add_assign() {
    let mut r = RectU16::of(0, 0, 12, 10);
    wrapping_add_assign(&mut r, &RectI16::of(5, 4, 3, 2));
    assert_eq!(r, RectU16::of(5, 4, 15, 12));
    wrapping_add_assign(&mut r, &RectI16::of(-4, -3, -2, -1));
    assert_eq!(r, RectU16::of(1, 1, 13, 11));
}

#[test]
fn wrapping_add_assign_small_rect_to_bounds() {
    let mut min_r = RectU16::of(2, 5, 12, 15);
    wrapping_add_assign(&mut min_r, &RectI16::of(-2, -5, 9, 7));
    assert_eq!(min_r, RectU16::of(0, 0, 21, 22));

    let mut max_r = RectU16::of(u16::MAX - 12, u16::MAX - 15, u16::MAX - 2, u16::MAX - 5);
    wrapping_add_assign(&mut max_r, &RectI16::of(-9, -7, 2, 5));
    assert_eq!(max_r, RectU16::of(u16::MAX - 21, u16::MAX - 22, u16::MAX, u16::MAX));
}

#[test]
fn wrapping_add_assign_big_rect_to_bounds() {
    let mut r = RectU16::of(2, 5, u16::MAX - 2, u16::MAX - 5);
    wrapping_add_assign(&mut r, &RectI16::of(-2, -5, 2, 5));
    assert_eq!(r, RectU16::largest());

    let mut min_r = RectU16::of(2, 5, u16::MAX, u16::MAX);
    wrapping_add_assign(&mut min_r, &RectI16::of(-2, -5, 0, 0));
    assert_eq!(min_r, RectU16::largest());

    let mut max_r = RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5);
    wrapping_add_assign(&mut max_r, &RectI16::of(0, 0, 2, 5));
    assert_eq!(max_r, RectU16::largest());
}

#[test]
fn wrapping_add_assign_small_rect_out_of_bounds() {
    let mut r_min = RectU16::of(10, 5, 20, 30);
    wrapping_add_assign(&mut r_min, &RectI16::of(-20, -20, 0, 0));
    assert_eq!(r_min, RectU16::of(u16::MAX - 9, u16::MAX - 14, 20, 30));

    let mut r_max = RectU16::of(u16::MAX - 20, u16::MAX - 30, u16::MAX - 5, u16::MAX - 10);
    wrapping_add_assign(&mut r_max, &RectI16::of(0, 0, 20, 20));
    assert_eq!(r_max, RectU16::of(u16::MAX - 20, u16::MAX - 30, 14, 9));
}

#[test]
fn wrapping_add_assign_big_rect_out_of_bounds() {
    let mut r_min = RectU16::of(10, 5, u16::MAX, u16::MAX);
    wrapping_add_assign(&mut r_min, &RectI16::of(-20, -20, 0, 0));
    assert_eq!(r_min, RectU16::of(u16::MAX - 9, u16::MAX - 14, u16::MAX, u16::MAX));

    let mut r_max = RectU16::of(0, 0, u16::MAX - 5, u16::MAX - 10);
    wrapping_add_assign(&mut r_max, &RectI16::of(0, 0, 20, 20));
    assert_eq!(r_max, RectU16::of(0, 0, 14, 9));
}

#[test]
fn wrapping_add_assign_small_rect_limits_out_of_bounds() {
    let mut r_min = RectU16::of(1, 1, 10, 10);
    wrapping_add_assign(&mut r_min, &RectI16::min());
    assert_eq!(r_min, RectU16::of((i16::MAX as u16) + 2, (i16::MAX as u16) + 2, (i16::MAX as u16) + 11, (i16::MAX as u16) + 11));

    let mut r_max = RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX - 1, u16::MAX - 1);
    wrapping_add_assign(&mut r_max, &RectI16::max());
    assert_eq!(r_max, RectU16::of((i16::MAX as u16) - 11, (i16::MAX as u16) - 11, (i16::MAX as u16) - 2, (i16::MAX as u16) - 2));
}

#[test]
fn wrapping_add_assign_big_rect_limits_out_of_bounds() {
    let mut r1 = RectU16::largest();
    wrapping_add_assign(&mut r1, &RectI16::min());
    assert_eq!(r1, RectU16::of((i16::MAX as u16) + 1, (i16::MAX as u16) + 1, i16::MAX as u16, i16::MAX as u16));

    let mut r2 = RectU16::largest();
    wrapping_add_assign(&mut r2, &RectI16::max());
    assert_eq!(r2, RectU16::of(i16::MAX as u16, i16::MAX as u16, (i16::MAX as u16) - 1, (i16::MAX as u16) - 1));

    let mut r_min = RectU16::of(1, 1, u16::MAX, u16::MAX);
    wrapping_add_assign(&mut r_min, &RectI16::min());
    assert_eq!(r_min, RectU16::of((i16::MAX as u16) + 2, (i16::MAX as u16) + 2, i16::MAX as u16, i16::MAX as u16));

    let mut r_max = RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1);
    wrapping_add_assign(&mut r_max, &RectI16::max());
    assert_eq!(r_max, RectU16::of(i16::MAX as u16, i16::MAX as u16, (i16::MAX as u16) - 2, (i16::MAX as u16) - 2));
}
