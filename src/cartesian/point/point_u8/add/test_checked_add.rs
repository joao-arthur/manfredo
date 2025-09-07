use super::{checked_add, checked_add_assign, try_checked_add, try_checked_add_assign};
use crate::cartesian::point::{point_i8::PointI8, point_u8::PointU8};

const MAX: u8 = u8::MAX;

#[test]
fn test() {
    assert_eq!(checked_add(&PointU8::min(), &PointI8::of(10, 13)), PointU8::of(10, 13));
    assert_eq!(checked_add(&PointU8::of(10, 13), &PointI8::of(-5, -3)), PointU8::of(5, 10));
}
