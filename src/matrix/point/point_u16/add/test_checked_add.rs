use super::{checked_add, checked_add_assign, try_checked_add, try_checked_add_assign};
use crate::matrix::point::{point_i16::PointI16, point_u16::PointU16};

const MAX: u16 = u16::MAX;

#[test]
fn test() {
    assert_eq!(checked_add(&PointU16::min(), &PointI16::of(10, 13)), PointU16::of(10, 13));
    assert_eq!(checked_add(&PointU16::of(10, 13), &PointI16::of(-5, -3)), PointU16::of(5, 10));
}
