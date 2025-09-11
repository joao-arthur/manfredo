use super::try_checked_add;
use crate::matrix::rect::{rect_i32::Rect as RectI, rect_u32::RectU32};

const MAX: u32 = u32::MAX;

#[test]
fn test() {
    assert_eq!(try_checked_add(&RectU32::of(0, 0, 12, 15), &RectI::of(5, 4, 3, 2)), Some(RectU32::of(5, 4, 15, 17)));
    assert_eq!(try_checked_add(&RectU32::of(5, 4, 15, 20), &RectI::of(-4, -3, -2, -1)), Some(RectU32::of(1, 1, 13, 19)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&RectU32::of(2, 5, MAX - 2, MAX - 5), &RectI::of(-2, -5, 2, 5)), Some(RectU32::largest()));
    assert_eq!(try_checked_add(&RectU32::of(2, 5, MAX, MAX), &RectI::of(-2, -5, 0, 0)), Some(RectU32::largest()));
    assert_eq!(try_checked_add(&RectU32::of(0, 0, MAX - 2, MAX - 5), &RectI::of(0, 0, 2, 5)), Some(RectU32::largest()));
}

#[test]
fn out_of_bounds() {
    let r = RectU32::of(10, 10, MAX - 10, MAX - 10);
    assert_eq!(try_checked_add(&r, &RectI::of(-20, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI::of(0, -20, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI::of(0, 0, 20, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI::of(0, 0, 0, 20)), None);
}

#[test]
fn edge_out_of_bounds() {
    let r = RectU32::largest();
    assert_eq!(try_checked_add(&r, &RectI::of(-1, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI::of(0, -1, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI::of(0, 0, 1, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI::of(0, 0, 0, 1)), None);
}

#[test]
fn limits_out_of_bounds() {
    let r = RectU32::largest();
    assert_eq!(try_checked_add(&r, &RectI::of(i32::MIN, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI::of(0, i32::MIN, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI::of(0, 0, i32::MAX, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI::of(0, 0, 0, i32::MAX)), None);
}
