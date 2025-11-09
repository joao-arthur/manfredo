use super::delta_max;
use crate::cartesian::{
    d1::point::point_i16::{MAX, MIN},
    d2::rect::rect_i16::Rect,
};

#[test]
fn test_delta_max() {
    assert_eq!(delta_max(&Rect::new((0, -5), (5, 5))), 10);
    assert_eq!(delta_max(&Rect::new((-5, 0), (4, 4))), 9);
}

#[test]
fn delta_max_0() {
    assert_eq!(delta_max(&Rect::new((1, 1), (1, 1))), 0);
    assert_eq!(delta_max(&Rect::new((-1, -1), (-1, -1))), 0);
    assert_eq!(delta_max(&Rect::new((5, 10), (5, 10))), 0);
}

#[test]
fn delta_max_1() {
    assert_eq!(delta_max(&Rect::new((-1, 0), (0, 0))), 1);
    assert_eq!(delta_max(&Rect::new((0, -1), (0, 0))), 1);
    assert_eq!(delta_max(&Rect::new((0, 0), (1, 0))), 1);
    assert_eq!(delta_max(&Rect::new((0, 0), (0, 1))), 1);
}

#[test]
fn delta_max_bounds() {
    assert_eq!(delta_max(&Rect::new((MIN + 1, MIN), (MAX, MAX))), u16::MAX);
    assert_eq!(delta_max(&Rect::new((MIN, MIN + 1), (MAX, MAX))), u16::MAX);
    assert_eq!(delta_max(&Rect::new((MIN, MIN), (MAX - 1, MAX))), u16::MAX);
    assert_eq!(delta_max(&Rect::new((MIN, MIN), (MAX, MAX - 1))), u16::MAX);
}
