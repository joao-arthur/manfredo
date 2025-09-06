use super::wrapping_add_assign;
use crate::cartesian::{
    point::point_f32::{MAX, MIN},
    rect::rect_f32::RectF32,
};

#[test]
fn test() {
    let mut r = RectF32::of(0.0, 0.0, 12.0, 15.0);
    wrapping_add_assign(&mut r, &RectF32::of(5.0, 4.0, 3.0, 2.0));
    assert_eq!(r, RectF32::of(5.0, 4.0, 15.0, 17.0));
    wrapping_add_assign(&mut r, &RectF32::of(-14.0, -13.0, -12.0, -11.0));
    assert_eq!(r, RectF32::of(-9.0, -9.0, 3.0, 6.0));
}

#[test]
fn small_rect_to_bounds() {
    let mut min_r = RectF32::of(MIN + 2.0, MIN + 5.0, MIN + 12.0, MIN + 15.0);
    wrapping_add_assign(&mut min_r, &RectF32::of(-2.0, -5.0, 9.0, 7.0));
    assert_eq!(min_r, RectF32::of(MIN, MIN, MIN + 21.0, MIN + 22.0));

    let mut max_r = RectF32::of(MAX - 12.0, MAX - 15.0, MAX - 2.0, MAX - 5.0);
    wrapping_add_assign(&mut max_r, &RectF32::of(-9.0, -7.0, 2.0, 5.0));
    assert_eq!(max_r, RectF32::of(MAX - 21.0, MAX - 22.0, MAX, MAX));
}

#[test]
fn big_rect_to_bounds() {
    let mut r = RectF32::of(MIN + 2.0, MIN + 5.0, MAX - 2.0, MAX - 5.0);
    wrapping_add_assign(&mut r, &RectF32::of(-2.0, -5.0, 2.0, 5.0));
    assert_eq!(r, RectF32::largest());

    let mut min_r = RectF32::of(MIN + 2.0, MIN + 5.0, MAX, MAX);
    wrapping_add_assign(&mut min_r, &RectF32::of(-2.0, -5.0, 0.0, 0.0));
    assert_eq!(min_r, RectF32::largest());

    let mut max_r = RectF32::of(MIN, MIN, MAX - 2.0, MAX - 5.0);
    wrapping_add_assign(&mut max_r, &RectF32::of(0.0, 0.0, 2.0, 5.0));
    assert_eq!(max_r, RectF32::largest());
}

#[test]
fn small_rect_out_of_bounds() {
    let mut r_min = RectF32::of(MIN + 10.0, MIN + 5.0, MIN + 20.0, MIN + 30.0);
    wrapping_add_assign(&mut r_min, &RectF32::of(-20.0, -20.0, 0.0, 0.0));
    assert_eq!(r_min, RectF32::of(MAX - 9.0, MAX - 14.0, MIN + 20.0, MIN + 30.0));

    let mut r_max = RectF32::of(MAX - 20.0, MAX - 30.0, MAX - 5.0, MAX - 10.0);
    wrapping_add_assign(&mut r_max, &RectF32::of(0.0, 0.0, 20.0, 20.0));
    assert_eq!(r_max, RectF32::of(MAX - 20.0, MAX - 30.0, MIN + 14.0, MIN + 9.0));
}

#[test]
fn big_rect_out_of_bounds() {
    let mut r_min = RectF32::of(MIN + 10.0, MIN + 5.0, MAX, MAX);
    wrapping_add_assign(&mut r_min, &RectF32::of(-20.0, -20.0, 0.0, 0.0));
    assert_eq!(r_min, RectF32::of(MAX - 9.0, MAX - 14.0, MAX, MAX));

    let mut r_max = RectF32::of(MIN, MIN, MAX - 5.0, MAX - 10.0);
    wrapping_add_assign(&mut r_max, &RectF32::of(0.0, 0.0, 20.0, 20.0));
    assert_eq!(r_max, RectF32::of(MIN, MIN, MIN + 14.0, MIN + 9.0));
}

#[test]
fn small_rect_limits_out_of_bounds() {
    let mut r_min = RectF32::of(MIN + 1.0, MIN + 1.0, MIN + 10.0, MIN + 10.0);
    wrapping_add_assign(&mut r_min, &RectF32::min());
    assert_eq!(r_min, RectF32::of(1.0, 1.0, 10.0, 10.0));

    let mut r_max = RectF32::of(MAX - 10.0, MAX - 10.0, MAX - 1.0, MAX - 1.0);
    wrapping_add_assign(&mut r_max, &RectF32::max());
    assert_eq!(r_max, RectF32::of(-12.0, -12.0, -3.0, -3.0));
}

#[test]
fn big_rect_limits_out_of_bounds() {
    let mut r1 = RectF32::largest();
    wrapping_add_assign(&mut r1, &RectF32::min());
    assert_eq!(r1, RectF32::of(0.0, 0.0, -1.0, -1.0));

    let mut r2 = RectF32::largest();
    wrapping_add_assign(&mut r2, &RectF32::max());
    assert_eq!(r2, RectF32::of(-1.0, -1.0, -2.0, -2.0));

    let mut r_min = RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX);
    wrapping_add_assign(&mut r_min, &RectF32::min());
    assert_eq!(r_min, RectF32::of(1.0, 1.0, -1.0, -1.0));

    let mut r_max = RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0);
    wrapping_add_assign(&mut r_max, &RectF32::max());
    assert_eq!(r_max, RectF32::of(-1.0, -1.0, -3.0, -3.0));
}
