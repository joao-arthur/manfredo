use super::delta_max;
use crate::cartesian::{
    d1::point::point_f32::{MAX, MIN},
    d2::rect::rect_f32::Rect,
};

#[test]
fn test_delta_max() {
    assert_eq!(delta_max(&Rect::new((0.0, -5.0), (5.0, 5.0))), 10.0);
    assert_eq!(delta_max(&Rect::new((-5.0, 0.0), (4.0, 4.0))), 9.0);
}

#[test]
fn delta_max_0() {
    assert_eq!(delta_max(&Rect::new((1.0, 1.0), (1.0, 1.0))), 0.0);
    assert_eq!(delta_max(&Rect::new((-1.0, -1.0), (-1.0, -1.0))), 0.0);
    assert_eq!(delta_max(&Rect::new((5.0, 10.0), (5.0, 10.0))), 0.0);
}

#[test]
fn delta_max_1() {
    assert_eq!(delta_max(&Rect::new((-1.0, 0.0), (0.0, 0.0))), 1.0);
    assert_eq!(delta_max(&Rect::new((0.0, -1.0), (0.0, 0.0))), 1.0);
    assert_eq!(delta_max(&Rect::new((0.0, 0.0), (1.0, 0.0))), 1.0);
    assert_eq!(delta_max(&Rect::new((0.0, 0.0), (0.0, 1.0))), 1.0);
}

#[test]
fn delta_max_bounds() {
    assert_eq!(delta_max(&Rect::new((MIN + 1.0, MIN + 2.0), (0.0, 0.0))), MAX);
    assert_eq!(delta_max(&Rect::new((MIN + 2.0, MIN + 1.0), (0.0, 0.0))), MAX);
    assert_eq!(delta_max(&Rect::new((0.0, 0.0), (MAX - 1.0, MAX))), MAX);
    assert_eq!(delta_max(&Rect::new((0.0, 0.0), (MAX, MAX - 1.0))), MAX);

    assert_eq!(delta_max(&Rect::new((1.0, 0.0), (MAX, MAX))), MAX);
    assert_eq!(delta_max(&Rect::new((0.0, 1.0), (MAX, MAX))), MAX);
    assert_eq!(delta_max(&Rect::new((0.0, 0.0), (MAX - 1.0, MAX))), MAX);
    assert_eq!(delta_max(&Rect::new((0.0, 0.0), (MAX, MAX - 1.0))), MAX);
}
