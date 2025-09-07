use super::checked_add;
use crate::matrix::point::point_i64::PointI64;

#[test]
fn test() {
    assert_eq!(checked_add(&PointI64::of(0, 0), &PointI64::of(10, 13)), PointI64::of(10, 13));
    assert_eq!(checked_add(&PointI64::of(10, 13), &PointI64::of(-5, -3)), PointI64::of(5, 10));
}
