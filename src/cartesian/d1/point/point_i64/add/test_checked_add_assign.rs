use super::checked_add_assign;
use crate::cartesian::d1::point::point_i64::Point;

#[test]
fn test() {
    let mut p = Point::zero();
    checked_add_assign(&mut p, &Point::new(10));
    assert_eq!(p, Point::new(10));
    checked_add_assign(&mut p, &Point::new(-25));
    assert_eq!(p, Point::new(-15));
}
