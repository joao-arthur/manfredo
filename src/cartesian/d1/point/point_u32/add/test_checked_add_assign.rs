use super::checked_add_assign;
use crate::cartesian::d1::point::{point_i32::Point as PointI, point_u32::Point};

#[test]
fn test() {
    let mut p = Point::min();
    checked_add_assign(&mut p, &PointI::of(10));
    assert_eq!(p, Point::of(10));
    checked_add_assign(&mut p, &PointI::of(-5));
    assert_eq!(p, Point::of(5));
}
