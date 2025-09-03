use super::try_checked_translate;
use crate::matrix::{point::point_i32::PointI32, rect::rect_u32::RectU32};

#[test]
fn test_try_checked_translate() {
    assert_eq!(try_checked_translate(&RectU32::of(0, 0, 12, 15), &PointI32::of(5, 4)), Some(RectU32::of(5, 4, 17, 19)));
    assert_eq!(try_checked_translate(&RectU32::of(5, 4, 17, 19), &PointI32::of(-4, -2)), Some(RectU32::of(1, 2, 13, 17)));
}

#[test]
fn try_checked_translate_to_bounds() {
    assert_eq!(
        try_checked_translate(&RectU32::of(2, 5, u32::MAX, u32::MAX), &PointI32::of(-2, -5)),
        Some(RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5))
    );
    assert_eq!(
        try_checked_translate(&RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5), &PointI32::of(2, 5)),
        Some(RectU32::of(2, 5, u32::MAX, u32::MAX))
    );
}

#[test]
fn try_checked_translate_out_of_bounds() {
    let r = RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 10);
    assert_eq!(try_checked_translate(&r, &PointI32::of(-20, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI32::of(0, -20)), None);
    assert_eq!(try_checked_translate(&r, &PointI32::of(20, 20)), None);
    assert_eq!(try_checked_translate(&r, &PointI32::of(20, 20)), None);
}

#[test]
fn try_checked_translate_limits_out_of_bounds() {
    let r = RectU32::largest();
    assert_eq!(try_checked_translate(&r, &PointI32::of(i32::MIN, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI32::of(0, i32::MIN)), None);
    assert_eq!(try_checked_translate(&r, &PointI32::of(i32::MAX, 0)), None);
    assert_eq!(try_checked_translate(&r, &PointI32::of(0, i32::MAX)), None);
}
