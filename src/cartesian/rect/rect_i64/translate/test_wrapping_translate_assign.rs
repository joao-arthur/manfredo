use super::wrapping_translate_assign;
use crate::cartesian::{point::point_i64::PointI64, rect::rect_i64::RectI64};

#[test]
fn test() {
    let mut r = RectI64::of(5, 9, 13, 37);
    wrapping_translate_assign(&mut r, &PointI64::of(-10, -20));
    assert_eq!(r, RectI64::of(-5, -11, 3, 17));
    wrapping_translate_assign(&mut r, &PointI64::of(6, -19));
    assert_eq!(r, RectI64::of(1, -30, 9, -2));
}

#[test]
fn to_bounds() {
    let mut min_r = RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX);
    wrapping_translate_assign(&mut min_r, &PointI64::of(-2, -5));
    assert_eq!(min_r, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5));

    let mut max_r = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5);
    wrapping_translate_assign(&mut max_r, &PointI64::of(2, 5));
    assert_eq!(max_r, RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX));
}

#[test]
fn out_of_bounds() {
    let mut r1 = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10);
    wrapping_translate_assign(&mut r1, &PointI64::of(-20, 0));
    assert_eq!(r1, RectI64::of(i64::MAX - 9, i64::MIN + 10, i64::MAX - 30, i64::MAX - 10));

    let mut r2 = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10);
    wrapping_translate_assign(&mut r2, &PointI64::of(0, -20));
    assert_eq!(r2, RectI64::of(i64::MIN + 10, i64::MAX - 9, i64::MAX - 10, i64::MAX - 30));

    let mut r3 = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10);
    wrapping_translate_assign(&mut r3, &PointI64::of(20, 0));
    assert_eq!(r3, RectI64::of(i64::MIN + 30, i64::MIN + 10, i64::MIN + 9, i64::MAX - 10));

    let mut r4 = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10);
    wrapping_translate_assign(&mut r4, &PointI64::of(0, 20));
    assert_eq!(r4, RectI64::of(i64::MIN + 10, i64::MIN + 30, i64::MAX - 10, i64::MIN + 9));
}

#[test]
fn limits_out_of_bounds() {
    let mut r1 = RectI64::largest();
    wrapping_translate_assign(&mut r1, &PointI64::of(i64::MIN, 0));
    assert_eq!(r1, RectI64::of(0, i64::MIN, -1, i64::MAX));

    let mut r2 = RectI64::largest();
    wrapping_translate_assign(&mut r2, &PointI64::of(0, i64::MIN));
    assert_eq!(r2, RectI64::of(i64::MIN, 0, i64::MAX, -1));

    let mut r3 = RectI64::largest();
    wrapping_translate_assign(&mut r3, &PointI64::of(i64::MAX, 0));
    assert_eq!(r3, RectI64::of(-1, i64::MIN, -2, i64::MAX));

    let mut r4 = RectI64::largest();
    wrapping_translate_assign(&mut r4, &PointI64::of(0, i64::MAX));
    assert_eq!(r4, RectI64::of(i64::MIN, -1, i64::MAX, -2));
}
