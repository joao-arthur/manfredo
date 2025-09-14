use super::checked_add;
use crate::matrix::d2::point::{point_i16::Point as PointI, point_u16::Point};

#[test]
fn test() {
    assert_eq!(checked_add(&Point::min(), &PointI::of(10, 13)), Point::of(10, 13));
    assert_eq!(checked_add(&Point::of(10, 13), &PointI::of(-5, -3)), Point::of(5, 10));
}
