use super::wrapping_translate;
use crate::cartesian::{
    d1::point::point_u64::MAX,
    d2::{point::point_i64::Point, rect::rect_u64::Rect},
};

#[test]
fn test() {
    assert_eq!(wrapping_translate(&Rect::of(0, 0, 12, 15), &Point::of(5, 4)), Rect::of(5, 4, 17, 19));
    assert_eq!(wrapping_translate(&Rect::of(5, 4, 17, 19), &Point::of(-4, -2)), Rect::of(1, 2, 13, 17));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_translate(&Rect::of(2, 5, MAX, MAX), &Point::of(-2, -5)), Rect::of(0, 0, MAX - 2, MAX - 5));
    assert_eq!(wrapping_translate(&Rect::of(0, 0, MAX - 2, MAX - 5), &Point::of(2, 5)), Rect::of(2, 5, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let r = Rect::of(10, 10, MAX - 10, MAX - 10);
    assert_eq!(wrapping_translate(&r, &Point::of(-20, 0)), Rect::of(MAX - 9, 10, MAX - 30, MAX - 10));
    assert_eq!(wrapping_translate(&r, &Point::of(0, -20)), Rect::of(10, MAX - 9, MAX - 10, MAX - 30));
    assert_eq!(wrapping_translate(&r, &Point::of(20, 0)), Rect::of(30, 10, 9, MAX - 10));
    assert_eq!(wrapping_translate(&r, &Point::of(0, 20)), Rect::of(10, 30, MAX - 10, 9));
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(wrapping_translate(&r, &Point::of(i64::MIN, 0)), Rect::of(MAX / 2 + 1, 0, MAX / 2, MAX));
    assert_eq!(wrapping_translate(&r, &Point::of(0, i64::MIN)), Rect::of(0, MAX / 2 + 1, MAX, MAX / 2));
    assert_eq!(wrapping_translate(&r, &Point::of(i64::MAX, 0)), Rect::of(MAX / 2, 0, MAX / 2 - 1, MAX));
    assert_eq!(wrapping_translate(&r, &Point::of(0, i64::MAX)), Rect::of(0, MAX / 2, MAX, MAX / 2 - 1));
}
