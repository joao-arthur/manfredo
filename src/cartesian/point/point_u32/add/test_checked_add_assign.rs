use super::{checked_add, checked_add_assign, try_checked_add, try_checked_add_assign};
use crate::cartesian::point::{point_i32::PointI32, point_u32::PointU32};

const MAX: u32 = u32::MAX;

#[test]
fn test() {
    let mut p = PointU32::min();
    checked_add_assign(&mut p, &PointI32::of(10, 13));
    assert_eq!(p, PointU32::of(10, 13));
    checked_add_assign(&mut p, &PointI32::of(-5, -3));
    assert_eq!(p, PointU32::of(5, 10));
}
