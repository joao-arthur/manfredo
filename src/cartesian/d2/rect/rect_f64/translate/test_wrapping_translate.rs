use super::wrapping_translate;
use crate::cartesian::{
    d1::point::point_f64::{MAX, MIN},
    d2::{point::point_f64::Point, rect::rect_f64::Rect},
};

#[test]
fn test() {
    assert_eq!(wrapping_translate(&Rect::new((0.0, 0.0), (12.0, 15.0)), &Point::new(5.0, 4.0)), Rect::new((5.0, 4.0), (17.0, 19.0)));
    assert_eq!(wrapping_translate(&Rect::new((5.0, 4.0), (17.0, 19.0)), &Point::new(-4.0, -2.0)), Rect::new((1.0, 2.0), (13.0, 17.0)));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_translate(&Rect::new((MIN + 2.0, MIN + 5.0), (MAX, MAX)), &Point::new(-2.0, -5.0)), Rect::new((MIN, MIN), (MAX - 2.0, MAX - 5.0)));
    assert_eq!(wrapping_translate(&Rect::new((MIN, MIN), (MAX - 2.0, MAX - 5.0)), &Point::new(2.0, 5.0)), Rect::new((MIN + 2.0, MIN + 5.0), (MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    let r = Rect::new((MIN + 10.0, MIN + 10.0), (MAX - 10.0, MAX - 10.0));
    assert_eq!(wrapping_translate(&r, &Point::new(-20.0, 0.0)), Rect::new((MAX - 9.0, MIN + 10.0), (MAX - 30.0, MAX - 10.0)));
    assert_eq!(wrapping_translate(&r, &Point::new(0.0, -20.0)), Rect::new((MIN + 10.0, MAX - 9.0), (MAX - 10.0, MAX - 30.0)));
    assert_eq!(wrapping_translate(&r, &Point::new(20.0, 0.0)), Rect::new((MIN + 30.0, MIN + 10.0), (MIN + 9.0, MAX - 10.0)));
    assert_eq!(wrapping_translate(&r, &Point::new(0.0, 20.0)), Rect::new((MIN + 10.0, MIN + 30.0), (MAX - 10.0, MIN + 9.0)));

    let r_min = Rect::new((MIN, MIN), (MIN + 10.0, MIN + 10.0));
    assert_eq!(wrapping_translate(&r_min, &Point::new(-20.0, -20.0)), Rect::new((MAX - 19.0, MAX - 19.0), (MAX - 9.0, MAX - 9.0)));

    let r_max = Rect::new((MAX, MAX), (MAX - 10.0, MAX - 10.0));
    assert_eq!(wrapping_translate(&r_max, &Point::new(20.0, 20.0)), Rect::new((MIN + 19.0, MIN + 19.0), (MIN + 9.0, MIN + 9.0)));
}

#[test]
fn edge_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(wrapping_translate(&r, &Point::new(-1.0, 0.0)), Rect::new((MAX, MIN), (MAX - 1.0, MAX)));
    assert_eq!(wrapping_translate(&r, &Point::new(0.0, -1.0)), Rect::new((MIN, MAX), (MAX, MAX - 1.0)));
    assert_eq!(wrapping_translate(&r, &Point::new(1.0, 0.0)), Rect::new((MIN + 1.0, MIN), (MIN, MAX)));
    assert_eq!(wrapping_translate(&r, &Point::new(0.0, 1.0)), Rect::new((MIN, MIN + 1.0), (MAX, MIN)));
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(wrapping_translate(&r, &Point::new(MIN, 0.0)), Rect::new((0.0, MIN), (-1.0, MAX)));
    assert_eq!(wrapping_translate(&r, &Point::new(0.0, MIN)), Rect::new((MIN, 0.0), (MAX, -1.0)));
    assert_eq!(wrapping_translate(&r, &Point::new(MAX, 0.0)), Rect::new((-1.0, MIN), (-2.0, MAX)));
    assert_eq!(wrapping_translate(&r, &Point::new(0.0, MAX)), Rect::new((MIN, -1.0), (MAX, -2.0)));
}
