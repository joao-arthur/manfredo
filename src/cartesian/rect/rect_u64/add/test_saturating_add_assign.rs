use super::saturating_add_assign;
use crate::cartesian::rect::{rect_i64::RectI64, rect_u64::RectU64};

#[test]
fn test_saturating_add_assign() {
    let mut r = RectU64::of(0, 0, 12, 10);
    saturating_add_assign(&mut r, &RectI64::of(5, 4, 3, 2));
    assert_eq!(r, RectU64::of(5, 4, 15, 12));
    saturating_add_assign(&mut r, &RectI64::of(-4, -3, -2, -1));
    assert_eq!(r, RectU64::of(1, 1, 13, 11));
}

#[test]
fn saturating_add_assign_to_bounds() {
    let mut r = RectU64::of(2, 5, u64::MAX - 2, u64::MAX - 5);
    saturating_add_assign(&mut r, &RectI64::of(-2, -5, 2, 5));
    assert_eq!(r, RectU64::largest());

    let mut r_min = RectU64::of(2, 5, u64::MAX, u64::MAX);
    saturating_add_assign(&mut r_min, &RectI64::of(-2, -5, 0, 0));
    assert_eq!(r_min, RectU64::largest());

    let mut r_max = RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5);
    saturating_add_assign(&mut r_max, &RectI64::of(0, 0, 2, 5));
    assert_eq!(r_max, RectU64::largest());
}

#[test]
fn saturating_add_assign_out_of_bounds() {
    let mut r1 = RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10);
    saturating_add_assign(&mut r1, &RectI64::of(-20, 0, 0, 0));
    assert_eq!(r1, RectU64::of(0, 10, u64::MAX - 10, u64::MAX - 10));

    let mut r2 = RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10);
    saturating_add_assign(&mut r2, &RectI64::of(0, -20, 0, 0));
    assert_eq!(r2, RectU64::of(10, 0, u64::MAX - 10, u64::MAX - 10));

    let mut r3 = RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10);
    saturating_add_assign(&mut r3, &RectI64::of(0, 0, 20, 0));
    assert_eq!(r3, RectU64::of(10, 10, u64::MAX, u64::MAX - 10));

    let mut r4 = RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10);
    saturating_add_assign(&mut r4, &RectI64::of(0, 0, 0, 20));
    assert_eq!(r4, RectU64::of(10, 10, u64::MAX - 10, u64::MAX));
}

#[test]
fn saturating_add_assign_edge_out_of_bounds() {
    let mut r = RectU64::largest();
    saturating_add_assign(&mut r, &RectI64::of(-1, 0, 0, 0));
    assert_eq!(r, RectU64::largest());
    saturating_add_assign(&mut r, &RectI64::of(0, -1, 0, 0));
    assert_eq!(r, RectU64::largest());
    saturating_add_assign(&mut r, &RectI64::of(0, 0, 1, 0));
    assert_eq!(r, RectU64::largest());
    saturating_add_assign(&mut r, &RectI64::of(0, 0, 0, 1));
    assert_eq!(r, RectU64::largest());
}

#[test]
fn saturating_add_assign_limits_out_of_bounds() {
    let mut r = RectU64::largest();
    saturating_add_assign(&mut r, &RectI64::of(i64::MIN, 0, 0, 0));
    assert_eq!(r, RectU64::largest());
    saturating_add_assign(&mut r, &RectI64::of(0, i64::MIN, 0, 0));
    assert_eq!(r, RectU64::largest());
    saturating_add_assign(&mut r, &RectI64::of(0, 0, i64::MAX, 0));
    assert_eq!(r, RectU64::largest());
    saturating_add_assign(&mut r, &RectI64::of(0, 0, 0, i64::MAX));
    assert_eq!(r, RectU64::largest());
}
