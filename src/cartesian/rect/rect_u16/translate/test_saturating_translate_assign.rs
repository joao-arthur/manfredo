use super::saturating_translate_assign;
use crate::cartesian::{point::point_i16::PointI16, rect::rect_u16::RectU16};

#[test]
fn test() {
    let mut r = RectU16::of(0, 0, 12, 15);
    saturating_translate_assign(&mut r, &PointI16::of(5, 4));
    assert_eq!(r, RectU16::of(5, 4, 17, 19));
    saturating_translate_assign(&mut r, &PointI16::of(-4, -2));
    assert_eq!(r, RectU16::of(1, 2, 13, 17));
}

#[test]
fn to_bounds() {
    let mut min_r = RectU16::of(2, 5, u16::MAX, u16::MAX);
    saturating_translate_assign(&mut min_r, &PointI16::of(-2, -5));
    assert_eq!(min_r, RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5));

    let mut max_r = RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5);
    saturating_translate_assign(&mut max_r, &PointI16::of(2, 5));
    assert_eq!(max_r, RectU16::of(2, 5, u16::MAX, u16::MAX));
}

#[test]
fn out_of_bounds() {
    let mut r1 = RectU16::of(10, 10, u16::MAX - 10, u16::MAX - 10);
    saturating_translate_assign(&mut r1, &PointI16::of(-20, 0));
    assert_eq!(r1, RectU16::of(0, 10, u16::MAX - 20, u16::MAX - 10));

    let mut r2 = RectU16::of(10, 10, u16::MAX - 10, u16::MAX - 10);
    saturating_translate_assign(&mut r2, &PointI16::of(0, -20));
    assert_eq!(r2, RectU16::of(10, 0, u16::MAX - 10, u16::MAX - 20));

    let mut r3 = RectU16::of(10, 10, u16::MAX - 10, u16::MAX - 10);
    saturating_translate_assign(&mut r3, &PointI16::of(20, 0));
    assert_eq!(r3, RectU16::of(20, 10, u16::MAX, u16::MAX - 10));

    let mut r4 = RectU16::of(10, 10, u16::MAX - 10, u16::MAX - 10);
    saturating_translate_assign(&mut r4, &PointI16::of(0, 20));
    assert_eq!(r4, RectU16::of(10, 20, u16::MAX - 10, u16::MAX));
}

#[test]
fn limits_out_of_bounds() {
    let mut r = RectU16::largest();
    saturating_translate_assign(&mut r, &PointI16::of(i16::MIN, 0));
    assert_eq!(r, RectU16::largest());
    saturating_translate_assign(&mut r, &PointI16::of(0, i16::MIN));
    assert_eq!(r, RectU16::largest());
    saturating_translate_assign(&mut r, &PointI16::of(i16::MAX, 0));
    assert_eq!(r, RectU16::largest());
    saturating_translate_assign(&mut r, &PointI16::of(0, i16::MAX));
    assert_eq!(r, RectU16::largest());
}
