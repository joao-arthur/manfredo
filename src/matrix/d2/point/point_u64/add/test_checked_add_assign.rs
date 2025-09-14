use super::checked_add_assign;
use crate::matrix::d2::point::{point_i64::Point as PointI, point_u64::Point};

#[test]
fn test() {
    let mut p = Point::min();
    checked_add_assign(&mut p, &PointI::of(10, 13));
    assert_eq!(p, Point::of(10, 13));
    checked_add_assign(&mut p, &PointI::of(-5, -3));
    assert_eq!(p, Point::of(5, 10));
}
