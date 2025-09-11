use super::checked_add;
use crate::matrix::point::{point_i8::PointI8 as PointI, point_u8::PointU8};

#[test]
fn test() {
    assert_eq!(checked_add(&PointU8::min(), &PointI::of(10, 13)), PointU8::of(10, 13));
    assert_eq!(checked_add(&PointU8::of(10, 13), &PointI::of(-5, -3)), PointU8::of(5, 10));
}
