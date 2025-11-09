use super::checked_add_assign;
use crate::cartesian::d1::point::{point_i32::Point as PointI, point_u32::Point};

#[test]
fn test() {
    let mut p = Point::min();
    checked_add_assign(&mut p, &PointI::new(10));
    assert_eq!(p, Point::new(10));
    checked_add_assign(&mut p, &PointI::new(-5));
    assert_eq!(p, Point::new(5));
}
