use super::wrapping_translate_assign;
use crate::cartesian::{point::point_i32::PointI32, rect::rect_u32::RectU32};

#[test]
fn test() {
    let mut r = RectU32::of(0, 0, 12, 15);
    wrapping_translate_assign(&mut r, &PointI32::of(5, 4));
    assert_eq!(r, RectU32::of(5, 4, 17, 19));
    wrapping_translate_assign(&mut r, &PointI32::of(-4, -2));
    assert_eq!(r, RectU32::of(1, 2, 13, 17));
}

#[test]
fn to_bounds() {
    let mut r_min = RectU32::of(2, 5, u32::MAX, u32::MAX);
    wrapping_translate_assign(&mut r_min, &PointI32::of(-2, -5));
    assert_eq!(r_min, RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5));

    let mut r_max = RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5);
    wrapping_translate_assign(&mut r_max, &PointI32::of(2, 5));
    assert_eq!(r_max, RectU32::of(2, 5, u32::MAX, u32::MAX));
}

#[test]
fn out_of_bounds() {
    let mut r1 = RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 10);
    wrapping_translate_assign(&mut r1, &PointI32::of(-20, 0));
    assert_eq!(r1, RectU32::of(u32::MAX - 9, 10, u32::MAX - 30, u32::MAX - 10));

    let mut r2 = RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 10);
    wrapping_translate_assign(&mut r2, &PointI32::of(0, -20));
    assert_eq!(r2, RectU32::of(10, u32::MAX - 9, u32::MAX - 10, u32::MAX - 30));

    let mut r3 = RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 10);
    wrapping_translate_assign(&mut r3, &PointI32::of(20, 0));
    assert_eq!(r3, RectU32::of(30, 10, 9, u32::MAX - 10));

    let mut r4 = RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 10);
    wrapping_translate_assign(&mut r4, &PointI32::of(0, 20));
    assert_eq!(r4, RectU32::of(10, 30, u32::MAX - 10, 9));
}

#[test]
fn limits_out_of_bounds() {
    let mut r1 = RectU32::largest();
    wrapping_translate_assign(&mut r1, &PointI32::of(i32::MIN, 0));
    assert_eq!(r1, RectU32::of(u32::MAX / 2 + 1, 0, u32::MAX / 2, u32::MAX));

    let mut r2 = RectU32::largest();
    wrapping_translate_assign(&mut r2, &PointI32::of(0, i32::MIN));
    assert_eq!(r2, RectU32::of(0, u32::MAX / 2 + 1, u32::MAX, u32::MAX / 2));

    let mut r3 = RectU32::largest();
    wrapping_translate_assign(&mut r3, &PointI32::of(i32::MAX, 0));
    assert_eq!(r3, RectU32::of(u32::MAX / 2, 0, u32::MAX / 2 - 1, u32::MAX));

    let mut r4 = RectU32::largest();
    wrapping_translate_assign(&mut r4, &PointI32::of(0, i32::MAX));
    assert_eq!(r4, RectU32::of(0, u32::MAX / 2, u32::MAX, u32::MAX / 2 - 1));
}
