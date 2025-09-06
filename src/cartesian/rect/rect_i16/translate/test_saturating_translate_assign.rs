use super::saturating_translate_assign;
use crate::cartesian::{point::point_i16::PointI16, rect::rect_i16::RectI16};

#[test]
fn test() {
    let mut r = RectI16::of(5, 9, 13, 37);
    saturating_translate_assign(&mut r, &PointI16::of(-10, -20));
    assert_eq!(r, RectI16::of(-5, -11, 3, 17));
    saturating_translate_assign(&mut r, &PointI16::of(6, -19));
    assert_eq!(r, RectI16::of(1, -30, 9, -2));
}

#[test]
fn to_bounds() {
    let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX);
    saturating_translate_assign(&mut min_r, &PointI16::of(-2, -5));
    assert_eq!(min_r, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5));

    let mut max_r = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5);
    saturating_translate_assign(&mut max_r, &PointI16::of(2, 5));
    assert_eq!(max_r, RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX));
}

#[test]
fn out_of_bounds() {
    let mut r1 = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10);
    saturating_translate_assign(&mut r1, &PointI16::of(-20, 0));
    assert_eq!(r1, RectI16::of(i16::MIN, i16::MIN + 10, i16::MAX - 20, i16::MAX - 10));

    let mut r2 = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10);
    saturating_translate_assign(&mut r2, &PointI16::of(0, -20));
    assert_eq!(r2, RectI16::of(i16::MIN + 10, i16::MIN, i16::MAX - 10, i16::MAX - 20));

    let mut r3 = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10);
    saturating_translate_assign(&mut r3, &PointI16::of(20, 0));
    assert_eq!(r3, RectI16::of(i16::MIN + 20, i16::MIN + 10, i16::MAX, i16::MAX - 10));

    let mut r4 = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10);
    saturating_translate_assign(&mut r4, &PointI16::of(0, 20));
    assert_eq!(r4, RectI16::of(i16::MIN + 10, i16::MIN + 20, i16::MAX - 10, i16::MAX));
}

#[test]
fn limits_out_of_bounds() {
    let mut r = RectI16::largest();
    saturating_translate_assign(&mut r, &PointI16::of(i16::MIN, 0));
    assert_eq!(r, RectI16::largest());
    saturating_translate_assign(&mut r, &PointI16::of(0, i16::MIN));
    assert_eq!(r, RectI16::largest());
    saturating_translate_assign(&mut r, &PointI16::of(i16::MAX, 0));
    assert_eq!(r, RectI16::largest());
    saturating_translate_assign(&mut r, &PointI16::of(0, i16::MAX));
    assert_eq!(r, RectI16::largest());
}
