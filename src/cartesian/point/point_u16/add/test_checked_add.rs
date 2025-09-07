use super::checked_add;
use crate::cartesian::point::{point_i16::PointI16, point_u16::PointU16};

#[test]
fn test() {
    assert_eq!(checked_add(&PointU16::min(), &PointI16::of(10, 13)), PointU16::of(10, 13));
    assert_eq!(checked_add(&PointU16::of(10, 13), &PointI16::of(-5, -3)), PointU16::of(5, 10));
}
