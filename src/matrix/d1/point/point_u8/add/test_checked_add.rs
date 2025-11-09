use super::checked_add;
use crate::matrix::d1::point::{point_i8::Point as PointI, point_u8::Point};

#[test]
fn test() {
    assert_eq!(checked_add(&Point::min(), &PointI::new(10)), Point::new(10));
    assert_eq!(checked_add(&Point::new(10), &PointI::new(-5)), Point::new(5));
}
