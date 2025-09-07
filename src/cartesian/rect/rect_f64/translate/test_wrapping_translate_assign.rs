use super::wrapping_translate_assign;
use crate::cartesian::{
    point::point_f64::{MAX, MIN, PointF64},
    rect::rect_f64::RectF64,
};

#[test]
fn test() {
    let mut r = RectF64::of(0.0, 0.0, 12.0, 15.0);
    wrapping_translate_assign(&mut r, &PointF64::of(5.0, 4.0));
    assert_eq!(r, RectF64::of(5.0, 4.0, 17.0, 19.0));
    wrapping_translate_assign(&mut r, &PointF64::of(-4.0, -2.0));
    assert_eq!(r, RectF64::of(1.0, 2.0, 13.0, 17.0));
}

#[test]
fn to_bounds() {
    let mut r_min = RectF64::of(MIN + 2.0, MIN + 5.0, MAX, MAX);
    wrapping_translate_assign(&mut r_min, &PointF64::of(-2.0, -5.0));
    assert_eq!(r_min, RectF64::of(MIN, MIN, MAX - 2.0, MAX - 5.0));

    let mut r_max = RectF64::of(MIN, MIN, MAX - 2.0, MAX - 5.0);
    wrapping_translate_assign(&mut r_max, &PointF64::of(2.0, 5.0));
    assert_eq!(r_max, RectF64::of(MIN + 2.0, MIN + 5.0, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r1 = RectF64::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    wrapping_translate_assign(&mut r1, &PointF64::of(-20.0, 0.0));
    assert_eq!(r1, RectF64::of(MAX - 9.0, MIN + 10.0, MAX - 30.0, MAX - 10.0));

    let mut r2 = RectF64::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    wrapping_translate_assign(&mut r2, &PointF64::of(0.0, -20.0));
    assert_eq!(r2, RectF64::of(MIN + 10.0, MAX - 9.0, MAX - 10.0, MAX - 30.0));

    let mut r3 = RectF64::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    wrapping_translate_assign(&mut r3, &PointF64::of(20.0, 0.0));
    assert_eq!(r3, RectF64::of(MIN + 30.0, MIN + 10.0, MIN + 9.0, MAX - 10.0));

    let mut r4 = RectF64::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 10.0);
    wrapping_translate_assign(&mut r4, &PointF64::of(0.0, 20.0));
    assert_eq!(r4, RectF64::of(MIN + 10.0, MIN + 30.0, MAX - 10.0, MIN + 9.0));

    let mut r_min = RectF64::of(MIN, MIN, MIN + 10.0, MIN + 10.0);
    wrapping_translate_assign(&mut r_min, &PointF64::of(-20.0, -20.0));
    assert_eq!(r_min, RectF64::of(MAX - 19.0, MAX - 19.0, MAX - 9.0, MAX - 9.0));

    let mut r_max = RectF64::of(MAX, MAX, MAX - 10.0, MAX - 10.0);
    wrapping_translate_assign(&mut r_max, &PointF64::of(20.0, 20.0));
    assert_eq!(r_max, RectF64::of(MIN + 19.0, MIN + 19.0, MIN + 9.0, MIN + 9.0));
}

#[test]
fn edge_out_of_bounds() {
    let mut r1 = RectF64::largest();
    wrapping_translate_assign(&mut r1, &PointF64::of(-1.0, 0.0));
    assert_eq!(r1, RectF64::of(MAX, MIN, MAX - 1.0, MAX));

    let mut r2 = RectF64::largest();
    wrapping_translate_assign(&mut r2, &PointF64::of(0.0, -1.0));
    assert_eq!(r2, RectF64::of(MIN, MAX, MAX, MAX - 1.0));

    let mut r3 = RectF64::largest();
    wrapping_translate_assign(&mut r3, &PointF64::of(1.0, 0.0));
    assert_eq!(r3, RectF64::of(MIN + 1.0, MIN, MIN, MAX));

    let mut r4 = RectF64::largest();
    wrapping_translate_assign(&mut r4, &PointF64::of(0.0, 1.0));
    assert_eq!(r4, RectF64::of(MIN, MIN + 1.0, MAX, MIN));
}

#[test]
fn limits_out_of_bounds() {
    let mut r1 = RectF64::largest();
    wrapping_translate_assign(&mut r1, &PointF64::of(MIN, 0.0));
    assert_eq!(r1, RectF64::of(0.0, MIN, -1.0, MAX));

    let mut r2 = RectF64::largest();
    wrapping_translate_assign(&mut r2, &PointF64::of(0.0, MIN));
    assert_eq!(r2, RectF64::of(MIN, 0.0, MAX, -1.0));

    let mut r3 = RectF64::largest();
    wrapping_translate_assign(&mut r3, &PointF64::of(MAX, 0.0));
    assert_eq!(r3, RectF64::of(-1.0, MIN, -2.0, MAX));

    let mut r4 = RectF64::largest();
    wrapping_translate_assign(&mut r4, &PointF64::of(0.0, MAX));
    assert_eq!(r4, RectF64::of(MIN, -1.0, MAX, -2.0));
}
