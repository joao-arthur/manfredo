
use super::try_checked_add;
use crate::cartesian::rect::{rect_i8::RectI8, rect_u8::RectU8};

#[test]
fn test_try_checked_add() {
    assert_eq!(try_checked_add(&RectU8::of(0, 0, 12, 15), &RectI8::of(5, 4, 3, 2)), Some(RectU8::of(5, 4, 15, 17)));
    assert_eq!(try_checked_add(&RectU8::of(5, 4, 15, 20), &RectI8::of(-4, -3, -2, -1)), Some(RectU8::of(1, 1, 13, 19)));
}

#[test]
fn try_checked_add_to_bounds() {
    assert_eq!(try_checked_add(&RectU8::of(2, 5, u8::MAX - 2, u8::MAX - 5), &RectI8::of(-2, -5, 2, 5)), Some(RectU8::largest()));
    assert_eq!(try_checked_add(&RectU8::of(2, 5, u8::MAX, u8::MAX), &RectI8::of(-2, -5, 0, 0)), Some(RectU8::largest()));
    assert_eq!(try_checked_add(&RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5), &RectI8::of(0, 0, 2, 5)), Some(RectU8::largest()));
}

#[test]
fn try_checked_add_edge_out_of_bounds() {
    let r = RectU8::largest();
    assert_eq!(try_checked_add(&r, &RectI8::of(-1, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI8::of(0, -1, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI8::of(0, 0, 1, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI8::of(0, 0, 0, 1)), None);
}

#[test]
fn try_checked_add_out_of_bounds() {
    let r = RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10);
    assert_eq!(try_checked_add(&r, &RectI8::of(-20, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI8::of(0, -20, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI8::of(0, 0, 20, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI8::of(0, 0, 0, 20)), None);
}

#[test]
fn try_checked_add_limits_out_of_bounds() {
    let r = RectU8::largest();
    assert_eq!(try_checked_add(&r, &RectI8::of(i8::MIN, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI8::of(0, i8::MIN, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI8::of(0, 0, i8::MAX, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI8::of(0, 0, 0, i8::MAX)), None);
}
