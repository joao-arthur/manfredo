use super::try_checked_add_assign;
use crate::cartesian::rect::{rect_i8::RectI8, rect_u8::RectU8};

#[test]
fn test() {
    let mut r = RectU8::of(0, 0, 12, 12);
    assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(5, 4, 3, 2)), Some(()));
    assert_eq!(r, RectU8::of(5, 4, 15, 14));
    assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(-4, -3, -2, -1)), Some(()));
    assert_eq!(r, RectU8::of(1, 1, 13, 13));
}

#[test]
fn to_bounds() {
    let mut r = RectU8::of(2, 5, u8::MAX - 2, u8::MAX - 5);
    assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(-2, -5, 2, 5)), Some(()));
    assert_eq!(r, RectU8::largest());

    let mut min_r = RectU8::of(2, 5, u8::MAX, u8::MAX);
    assert_eq!(try_checked_add_assign(&mut min_r, &RectI8::of(-2, -5, 0, 0)), Some(()));
    assert_eq!(min_r, RectU8::largest());

    let mut max_r = RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5);
    assert_eq!(try_checked_add_assign(&mut max_r, &RectI8::of(0, 0, 2, 5)), Some(()));
    assert_eq!(max_r, RectU8::largest());
}

#[test]
fn out_of_bounds() {
    let mut r = RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10);
    assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(-20, 0, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(0, -20, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(0, 0, 20, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(0, 0, 0, 20)), None);
    assert_eq!(r, RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10));
}

#[test]
fn edge_out_of_bounds() {
    let mut r = RectU8::largest();
    assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(-1, 0, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(0, -1, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(0, 0, 1, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(0, 0, 0, 1)), None);
    assert_eq!(r, RectU8::largest());
}

#[test]
fn limits_out_of_bounds() {
    let mut r = RectU8::largest();
    assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(i8::MIN, 0, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(0, i8::MIN, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(0, 0, i8::MAX, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI8::of(0, 0, 0, i8::MAX)), None);
    assert_eq!(r, RectU8::largest());
}
