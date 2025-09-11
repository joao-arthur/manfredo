use super::checked_add;
use crate::cartesian::point::{point_i16::PointI16 as PointI, point_u16::PointU16};

#[test]
fn test() {
    assert_eq!(checked_add(&PointU16::min(), &PointI::of(10, 13)), PointU16::of(10, 13));
    assert_eq!(checked_add(&PointU16::of(10, 13), &PointI::of(-5, -3)), PointU16::of(5, 10));
}
