use super::saturating_translate_assign;
use crate::matrix::{
    d1::point::point_i16::{MAX, MIN},
    d2::{point::point_i16::Point, rect::rect_i16::Rect},
};

#[test]
fn test() {
    let mut r = Rect::of((5, 9), (13, 37));
    saturating_translate_assign(&mut r, &Point::of(-10, -20));
    assert_eq!(r, Rect::of((-5, -11), (3, 17)));
    saturating_translate_assign(&mut r, &Point::of(6, -19));
    assert_eq!(r, Rect::of((1, -30), (9, -2)));
}

#[test]
fn to_bounds() {
    let mut r_min = Rect::of((MIN + 2, MIN + 5), (MAX, MAX));
    saturating_translate_assign(&mut r_min, &Point::of(-2, -5));
    assert_eq!(r_min, Rect::of((MIN, MIN), (MAX - 2, MAX - 5)));

    let mut r_max = Rect::of((MIN, MIN), (MAX - 2, MAX - 5));
    saturating_translate_assign(&mut r_max, &Point::of(2, 5));
    assert_eq!(r_max, Rect::of((MIN + 2, MIN + 5), (MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    let mut r1 = Rect::of((MIN + 10, MIN + 10), (MAX - 10, MAX - 10));
    saturating_translate_assign(&mut r1, &Point::of(-20, 0));
    assert_eq!(r1, Rect::of((MIN, MIN + 10), (MAX - 20, MAX - 10)));

    let mut r2 = Rect::of((MIN + 10, MIN + 10), (MAX - 10, MAX - 10));
    saturating_translate_assign(&mut r2, &Point::of(0, -20));
    assert_eq!(r2, Rect::of((MIN + 10, MIN), (MAX - 10, MAX - 20)));

    let mut r3 = Rect::of((MIN + 10, MIN + 10), (MAX - 10, MAX - 10));
    saturating_translate_assign(&mut r3, &Point::of(20, 0));
    assert_eq!(r3, Rect::of((MIN + 20, MIN + 10), (MAX, MAX - 10)));

    let mut r4 = Rect::of((MIN + 10, MIN + 10), (MAX - 10, MAX - 10));
    saturating_translate_assign(&mut r4, &Point::of(0, 20));
    assert_eq!(r4, Rect::of((MIN + 10, MIN + 20), (MAX - 10, MAX)));
}

#[test]
fn limits_out_of_bounds() {
    let mut r = Rect::largest();
    saturating_translate_assign(&mut r, &Point::of(MIN, 0));
    assert_eq!(r, Rect::largest());
    saturating_translate_assign(&mut r, &Point::of(0, MIN));
    assert_eq!(r, Rect::largest());
    saturating_translate_assign(&mut r, &Point::of(MAX, 0));
    assert_eq!(r, Rect::largest());
    saturating_translate_assign(&mut r, &Point::of(0, MAX));
    assert_eq!(r, Rect::largest());
}
