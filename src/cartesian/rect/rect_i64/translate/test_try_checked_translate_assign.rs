use super::try_checked_translate_assign;
use crate::cartesian::{point::point_i64::PointI64, rect::rect_i64::RectI64};

#[test]
fn test_try_checked_translate_assign() {
    let mut r = RectI64::of(0, 0, 10, 10);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(10, 20)), Some(()));
    assert_eq!(r, RectI64::of(10, 20, 20, 30));
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::of(-20, -15)), Some(()));
    assert_eq!(r, RectI64::of(-10, 5, 0, 15));
}

#[test]
fn try_checked_translate_assign_small_rect_to_bounds() {
    let mut min_r = RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MIN + 12, i64::MIN + 15);
    assert_eq!(try_checked_translate_assign(&mut min_r, &PointI64::of(-2, -5)), Some(()));
    assert_eq!(min_r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10));

    let mut max_r = RectI64::of(i64::MAX - 12, i64::MAX - 15, i64::MAX - 2, i64::MAX - 5);
    assert_eq!(try_checked_translate_assign(&mut max_r, &PointI64::of(2, 5)), Some(()));
    assert_eq!(max_r, RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX, i64::MAX));
}

#[test]
fn try_checked_translate_assign_big_rect_to_bounds() {
    let mut min_r = RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX);
    assert_eq!(try_checked_translate_assign(&mut min_r, &PointI64::of(-2, -5)), Some(()));
    assert_eq!(min_r, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5));

    let mut max_r = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 5);
    assert_eq!(try_checked_translate_assign(&mut max_r, &PointI64::of(2, 5)), Some(()));
    assert_eq!(max_r, RectI64::of(i64::MIN + 2, i64::MIN + 5, i64::MAX, i64::MAX));
}

#[test]
fn try_checked_translate_assign_small_rect_out_of_bounds() {
    let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MIN + 20, i64::MIN + 30);
    assert_eq!(try_checked_translate_assign(&mut r_min, &PointI64::of(-20, -20)), None);
    assert_eq!(r_min, RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MIN + 20, i64::MIN + 30));

    let mut r_max = RectI64::of(i64::MAX - 20, i64::MAX - 30, i64::MAX - 5, i64::MAX - 10);
    assert_eq!(try_checked_translate_assign(&mut r_max, &PointI64::of(20, 20)), None);
    assert_eq!(r_max, RectI64::of(i64::MAX - 20, i64::MAX - 30, i64::MAX - 5, i64::MAX - 10));
}

#[test]
fn try_checked_translate_assign_big_rect_out_of_bounds() {
    let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MAX, i64::MAX);
    assert_eq!(try_checked_translate_assign(&mut r_min, &PointI64::of(-20, -20)), None);
    assert_eq!(r_min, RectI64::of(i64::MIN + 10, i64::MIN + 5, i64::MAX, i64::MAX));

    let mut r_max = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 5, i64::MAX - 10);
    assert_eq!(try_checked_translate_assign(&mut r_max, &PointI64::of(20, 20)), None);
    assert_eq!(r_max, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 5, i64::MAX - 10));
}

#[test]
fn try_checked_translate_assign_small_rect_limits_out_of_bounds() {
    let mut r_min = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MIN + 10, i64::MIN + 10);
    assert_eq!(try_checked_translate_assign(&mut r_min, &PointI64::min()), None);
    assert_eq!(try_checked_translate_assign(&mut r_min, &PointI64::of(i64::MIN, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r_min, &PointI64::of(0, i64::MIN)), None);
    assert_eq!(r_min, RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MIN + 10, i64::MIN + 10));

    let mut r_max = RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX - 1, i64::MAX - 1);
    assert_eq!(try_checked_translate_assign(&mut r_max, &PointI64::max()), None);
    assert_eq!(try_checked_translate_assign(&mut r_max, &PointI64::of(i64::MAX, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r_max, &PointI64::of(0, i64::MAX)), None);
    assert_eq!(r_max, RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX - 1, i64::MAX - 1));
}

#[test]
fn try_checked_translate_assign_big_rect_limits_out_of_bounds() {
    let mut r = RectI64::largest();
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::min()), None);
    assert_eq!(try_checked_translate_assign(&mut r, &PointI64::max()), None);
    assert_eq!(r, RectI64::largest());

    let mut r_min = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1);
    assert_eq!(try_checked_translate_assign(&mut r_min, &PointI64::max()), None);
    assert_eq!(try_checked_translate_assign(&mut r_min, &PointI64::of(i64::MAX, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r_min, &PointI64::of(0, i64::MAX)), None);
    assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1));

    let mut r_max = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX);
    assert_eq!(try_checked_translate_assign(&mut r_max, &PointI64::min()), None);
    assert_eq!(try_checked_translate_assign(&mut r_max, &PointI64::of(i64::MIN, 0)), None);
    assert_eq!(try_checked_translate_assign(&mut r_max, &PointI64::of(0, i64::MIN)), None);
    assert_eq!(r_max, RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX));
}
