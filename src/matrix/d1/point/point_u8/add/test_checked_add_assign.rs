use super::checked_add_assign;
use crate::matrix::d1::point::{point_i8::Point as PointI, point_u8::Point};

#[test]
fn test() {
    let mut p = Point::min();
    checked_add_assign(&mut p, &PointI::of(10));
    assert_eq!(p, Point::of(10));
    checked_add_assign(&mut p, &PointI::of(-5));
    assert_eq!(p, Point::of(5));
}
