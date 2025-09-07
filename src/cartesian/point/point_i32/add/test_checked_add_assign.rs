use super::{checked_add, checked_add_assign, try_checked_add, try_checked_add_assign};
use crate::cartesian::point::point_i32::PointI32;

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

#[test]
fn test() {
    let mut p = PointI32::of(0, 0);
    checked_add_assign(&mut p, &PointI32::of(10, 13));
    assert_eq!(p, PointI32::of(10, 13));
    checked_add_assign(&mut p, &PointI32::of(-25, -30));
    assert_eq!(p, PointI32::of(-15, -17));
}
