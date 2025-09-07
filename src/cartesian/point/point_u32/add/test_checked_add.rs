use super::checked_add;
use crate::cartesian::point::{point_i32::PointI32, point_u32::PointU32};

#[test]
fn test() {
    assert_eq!(checked_add(&PointU32::min(), &PointI32::of(10, 13)), PointU32::of(10, 13));
    assert_eq!(checked_add(&PointU32::of(10, 13), &PointI32::of(-5, -3)), PointU32::of(5, 10));
}
