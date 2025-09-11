use super::wrapping_add_assign;
use crate::cartesian::rect::{rect_i64::Rect, rect_u64::RectU64};

const MAX: u64 = u64::MAX;

#[test]
fn test() {
    let mut r = RectU64::of(0, 0, 12, 10);
    wrapping_add_assign(&mut r, &Rect::of(5, 4, 3, 2));
    assert_eq!(r, RectU64::of(5, 4, 15, 12));
    wrapping_add_assign(&mut r, &Rect::of(-4, -3, -2, -1));
    assert_eq!(r, RectU64::of(1, 1, 13, 11));
}

#[test]
fn to_bounds() {
    let mut r = RectU64::of(2, 5, MAX - 2, MAX - 5);
    wrapping_add_assign(&mut r, &Rect::of(-2, -5, 2, 5));
    assert_eq!(r, RectU64::largest());

    let mut r_min = RectU64::of(2, 5, MAX, MAX);
    wrapping_add_assign(&mut r_min, &Rect::of(-2, -5, 0, 0));
    assert_eq!(r_min, RectU64::largest());

    let mut r_max = RectU64::of(0, 0, MAX - 2, MAX - 5);
    wrapping_add_assign(&mut r_max, &Rect::of(0, 0, 2, 5));
    assert_eq!(r_max, RectU64::largest());
}

#[test]
fn out_of_bounds() {
    let mut r1 = RectU64::of(10, 10, MAX - 10, MAX - 10);
    wrapping_add_assign(&mut r1, &Rect::of(-20, 0, 0, 0));
    assert_eq!(r1, RectU64::of(MAX - 9, 10, MAX - 10, MAX - 10));

    let mut r2 = RectU64::of(10, 10, MAX - 10, MAX - 10);
    wrapping_add_assign(&mut r2, &Rect::of(0, -20, 0, 0));
    assert_eq!(r2, RectU64::of(10, MAX - 9, MAX - 10, MAX - 10));

    let mut r3 = RectU64::of(10, 10, MAX - 10, MAX - 10);
    wrapping_add_assign(&mut r3, &Rect::of(0, 0, 20, 0));
    assert_eq!(r3, RectU64::of(10, 10, 9, MAX - 10));

    let mut r4 = RectU64::of(10, 10, MAX - 10, MAX - 10);
    wrapping_add_assign(&mut r4, &Rect::of(0, 0, 0, 20));
    assert_eq!(r4, RectU64::of(10, 10, MAX - 10, 9));
}

#[test]
fn edge_out_of_bounds() {
    let mut r1 = RectU64::largest();
    wrapping_add_assign(&mut r1, &Rect::of(-1, 0, 0, 0));
    assert_eq!(r1, RectU64::of(MAX, 0, MAX, MAX));

    let mut r2 = RectU64::largest();
    wrapping_add_assign(&mut r2, &Rect::of(0, -1, 0, 0));
    assert_eq!(r2, RectU64::of(0, MAX, MAX, MAX));

    let mut r3 = RectU64::largest();
    wrapping_add_assign(&mut r3, &Rect::of(0, 0, 1, 0));
    assert_eq!(r3, RectU64::of(0, 0, 0, MAX));

    let mut r4 = RectU64::largest();
    wrapping_add_assign(&mut r4, &Rect::of(0, 0, 0, 1));
    assert_eq!(r4, RectU64::of(0, 0, MAX, 0));
}

#[test]
fn limits_out_of_bounds() {
    let mut r1 = RectU64::largest();
    wrapping_add_assign(&mut r1, &Rect::of(i64::MIN, 0, 0, 0));
    assert_eq!(r1, RectU64::of(MAX / 2 + 1, 0, MAX, MAX));

    let mut r2 = RectU64::largest();
    wrapping_add_assign(&mut r2, &Rect::of(0, i64::MIN, 0, 0));
    assert_eq!(r2, RectU64::of(0, MAX / 2 + 1, MAX, MAX));

    let mut r3 = RectU64::largest();
    wrapping_add_assign(&mut r3, &Rect::of(0, 0, i64::MAX, 0));
    assert_eq!(r3, RectU64::of(0, 0, MAX / 2 - 1, MAX));

    let mut r4 = RectU64::largest();
    wrapping_add_assign(&mut r4, &Rect::of(0, 0, 0, i64::MAX));
    assert_eq!(r4, RectU64::of(0, 0, MAX, MAX / 2 - 1));
}
