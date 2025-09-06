use super::try_checked_add_assign;
use crate::cartesian::rect::{rect_i64::RectI64, rect_u64::RectU64};

#[test]
fn test() {
    let mut r = RectU64::of(0, 0, 12, 10);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(5, 4, 3, 2)), Some(()));
    assert_eq!(r, RectU64::of(5, 4, 15, 12));
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(-4, -3, -2, -1)), Some(()));
    assert_eq!(r, RectU64::of(1, 1, 13, 11));
}

#[test]
fn to_bounds() {
    let mut r = RectU64::of(2, 5, u64::MAX - 2, u64::MAX - 5);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(-2, -5, 2, 5)), Some(()));
    assert_eq!(r, RectU64::largest());

    let mut r_min = RectU64::of(2, 5, u64::MAX, u64::MAX);
    assert_eq!(try_checked_add_assign(&mut r_min, &RectI64::of(-2, -5, 0, 0)), Some(()));
    assert_eq!(r_min, RectU64::largest());

    let mut r_max = RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5);
    assert_eq!(try_checked_add_assign(&mut r_max, &RectI64::of(0, 0, 2, 5)), Some(()));
    assert_eq!(r_max, RectU64::largest());
}

#[test]
fn out_of_bounds() {
    let mut r = RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(-20, 0, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(0, -20, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(0, 0, 20, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(0, 0, 0, 20)), None);
    assert_eq!(r, RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10));
}

#[test]
fn edge_out_of_bounds() {
    let mut r = RectU64::largest();
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(-1, 0, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(0, -1, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(0, 0, 1, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(0, 0, 0, 1)), None);
    assert_eq!(r, RectU64::largest());
}

#[test]
fn limits_out_of_bounds() {
    let mut r = RectU64::largest();
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(i64::MIN, 0, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(0, i64::MIN, 0, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(0, 0, i64::MAX, 0)), None);
    assert_eq!(try_checked_add_assign(&mut r, &RectI64::of(0, 0, 0, i64::MAX)), None);
    assert_eq!(r, RectU64::largest());
}
