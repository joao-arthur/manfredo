use super::saturating_translate_assign;
use crate::cartesian::{point::point_i16::PointI16, rect::rect_i16::RectI16};

const MIN: i16 = i16::MIN;
const MAX: i16 = i16::MAX;

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
    let mut r_min = RectI16::of(MIN + 2, MIN + 5, MAX, MAX);
    saturating_translate_assign(&mut r_min, &PointI16::of(-2, -5));
    assert_eq!(r_min, RectI16::of(MIN, MIN, MAX - 2, MAX - 5));

    let mut r_max = RectI16::of(MIN, MIN, MAX - 2, MAX - 5);
    saturating_translate_assign(&mut r_max, &PointI16::of(2, 5));
    assert_eq!(r_max, RectI16::of(MIN + 2, MIN + 5, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r1 = RectI16::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    saturating_translate_assign(&mut r1, &PointI16::of(-20, 0));
    assert_eq!(r1, RectI16::of(MIN, MIN + 10, MAX - 20, MAX - 10));

    let mut r2 = RectI16::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    saturating_translate_assign(&mut r2, &PointI16::of(0, -20));
    assert_eq!(r2, RectI16::of(MIN + 10, MIN, MAX - 10, MAX - 20));

    let mut r3 = RectI16::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    saturating_translate_assign(&mut r3, &PointI16::of(20, 0));
    assert_eq!(r3, RectI16::of(MIN + 20, MIN + 10, MAX, MAX - 10));

    let mut r4 = RectI16::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    saturating_translate_assign(&mut r4, &PointI16::of(0, 20));
    assert_eq!(r4, RectI16::of(MIN + 10, MIN + 20, MAX - 10, MAX));
}

#[test]
fn limits_out_of_bounds() {
    let mut r = RectI16::largest();
    saturating_translate_assign(&mut r, &PointI16::of(MIN, 0));
    assert_eq!(r, RectI16::largest());
    saturating_translate_assign(&mut r, &PointI16::of(0, MIN));
    assert_eq!(r, RectI16::largest());
    saturating_translate_assign(&mut r, &PointI16::of(MAX, 0));
    assert_eq!(r, RectI16::largest());
    saturating_translate_assign(&mut r, &PointI16::of(0, MAX));
    assert_eq!(r, RectI16::largest());
}
