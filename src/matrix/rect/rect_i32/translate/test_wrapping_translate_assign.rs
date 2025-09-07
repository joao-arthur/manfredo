use super::wrapping_translate_assign;
use crate::matrix::{point::point_i32::PointI32, rect::rect_i32::RectI32};

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

#[test]
fn test() {
    let mut r = RectI32::of(5, 9, 13, 37);
    wrapping_translate_assign(&mut r, &PointI32::of(-10, -20));
    assert_eq!(r, RectI32::of(-5, -11, 3, 17));
    wrapping_translate_assign(&mut r, &PointI32::of(6, -19));
    assert_eq!(r, RectI32::of(1, -30, 9, -2));
}

#[test]
fn to_bounds() {
    let mut r_min = RectI32::of(MIN + 2, MIN + 5, MAX, MAX);
    wrapping_translate_assign(&mut r_min, &PointI32::of(-2, -5));
    assert_eq!(r_min, RectI32::of(MIN, MIN, MAX - 2, MAX - 5));

    let mut r_max = RectI32::of(MIN, MIN, MAX - 2, MAX - 5);
    wrapping_translate_assign(&mut r_max, &PointI32::of(2, 5));
    assert_eq!(r_max, RectI32::of(MIN + 2, MIN + 5, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r1 = RectI32::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    wrapping_translate_assign(&mut r1, &PointI32::of(-20, 0));
    assert_eq!(r1, RectI32::of(MAX - 9, MIN + 10, MAX - 30, MAX - 10));

    let mut r2 = RectI32::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    wrapping_translate_assign(&mut r2, &PointI32::of(0, -20));
    assert_eq!(r2, RectI32::of(MIN + 10, MAX - 9, MAX - 10, MAX - 30));

    let mut r3 = RectI32::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    wrapping_translate_assign(&mut r3, &PointI32::of(20, 0));
    assert_eq!(r3, RectI32::of(MIN + 30, MIN + 10, MIN + 9, MAX - 10));

    let mut r4 = RectI32::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    wrapping_translate_assign(&mut r4, &PointI32::of(0, 20));
    assert_eq!(r4, RectI32::of(MIN + 10, MIN + 30, MAX - 10, MIN + 9));
}

#[test]
fn limits_out_of_bounds() {
    let mut r1 = RectI32::largest();
    wrapping_translate_assign(&mut r1, &PointI32::of(MIN, 0));
    assert_eq!(r1, RectI32::of(0, MIN, -1, MAX));

    let mut r2 = RectI32::largest();
    wrapping_translate_assign(&mut r2, &PointI32::of(0, MIN));
    assert_eq!(r2, RectI32::of(MIN, 0, MAX, -1));

    let mut r3 = RectI32::largest();
    wrapping_translate_assign(&mut r3, &PointI32::of(MAX, 0));
    assert_eq!(r3, RectI32::of(-1, MIN, -2, MAX));

    let mut r4 = RectI32::largest();
    wrapping_translate_assign(&mut r4, &PointI32::of(0, MAX));
    assert_eq!(r4, RectI32::of(MIN, -1, MAX, -2));
}
