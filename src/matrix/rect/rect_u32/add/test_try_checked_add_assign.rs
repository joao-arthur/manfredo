use super::try_checked_add_assign;
use crate::matrix::rect::{rect_i32::RectI32, rect_u32::RectU32};

#[test]
fn test_try_checked_add_assign() {
    let mut r = RectU32::of(0, 0, 12, 12);
    assert_eq!(try_checked_add_assign(&mut r, &RectI32::of(5, 4, 3, 2)), Some(()));
    assert_eq!(r, RectU32::of(5, 4, 15, 14));
    assert_eq!(try_checked_add_assign(&mut r, &RectI32::of(-4, -3, -2, -1)), Some(()));
    assert_eq!(r, RectU32::of(1, 1, 13, 13));
}

#[test]
fn try_checked_add_assign_to_bounds() {
    let mut r = RectU32::of(2, 5, u32::MAX - 2, u32::MAX - 5);
    assert_eq!(try_checked_add_assign(&mut r, &RectI32::of(-2, -5, 2, 5)), Some(()));
    assert_eq!(r, RectU32::largest());

    let mut min_r = RectU32::of(2, 5, u32::MAX, u32::MAX);
    assert_eq!(try_checked_add_assign(&mut min_r, &RectI32::of(-2, -5, 0, 0)), Some(()));
    assert_eq!(min_r, RectU32::largest());

    let mut max_r = RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5);
    assert_eq!(try_checked_add_assign(&mut max_r, &RectI32::of(0, 0, 2, 5)), Some(()));
    assert_eq!(max_r, RectU32::largest());
}

#[test]
fn try_checked_add_assign_out_of_bounds() {
    let mut r = RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 10);
    assert_eq!(try_checked_add_assign(&mut r, &RectI32::of(-20, 0, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI32::of(0, -20, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI32::of(0, 0, 20, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI32::of(0, 0, 0, 20)), None);
    assert_eq!(r, RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 10));
}

#[test]
fn try_checked_add_assign_edge_out_of_bounds() {
    let mut r = RectU32::largest();
    assert_eq!(try_checked_add_assign(&mut r, &RectI32::of(-1, 0, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI32::of(0, -1, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI32::of(0, 0, 1, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI32::of(0, 0, 0, 1)), None);
    assert_eq!(r, RectU32::largest());
}

#[test]
fn try_checked_add_assign_limits_out_of_bounds() {
    let mut r = RectU32::largest();
    assert_eq!(try_checked_add_assign(&mut r, &RectI32::of(i32::MIN, 0, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI32::of(0, i32::MIN, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI32::of(0, 0, i32::MAX, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI32::of(0, 0, 0, i32::MAX)), None);
    assert_eq!(r, RectU32::largest());
}
