use super::wrapping_add_assign;
use crate::matrix::rect::{rect_i16::Rect, rect_u16::RectU16};

const MAX: u16 = u16::MAX;

#[test]
fn test() {
    let mut r = RectU16::of(0, 0, 12, 10);
    wrapping_add_assign(&mut r, &Rect::of(5, 4, 3, 2));
    assert_eq!(r, RectU16::of(5, 4, 15, 12));
    wrapping_add_assign(&mut r, &Rect::of(-4, -3, -2, -1));
    assert_eq!(r, RectU16::of(1, 1, 13, 11));
}

#[test]
fn to_bounds() {
    let mut r = RectU16::of(2, 5, MAX - 2, MAX - 5);
    wrapping_add_assign(&mut r, &Rect::of(-2, -5, 2, 5));
    assert_eq!(r, RectU16::largest());

    let mut r_min = RectU16::of(2, 5, MAX, MAX);
    wrapping_add_assign(&mut r_min, &Rect::of(-2, -5, 0, 0));
    assert_eq!(r_min, RectU16::largest());

    let mut r_max = RectU16::of(0, 0, MAX - 2, MAX - 5);
    wrapping_add_assign(&mut r_max, &Rect::of(0, 0, 2, 5));
    assert_eq!(r_max, RectU16::largest());
}

#[test]
fn out_of_bounds() {
    let mut r1 = RectU16::of(10, 10, MAX - 10, MAX - 10);
    wrapping_add_assign(&mut r1, &Rect::of(-20, 0, 0, 0));
    assert_eq!(r1, RectU16::of(MAX - 9, 10, MAX - 10, MAX - 10));

    let mut r2 = RectU16::of(10, 10, MAX - 10, MAX - 10);
    wrapping_add_assign(&mut r2, &Rect::of(0, -20, 0, 0));
    assert_eq!(r2, RectU16::of(10, MAX - 9, MAX - 10, MAX - 10));

    let mut r3 = RectU16::of(10, 10, MAX - 10, MAX - 10);
    wrapping_add_assign(&mut r3, &Rect::of(0, 0, 20, 0));
    assert_eq!(r3, RectU16::of(10, 10, 9, MAX - 10));

    let mut r4 = RectU16::of(10, 10, MAX - 10, MAX - 10);
    wrapping_add_assign(&mut r4, &Rect::of(0, 0, 0, 20));
    assert_eq!(r4, RectU16::of(10, 10, MAX - 10, 9));
}

#[test]
fn edge_out_of_bounds() {
    let mut r1 = RectU16::largest();
    wrapping_add_assign(&mut r1, &Rect::of(-1, 0, 0, 0));
    assert_eq!(r1, RectU16::of(MAX, 0, MAX, MAX));

    let mut r2 = RectU16::largest();
    wrapping_add_assign(&mut r2, &Rect::of(0, -1, 0, 0));
    assert_eq!(r2, RectU16::of(0, MAX, MAX, MAX));

    let mut r3 = RectU16::largest();
    wrapping_add_assign(&mut r3, &Rect::of(0, 0, 1, 0));
    assert_eq!(r3, RectU16::of(0, 0, 0, MAX));

    let mut r4 = RectU16::largest();
    wrapping_add_assign(&mut r4, &Rect::of(0, 0, 0, 1));
    assert_eq!(r4, RectU16::of(0, 0, MAX, 0));
}

#[test]
fn limits_out_of_bounds() {
    let mut r1 = RectU16::largest();
    wrapping_add_assign(&mut r1, &Rect::of(i16::MIN, 0, 0, 0));
    assert_eq!(r1, RectU16::of(MAX / 2 + 1, 0, MAX, MAX));

    let mut r2 = RectU16::largest();
    wrapping_add_assign(&mut r2, &Rect::of(0, i16::MIN, 0, 0));
    assert_eq!(r2, RectU16::of(0, MAX / 2 + 1, MAX, MAX));

    let mut r3 = RectU16::largest();
    wrapping_add_assign(&mut r3, &Rect::of(0, 0, i16::MAX, 0));
    assert_eq!(r3, RectU16::of(0, 0, MAX / 2 - 1, MAX));

    let mut r4 = RectU16::largest();
    wrapping_add_assign(&mut r4, &Rect::of(0, 0, 0, i16::MAX));
    assert_eq!(r4, RectU16::of(0, 0, MAX, MAX / 2 - 1));
}
