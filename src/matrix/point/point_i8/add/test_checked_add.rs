use super::checked_add;
use crate::matrix::point::point_i8::PointI8;

#[test]
fn test() {
    assert_eq!(checked_add(&PointI8::of(0, 0), &PointI8::of(10, 13)), PointI8::of(10, 13));
    assert_eq!(checked_add(&PointI8::of(10, 13), &PointI8::of(-5, -3)), PointI8::of(5, 10));
}
