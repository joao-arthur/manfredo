use super::saturating_translate_assign;
use crate::matrix::{point::point_i32::PointI32, rect::rect_i32::RectI32};

#[test]
fn test() {
    let mut r = RectI32::of(5, 9, 13, 37);
    saturating_translate_assign(&mut r, &PointI32::of(-10, -20));
    assert_eq!(r, RectI32::of(-5, -11, 3, 17));
    saturating_translate_assign(&mut r, &PointI32::of(6, -19));
    assert_eq!(r, RectI32::of(1, -30, 9, -2));
}

#[test]
fn to_bounds() {
    let mut min_r = RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX);
    saturating_translate_assign(&mut min_r, &PointI32::of(-2, -5));
    assert_eq!(min_r, RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5));

    let mut max_r = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 5);
    saturating_translate_assign(&mut max_r, &PointI32::of(2, 5));
    assert_eq!(max_r, RectI32::of(i32::MIN + 2, i32::MIN + 5, i32::MAX, i32::MAX));
}

#[test]
fn out_of_bounds() {
    let mut r1 = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 10, i32::MAX - 10);
    saturating_translate_assign(&mut r1, &PointI32::of(-20, 0));
    assert_eq!(r1, RectI32::of(i32::MIN, i32::MIN + 10, i32::MAX - 20, i32::MAX - 10));

    let mut r2 = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 10, i32::MAX - 10);
    saturating_translate_assign(&mut r2, &PointI32::of(0, -20));
    assert_eq!(r2, RectI32::of(i32::MIN + 10, i32::MIN, i32::MAX - 10, i32::MAX - 20));

    let mut r3 = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 10, i32::MAX - 10);
    saturating_translate_assign(&mut r3, &PointI32::of(20, 0));
    assert_eq!(r3, RectI32::of(i32::MIN + 20, i32::MIN + 10, i32::MAX, i32::MAX - 10));

    let mut r4 = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 10, i32::MAX - 10);
    saturating_translate_assign(&mut r4, &PointI32::of(0, 20));
    assert_eq!(r4, RectI32::of(i32::MIN + 10, i32::MIN + 20, i32::MAX - 10, i32::MAX));
}

#[test]
fn limits_out_of_bounds() {
    let mut r = RectI32::largest();
    saturating_translate_assign(&mut r, &PointI32::of(i32::MIN, 0));
    assert_eq!(r, RectI32::largest());
    saturating_translate_assign(&mut r, &PointI32::of(0, i32::MIN));
    assert_eq!(r, RectI32::largest());
    saturating_translate_assign(&mut r, &PointI32::of(i32::MAX, 0));
    assert_eq!(r, RectI32::largest());
    saturating_translate_assign(&mut r, &PointI32::of(0, i32::MAX));
    assert_eq!(r, RectI32::largest());
}
