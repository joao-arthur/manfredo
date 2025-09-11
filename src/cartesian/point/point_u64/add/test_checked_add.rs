use super::checked_add;
use crate::cartesian::point::{point_i64::PointI64 as PointI, point_u64::PointU64};

#[test]
fn test() {
    assert_eq!(checked_add(&PointU64::min(), &PointI::of(10, 13)), PointU64::of(10, 13));
    assert_eq!(checked_add(&PointU64::of(10, 13), &PointI::of(-5, -3)), PointU64::of(5, 10));
}
