use super::wrapping_translate_assign;
use crate::cartesian::{point::point_i64::PointI64, rect::rect_u64::RectU64};

const MAX: u64 = u64::MAX;

#[test]
fn test() {
    let mut r = RectU64::of(0, 0, 12, 15);
    wrapping_translate_assign(&mut r, &PointI64::of(5, 4));
    assert_eq!(r, RectU64::of(5, 4, 17, 19));
    wrapping_translate_assign(&mut r, &PointI64::of(-4, -2));
    assert_eq!(r, RectU64::of(1, 2, 13, 17));
}

#[test]
fn to_bounds() {
    let mut r_min = RectU64::of(2, 5, MAX, MAX);
    wrapping_translate_assign(&mut r_min, &PointI64::of(-2, -5));
    assert_eq!(r_min, RectU64::of(0, 0, MAX - 2, MAX - 5));

    let mut r_max = RectU64::of(0, 0, MAX - 2, MAX - 5);
    wrapping_translate_assign(&mut r_max, &PointI64::of(2, 5));
    assert_eq!(r_max, RectU64::of(2, 5, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r1 = RectU64::of(10, 10, MAX - 10, MAX - 10);
    wrapping_translate_assign(&mut r1, &PointI64::of(-20, 0));
    assert_eq!(r1, RectU64::of(MAX - 9, 10, MAX - 30, MAX - 10));

    let mut r2 = RectU64::of(10, 10, MAX - 10, MAX - 10);
    wrapping_translate_assign(&mut r2, &PointI64::of(0, -20));
    assert_eq!(r2, RectU64::of(10, MAX - 9, MAX - 10, MAX - 30));

    let mut r3 = RectU64::of(10, 10, MAX - 10, MAX - 10);
    wrapping_translate_assign(&mut r3, &PointI64::of(20, 0));
    assert_eq!(r3, RectU64::of(30, 10, 9, MAX - 10));

    let mut r4 = RectU64::of(10, 10, MAX - 10, MAX - 10);
    wrapping_translate_assign(&mut r4, &PointI64::of(0, 20));
    assert_eq!(r4, RectU64::of(10, 30, MAX - 10, 9));
}

#[test]
fn limits_out_of_bounds() {
    let mut r1 = RectU64::largest();
    wrapping_translate_assign(&mut r1, &PointI64::of(i64::MIN, 0));
    assert_eq!(r1, RectU64::of(MAX / 2 + 1, 0, MAX / 2, MAX));

    let mut r2 = RectU64::largest();
    wrapping_translate_assign(&mut r2, &PointI64::of(0, i64::MIN));
    assert_eq!(r2, RectU64::of(0, MAX / 2 + 1, MAX, MAX / 2));

    let mut r3 = RectU64::largest();
    wrapping_translate_assign(&mut r3, &PointI64::of(i64::MAX, 0));
    assert_eq!(r3, RectU64::of(MAX / 2, 0, MAX / 2 - 1, MAX));

    let mut r4 = RectU64::largest();
    wrapping_translate_assign(&mut r4, &PointI64::of(0, i64::MAX));
    assert_eq!(r4, RectU64::of(0, MAX / 2, MAX, MAX / 2 - 1));
}
