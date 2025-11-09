use super::wrapping_translate_assign;
use crate::cartesian::{
    d1::point::point_i16::{MAX, MIN},
    d2::{point::point_i16::Point, rect::rect_i16::Rect},
};

#[test]
fn test() {
    let mut r = Rect::new((5, 9), (13, 37));
    wrapping_translate_assign(&mut r, &Point::new(-10, -20));
    assert_eq!(r, Rect::new((-5, -11), (3, 17)));
    wrapping_translate_assign(&mut r, &Point::new(6, -19));
    assert_eq!(r, Rect::new((1, -30), (9, -2)));
}

#[test]
fn to_bounds() {
    let mut r_min = Rect::new((MIN + 2, MIN + 5), (MAX, MAX));
    wrapping_translate_assign(&mut r_min, &Point::new(-2, -5));
    assert_eq!(r_min, Rect::new((MIN, MIN), (MAX - 2, MAX - 5)));

    let mut r_max = Rect::new((MIN, MIN), (MAX - 2, MAX - 5));
    wrapping_translate_assign(&mut r_max, &Point::new(2, 5));
    assert_eq!(r_max, Rect::new((MIN + 2, MIN + 5), (MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    let mut r1 = Rect::new((MIN + 10, MIN + 10), (MAX - 10, MAX - 10));
    wrapping_translate_assign(&mut r1, &Point::new(-20, 0));
    assert_eq!(r1, Rect::new((MAX - 9, MIN + 10), (MAX - 30, MAX - 10)));

    let mut r2 = Rect::new((MIN + 10, MIN + 10), (MAX - 10, MAX - 10));
    wrapping_translate_assign(&mut r2, &Point::new(0, -20));
    assert_eq!(r2, Rect::new((MIN + 10, MAX - 9), (MAX - 10, MAX - 30)));

    let mut r3 = Rect::new((MIN + 10, MIN + 10), (MAX - 10, MAX - 10));
    wrapping_translate_assign(&mut r3, &Point::new(20, 0));
    assert_eq!(r3, Rect::new((MIN + 30, MIN + 10), (MIN + 9, MAX - 10)));

    let mut r4 = Rect::new((MIN + 10, MIN + 10), (MAX - 10, MAX - 10));
    wrapping_translate_assign(&mut r4, &Point::new(0, 20));
    assert_eq!(r4, Rect::new((MIN + 10, MIN + 30), (MAX - 10, MIN + 9)));
}

#[test]
fn limits_out_of_bounds() {
    let mut r1 = Rect::largest();
    wrapping_translate_assign(&mut r1, &Point::new(MIN, 0));
    assert_eq!(r1, Rect::new((0, MIN), (-1, MAX)));

    let mut r2 = Rect::largest();
    wrapping_translate_assign(&mut r2, &Point::new(0, MIN));
    assert_eq!(r2, Rect::new((MIN, 0), (MAX, -1)));

    let mut r3 = Rect::largest();
    wrapping_translate_assign(&mut r3, &Point::new(MAX, 0));
    assert_eq!(r3, Rect::new((-1, MIN), (-2, MAX)));

    let mut r4 = Rect::largest();
    wrapping_translate_assign(&mut r4, &Point::new(0, MAX));
    assert_eq!(r4, Rect::new((MIN, -1), (MAX, -2)));
}
