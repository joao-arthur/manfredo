use super::try_checked_translate_assign;
use crate::cartesian::{point::point_i32::PointI32, rect::rect_i32::RectI32};

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

#[test]
fn test() {
    let mut r = RectI32::of(5, 9, 13, 37);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(-10, -20)), Some(()));
    assert_eq!(r, RectI32::of(-5, -11, 3, 17));
    assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(6, -19)), Some(()));
    assert_eq!(r, RectI32::of(1, -30, 9, -2));
}

#[test]
fn to_bounds() {
    let mut r_min = RectI32::of(MIN + 2, MIN + 5, MAX, MAX);
    assert_eq!(try_checked_translate_assign(&mut r_min, &PointI32::of(-2, -5)), Some(()));
    assert_eq!(r_min, RectI32::of(MIN, MIN, MAX - 2, MAX - 5));

    let mut r_max = RectI32::of(MIN, MIN, MAX - 2, MAX - 5);
    assert_eq!(try_checked_translate_assign(&mut r_max, &PointI32::of(2, 5)), Some(()));
    assert_eq!(r_max, RectI32::of(MIN + 2, MIN + 5, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = RectI32::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(-20, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(0, -20)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(20, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(0, 20)), None);
    assert_eq!(r, RectI32::of(MIN + 10, MIN + 10, MAX - 10, MAX - 10));
}

#[test]
fn limits_out_of_bounds() {
    let mut r = RectI32::largest();
    assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(MIN, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(0, MIN)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(MAX, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(0, MAX)), None);
    assert_eq!(r, RectI32::largest());
}
