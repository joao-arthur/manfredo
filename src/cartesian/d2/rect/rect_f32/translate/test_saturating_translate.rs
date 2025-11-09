use super::saturating_translate;
use crate::cartesian::{
    d1::point::point_f32::{MAX, MIN},
    d2::{point::point_f32::Point, rect::rect_f32::Rect},
};

#[test]
fn test() {
    assert_eq!(saturating_translate(&Rect::new((0.0, 0.0), (10.0, 10.0)), &Point::new(10.0, 20.0)), Rect::new((10.0, 20.0), (20.0, 30.0)));
    assert_eq!(saturating_translate(&Rect::new((10.0, 20.0), (20.0, 30.0)), &Point::new(-20.0, -15.0)), Rect::new((-10.0, 5.0), (0.0, 15.0)));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_translate(&Rect::new((MIN + 2.0, MIN + 5.0), (0.0, 0.0)), &Point::new(-2.0, -5.0)), Rect::new((MIN, MIN), (-2.0, -5.0)));
    assert_eq!(saturating_translate(&Rect::new((MIN, MIN), (-2.0, -5.0)), &Point::new(2.0, 5.0)), Rect::new((MIN + 2.0, MIN + 5.0), (0.0, 0.0)));
    assert_eq!(saturating_translate(&Rect::new((2.0, 5.0), (MAX, MAX)), &Point::new(-2.0, -5.0)), Rect::new((0.0, 0.0), (MAX - 2.0, MAX - 5.0)));
    assert_eq!(saturating_translate(&Rect::new((0.0, 0.0), (MAX - 2.0, MAX - 5.0)), &Point::new(2.0, 5.0)), Rect::new((2.0, 5.0), (MAX, MAX)));
}

#[test]
fn out_of_bounds() {
    let r_min = Rect::new((MIN + 10.0, MIN + 10.0), (0.0, 0.0));
    assert_eq!(saturating_translate(&r_min, &Point::new(-20.0, 0.0)), Rect::new((MIN, MIN + 10.0), (-10.0, 0.0)));
    assert_eq!(saturating_translate(&r_min, &Point::new(0.0, -20.0)), Rect::new((MIN + 10.0, MIN), (0.0, -10.0)));
    assert_eq!(saturating_translate(&r_min, &Point::new(20.0, 0.0)), Rect::new((MIN + 30.0, MIN + 10.0), (20.0, 0.0)));
    assert_eq!(saturating_translate(&r_min, &Point::new(0.0, 20.0)), Rect::new((MIN + 10.0, MIN + 30.0), (0.0, 20.0)));

    let r_max = Rect::new((0.0, 0.0), (MAX - 10.0, MAX - 10.0));
    assert_eq!(saturating_translate(&r_max, &Point::new(-20.0, 0.0)), Rect::new((-20.0, 0.0), (MAX - 30.0, MAX - 10.0)));
    assert_eq!(saturating_translate(&r_max, &Point::new(0.0, -20.0)), Rect::new((0.0, -20.0), (MAX - 10.0, MAX - 30.0)));
    assert_eq!(saturating_translate(&r_max, &Point::new(20.0, 0.0)), Rect::new((10.0, 0.0), (MAX, MAX - 10.0)));
    assert_eq!(saturating_translate(&r_max, &Point::new(0.0, 20.0)), Rect::new((0.0, 10.0), (MAX - 10.0, MAX)));
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(saturating_translate(&r, &Point::new(MIN, 0.0)), Rect::new((MIN, MIN), (-1.0, -1.0)));
    assert_eq!(saturating_translate(&r, &Point::new(0.0, MIN)), Rect::new((MIN, MIN), (-1.0, -1.0)));
    assert_eq!(saturating_translate(&r, &Point::new(MAX, 0.0)), Rect::new((-1.0, MIN), (MAX - 1.0, -1.0)));
    assert_eq!(saturating_translate(&r, &Point::new(0.0, MAX)), Rect::new((MIN, -1.0), (-1.0, MAX - 1.0)));
}
