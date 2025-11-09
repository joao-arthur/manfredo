use super::checked_add_assign;
use crate::cartesian::d2::point::point_i16::Point;

#[test]
fn test() {
    let mut p = Point::zero();
    checked_add_assign(&mut p, &Point::new(10, 13));
    assert_eq!(p, Point::new(10, 13));
    checked_add_assign(&mut p, &Point::new(-25, -30));
    assert_eq!(p, Point::new(-15, -17));
}
