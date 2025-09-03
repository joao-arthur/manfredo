use super::saturating_translate_assign;
use crate::matrix::{point::point_i8::PointI8, rect::rect_u8::RectU8};

#[test]
fn test_saturating_translate_assign() {
    let mut r = RectU8::of(0, 0, 12, 15);
    saturating_translate_assign(&mut r, &PointI8::of(5, 4));
    assert_eq!(r, RectU8::of(5, 4, 17, 19));
    saturating_translate_assign(&mut r, &PointI8::of(-4, -2));
    assert_eq!(r, RectU8::of(1, 2, 13, 17));
}

#[test]
fn saturating_translate_assign_to_bounds() {
    let mut min_r = RectU8::of(2, 5, u8::MAX, u8::MAX);
    saturating_translate_assign(&mut min_r, &PointI8::of(-2, -5));
    assert_eq!(min_r, RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5));

    let mut max_r = RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5);
    saturating_translate_assign(&mut max_r, &PointI8::of(2, 5));
    assert_eq!(max_r, RectU8::of(2, 5, u8::MAX, u8::MAX));
}

#[test]
fn saturating_translate_assign_out_of_bounds() {
    let mut r1 = RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10);
    saturating_translate_assign(&mut r1, &PointI8::of(-20, 0));
    assert_eq!(r1, RectU8::of(0, 10, u8::MAX - 20, u8::MAX - 10));

    let mut r2 = RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10);
    saturating_translate_assign(&mut r2, &PointI8::of(0, -20));
    assert_eq!(r2, RectU8::of(10, 0, u8::MAX - 10, u8::MAX - 20));

    let mut r3 = RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10);
    saturating_translate_assign(&mut r3, &PointI8::of(20, 0));
    assert_eq!(r3, RectU8::of(20, 10, u8::MAX, u8::MAX - 10));

    let mut r4 = RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 10);
    saturating_translate_assign(&mut r4, &PointI8::of(0, 20));
    assert_eq!(r4, RectU8::of(10, 20, u8::MAX - 10, u8::MAX));
}

#[test]
fn saturating_translate_assign_limits_out_of_bounds() {
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
