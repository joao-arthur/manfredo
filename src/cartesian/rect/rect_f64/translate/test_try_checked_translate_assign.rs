use super::try_checked_translate_assign;
use crate::cartesian::{
    point::point_f64::{MAX, MIN, PointF64},
    rect::rect_f64::RectF64,
};

#[test]
fn test() {
    let mut r = RectF64::of(0.0, 0.0, 10.0, 10.0);
    assert_eq!(try_checked_translate_assign(&mut r, &PointF64::of(10.0, 20.0)), Some(()));
    assert_eq!(r, RectF64::of(10.0, 20.0, 20.0, 30.0));
    assert_eq!(try_checked_translate_assign(&mut r, &PointF64::of(-20.0, -15.0)), Some(()));
    assert_eq!(r, RectF64::of(-10.0, 5.0, 0.0, 15.0));
}

#[test]
fn to_bounds() {
    let mut r_min = RectF64::of(MIN + 2.0, MIN + 5.0, MAX, MAX);
    assert_eq!(try_checked_translate_assign(&mut r_min, &PointF64::of(-2.0, -5.0)), Some(()));
    assert_eq!(r_min, RectF64::of(MIN, MIN, MAX - 2.0, MAX - 5.0));

    let mut r_max = RectF64::of(MIN, MIN, MAX - 2.0, MAX - 5.0);
    assert_eq!(try_checked_translate_assign(&mut r_max, &PointF64::of(2.0, 5.0)), Some(()));
    assert_eq!(r_max, RectF64::of(MIN + 2.0, MIN + 5.0, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = RectF64::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    assert_eq!(try_checked_translate_assign(&mut r, &PointF64::of(-20.0, 0.0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointF64::of(0.0, -20.0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointF64::of(20.0, 0.0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointF64::of(0.0, 20.0)), None);
    assert_eq!(r, RectF64::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0));
}

#[test]
fn limits_out_of_bounds() {
    let mut r = RectF64::largest();
    assert_eq!(try_checked_translate_assign(&mut r, &PointF64::of(MIN, 0.0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointF64::of(0.0, MIN)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointF64::of(MAX, 0.0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointF64::of(0.0, MAX)), None);
    assert_eq!(r, RectF64::largest());
}
