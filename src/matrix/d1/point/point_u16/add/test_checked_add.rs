use super::checked_add;
use crate::matrix::d1::point::{point_i16::Point as PointI, point_u16::Point};

#[test]
fn test() {
    assert_eq!(checked_add(&Point::min(), &PointI::of(10)), Point::of(10));
    assert_eq!(checked_add(&Point::of(10), &PointI::of(-5)), Point::of(5));
}
