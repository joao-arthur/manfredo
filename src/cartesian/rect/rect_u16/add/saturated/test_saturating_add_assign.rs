use super::saturating_add_assign;
use crate::cartesian::rect::{rect_i16::RectI16, rect_u16::RectU16};

#[test]
fn test_saturating_add_assign() {
    let mut r = RectU16::of(0, 0, 12, 10);
    saturating_add_assign(&mut r, &RectI16::of(5, 4, 3, 2));
    assert_eq!(r, RectU16::of(5, 4, 15, 12));
    saturating_add_assign(&mut r, &RectI16::of(-4, -3, -2, -1));
    assert_eq!(r, RectU16::of(1, 1, 13, 11));
}
#[test]
fn saturating_add_assign_to_bounds() {
    let mut r = RectU16::of(2, 5, u16::MAX - 2, u16::MAX - 5);
    saturating_add_assign(&mut r, &RectI16::of(-2, -5, 2, 5));
    assert_eq!(r, RectU16::largest());

    let mut r_min = RectU16::of(2, 5, u16::MAX, u16::MAX);
    saturating_add_assign(&mut r_min, &RectI16::of(-2, -5, 0, 0));
    assert_eq!(r_min, RectU16::largest());

    let mut r_max = RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5);
    saturating_add_assign(&mut r_max, &RectI16::of(0, 0, 2, 5));
    assert_eq!(r_max, RectU16::largest());
}

#[test]
fn saturating_add_assign_edge_out_of_bounds() {
    let mut r = RectU16::largest();
    saturating_add_assign(&mut r, &RectI16::of(-1, 0, 0, 0));
    assert_eq!(r, RectU16::largest());
    saturating_add_assign(&mut r, &RectI16::of(0, -1, 0, 0));
    assert_eq!(r, RectU16::largest());
    saturating_add_assign(&mut r, &RectI16::of(0, 0, 1, 0));
    assert_eq!(r, RectU16::largest());
    saturating_add_assign(&mut r, &RectI16::of(0, 0, 0, 1));
    assert_eq!(r, RectU16::largest());
}

#[test]
fn saturating_add_assign_out_of_bounds() {
    let mut r1 = RectU16::of(10, 10, u16::MAX - 10, u16::MAX - 10);
    saturating_add_assign(&mut r1, &RectI16::of(-20, 0, 0, 0));
    assert_eq!(r1, RectU16::of(0, 10, u16::MAX - 10, u16::MAX - 10));

    let mut r2 = RectU16::of(10, 10, u16::MAX - 10, u16::MAX - 10);
    saturating_add_assign(&mut r2, &RectI16::of(0, -20, 0, 0));
    assert_eq!(r2, RectU16::of(10, 0, u16::MAX - 10, u16::MAX - 10));

    let mut r3 = RectU16::of(10, 10, u16::MAX - 10, u16::MAX - 10);
    saturating_add_assign(&mut r3, &RectI16::of(0, 0, 20, 0));
    assert_eq!(r3, RectU16::of(10, 10, u16::MAX, u16::MAX - 10));

    let mut r4 = RectU16::of(10, 10, u16::MAX - 10, u16::MAX - 10);
    saturating_add_assign(&mut r4, &RectI16::of(0, 0, 0, 20));
    assert_eq!(r4, RectU16::of(10, 10, u16::MAX - 10, u16::MAX));
}

#[test]
fn saturating_add_assign_limits_out_of_bounds() {
    let mut r = RectU16::largest();
    saturating_add_assign(&mut r, &RectI16::of(i16::MIN, 0, 0, 0));
    assert_eq!(r, RectU16::largest());
    saturating_add_assign(&mut r, &RectI16::of(0, i16::MIN, 0, 0));
    assert_eq!(r, RectU16::largest());
    saturating_add_assign(&mut r, &RectI16::of(0, 0, i16::MAX, 0));
    assert_eq!(r, RectU16::largest());
    saturating_add_assign(&mut r, &RectI16::of(0, 0, 0, i16::MAX));
    assert_eq!(r, RectU16::largest());
}
