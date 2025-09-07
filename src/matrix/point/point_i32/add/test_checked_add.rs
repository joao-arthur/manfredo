use super::checked_add;
use crate::matrix::point::point_i32::PointI32;

#[test]
fn test() {
    assert_eq!(checked_add(&PointI32::of(0, 0), &PointI32::of(10, 13)), PointI32::of(10, 13));
    assert_eq!(checked_add(&PointI32::of(10, 13), &PointI32::of(-5, -3)), PointI32::of(5, 10));
}
