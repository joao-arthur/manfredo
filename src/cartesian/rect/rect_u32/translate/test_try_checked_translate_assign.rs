use super::try_checked_translate_assign;
use crate::cartesian::{point::point_i32::PointI32, rect::rect_u32::Rect};

const MAX: u32 = u32::MAX;

#[test]
fn test() {
    let mut r = Rect::of(0, 0, 12, 15);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(5, 4)), Some(()));
    assert_eq!(r, Rect::of(5, 4, 17, 19));
    assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(-4, -2)), Some(()));
    assert_eq!(r, Rect::of(1, 2, 13, 17));
}

#[test]
fn to_bounds() {
    let mut r_min = Rect::of(2, 5, MAX, MAX);
    assert_eq!(try_checked_translate_assign(&mut r_min, &PointI32::of(-2, -5)), Some(()));
    assert_eq!(r_min, Rect::of(0, 0, MAX - 2, MAX - 5));

    let mut r_max = Rect::of(0, 0, MAX - 2, MAX - 5);
    assert_eq!(try_checked_translate_assign(&mut r_max, &PointI32::of(2, 5)), Some(()));
    assert_eq!(r_max, Rect::of(2, 5, MAX, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = Rect::of(10, 10, MAX - 10, MAX - 10);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(-20, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(0, -20)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(20, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(0, 20)), None);
    assert_eq!(r, Rect::of(10, 10, MAX - 10, MAX - 10));
}

#[test]
fn limits_out_of_bounds() {
    let mut r = Rect::largest();
    assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(i32::MIN, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(0, i32::MIN)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(i32::MAX, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI32::of(0, i32::MAX)), None);
    assert_eq!(r, Rect::largest());
}
