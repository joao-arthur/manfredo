use super::checked_add;
use crate::cartesian::point::{point_i32::PointI32 as PointI, point_u32::PointU32};

#[test]
fn test() {
    assert_eq!(checked_add(&PointU32::min(), &PointI::of(10, 13)), PointU32::of(10, 13));
    assert_eq!(checked_add(&PointU32::of(10, 13), &PointI::of(-5, -3)), PointU32::of(5, 10));
}
