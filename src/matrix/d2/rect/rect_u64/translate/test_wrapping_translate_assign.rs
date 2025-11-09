use super::wrapping_translate_assign;
use crate::matrix::{
    d1::point::point_u64::MAX,
    d2::{point::point_i64::Point, rect::rect_u64::Rect},
};

#[test]
fn test() {
    let mut r = Rect::new((0, 0), (12, 15));
    wrapping_translate_assign(&mut r, &Point::new(5, 4));
    assert_eq!(r, Rect::new((5, 4), (17, 19)));
    wrapping_translate_assign(&mut r, &Point::new(-4, -2));
    assert_eq!(r, Rect::new((1, 2), (13, 17)));
}

#[test]
fn to_bounds() {
    let mut r_min = Rect::new((2, 5), (MAX, MAX));
    wrapping_translate_assign(&mut r_min, &Point::new(-2, -5));
    assert_eq!(r_min, Rect::new((0, 0), (MAX - 2, MAX - 5)));

    let mut r_max = Rect::new((0, 0), (MAX - 2, MAX - 5));
    wrapping_translate_assign(&mut r_max, &Point::new(2, 5));
    assert_eq!(r_max, Rect::new((2, 5), (MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    let mut r1 = Rect::new((10, 10), (MAX - 10, MAX - 10));
    wrapping_translate_assign(&mut r1, &Point::new(-20, 0));
    assert_eq!(r1, Rect::new((MAX - 9, 10), (MAX - 30, MAX - 10)));

    let mut r2 = Rect::new((10, 10), (MAX - 10, MAX - 10));
    wrapping_translate_assign(&mut r2, &Point::new(0, -20));
    assert_eq!(r2, Rect::new((10, MAX - 9), (MAX - 10, MAX - 30)));

    let mut r3 = Rect::new((10, 10), (MAX - 10, MAX - 10));
    wrapping_translate_assign(&mut r3, &Point::new(20, 0));
    assert_eq!(r3, Rect::new((30, 10), (9, MAX - 10)));

    let mut r4 = Rect::new((10, 10), (MAX - 10, MAX - 10));
    wrapping_translate_assign(&mut r4, &Point::new(0, 20));
    assert_eq!(r4, Rect::new((10, 30), (MAX - 10, 9)));
}

#[test]
fn limits_out_of_bounds() {
    let mut r1 = Rect::largest();
    wrapping_translate_assign(&mut r1, &Point::new(i64::MIN, 0));
    assert_eq!(r1, Rect::new((MAX / 2 + 1, 0), (MAX / 2, MAX)));

    let mut r2 = Rect::largest();
    wrapping_translate_assign(&mut r2, &Point::new(0, i64::MIN));
    assert_eq!(r2, Rect::new((0, MAX / 2 + 1), (MAX, MAX / 2)));

    let mut r3 = Rect::largest();
    wrapping_translate_assign(&mut r3, &Point::new(i64::MAX, 0));
    assert_eq!(r3, Rect::new((MAX / 2, 0), (MAX / 2 - 1, MAX)));

    let mut r4 = Rect::largest();
    wrapping_translate_assign(&mut r4, &Point::new(0, i64::MAX));
    assert_eq!(r4, Rect::new((0, MAX / 2), (MAX, MAX / 2 - 1)));
}
