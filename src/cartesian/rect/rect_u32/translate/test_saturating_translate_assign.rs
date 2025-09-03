use super::saturating_translate_assign;
use crate::cartesian::{point::point_i32::PointI32, rect::rect_u32::RectU32};

#[test]
fn test_saturating_translate_assign() {
    let mut r = RectU32::of(0, 0, 12, 15);
    saturating_translate_assign(&mut r, &PointI32::of(5, 4));
    assert_eq!(r, RectU32::of(5, 4, 17, 19));
    saturating_translate_assign(&mut r, &PointI32::of(-4, -2));
    assert_eq!(r, RectU32::of(1, 2, 13, 17));
}

#[test]
fn saturating_translate_assign_to_bounds() {
    let mut min_r = RectU32::of(2, 5, u32::MAX, u32::MAX);
    saturating_translate_assign(&mut min_r, &PointI32::of(-2, -5));
    assert_eq!(min_r, RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5));

    let mut max_r = RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5);
    saturating_translate_assign(&mut max_r, &PointI32::of(2, 5));
    assert_eq!(max_r, RectU32::of(2, 5, u32::MAX, u32::MAX));
}

#[test]
fn saturating_translate_assign_out_of_bounds() {
    let mut r1 = RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 10);
    saturating_translate_assign(&mut r1, &PointI32::of(-20, 0));
    assert_eq!(r1, RectU32::of(0, 10, u32::MAX - 20, u32::MAX - 10));

    let mut r2 = RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 10);
    saturating_translate_assign(&mut r2, &PointI32::of(0, -20));
    assert_eq!(r2, RectU32::of(10, 0, u32::MAX - 10, u32::MAX - 20));

    let mut r3 = RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 10);
    saturating_translate_assign(&mut r3, &PointI32::of(20, 0));
    assert_eq!(r3, RectU32::of(20, 10, u32::MAX, u32::MAX - 10));

    let mut r4 = RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 10);
    saturating_translate_assign(&mut r4, &PointI32::of(0, 20));
    assert_eq!(r4, RectU32::of(10, 20, u32::MAX - 10, u32::MAX));
}

#[test]
fn saturating_translate_assign_limits_out_of_bounds() {
    let mut r = RectU32::largest();
    saturating_translate_assign(&mut r, &PointI32::of(i32::MIN, 0));
    assert_eq!(r, RectU32::largest());
    saturating_translate_assign(&mut r, &PointI32::of(0, i32::MIN));
    assert_eq!(r, RectU32::largest());
    saturating_translate_assign(&mut r, &PointI32::of(i32::MAX, 0));
    assert_eq!(r, RectU32::largest());
    saturating_translate_assign(&mut r, &PointI32::of(0, i32::MAX));
    assert_eq!(r, RectU32::largest());
}
