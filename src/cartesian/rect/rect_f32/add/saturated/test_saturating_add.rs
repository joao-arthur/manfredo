use super::saturating_add;
use crate::cartesian::{
    point::point_f32::{MAX, MIN},
    rect::rect_f32::RectF32,
};

#[test]
fn test_saturating_add() {
    assert_eq!(saturating_add(&RectF32::of(0.0, 0.0, 12.0, 15.0), &RectF32::of(5.0, 4.0, 3.0, 2.0)), RectF32::of(5.0, 4.0, 15.0, 17.0));
    assert_eq!(saturating_add(&RectF32::of(5.0, 4.0, 15.0, 17.0), &RectF32::of(-14.0, -13.0, -12.0, -11.0)), RectF32::of(-9.0, -9.0, 3.0, 6.0));
}

#[test]
fn saturating_add_small_rect_to_bounds() {
    assert_eq!(
        saturating_add(&RectF32::of(MIN + 2.0, MIN + 5.0, MIN + 12.0, MIN + 15.0), &RectF32::of(-2.0, -5.0, 9.0, 7.0)),
        RectF32::of(MIN, MIN, MIN + 21.0, MIN + 22.0)
    );
    assert_eq!(
        saturating_add(&RectF32::of(MAX - 12.0, MAX - 15.0, MAX - 2.0, MAX - 5.0), &RectF32::of(-9.0, -7.0, 2.0, 5.0)),
        RectF32::of(MAX - 21.0, MAX - 22.0, MAX, MAX)
    );
}

#[test]
fn saturating_add_big_rect_to_bounds() {
    assert_eq!(saturating_add(&RectF32::of(MIN + 2.0, MIN + 5.0, MAX - 2.0, MAX - 5.0), &RectF32::of(-2.0, -5.0, 2.0, 5.0)), RectF32::largest());
    assert_eq!(saturating_add(&RectF32::of(MIN + 2.0, MIN + 5.0, MAX, MAX), &RectF32::of(-2.0, -5.0, 0.0, 0.0)), RectF32::largest());
    assert_eq!(saturating_add(&RectF32::of(MIN, MIN, MAX - 2.0, MAX - 5.0), &RectF32::of(0.0, 0.0, 2.0, 5.0)), RectF32::largest());
}

#[test]
fn saturating_add_small_rect_out_of_bounds() {
    assert_eq!(
        saturating_add(&RectF32::of(MIN + 10.0, MIN + 5.0, MIN + 20.0, MIN + 30.0), &RectF32::of(-20.0, -20.0, 0.0, 0.0)),
        RectF32::of(MIN, MIN, MIN + 20.0, MIN + 30.0)
    );
    assert_eq!(
        saturating_add(&RectF32::of(MAX - 20.0, MAX - 30.0, MAX - 5.0, MAX - 10.0), &RectF32::of(0.0, 0.0, 20.0, 20.0)),
        RectF32::of(MAX - 20.0, MAX - 30.0, MAX, MAX)
    );
}

#[test]
fn saturating_add_big_rect_out_of_bounds() {
    assert_eq!(saturating_add(&RectF32::of(MIN + 10.0, MIN + 5.0, MAX, MAX), &RectF32::of(-20.0, -20.0, 0.0, 0.0)), RectF32::largest());
    assert_eq!(saturating_add(&RectF32::of(MIN, MIN, MAX - 5.0, MAX - 10.0), &RectF32::of(0.0, 0.0, 20.0, 20.0)), RectF32::largest());
}

#[test]
fn saturating_add_small_rect_limits_out_of_bounds() {
    assert_eq!(saturating_add(&RectF32::of(MIN + 1.0, MIN + 1.0, MIN + 10.0, MIN + 10.0), &RectF32::min()), RectF32::min());
    assert_eq!(saturating_add(&RectF32::of(MAX - 10.0, MAX - 10.0, MAX - 1.0, MAX - 1.0), &RectF32::max()), RectF32::max());
}

#[test]
fn saturating_add_big_rect_limits_out_of_bounds() {
    assert_eq!(saturating_add(&RectF32::largest(), &RectF32::largest()), RectF32::largest());

    let r_large = RectF32::of(MIN + 1.0, MIN + 1.0, MAX - 1.0, MAX - 1.0);
    assert_eq!(saturating_add(&r_large, &RectF32::largest()), RectF32::largest());
    assert_eq!(saturating_add(&r_large, &RectF32::of(MIN, 0.0, 0.0, 0.0)), RectF32::of(MIN, MIN + 1.0, MAX - 1.0, MAX - 1.0));
    assert_eq!(saturating_add(&r_large, &RectF32::of(0.0, MIN, 0.0, 0.0)), RectF32::of(MIN + 1.0, MIN, MAX - 1.0, MAX - 1.0));
    assert_eq!(saturating_add(&r_large, &RectF32::of(0.0, 0.0, MAX, 0.0)), RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX - 1.0));
    assert_eq!(saturating_add(&r_large, &RectF32::of(0.0, 0.0, 0.0, MAX)), RectF32::of(MIN + 1.0, MIN + 1.0, MAX - 1.0, MAX));
}
