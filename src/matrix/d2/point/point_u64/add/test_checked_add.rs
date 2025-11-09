use super::checked_add;
use crate::matrix::d2::point::{point_i64::Point as PointI, point_u64::Point};

#[test]
fn test() {
    assert_eq!(checked_add(&Point::min(), &PointI::new(10, 13)), Point::new(10, 13));
    assert_eq!(checked_add(&Point::new(10, 13), &PointI::new(-5, -3)), Point::new(5, 10));
}
