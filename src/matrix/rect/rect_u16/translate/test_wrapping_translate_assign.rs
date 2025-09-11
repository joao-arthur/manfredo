use super::wrapping_translate_assign;
use crate::matrix::{point::point_i16::Point, rect::rect_u16::Rect};

const MAX: u16 = u16::MAX;

#[test]
fn test() {
    let mut r = Rect::of(0, 0, 12, 15);
    wrapping_translate_assign(&mut r, &Point::of(5, 4));
    assert_eq!(r, Rect::of(5, 4, 17, 19));
    wrapping_translate_assign(&mut r, &Point::of(-4, -2));
    assert_eq!(r, Rect::of(1, 2, 13, 17));
}

#[test]
fn to_bounds() {
    let mut r_min = Rect::of(2, 5, MAX, MAX);
    wrapping_translate_assign(&mut r_min, &Point::of(-2, -5));
    assert_eq!(r_min, Rect::of(0, 0, MAX - 2, MAX - 5));

    let mut r_max = Rect::of(0, 0, MAX - 2, MAX - 5);
    wrapping_translate_assign(&mut r_max, &Point::of(2, 5));
    assert_eq!(r_max, Rect::of(2, 5, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r1 = Rect::of(10, 10, MAX - 10, MAX - 10);
    wrapping_translate_assign(&mut r1, &Point::of(-20, 0));
    assert_eq!(r1, Rect::of(MAX - 9, 10, MAX - 30, MAX - 10));

    let mut r2 = Rect::of(10, 10, MAX - 10, MAX - 10);
    wrapping_translate_assign(&mut r2, &Point::of(0, -20));
    assert_eq!(r2, Rect::of(10, MAX - 9, MAX - 10, MAX - 30));

    let mut r3 = Rect::of(10, 10, MAX - 10, MAX - 10);
    wrapping_translate_assign(&mut r3, &Point::of(20, 0));
    assert_eq!(r3, Rect::of(30, 10, 9, MAX - 10));

    let mut r4 = Rect::of(10, 10, MAX - 10, MAX - 10);
    wrapping_translate_assign(&mut r4, &Point::of(0, 20));
    assert_eq!(r4, Rect::of(10, 30, MAX - 10, 9));
}

#[test]
fn limits_out_of_bounds() {
    let mut r1 = Rect::largest();
    wrapping_translate_assign(&mut r1, &Point::of(i16::MIN, 0));
    assert_eq!(r1, Rect::of(MAX / 2 + 1, 0, MAX / 2, MAX));

    let mut r2 = Rect::largest();
    wrapping_translate_assign(&mut r2, &Point::of(0, i16::MIN));
    assert_eq!(r2, Rect::of(0, MAX / 2 + 1, MAX, MAX / 2));

    let mut r3 = Rect::largest();
    wrapping_translate_assign(&mut r3, &Point::of(i16::MAX, 0));
    assert_eq!(r3, Rect::of(MAX / 2, 0, MAX / 2 - 1, MAX));

    let mut r4 = Rect::largest();
    wrapping_translate_assign(&mut r4, &Point::of(0, i16::MAX));
    assert_eq!(r4, Rect::of(0, MAX / 2, MAX, MAX / 2 - 1));
}
