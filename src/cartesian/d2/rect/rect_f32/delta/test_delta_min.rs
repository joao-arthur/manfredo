use super::delta_min;
use crate::cartesian::{
    d1::point::point_f32::{MAX, MIN},
    d2::rect::rect_f32::Rect,
};

#[test]
fn test_delta_min() {
    assert_eq!(delta_min(&Rect::of((0.0, -5.0), (5.0, 5.0))), 5.0);
    assert_eq!(delta_min(&Rect::of((-5.0, 0.0), (4.0, 4.0))), 4.0);
}

#[test]
fn delta_min_0() {
    assert_eq!(delta_min(&Rect::of((-1.0, 0.0), (0.0, 0.0))), 0.0);
    assert_eq!(delta_min(&Rect::of((0.0, -1.0), (0.0, 0.0))), 0.0);
    assert_eq!(delta_min(&Rect::of((0.0, 0.0), (1.0, 0.0))), 0.0);
    assert_eq!(delta_min(&Rect::of((0.0, 0.0), (0.0, 1.0))), 0.0);
}

#[test]
fn delta_min_1() {
    assert_eq!(delta_min(&Rect::of((4.0, -5.0), (5.0, 5.0))), 1.0);
    assert_eq!(delta_min(&Rect::of((-5.0, 4.0), (5.0, 5.0))), 1.0);
    assert_eq!(delta_min(&Rect::of((-5.0, -5.0), (-4.0, 5.0))), 1.0);
    assert_eq!(delta_min(&Rect::of((-5.0, -5.0), (5.0, -4.0))), 1.0);
}

#[test]
fn delta_min_bounds() {
    assert_eq!(delta_min(&Rect::of((MIN + 1.0, MIN + 1.0), (0.0, 0.0))), MAX);
    assert_eq!(delta_min(&Rect::of((0.0, 0.0), (MAX, MAX))), MAX);
}
