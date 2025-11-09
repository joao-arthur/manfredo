use super::checked_add_assign;
use crate::cartesian::d2::point::point_f64::Point;

#[test]
fn test() {
    let mut p = Point::zero();
    checked_add_assign(&mut p, &Point::new(10.0, 13.0));
    assert_eq!(p, Point::new(10.0, 13.0));
    checked_add_assign(&mut p, &Point::new(-25.0, -30.0));
    assert_eq!(p, Point::new(-15.0, -17.0));
}
