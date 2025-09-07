use super::{checked_add, checked_add_assign, try_checked_add, try_checked_add_assign};
use crate::matrix::point::point_i8::PointI8;

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[test]
fn test() {
    assert_eq!(checked_add(&PointI8::of(0, 0), &PointI8::of(10, 13)), PointI8::of(10, 13));
    assert_eq!(checked_add(&PointI8::of(10, 13), &PointI8::of(-5, -3)), PointI8::of(5, 10));
}
