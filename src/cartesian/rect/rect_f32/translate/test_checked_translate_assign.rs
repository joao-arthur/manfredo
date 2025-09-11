use super::checked_translate_assign;
use crate::cartesian::{point::point_f32::PointF32, rect::rect_f32::Rect};

#[test]
fn test() {
    let mut r = Rect::of(0.0, 0.0, 10.0, 10.0);
    checked_translate_assign(&mut r, &PointF32::of(10.0, 20.0));
    assert_eq!(r, Rect::of(10.0, 20.0, 20.0, 30.0));
    checked_translate_assign(&mut r, &PointF32::of(-20.0, -15.0));
    assert_eq!(r, Rect::of(-10.0, 5.0, 0.0, 15.0));
}
