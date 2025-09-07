use super::checked_add;
use crate::cartesian::point::point_f64::PointF64;

#[test]
fn test() {
    assert_eq!(checked_add(&PointF64::of(0.0, 0.0), &PointF64::of(10.0, 13.0)), PointF64::of(10.0, 13.0));
    assert_eq!(checked_add(&PointF64::of(10.0, 13.0), &PointF64::of(-5.0, -3.0)), PointF64::of(5.0, 10.0));
}
