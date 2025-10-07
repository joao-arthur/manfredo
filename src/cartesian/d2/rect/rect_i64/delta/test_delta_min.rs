use super::delta_min;
use crate::cartesian::d2::rect::rect_i64::Rect;

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn test_delta_min() {
    assert_eq!(delta_min(&Rect::of(0, -5, 5, 5)), 5);
    assert_eq!(delta_min(&Rect::of(-5, 0, 4, 4)), 4);
}

#[test]
fn delta_min_0() {
    assert_eq!(delta_min(&Rect::of(-1, 0, 0, 0)), 0);
    assert_eq!(delta_min(&Rect::of(0, -1, 0, 0)), 0);
    assert_eq!(delta_min(&Rect::of(0, 0, 1, 0)), 0);
    assert_eq!(delta_min(&Rect::of(0, 0, 0, 1)), 0);
}

#[test]
fn delta_min_1() {
    assert_eq!(delta_min(&Rect::of(4, -5, 5, 5)), 1);
    assert_eq!(delta_min(&Rect::of(-5, 4, 5, 5)), 1);
    assert_eq!(delta_min(&Rect::of(-5, -5, -4, 5)), 1);
    assert_eq!(delta_min(&Rect::of(-5, -5, 5, -4)), 1);
}

#[test]
fn delta_min_bounds() {
    assert_eq!(delta_min(&Rect::of(MIN, MIN, MAX, MAX)), u64::MAX);
}
