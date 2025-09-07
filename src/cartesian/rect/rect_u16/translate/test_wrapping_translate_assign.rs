use super::wrapping_translate_assign;
use crate::cartesian::{point::point_i16::PointI16, rect::rect_u16::RectU16};

const MAX: u16 = u16::MAX;

#[test]
fn test() {
    let mut r = RectU16::of(0, 0, 12, 15);
    wrapping_translate_assign(&mut r, &PointI16::of(5, 4));
    assert_eq!(r, RectU16::of(5, 4, 17, 19));
    wrapping_translate_assign(&mut r, &PointI16::of(-4, -2));
    assert_eq!(r, RectU16::of(1, 2, 13, 17));
}

#[test]
fn to_bounds() {
    let mut r_min = RectU16::of(2, 5, MAX, MAX);
    wrapping_translate_assign(&mut r_min, &PointI16::of(-2, -5));
    assert_eq!(r_min, RectU16::of(0, 0, MAX - 2, MAX - 5));

    let mut r_max = RectU16::of(0, 0, MAX - 2, MAX - 5);
    wrapping_translate_assign(&mut r_max, &PointI16::of(2, 5));
    assert_eq!(r_max, RectU16::of(2, 5, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r1 = RectU16::of(10, 10, MAX - 10, MAX - 10);
    wrapping_translate_assign(&mut r1, &PointI16::of(-20, 0));
    assert_eq!(r1, RectU16::of(MAX - 9, 10, MAX - 30, MAX - 10));

    let mut r2 = RectU16::of(10, 10, MAX - 10, MAX - 10);
    wrapping_translate_assign(&mut r2, &PointI16::of(0, -20));
    assert_eq!(r2, RectU16::of(10, MAX - 9, MAX - 10, MAX - 30));

    let mut r3 = RectU16::of(10, 10, MAX - 10, MAX - 10);
    wrapping_translate_assign(&mut r3, &PointI16::of(20, 0));
    assert_eq!(r3, RectU16::of(30, 10, 9, MAX - 10));

    let mut r4 = RectU16::of(10, 10, MAX - 10, MAX - 10);
    wrapping_translate_assign(&mut r4, &PointI16::of(0, 20));
    assert_eq!(r4, RectU16::of(10, 30, MAX - 10, 9));
}

#[test]
fn limits_out_of_bounds() {
    let mut r1 = RectU16::largest();
    wrapping_translate_assign(&mut r1, &PointI16::of(i16::MIN, 0));
    assert_eq!(r1, RectU16::of(MAX / 2 + 1, 0, MAX / 2, MAX));

    let mut r2 = RectU16::largest();
    wrapping_translate_assign(&mut r2, &PointI16::of(0, i16::MIN));
    assert_eq!(r2, RectU16::of(0, MAX / 2 + 1, MAX, MAX / 2));

    let mut r3 = RectU16::largest();
    wrapping_translate_assign(&mut r3, &PointI16::of(i16::MAX, 0));
    assert_eq!(r3, RectU16::of(MAX / 2, 0, MAX / 2 - 1, MAX));

    let mut r4 = RectU16::largest();
    wrapping_translate_assign(&mut r4, &PointI16::of(0, i16::MAX));
    assert_eq!(r4, RectU16::of(0, MAX / 2, MAX, MAX / 2 - 1));
}
