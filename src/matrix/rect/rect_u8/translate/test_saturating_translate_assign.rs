use super::saturating_translate_assign;
use crate::matrix::{point::point_i8::PointI8, rect::rect_u8::RectU8};

const MAX: u8 = u8::MAX;

#[test]
fn test() {
    let mut r = RectU8::of(0, 0, 12, 15);
    saturating_translate_assign(&mut r, &PointI8::of(5, 4));
    assert_eq!(r, RectU8::of(5, 4, 17, 19));
    saturating_translate_assign(&mut r, &PointI8::of(-4, -2));
    assert_eq!(r, RectU8::of(1, 2, 13, 17));
}

#[test]
fn to_bounds() {
    let mut r_min = RectU8::of(2, 5, MAX, MAX);
    saturating_translate_assign(&mut r_min, &PointI8::of(-2, -5));
    assert_eq!(r_min, RectU8::of(0, 0, MAX - 2, MAX - 5));

    let mut r_max = RectU8::of(0, 0, MAX - 2, MAX - 5);
    saturating_translate_assign(&mut r_max, &PointI8::of(2, 5));
    assert_eq!(r_max, RectU8::of(2, 5, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r1 = RectU8::of(10, 10, MAX - 10, MAX - 10);
    saturating_translate_assign(&mut r1, &PointI8::of(-20, 0));
    assert_eq!(r1, RectU8::of(0, 10, MAX - 20, MAX - 10));

    let mut r2 = RectU8::of(10, 10, MAX - 10, MAX - 10);
    saturating_translate_assign(&mut r2, &PointI8::of(0, -20));
    assert_eq!(r2, RectU8::of(10, 0, MAX - 10, MAX - 20));

    let mut r3 = RectU8::of(10, 10, MAX - 10, MAX - 10);
    saturating_translate_assign(&mut r3, &PointI8::of(20, 0));
    assert_eq!(r3, RectU8::of(20, 10, MAX, MAX - 10));

    let mut r4 = RectU8::of(10, 10, MAX - 10, MAX - 10);
    saturating_translate_assign(&mut r4, &PointI8::of(0, 20));
    assert_eq!(r4, RectU8::of(10, 20, MAX - 10, MAX));
}

#[test]
fn limits_out_of_bounds() {
    let mut r = RectU8::largest();
    saturating_translate_assign(&mut r, &PointI8::of(i8::MIN, 0));
    assert_eq!(r, RectU8::largest());
    saturating_translate_assign(&mut r, &PointI8::of(0, i8::MIN));
    assert_eq!(r, RectU8::largest());
    saturating_translate_assign(&mut r, &PointI8::of(i8::MAX, 0));
    assert_eq!(r, RectU8::largest());
    saturating_translate_assign(&mut r, &PointI8::of(0, i8::MAX));
    assert_eq!(r, RectU8::largest());
}
