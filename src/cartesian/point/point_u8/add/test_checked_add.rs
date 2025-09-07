use super::checked_add;
use crate::cartesian::point::{point_i8::PointI8, point_u8::PointU8};

#[test]
fn test() {
    assert_eq!(checked_add(&PointU8::min(), &PointI8::of(10, 13)), PointU8::of(10, 13));
    assert_eq!(checked_add(&PointU8::of(10, 13), &PointI8::of(-5, -3)), PointU8::of(5, 10));
}
