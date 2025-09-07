use super::try_checked_add;
use crate::cartesian::{
    point::point_f32::{MAX, MIN},
    rect::rect_f32::RectF32,
};

#[test]
fn test() {
    assert_eq!(try_checked_add(&RectF32::of(-7.0, 9.0, -12.0, 15.0), &RectF32::of(5.0, 4.0, 3.0, 2.0)), Some(RectF32::of(-2.0, 13.0, -9.0, 17.0)));
    assert_eq!(try_checked_add(&RectF32::of(-2.0, 13.0, -9.0, 17.0), &RectF32::of(9.0, -10.0, 11.0, -12.0)), Some(RectF32::of(7.0, 3.0, 2.0, 5.0)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&RectF32::of(MIN + 2.0, MIN + 5.0, MAX - 2.0, MAX - 5.0), &RectF32::of(-2.0, -5.0, 2.0, 5.0)), Some(RectF32::largest()));
    assert_eq!(try_checked_add(&RectF32::of(MIN + 2.0, MIN + 5.0, MAX, MAX), &RectF32::of(-2.0, -5.0, 0.0, 0.0)), Some(RectF32::largest()));
    assert_eq!(try_checked_add(&RectF32::of(MIN, MIN, MAX - 2.0, MAX - 5.0), &RectF32::of(0.0, 0.0, 2.0, 5.0)), Some(RectF32::largest()));
}

#[test]
fn out_of_bounds() {
    let r = RectF32::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    assert_eq!(try_checked_add(&r, &RectF32::of(-20.0, 0.0, 0.0, 0.0)), None);
    assert_eq!(try_checked_add(&r, &RectF32::of(0.0, -20.0, 0.0, 0.0)), None);
    assert_eq!(try_checked_add(&r, &RectF32::of(0.0, 0.0, 20.0, 0.0)), None);
    assert_eq!(try_checked_add(&r, &RectF32::of(0.0, 0.0, 0.0, 20.0)), None);
}

#[test]
fn edge_out_of_bounds() {
    let r = RectF32::largest();
    assert_eq!(try_checked_add(&r, &RectF32::of(-1.0, 0.0, 0.0, 0.0)), None);
    assert_eq!(try_checked_add(&r, &RectF32::of(0.0, -1.0, 0.0, 0.0)), None);
    assert_eq!(try_checked_add(&r, &RectF32::of(0.0, 0.0, 1.0, 0.0)), None);
    assert_eq!(try_checked_add(&r, &RectF32::of(0.0, 0.0, 0.0, 1.0)), None);
}

#[test]
fn limits_out_of_bounds() {
    let r = RectF32::largest();
    assert_eq!(try_checked_add(&r, &RectF32::of(MIN, 0.0, 0.0, 0.0)), None);
    assert_eq!(try_checked_add(&r, &RectF32::of(0.0, MIN, 0.0, 0.0)), None);
    assert_eq!(try_checked_add(&r, &RectF32::of(0.0, 0.0, MAX, 0.0)), None);
    assert_eq!(try_checked_add(&r, &RectF32::of(0.0, 0.0, 0.0, MAX)), None);
}
