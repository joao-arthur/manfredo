use super::saturating_translate;
use crate::cartesian::{
    d1::point::point_i64::{MAX, MIN},
    d2::{point::point_i64::Point, rect::rect_i64::Rect},
};

#[test]
fn test() {
    assert_eq!(saturating_translate(&Rect::of((5, 9), (13, 37)), &Point::of(-10, -20)), Rect::of((-5, -11), (3, 17)));
    assert_eq!(saturating_translate(&Rect::of((-5, -11), (3, 17)), &Point::of(6, -19)), Rect::of((1, -30), (9, -2)));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_translate(&Rect::of((MIN + 2, MIN + 5), (MAX, MAX)), &Point::of(-2, -5)), Rect::of((MIN, MIN), (MAX - 2, MAX - 5)));
    assert_eq!(saturating_translate(&Rect::of((MIN, MIN), (MAX - 2, MAX - 5)), &Point::of(2, 5)), Rect::of((MIN + 2, MIN + 5), (MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    let r = Rect::of((MIN + 10, MIN + 10), (MAX - 10, MAX - 10));
    assert_eq!(saturating_translate(&r, &Point::of(-20, 0)), Rect::of((MIN, MIN + 10), (MAX - 20, MAX - 10)));
    assert_eq!(saturating_translate(&r, &Point::of(0, -20)), Rect::of((MIN + 10, MIN), (MAX - 10, MAX - 20)));
    assert_eq!(saturating_translate(&r, &Point::of(20, 0)), Rect::of((MIN + 20, MIN + 10), (MAX, MAX - 10)));
    assert_eq!(saturating_translate(&r, &Point::of(0, 20)), Rect::of((MIN + 10, MIN + 20), (MAX - 10, MAX)));
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(saturating_translate(&r, &Point::of(MIN, 0)), Rect::largest());
    assert_eq!(saturating_translate(&r, &Point::of(0, MIN)), Rect::largest());
    assert_eq!(saturating_translate(&r, &Point::of(MAX, 0)), Rect::largest());
    assert_eq!(saturating_translate(&r, &Point::of(0, MAX)), Rect::largest());
}
