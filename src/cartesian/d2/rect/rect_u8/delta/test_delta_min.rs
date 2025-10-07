use super::delta_min;
use crate::cartesian::d2::rect::rect_u8::Rect;

const MAX: u8 = u8::MAX;

#[test]
fn test_delta_min() {
    assert_eq!(delta_min(&Rect::of(0, 5, 10, 10)), 5);
    assert_eq!(delta_min(&Rect::of(5, 0, 9, 9)), 4);
}

#[test]
fn delta_min_0() {
    assert_eq!(delta_min(&Rect::of(4, 5, 5, 5)), 0);
    assert_eq!(delta_min(&Rect::of(5, 4, 5, 5)), 0);
    assert_eq!(delta_min(&Rect::of(5, 5, 6, 5)), 0);
    assert_eq!(delta_min(&Rect::of(5, 5, 5, 6)), 0);
}

#[test]
fn delta_min_1() {
    assert_eq!(delta_min(&Rect::of(0, 5, 6, 6)), 1);
    assert_eq!(delta_min(&Rect::of(5, 0, 6, 6)), 1);
    assert_eq!(delta_min(&Rect::of(5, 5, 10, 6)), 1);
    assert_eq!(delta_min(&Rect::of(5, 5, 6, 10)), 1);
}

#[test]
fn delta_min_bounds() {
    assert_eq!(delta_min(&Rect::of(0, 0, MAX, MAX)), MAX);
}
