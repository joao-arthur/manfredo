use super::checked_add_assign;
use crate::cartesian::d2::point::point_f64::Point;

#[test]
fn test() {
    let mut p = Point::zero();
    checked_add_assign(&mut p, &Point::of(10.0, 13.0));
    assert_eq!(p, Point::of(10.0, 13.0));
    checked_add_assign(&mut p, &Point::of(-25.0, -30.0));
    assert_eq!(p, Point::of(-15.0, -17.0));
}
