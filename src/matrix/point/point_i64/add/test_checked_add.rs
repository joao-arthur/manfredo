use super::{checked_add, checked_add_assign, try_checked_add, try_checked_add_assign};
use crate::matrix::point::point_i64::PointI64;

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn test() {
    assert_eq!(checked_add(&PointI64::of(0, 0), &PointI64::of(10, 13)), PointI64::of(10, 13));
    assert_eq!(checked_add(&PointI64::of(10, 13), &PointI64::of(-5, -3)), PointI64::of(5, 10));
}
