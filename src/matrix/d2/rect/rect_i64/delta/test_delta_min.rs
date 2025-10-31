use super::delta_min;
use crate::matrix::{
    d1::point::point_i64::{MAX, MIN},
    d2::rect::rect_i64::Rect,
};

#[test]
fn test_delta_min() {
    assert_eq!(delta_min(&Rect::of((0, -5), (5, 5))), 5);
    assert_eq!(delta_min(&Rect::of((-5, 0), (4, 4))), 4);
}

#[test]
fn delta_min_0() {
    assert_eq!(delta_min(&Rect::of((-1, 0), (0, 0))), 0);
    assert_eq!(delta_min(&Rect::of((0, -1), (0, 0))), 0);
    assert_eq!(delta_min(&Rect::of((0, 0), (1, 0))), 0);
    assert_eq!(delta_min(&Rect::of((0, 0), (0, 1))), 0);
}

#[test]
fn delta_min_1() {
    assert_eq!(delta_min(&Rect::of((4, -5), (5, 5))), 1);
    assert_eq!(delta_min(&Rect::of((-5, 4), (5, 5))), 1);
    assert_eq!(delta_min(&Rect::of((-5, -5), (-4, 5))), 1);
    assert_eq!(delta_min(&Rect::of((-5, -5), (5, -4))), 1);
}

#[test]
fn delta_min_bounds() {
    assert_eq!(delta_min(&Rect::of((MIN, MIN), (MAX, MAX))), u64::MAX);
}
