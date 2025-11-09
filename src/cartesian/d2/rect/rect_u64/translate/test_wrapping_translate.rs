use super::wrapping_translate;
use crate::cartesian::{
    d1::point::point_u64::MAX,
    d2::{point::point_i64::Point, rect::rect_u64::Rect},
};

#[test]
fn test() {
    assert_eq!(wrapping_translate(&Rect::new((0, 0), (12, 15)), &Point::new(5, 4)), Rect::new((5, 4), (17, 19)));
    assert_eq!(wrapping_translate(&Rect::new((5, 4), (17, 19)), &Point::new(-4, -2)), Rect::new((1, 2), (13, 17)));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_translate(&Rect::new((2, 5), (MAX, MAX)), &Point::new(-2, -5)), Rect::new((0, 0), (MAX - 2, MAX - 5)));
    assert_eq!(wrapping_translate(&Rect::new((0, 0), (MAX - 2, MAX - 5)), &Point::new(2, 5)), Rect::new((2, 5), (MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    let r = Rect::new((10, 10), (MAX - 10, MAX - 10));
    assert_eq!(wrapping_translate(&r, &Point::new(-20, 0)), Rect::new((MAX - 9, 10), (MAX - 30, MAX - 10)));
    assert_eq!(wrapping_translate(&r, &Point::new(0, -20)), Rect::new((10, MAX - 9), (MAX - 10, MAX - 30)));
    assert_eq!(wrapping_translate(&r, &Point::new(20, 0)), Rect::new((30, 10), (9, MAX - 10)));
    assert_eq!(wrapping_translate(&r, &Point::new(0, 20)), Rect::new((10, 30), (MAX - 10, 9)));
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(wrapping_translate(&r, &Point::new(i64::MIN, 0)), Rect::new((MAX / 2 + 1, 0), (MAX / 2, MAX)));
    assert_eq!(wrapping_translate(&r, &Point::new(0, i64::MIN)), Rect::new((0, MAX / 2 + 1), (MAX, MAX / 2)));
    assert_eq!(wrapping_translate(&r, &Point::new(i64::MAX, 0)), Rect::new((MAX / 2, 0), (MAX / 2 - 1, MAX)));
    assert_eq!(wrapping_translate(&r, &Point::new(0, i64::MAX)), Rect::new((0, MAX / 2), (MAX, MAX / 2 - 1)));
}
