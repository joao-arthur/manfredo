use super::checked_translate;
use crate::cartesian::{point::point_f32::PointF32, rect::rect_f32::Rect};

#[test]
fn test() {
    assert_eq!(checked_translate(&Rect::of(0.0, 0.0, 10.0, 10.0), &PointF32::of(10.0, 20.0)), Rect::of(10.0, 20.0, 20.0, 30.0));
    assert_eq!(checked_translate(&Rect::of(10.0, 20.0, 20.0, 30.0), &PointF32::of(-20.0, -15.0)), Rect::of(-10.0, 5.0, 0.0, 15.0));
}
