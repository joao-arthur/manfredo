use super::checked_add_assign;
use crate::cartesian::point::point_f32::PointF32;

#[test]
fn test() {
    let mut p = PointF32::of(0.0, 0.0);
    checked_add_assign(&mut p, &PointF32::of(10.0, 13.0));
    assert_eq!(p, PointF32::of(10.0, 13.0));
    checked_add_assign(&mut p, &PointF32::of(-25.0, -30.0));
    assert_eq!(p, PointF32::of(-15.0, -17.0));
}
