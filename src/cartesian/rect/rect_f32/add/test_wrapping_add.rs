use super::wrapping_add;
use crate::cartesian::{
    point::point_f32::{MAX, MIN},
    rect::rect_f32::RectF32,
};

#[test]
fn test_wrapping_add() {
    assert_eq!(wrapping_add(&RectF32::of(0.0, 0.0, 12.0, 15.0), &RectF32::of(5.0, 4.0, 3.0, 2.0)), RectF32::of(5.0, 4.0, 15.0, 17.0));
    assert_eq!(wrapping_add(&RectF32::of(5.0, 4.0, 15.0, 17.0), &RectF32::of(-14.0, -13.0, -12.0, -11.0)), RectF32::of(-9.0, -9.0, 3.0, 6.0));
}

#[test]
fn wrapping_add_small_rect_to_bounds() {
    assert_eq!(wrapping_add(&RectF32::of(MIN + 2.0, MIN + 5.0, MIN + 12.0, MIN + 15.0), &RectF32::of(-2.0, -5.0, 9.0, 7.0)), RectF32::of(MIN, MIN, MIN + 21.0, MIN + 22.0));
    assert_eq!(wrapping_add(&RectF32::of(MAX - 12.0, MAX - 15.0, MAX - 2.0, MAX - 5.0), &RectF32::of(-9.0, -7.0, 2.0, 5.0)), RectF32::of(MAX - 21.0, MAX - 22.0, MAX, MAX));
}

#[test]
fn wrapping_add_big_rect_to_bounds() {
    assert_eq!(wrapping_add(&RectF32::of(MIN + 2.0, MIN + 5.0, MAX - 2.0, MAX - 5.0), &RectF32::of(-2.0, -5.0, 2.0, 5.0)), RectF32::largest());
    assert_eq!(wrapping_add(&RectF32::of(MIN + 2.0, MIN + 5.0, MAX, MAX), &RectF32::of(-2.0, -5.0, 0.0, 0.0)), RectF32::largest());
    assert_eq!(wrapping_add(&RectF32::of(MIN, MIN, MAX - 2.0, MAX - 5.0), &RectF32::of(0.0, 0.0, 2.0, 5.0)), RectF32::largest());
}

#[test]
fn wrapping_add_small_rect_out_of_bounds() {
    assert_eq!(wrapping_add(&RectF32::of(MIN + 10.0, MIN + 5.0, MIN + 20.0, MIN + 30.0), &RectF32::of(-20.0, -20.0, 0.0, 0.0)), RectF32::of(MAX - 9.0, MAX - 14.0, MIN + 20.0, MIN + 30.0));
    assert_eq!(wrapping_add(&RectF32::of(MAX - 20.0, MAX - 30.0, MAX - 5.0, MAX - 10.0), &RectF32::of(0.0, 0.0, 20.0, 20.0)), RectF32::of(MAX - 20.0, MAX - 30.0, MIN + 14.0, MIN + 9.0));
}

#[test]
fn wrapping_add_big_rect_out_of_bounds() {
    assert_eq!(wrapping_add(&RectF32::of(MIN + 10.0, MIN + 5.0, MAX, MAX), &RectF32::of(-20.0, -20.0, 0.0, 0.0)), RectF32::of(MAX - 9.0, MAX - 14.0, MAX, MAX));
    assert_eq!(wrapping_add(&RectF32::of(MIN, MIN, MAX - 5.0, MAX - 10.0), &RectF32::of(0.0, 0.0, 20.0, 20.0)), RectF32::of(MIN, MIN, MIN + 14.0, MIN + 9.0));
}

#[test]
fn wrapping_add_small_rect_limits_out_of_bounds() {
    assert_eq!(wrapping_add(&RectF32::of(MIN + 1.0, MIN + 1.0, MIN + 10.0, MIN + 10.0), &RectF32::min()), RectF32::of(1.0, 1.0, 10.0, 10.0));
    assert_eq!(wrapping_add(&RectF32::of(MAX - 10.0, MAX - 10.0, MAX - 1.0, MAX - 1.0), &RectF32::max()), RectF32::of(-12.0, -12.0, -3.0, -3.0));
}

#[test]
fn wrapping_add_big_rect_limits_out_of_bounds() {
    assert_eq!(wrapping_add(&RectF32::largest(), &RectF32::min()), RectF32::of(0.0, 0.0, -1.0, -1.0));
    assert_eq!(wrapping_add(&RectF32::largest(), &RectF32::max()), RectF32::of(-1.0, -1.0, -2.0, -2.0));
    assert_eq!(wrapping_add(&RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX), &RectF32::min()), RectF32::of(1.0, 1.0, -1.0, -1.0));
    assert_eq!(wrapping_add(&RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0), &RectF32::max()), RectF32::of(-1.0, -1.0, -3.0, -3.0));
}
