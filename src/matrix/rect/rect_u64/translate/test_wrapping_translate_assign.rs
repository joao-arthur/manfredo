use super::wrapping_translate_assign;
use crate::matrix::{point::point_i64::PointI64, rect::rect_u64::RectU64};

#[test]
fn test_wrapping_translate_assign() {
    let mut r = RectU64::of(0, 0, 12, 15);
    wrapping_translate_assign(&mut r, &PointI64::of(5, 4));
    assert_eq!(r, RectU64::of(5, 4, 17, 19));
    wrapping_translate_assign(&mut r, &PointI64::of(-4, -2));
    assert_eq!(r, RectU64::of(1, 2, 13, 17));
}

#[test]
fn wrapping_translate_assign_to_bounds() {
    let mut min_r = RectU64::of(2, 5, u64::MAX, u64::MAX);
    wrapping_translate_assign(&mut min_r, &PointI64::of(-2, -5));
    assert_eq!(min_r, RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5));

    let mut max_r = RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 5);
    wrapping_translate_assign(&mut max_r, &PointI64::of(2, 5));
    assert_eq!(max_r, RectU64::of(2, 5, u64::MAX, u64::MAX));
}

#[test]
fn wrapping_translate_assign_out_of_bounds() {
    let mut r1 = RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10);
    wrapping_translate_assign(&mut r1, &PointI64::of(-20, 0));
    assert_eq!(r1, RectU64::of(u64::MAX - 9, 10, u64::MAX - 30, u64::MAX - 10));

    let mut r2 = RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10);
    wrapping_translate_assign(&mut r2, &PointI64::of(0, -20));
    assert_eq!(r2, RectU64::of(10, u64::MAX - 9, u64::MAX - 10, u64::MAX - 30));

    let mut r3 = RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10);
    wrapping_translate_assign(&mut r3, &PointI64::of(20, 0));
    assert_eq!(r3, RectU64::of(30, 10, 9, u64::MAX - 10));

    let mut r4 = RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 10);
    wrapping_translate_assign(&mut r4, &PointI64::of(0, 20));
    assert_eq!(r4, RectU64::of(10, 30, u64::MAX - 10, 9));
}

#[test]
fn wrapping_translate_assign_limits_out_of_bounds() {
    let mut r1 = RectU64::largest();
    wrapping_translate_assign(&mut r1, &PointI64::of(i64::MIN, 0));
    assert_eq!(r1, RectU64::of(u64::MAX / 2 + 1, 0, u64::MAX / 2, u64::MAX));

    let mut r2 = RectU64::largest();
    wrapping_translate_assign(&mut r2, &PointI64::of(0, i64::MIN));
    assert_eq!(r2, RectU64::of(0, u64::MAX / 2 + 1, u64::MAX, u64::MAX / 2));

    let mut r3 = RectU64::largest();
    wrapping_translate_assign(&mut r3, &PointI64::of(i64::MAX, 0));
    assert_eq!(r3, RectU64::of(u64::MAX / 2, 0, u64::MAX / 2 - 1, u64::MAX));

    let mut r4 = RectU64::largest();
    wrapping_translate_assign(&mut r4, &PointI64::of(0, i64::MAX));
    assert_eq!(r4, RectU64::of(0, u64::MAX / 2, u64::MAX, u64::MAX / 2 - 1));
}
