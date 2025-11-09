use super::checked_add_assign;
use crate::matrix::d2::point::{point_i32::Point as PointI, point_u32::Point};

#[test]
fn test() {
    let mut p = Point::min();
    checked_add_assign(&mut p, &PointI::new(10, 13));
    assert_eq!(p, Point::new(10, 13));
    checked_add_assign(&mut p, &PointI::new(-5, -3));
    assert_eq!(p, Point::new(5, 10));
}
