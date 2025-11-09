use super::checked_add_assign;
use crate::cartesian::d1::point::{point_i16::Point as PointI, point_u16::Point};

#[test]
fn test() {
    let mut p = Point::min();
    checked_add_assign(&mut p, &PointI::new(10));
    assert_eq!(p, Point::new(10));
    checked_add_assign(&mut p, &PointI::new(-5));
    assert_eq!(p, Point::new(5));
}
