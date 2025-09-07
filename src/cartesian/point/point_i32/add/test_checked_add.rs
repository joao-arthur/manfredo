use super::{checked_add, checked_add_assign, try_checked_add, try_checked_add_assign};
use crate::cartesian::point::point_i32::PointI32;

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

#[test]
fn test() {
    assert_eq!(checked_add(&PointI32::of(0, 0), &PointI32::of(10, 13)), PointI32::of(10, 13));
    assert_eq!(checked_add(&PointI32::of(10, 13), &PointI32::of(-5, -3)), PointI32::of(5, 10));
}
