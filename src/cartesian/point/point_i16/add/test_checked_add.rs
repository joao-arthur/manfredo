use super::{checked_add, checked_add_assign, try_checked_add, try_checked_add_assign};
use crate::cartesian::point::point_i16::PointI16;

const MIN: i16 = i16::MIN;
const MAX: i16 = i16::MAX;

#[test]
fn test() {
    assert_eq!(checked_add(&PointI16::of(0, 0), &PointI16::of(10, 13)), PointI16::of(10, 13));
    assert_eq!(checked_add(&PointI16::of(10, 13), &PointI16::of(-5, -3)), PointI16::of(5, 10));
}
