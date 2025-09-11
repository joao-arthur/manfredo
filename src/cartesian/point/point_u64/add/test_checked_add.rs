use super::checked_add;
use crate::cartesian::point::{point_i64::PointI64 as PointI, point_u64::Point};

#[test]
fn test() {
    assert_eq!(checked_add(&Point::min(), &PointI::of(10, 13)), Point::of(10, 13));
    assert_eq!(checked_add(&Point::of(10, 13), &PointI::of(-5, -3)), Point::of(5, 10));
}
