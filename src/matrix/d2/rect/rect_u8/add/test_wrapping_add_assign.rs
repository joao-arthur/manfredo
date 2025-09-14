use super::wrapping_add_assign;
use crate::matrix::d2::rect::{rect_i8::Rect as RectI, rect_u8::Rect};

const MAX: u8 = u8::MAX;

#[test]
fn test() {
    let mut r = Rect::of(0, 0, 12, 10);
    wrapping_add_assign(&mut r, &RectI::of(5, 4, 3, 2));
    assert_eq!(r, Rect::of(5, 4, 15, 12));
    wrapping_add_assign(&mut r, &RectI::of(-4, -3, -2, -1));
    assert_eq!(r, Rect::of(1, 1, 13, 11));
}

#[test]
fn to_bounds() {
    let mut r = Rect::of(2, 5, MAX - 2, MAX - 5);
    wrapping_add_assign(&mut r, &RectI::of(-2, -5, 2, 5));
    assert_eq!(r, Rect::largest());

    let mut r_min = Rect::of(2, 5, MAX, MAX);
    wrapping_add_assign(&mut r_min, &RectI::of(-2, -5, 0, 0));
    assert_eq!(r_min, Rect::largest());

    let mut r_max = Rect::of(0, 0, MAX - 2, MAX - 5);
    wrapping_add_assign(&mut r_max, &RectI::of(0, 0, 2, 5));
    assert_eq!(r_max, Rect::largest());
}

#[test]
fn out_of_bounds() {
    let mut r1 = Rect::of(10, 10, MAX - 10, MAX - 10);
    wrapping_add_assign(&mut r1, &RectI::of(-20, 0, 0, 0));
    assert_eq!(r1, Rect::of(MAX - 9, 10, MAX - 10, MAX - 10));

    let mut r2 = Rect::of(10, 10, MAX - 10, MAX - 10);
    wrapping_add_assign(&mut r2, &RectI::of(0, -20, 0, 0));
    assert_eq!(r2, Rect::of(10, MAX - 9, MAX - 10, MAX - 10));

    let mut r3 = Rect::of(10, 10, MAX - 10, MAX - 10);
    wrapping_add_assign(&mut r3, &RectI::of(0, 0, 20, 0));
    assert_eq!(r3, Rect::of(10, 10, 9, MAX - 10));

    let mut r4 = Rect::of(10, 10, MAX - 10, MAX - 10);
    wrapping_add_assign(&mut r4, &RectI::of(0, 0, 0, 20));
    assert_eq!(r4, Rect::of(10, 10, MAX - 10, 9));
}

#[test]
fn edge_out_of_bounds() {
    let mut r1 = Rect::largest();
    wrapping_add_assign(&mut r1, &RectI::of(-1, 0, 0, 0));
    assert_eq!(r1, Rect::of(MAX, 0, MAX, MAX));

    let mut r2 = Rect::largest();
    wrapping_add_assign(&mut r2, &RectI::of(0, -1, 0, 0));
    assert_eq!(r2, Rect::of(0, MAX, MAX, MAX));

    let mut r3 = Rect::largest();
    wrapping_add_assign(&mut r3, &RectI::of(0, 0, 1, 0));
    assert_eq!(r3, Rect::of(0, 0, 0, MAX));

    let mut r4 = Rect::largest();
    wrapping_add_assign(&mut r4, &RectI::of(0, 0, 0, 1));
    assert_eq!(r4, Rect::of(0, 0, MAX, 0));
}

#[test]
fn limits_out_of_bounds() {
    let mut r1 = Rect::largest();
    wrapping_add_assign(&mut r1, &RectI::of(i8::MIN, 0, 0, 0));
    assert_eq!(r1, Rect::of(MAX / 2 + 1, 0, MAX, MAX));

    let mut r2 = Rect::largest();
    wrapping_add_assign(&mut r2, &RectI::of(0, i8::MIN, 0, 0));
    assert_eq!(r2, Rect::of(0, MAX / 2 + 1, MAX, MAX));

    let mut r3 = Rect::largest();
    wrapping_add_assign(&mut r3, &RectI::of(0, 0, i8::MAX, 0));
    assert_eq!(r3, Rect::of(0, 0, MAX / 2 - 1, MAX));

    let mut r4 = Rect::largest();
    wrapping_add_assign(&mut r4, &RectI::of(0, 0, 0, i8::MAX));
    assert_eq!(r4, Rect::of(0, 0, MAX, MAX / 2 - 1));
}
