use super::delta_max;
use crate::cartesian::d2::rect::rect_u8::Rect;

const MAX: u8 = u8::MAX;

#[test]
fn test_delta_max() {
    assert_eq!(delta_max(&Rect::of(0, 5, 10, 10)), 10);
    assert_eq!(delta_max(&Rect::of(5, 0, 9, 9)), 9);
}

#[test]
fn delta_max_0() {
    assert_eq!(delta_max(&Rect::of(0, 0, 0, 0)), 0);
    assert_eq!(delta_max(&Rect::of(1, 1, 1, 1)), 0);
    assert_eq!(delta_max(&Rect::of(5, 10, 5, 10)), 0);
}

#[test]
fn delta_max_1() {
    assert_eq!(delta_max(&Rect::of(4, 5, 5, 5)), 1);
    assert_eq!(delta_max(&Rect::of(5, 4, 5, 5)), 1);
    assert_eq!(delta_max(&Rect::of(5, 5, 6, 5)), 1);
    assert_eq!(delta_max(&Rect::of(5, 5, 5, 6)), 1);
}

#[test]
fn delta_max_bounds() {
    assert_eq!(delta_max(&Rect::of(1, 0, MAX, MAX)), MAX);
    assert_eq!(delta_max(&Rect::of(0, 1, MAX, MAX)), MAX);
    assert_eq!(delta_max(&Rect::of(0, 0, MAX - 1, MAX)), MAX);
    assert_eq!(delta_max(&Rect::of(0, 0, MAX, MAX - 1)), MAX);
}
