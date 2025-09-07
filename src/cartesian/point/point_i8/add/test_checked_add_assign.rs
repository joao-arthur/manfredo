use super::{checked_add, checked_add_assign, try_checked_add, try_checked_add_assign};
use crate::cartesian::point::point_i8::PointI8;

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[test]
fn test() {
    let mut p = PointI8::of(0, 0);
    checked_add_assign(&mut p, &PointI8::of(10, 13));
    assert_eq!(p, PointI8::of(10, 13));
    checked_add_assign(&mut p, &PointI8::of(-25, -30));
    assert_eq!(p, PointI8::of(-15, -17));
}
