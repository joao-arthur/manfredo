use super::saturating_translate_assign;
use crate::matrix::{point::point_i64::PointI64, rect::rect_u64::Rect};

const MAX: u64 = u64::MAX;

#[test]
fn test() {
    let mut r = Rect::of(0, 0, 12, 15);
    saturating_translate_assign(&mut r, &PointI64::of(5, 4));
    assert_eq!(r, Rect::of(5, 4, 17, 19));
    saturating_translate_assign(&mut r, &PointI64::of(-4, -2));
    assert_eq!(r, Rect::of(1, 2, 13, 17));
}

#[test]
fn to_bounds() {
    let mut r_min = Rect::of(2, 5, MAX, MAX);
    saturating_translate_assign(&mut r_min, &PointI64::of(-2, -5));
    assert_eq!(r_min, Rect::of(0, 0, MAX - 2, MAX - 5));

    let mut r_max = Rect::of(0, 0, MAX - 2, MAX - 5);
    saturating_translate_assign(&mut r_max, &PointI64::of(2, 5));
    assert_eq!(r_max, Rect::of(2, 5, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r1 = Rect::of(10, 10, MAX - 10, MAX - 10);
    saturating_translate_assign(&mut r1, &PointI64::of(-20, 0));
    assert_eq!(r1, Rect::of(0, 10, MAX - 20, MAX - 10));

    let mut r2 = Rect::of(10, 10, MAX - 10, MAX - 10);
    saturating_translate_assign(&mut r2, &PointI64::of(0, -20));
    assert_eq!(r2, Rect::of(10, 0, MAX - 10, MAX - 20));

    let mut r3 = Rect::of(10, 10, MAX - 10, MAX - 10);
    saturating_translate_assign(&mut r3, &PointI64::of(20, 0));
    assert_eq!(r3, Rect::of(20, 10, MAX, MAX - 10));

    let mut r4 = Rect::of(10, 10, MAX - 10, MAX - 10);
    saturating_translate_assign(&mut r4, &PointI64::of(0, 20));
    assert_eq!(r4, Rect::of(10, 20, MAX - 10, MAX));
}

#[test]
fn limits_out_of_bounds() {
    let mut r = Rect::largest();
    saturating_translate_assign(&mut r, &PointI64::of(i64::MIN, 0));
    assert_eq!(r, Rect::largest());
    saturating_translate_assign(&mut r, &PointI64::of(0, i64::MIN));
    assert_eq!(r, Rect::largest());
    saturating_translate_assign(&mut r, &PointI64::of(i64::MAX, 0));
    assert_eq!(r, Rect::largest());
    saturating_translate_assign(&mut r, &PointI64::of(0, i64::MAX));
    assert_eq!(r, Rect::largest());
}
