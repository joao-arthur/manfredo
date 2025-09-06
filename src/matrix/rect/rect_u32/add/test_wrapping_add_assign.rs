use super::wrapping_add_assign;
use crate::matrix::rect::{rect_i32::RectI32, rect_u32::RectU32};

#[test]
fn test() {
    let mut r = RectU32::of(0, 0, 12, 10);
    wrapping_add_assign(&mut r, &RectI32::of(5, 4, 3, 2));
    assert_eq!(r, RectU32::of(5, 4, 15, 12));
    wrapping_add_assign(&mut r, &RectI32::of(-4, -3, -2, -1));
    assert_eq!(r, RectU32::of(1, 1, 13, 11));
}

#[test]
fn to_bounds() {
    let mut r = RectU32::of(2, 5, u32::MAX - 2, u32::MAX - 5);
    wrapping_add_assign(&mut r, &RectI32::of(-2, -5, 2, 5));
    assert_eq!(r, RectU32::largest());

    let mut r_min = RectU32::of(2, 5, u32::MAX, u32::MAX);
    wrapping_add_assign(&mut r_min, &RectI32::of(-2, -5, 0, 0));
    assert_eq!(r_min, RectU32::largest());

    let mut r_max = RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5);
    wrapping_add_assign(&mut r_max, &RectI32::of(0, 0, 2, 5));
    assert_eq!(r_max, RectU32::largest());
}

#[test]
fn out_of_bounds() {
    let mut r1 = RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 10);
    wrapping_add_assign(&mut r1, &RectI32::of(-20, 0, 0, 0));
    assert_eq!(r1, RectU32::of(u32::MAX - 9, 10, u32::MAX - 10, u32::MAX - 10));

    let mut r2 = RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 10);
    wrapping_add_assign(&mut r2, &RectI32::of(0, -20, 0, 0));
    assert_eq!(r2, RectU32::of(10, u32::MAX - 9, u32::MAX - 10, u32::MAX - 10));

    let mut r3 = RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 10);
    wrapping_add_assign(&mut r3, &RectI32::of(0, 0, 20, 0));
    assert_eq!(r3, RectU32::of(10, 10, 9, u32::MAX - 10));

    let mut r4 = RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 10);
    wrapping_add_assign(&mut r4, &RectI32::of(0, 0, 0, 20));
    assert_eq!(r4, RectU32::of(10, 10, u32::MAX - 10, 9));
}

#[test]
fn edge_out_of_bounds() {
    let mut r1 = RectU32::largest();
    wrapping_add_assign(&mut r1, &RectI32::of(-1, 0, 0, 0));
    assert_eq!(r1, RectU32::of(u32::MAX, 0, u32::MAX, u32::MAX));

    let mut r2 = RectU32::largest();
    wrapping_add_assign(&mut r2, &RectI32::of(0, -1, 0, 0));
    assert_eq!(r2, RectU32::of(0, u32::MAX, u32::MAX, u32::MAX));

    let mut r3 = RectU32::largest();
    wrapping_add_assign(&mut r3, &RectI32::of(0, 0, 1, 0));
    assert_eq!(r3, RectU32::of(0, 0, 0, u32::MAX));

    let mut r4 = RectU32::largest();
    wrapping_add_assign(&mut r4, &RectI32::of(0, 0, 0, 1));
    assert_eq!(r4, RectU32::of(0, 0, u32::MAX, 0));
}

#[test]
fn limits_out_of_bounds() {
    let mut r1 = RectU32::largest();
    wrapping_add_assign(&mut r1, &RectI32::of(i32::MIN, 0, 0, 0));
    assert_eq!(r1, RectU32::of(u32::MAX / 2 + 1, 0, u32::MAX, u32::MAX));

    let mut r2 = RectU32::largest();
    wrapping_add_assign(&mut r2, &RectI32::of(0, i32::MIN, 0, 0));
    assert_eq!(r2, RectU32::of(0, u32::MAX / 2 + 1, u32::MAX, u32::MAX));

    let mut r3 = RectU32::largest();
    wrapping_add_assign(&mut r3, &RectI32::of(0, 0, i32::MAX, 0));
    assert_eq!(r3, RectU32::of(0, 0, u32::MAX / 2 - 1, u32::MAX));

    let mut r4 = RectU32::largest();
    wrapping_add_assign(&mut r4, &RectI32::of(0, 0, 0, i32::MAX));
    assert_eq!(r4, RectU32::of(0, 0, u32::MAX, u32::MAX / 2 - 1));
}
