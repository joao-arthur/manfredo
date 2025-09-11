use super::checked_add;
use crate::cartesian::point::{point_i32::PointI32 as PointI, point_u32::Point};

#[test]
fn test() {
    assert_eq!(checked_add(&Point::min(), &PointI::of(10, 13)), Point::of(10, 13));
    assert_eq!(checked_add(&Point::of(10, 13), &PointI::of(-5, -3)), Point::of(5, 10));
}
