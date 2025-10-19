use super::wrapping_translate;
use crate::cartesian::{
    d1::point::point_i32::{MAX, MIN},
    d2::{point::point_i32::Point, rect::rect_i32::Rect},
};

#[test]
fn test() {
    assert_eq!(wrapping_translate(&Rect::of(5, 9, 13, 37), &Point::of(-10, -20)), Rect::of(-5, -11, 3, 17));
    assert_eq!(wrapping_translate(&Rect::of(-5, -11, 3, 17), &Point::of(6, -19)), Rect::of(1, -30, 9, -2));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_translate(&Rect::of(MIN + 2, MIN + 5, MAX, MAX), &Point::of(-2, -5)), Rect::of(MIN, MIN, MAX - 2, MAX - 5));
    assert_eq!(wrapping_translate(&Rect::of(MIN, MIN, MAX - 2, MAX - 5), &Point::of(2, 5)), Rect::of(MIN + 2, MIN + 5, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let r = Rect::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    assert_eq!(wrapping_translate(&r, &Point::of(-20, 0)), Rect::of(MAX - 9, MIN + 10, MAX - 30, MAX - 10));
    assert_eq!(wrapping_translate(&r, &Point::of(0, -20)), Rect::of(MIN + 10, MAX - 9, MAX - 10, MAX - 30));
    assert_eq!(wrapping_translate(&r, &Point::of(20, 0)), Rect::of(MIN + 30, MIN + 10, MIN + 9, MAX - 10));
    assert_eq!(wrapping_translate(&r, &Point::of(0, 20)), Rect::of(MIN + 10, MIN + 30, MAX - 10, MIN + 9));
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(wrapping_translate(&r, &Point::of(MIN, 0)), Rect::of(0, MIN, -1, MAX));
    assert_eq!(wrapping_translate(&r, &Point::of(0, MIN)), Rect::of(MIN, 0, MAX, -1));
    assert_eq!(wrapping_translate(&r, &Point::of(MAX, 0)), Rect::of(-1, MIN, -2, MAX));
    assert_eq!(wrapping_translate(&r, &Point::of(0, MAX)), Rect::of(MIN, -1, MAX, -2));
}
