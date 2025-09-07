use super::try_saturating_resize;
use crate::cartesian::rect::rect_u16::RectU16;

const MAX: u16 = u16::MAX;

#[test]
fn odd() {
    assert_eq!(try_saturating_resize(&RectU16::of(5, 5, 15, 15), 9), Some(RectU16::of(6, 6, 14, 14)));
    assert_eq!(try_saturating_resize(&RectU16::of(6, 6, 14, 14), 7), Some(RectU16::of(7, 7, 13, 13)));
    assert_eq!(try_saturating_resize(&RectU16::of(7, 7, 13, 13), 5), Some(RectU16::of(8, 8, 12, 12)));
    assert_eq!(try_saturating_resize(&RectU16::of(8, 8, 12, 12), 3), Some(RectU16::of(9, 9, 11, 11)));
    assert_eq!(try_saturating_resize(&RectU16::of(9, 9, 11, 11), 9), Some(RectU16::of(6, 6, 14, 14)));
}

#[test]
fn even() {
    assert_eq!(try_saturating_resize(&RectU16::of(5, 5, 14, 14), 10), Some(RectU16::of(5, 5, 14, 14)));
    assert_eq!(try_saturating_resize(&RectU16::of(5, 5, 14, 14), 8), Some(RectU16::of(6, 6, 13, 13)));
    assert_eq!(try_saturating_resize(&RectU16::of(6, 6, 13, 13), 6), Some(RectU16::of(7, 7, 12, 12)));
    assert_eq!(try_saturating_resize(&RectU16::of(7, 7, 12, 12), 4), Some(RectU16::of(8, 8, 11, 11)));
    assert_eq!(try_saturating_resize(&RectU16::of(8, 8, 11, 11), 8), Some(RectU16::of(6, 6, 13, 13)));
}

#[test]
fn small_size() {
    let r = RectU16::of(10, 10, 20, 20);
    assert_eq!(try_saturating_resize(&r, 0), None);
    assert_eq!(try_saturating_resize(&r, 1), None);
    assert_eq!(try_saturating_resize(&r, 2), None);
}

#[test]
fn same_size() {
    assert_eq!(try_saturating_resize(&RectU16::of(0, 0, 2, 2), 3), Some(RectU16::of(0, 0, 2, 2)));
    assert_eq!(try_saturating_resize(&RectU16::of(0, 0, 3, 3), 4), Some(RectU16::of(0, 0, 3, 3)));
    assert_eq!(try_saturating_resize(&RectU16::of(MAX - 2, MAX - 2, MAX, MAX), 3), Some(RectU16::of(MAX - 2, MAX - 2, MAX, MAX)));
    assert_eq!(try_saturating_resize(&RectU16::of(MAX - 3, MAX - 3, MAX, MAX), 4), Some(RectU16::of(MAX - 3, MAX - 3, MAX, MAX)));
}

#[test]
fn bounds() {
    assert_eq!(try_saturating_resize(&RectU16::of(0, 0, 2, 2), 11), Some(RectU16::of(0, 0, 10, 10)));
    assert_eq!(try_saturating_resize(&RectU16::of(MAX - 2, MAX - 2, MAX, MAX), 11), Some(RectU16::of(MAX - 10, MAX - 10, MAX, MAX)));
}

#[test]
fn small_rect_limits() {
    assert_eq!(try_saturating_resize(&RectU16::of(0, 0, 2, 2), MAX), Some(RectU16::of(0, 0, MAX - 1, MAX - 1)));
    assert_eq!(try_saturating_resize(&RectU16::of(MAX - 2, MAX - 2, MAX, MAX), MAX), Some(RectU16::of(1, 1, MAX, MAX)));
}

#[test]
fn big_rect_limits() {
    assert_eq!(try_saturating_resize(&RectU16::of(0, 0, MAX - 1, MAX - 1), MAX), Some(RectU16::of(0, 0, MAX - 1, MAX - 1)));
    assert_eq!(try_saturating_resize(&RectU16::of(1, 1, MAX, MAX), MAX), Some(RectU16::of(1, 1, MAX, MAX)));
    assert_eq!(try_saturating_resize(&RectU16::largest(), MAX), Some(RectU16::of(0, 0, MAX - 1, MAX - 1)));
}
