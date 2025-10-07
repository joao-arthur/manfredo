use super::delta_max;
use crate::cartesian::d2::rect::rect_i8::Rect;

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[test]
fn test_delta_max() {
    assert_eq!(delta_max(&Rect::of(0, -5, 5, 5)), 10);
    assert_eq!(delta_max(&Rect::of(-5, 0, 4, 4)), 9);
}

#[test]
fn delta_max_0() {
    assert_eq!(delta_max(&Rect::of(1, 1, 1, 1)), 0);
    assert_eq!(delta_max(&Rect::of(-1, -1, -1, -1)), 0);
    assert_eq!(delta_max(&Rect::of(5, 10, 5, 10)), 0);
}

#[test]
fn delta_max_1() {
    assert_eq!(delta_max(&Rect::of(-1, 0, 0, 0)), 1);
    assert_eq!(delta_max(&Rect::of(0, -1, 0, 0)), 1);
    assert_eq!(delta_max(&Rect::of(0, 0, 1, 0)), 1);
    assert_eq!(delta_max(&Rect::of(0, 0, 0, 1)), 1);
}

#[test]
fn delta_max_bounds() {
    assert_eq!(delta_max(&Rect::of(MIN + 1, MIN, MAX, MAX)), u8::MAX);
    assert_eq!(delta_max(&Rect::of(MIN, MIN + 1, MAX, MAX)), u8::MAX);
    assert_eq!(delta_max(&Rect::of(MIN, MIN, MAX - 1, MAX)), u8::MAX);
    assert_eq!(delta_max(&Rect::of(MIN, MIN, MAX, MAX - 1)), u8::MAX);
}
