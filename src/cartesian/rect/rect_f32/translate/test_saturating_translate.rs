use super::saturating_translate;
use crate::cartesian::{
    point::point_f32::{MAX, MIN, Point},
    rect::rect_f32::Rect,
};

#[test]
fn test() {
    assert_eq!(saturating_translate(&Rect::of(0.0, 0.0, 10.0, 10.0), &Point::of(10.0, 20.0)), Rect::of(10.0, 20.0, 20.0, 30.0));
    assert_eq!(saturating_translate(&Rect::of(10.0, 20.0, 20.0, 30.0), &Point::of(-20.0, -15.0)), Rect::of(-10.0, 5.0, 0.0, 15.0));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_translate(&Rect::of(MIN + 2.0, MIN + 5.0, 0.0, 0.0), &Point::of(-2.0, -5.0)), Rect::of(MIN, MIN, -2.0, -5.0));
    assert_eq!(saturating_translate(&Rect::of(MIN, MIN, -2.0, -5.0), &Point::of(2.0, 5.0)), Rect::of(MIN + 2.0, MIN + 5.0, 0.0, 0.0));
    assert_eq!(saturating_translate(&Rect::of(2.0, 5.0, MAX, MAX), &Point::of(-2.0, -5.0)), Rect::of(0.0, 0.0, MAX - 2.0, MAX - 5.0));
    assert_eq!(saturating_translate(&Rect::of(0.0, 0.0, MAX - 2.0, MAX - 5.0), &Point::of(2.0, 5.0)), Rect::of(2.0, 5.0, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let r_min = Rect::of(MIN + 10.0, MIN + 10.0, 0.0, 0.0);
    assert_eq!(saturating_translate(&r_min, &Point::of(-20.0, 0.0)), Rect::of(MIN, MIN + 10.0, -10.0, 0.0));
    assert_eq!(saturating_translate(&r_min, &Point::of(0.0, -20.0)), Rect::of(MIN + 10.0, MIN, 0.0, -10.0));
    assert_eq!(saturating_translate(&r_min, &Point::of(20.0, 0.0)), Rect::of(MIN + 30.0, MIN + 10.0, 20.0, 0.0));
    assert_eq!(saturating_translate(&r_min, &Point::of(0.0, 20.0)), Rect::of(MIN + 10.0, MIN + 30.0, 0.0, 20.0));

    let r_max = Rect::of(0.0, 0.0, MAX - 10.0, MAX - 10.0);
    assert_eq!(saturating_translate(&r_max, &Point::of(-20.0, 0.0)), Rect::of(-20.0, 0.0, MAX - 30.0, MAX - 10.0));
    assert_eq!(saturating_translate(&r_max, &Point::of(0.0, -20.0)), Rect::of(0.0, -20.0, MAX - 10.0, MAX - 30.0));
    assert_eq!(saturating_translate(&r_max, &Point::of(20.0, 0.0)), Rect::of(10.0, 0.0, MAX, MAX - 10.0));
    assert_eq!(saturating_translate(&r_max, &Point::of(0.0, 20.0)), Rect::of(0.0, 10.0, MAX - 10.0, MAX));
}

#[test]
fn limits_out_of_bounds() {
    let r = Rect::largest();
    assert_eq!(saturating_translate(&r, &Point::of(MIN, 0.0)), Rect::of(MIN, MIN, -1.0, -1.0));
    assert_eq!(saturating_translate(&r, &Point::of(0.0, MIN)), Rect::of(MIN, MIN, -1.0, -1.0));
    assert_eq!(saturating_translate(&r, &Point::of(MAX, 0.0)), Rect::of(-1.0, MIN, MAX - 1.0, -1.0));
    assert_eq!(saturating_translate(&r, &Point::of(0.0, MAX)), Rect::of(MIN, -1.0, -1.0, MAX - 1.0));
}
