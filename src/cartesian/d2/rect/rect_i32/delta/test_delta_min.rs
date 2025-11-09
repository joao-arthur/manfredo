use super::delta_min;
use crate::cartesian::{
    d1::point::point_i32::{MAX, MIN},
    d2::rect::rect_i32::Rect,
};

#[test]
fn test_delta_min() {
    assert_eq!(delta_min(&Rect::new((0, -5), (5, 5))), 5);
    assert_eq!(delta_min(&Rect::new((-5, 0), (4, 4))), 4);
}

#[test]
fn delta_min_0() {
    assert_eq!(delta_min(&Rect::new((-1, 0), (0, 0))), 0);
    assert_eq!(delta_min(&Rect::new((0, -1), (0, 0))), 0);
    assert_eq!(delta_min(&Rect::new((0, 0), (1, 0))), 0);
    assert_eq!(delta_min(&Rect::new((0, 0), (0, 1))), 0);
}

#[test]
fn delta_min_1() {
    assert_eq!(delta_min(&Rect::new((4, -5), (5, 5))), 1);
    assert_eq!(delta_min(&Rect::new((-5, 4), (5, 5))), 1);
    assert_eq!(delta_min(&Rect::new((-5, -5), (-4, 5))), 1);
    assert_eq!(delta_min(&Rect::new((-5, -5), (5, -4))), 1);
}

#[test]
fn delta_min_bounds() {
    assert_eq!(delta_min(&Rect::new((MIN, MIN), (MAX, MAX))), u32::MAX);
}
