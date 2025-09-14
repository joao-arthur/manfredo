use super::saturating_translate_assign;
use crate::matrix::d2::{point::point_i32::Point, rect::rect_u32::Rect};

const MAX: u32 = u32::MAX;

#[test]
fn test() {
    let mut r = Rect::of(0, 0, 12, 15);
    saturating_translate_assign(&mut r, &Point::of(5, 4));
    assert_eq!(r, Rect::of(5, 4, 17, 19));
    saturating_translate_assign(&mut r, &Point::of(-4, -2));
    assert_eq!(r, Rect::of(1, 2, 13, 17));
}

#[test]
fn to_bounds() {
    let mut r_min = Rect::of(2, 5, MAX, MAX);
    saturating_translate_assign(&mut r_min, &Point::of(-2, -5));
    assert_eq!(r_min, Rect::of(0, 0, MAX - 2, MAX - 5));

    let mut r_max = Rect::of(0, 0, MAX - 2, MAX - 5);
    saturating_translate_assign(&mut r_max, &Point::of(2, 5));
    assert_eq!(r_max, Rect::of(2, 5, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r1 = Rect::of(10, 10, MAX - 10, MAX - 10);
    saturating_translate_assign(&mut r1, &Point::of(-20, 0));
    assert_eq!(r1, Rect::of(0, 10, MAX - 20, MAX - 10));

    let mut r2 = Rect::of(10, 10, MAX - 10, MAX - 10);
    saturating_translate_assign(&mut r2, &Point::of(0, -20));
    assert_eq!(r2, Rect::of(10, 0, MAX - 10, MAX - 20));

    let mut r3 = Rect::of(10, 10, MAX - 10, MAX - 10);
    saturating_translate_assign(&mut r3, &Point::of(20, 0));
    assert_eq!(r3, Rect::of(20, 10, MAX, MAX - 10));

    let mut r4 = Rect::of(10, 10, MAX - 10, MAX - 10);
    saturating_translate_assign(&mut r4, &Point::of(0, 20));
    assert_eq!(r4, Rect::of(10, 20, MAX - 10, MAX));
}

#[test]
fn limits_out_of_bounds() {
    let mut r = Rect::largest();
    saturating_translate_assign(&mut r, &Point::of(i32::MIN, 0));
    assert_eq!(r, Rect::largest());
    saturating_translate_assign(&mut r, &Point::of(0, i32::MIN));
    assert_eq!(r, Rect::largest());
    saturating_translate_assign(&mut r, &Point::of(i32::MAX, 0));
    assert_eq!(r, Rect::largest());
    saturating_translate_assign(&mut r, &Point::of(0, i32::MAX));
    assert_eq!(r, Rect::largest());
}
