use super::saturating_translate_assign;
use crate::matrix::{point::point_i8::PointI8, rect::rect_i8::RectI8};

#[test]
fn test() {
    let mut r = RectI8::of(5, 9, 13, 37);
    saturating_translate_assign(&mut r, &PointI8::of(-10, -20));
    assert_eq!(r, RectI8::of(-5, -11, 3, 17));
    saturating_translate_assign(&mut r, &PointI8::of(6, -19));
    assert_eq!(r, RectI8::of(1, -30, 9, -2));
}

#[test]
fn to_bounds() {
    let mut min_r = RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX);
    saturating_translate_assign(&mut min_r, &PointI8::of(-2, -5));
    assert_eq!(min_r, RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5));

    let mut max_r = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 5);
    saturating_translate_assign(&mut max_r, &PointI8::of(2, 5));
    assert_eq!(max_r, RectI8::of(i8::MIN + 2, i8::MIN + 5, i8::MAX, i8::MAX));
}

#[test]
fn out_of_bounds() {
    let mut r1 = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10);
    saturating_translate_assign(&mut r1, &PointI8::of(-20, 0));
    assert_eq!(r1, RectI8::of(i8::MIN, i8::MIN + 10, i8::MAX - 20, i8::MAX - 10));

    let mut r2 = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10);
    saturating_translate_assign(&mut r2, &PointI8::of(0, -20));
    assert_eq!(r2, RectI8::of(i8::MIN + 10, i8::MIN, i8::MAX - 10, i8::MAX - 20));

    let mut r3 = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10);
    saturating_translate_assign(&mut r3, &PointI8::of(20, 0));
    assert_eq!(r3, RectI8::of(i8::MIN + 20, i8::MIN + 10, i8::MAX, i8::MAX - 10));

    let mut r4 = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10);
    saturating_translate_assign(&mut r4, &PointI8::of(0, 20));
    assert_eq!(r4, RectI8::of(i8::MIN + 10, i8::MIN + 20, i8::MAX - 10, i8::MAX));
}

#[test]
fn limits_out_of_bounds() {
    let mut r = RectI8::largest();
    saturating_translate_assign(&mut r, &PointI8::of(i8::MIN, 0));
    assert_eq!(r, RectI8::largest());
    saturating_translate_assign(&mut r, &PointI8::of(0, i8::MIN));
    assert_eq!(r, RectI8::largest());
    saturating_translate_assign(&mut r, &PointI8::of(i8::MAX, 0));
    assert_eq!(r, RectI8::largest());
    saturating_translate_assign(&mut r, &PointI8::of(0, i8::MAX));
    assert_eq!(r, RectI8::largest());
}
