use super::saturating_add_assign;
use crate::cartesian::rect::{rect_i32::Rect as RectI, rect_u32::RectU32};

const MAX: u32 = u32::MAX;

#[test]
fn test() {
    let mut r = RectU32::of(0, 0, 12, 10);
    saturating_add_assign(&mut r, &RectI::of(5, 4, 3, 2));
    assert_eq!(r, RectU32::of(5, 4, 15, 12));
    saturating_add_assign(&mut r, &RectI::of(-4, -3, -2, -1));
    assert_eq!(r, RectU32::of(1, 1, 13, 11));
}

#[test]
fn to_bounds() {
    let mut r = RectU32::of(2, 5, MAX - 2, MAX - 5);
    saturating_add_assign(&mut r, &RectI::of(-2, -5, 2, 5));
    assert_eq!(r, RectU32::largest());

    let mut r_min = RectU32::of(2, 5, MAX, MAX);
    saturating_add_assign(&mut r_min, &RectI::of(-2, -5, 0, 0));
    assert_eq!(r_min, RectU32::largest());

    let mut r_max = RectU32::of(0, 0, MAX - 2, MAX - 5);
    saturating_add_assign(&mut r_max, &RectI::of(0, 0, 2, 5));
    assert_eq!(r_max, RectU32::largest());
}

#[test]
fn out_of_bounds() {
    let mut r1 = RectU32::of(10, 10, MAX - 10, MAX - 10);
    saturating_add_assign(&mut r1, &RectI::of(-20, 0, 0, 0));
    assert_eq!(r1, RectU32::of(0, 10, MAX - 10, MAX - 10));

    let mut r2 = RectU32::of(10, 10, MAX - 10, MAX - 10);
    saturating_add_assign(&mut r2, &RectI::of(0, -20, 0, 0));
    assert_eq!(r2, RectU32::of(10, 0, MAX - 10, MAX - 10));

    let mut r3 = RectU32::of(10, 10, MAX - 10, MAX - 10);
    saturating_add_assign(&mut r3, &RectI::of(0, 0, 20, 0));
    assert_eq!(r3, RectU32::of(10, 10, MAX, MAX - 10));

    let mut r4 = RectU32::of(10, 10, MAX - 10, MAX - 10);
    saturating_add_assign(&mut r4, &RectI::of(0, 0, 0, 20));
    assert_eq!(r4, RectU32::of(10, 10, MAX - 10, MAX));
}

#[test]
fn edge_out_of_bounds() {
    let mut r = RectU32::largest();
    saturating_add_assign(&mut r, &RectI::of(-1, 0, 0, 0));
    assert_eq!(r, RectU32::largest());
    saturating_add_assign(&mut r, &RectI::of(0, -1, 0, 0));
    assert_eq!(r, RectU32::largest());
    saturating_add_assign(&mut r, &RectI::of(0, 0, 1, 0));
    assert_eq!(r, RectU32::largest());
    saturating_add_assign(&mut r, &RectI::of(0, 0, 0, 1));
    assert_eq!(r, RectU32::largest());
}

#[test]
fn limits_out_of_bounds() {
    let mut r = RectU32::largest();
    saturating_add_assign(&mut r, &RectI::of(i32::MIN, 0, 0, 0));
    assert_eq!(r, RectU32::largest());
    saturating_add_assign(&mut r, &RectI::of(0, i32::MIN, 0, 0));
    assert_eq!(r, RectU32::largest());
    saturating_add_assign(&mut r, &RectI::of(0, 0, i32::MAX, 0));
    assert_eq!(r, RectU32::largest());
    saturating_add_assign(&mut r, &RectI::of(0, 0, 0, i32::MAX));
    assert_eq!(r, RectU32::largest());
}
