
use super::try_checked_add;
use crate::cartesian::rect::{rect_i32::RectI32, rect_u32::RectU32};

#[test]
fn test_try_checked_add() {
    assert_eq!(try_checked_add(&RectU32::of(0, 0, 12, 15), &RectI32::of(5, 4, 3, 2)), Some(RectU32::of(5, 4, 15, 17)));
    assert_eq!(try_checked_add(&RectU32::of(5, 4, 15, 20), &RectI32::of(-4, -3, -2, -1)), Some(RectU32::of(1, 1, 13, 19)));
}

#[test]
fn try_checked_add_to_bounds() {
    assert_eq!(try_checked_add(&RectU32::of(2, 5, u32::MAX - 2, u32::MAX - 5), &RectI32::of(-2, -5, 2, 5)), Some(RectU32::largest()));
    assert_eq!(try_checked_add(&RectU32::of(2, 5, u32::MAX, u32::MAX), &RectI32::of(-2, -5, 0, 0)), Some(RectU32::largest()));
    assert_eq!(try_checked_add(&RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5), &RectI32::of(0, 0, 2, 5)), Some(RectU32::largest()));
}

#[test]
fn try_checked_add_edge_out_of_bounds() {
    let r = RectU32::largest();
    assert_eq!(try_checked_add(&r, &RectI32::of(-1, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI32::of(0, -1, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI32::of(0, 0, 1, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI32::of(0, 0, 0, 1)), None);
}

#[test]
fn try_checked_add_out_of_bounds() {
    let r = RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 10);
    assert_eq!(try_checked_add(&r, &RectI32::of(-20, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI32::of(0, -20, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI32::of(0, 0, 20, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI32::of(0, 0, 0, 20)), None);
}

#[test]
fn try_checked_add_limits_out_of_bounds() {
    let r = RectU32::largest();
    assert_eq!(try_checked_add(&r, &RectI32::of(i32::MIN, 0, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI32::of(0, i32::MIN, 0, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI32::of(0, 0, i32::MAX, 0)), None);
    assert_eq!(try_checked_add(&r, &RectI32::of(0, 0, 0, i32::MAX)), None);
}
