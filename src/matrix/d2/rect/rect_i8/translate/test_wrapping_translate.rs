use super::wrapping_translate;
use crate::matrix::{
    d1::point::point_i8::{MAX, MIN},
    d2::{point::point_i8::Point, rect::rect_i8::Rect},
};

#[test]
fn test() {
    assert_eq!(wrapping_translate(&Rect::new((5, 9), (13, 37)), &Point::new(-10, -20)), Rect::new((-5, -11), (3, 17)));
    assert_eq!(wrapping_translate(&Rect::new((-5, -11), (3, 17)), &Point::new(6, -19)), Rect::new((1, -30), (9, -2)));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_translate(&Rect::new((MIN + 2, MIN + 5), (MAX, MAX)), &Point::new(-2, -5)), Rect::new((MIN, MIN), (MAX - 2, MAX - 5)));
    assert_eq!(wrapping_translate(&Rect::new((MIN, MIN), (MAX - 2, MAX - 5)), &Point::new(2, 5)), Rect::new((MIN + 2, MIN + 5), (MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    let r = Rect::new((MIN + 10, MIN + 10), (MAX - 10, MAX - 10));
    assert_eq!(wrapping_translate(&r, &Point::new(-20, 0)), Rect::new((MAX - 9, MIN + 10), (MAX - 30, MAX - 10)));
    assert_eq!(wrapping_translate(&r, &Point::new(0, -20)), Rect::new((MIN + 10, MAX - 9), (MAX - 10, MAX - 30)));
    assert_eq!(wrapping_translate(&r, &Point::new(20, 0)), Rect::new((MIN + 30, MIN + 10), (MIN + 9, MAX - 10)));
    assert_eq!(wrapping_translate(&r, &Point::new(0, 20)), Rect::new((MIN + 10, MIN + 30), (MAX - 10, MIN + 9)));
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(wrapping_translate(&r, &Point::new(MIN, 0)), Rect::new((0, MIN), (-1, MAX)));
    assert_eq!(wrapping_translate(&r, &Point::new(0, MIN)), Rect::new((MIN, 0), (MAX, -1)));
    assert_eq!(wrapping_translate(&r, &Point::new(MAX, 0)), Rect::new((-1, MIN), (-2, MAX)));
    assert_eq!(wrapping_translate(&r, &Point::new(0, MAX)), Rect::new((MIN, -1), (MAX, -2)));
}
