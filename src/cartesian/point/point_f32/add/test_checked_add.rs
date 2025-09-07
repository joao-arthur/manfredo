use super::checked_add;
use crate::cartesian::point::point_f32::PointF32;

#[test]
fn test() {
    assert_eq!(checked_add(&PointF32::of(0.0, 0.0), &PointF32::of(10.0, 13.0)), PointF32::of(10.0, 13.0));
    assert_eq!(checked_add(&PointF32::of(10.0, 13.0), &PointF32::of(-5.0, -3.0)), PointF32::of(5.0, 10.0));
}
