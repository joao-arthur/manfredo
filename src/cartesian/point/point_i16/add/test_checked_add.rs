use super::checked_add;
use crate::cartesian::point::point_i16::PointI16;

#[test]
fn test() {
    assert_eq!(checked_add(&PointI16::of(0, 0), &PointI16::of(10, 13)), PointI16::of(10, 13));
    assert_eq!(checked_add(&PointI16::of(10, 13), &PointI16::of(-5, -3)), PointI16::of(5, 10));
}
