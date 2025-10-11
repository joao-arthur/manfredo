use super::checked_add_assign;
use crate::cartesian::d1::point::point_f64::Point;

#[test]
fn test() {
    let mut p = Point::of(0.0);
    checked_add_assign(&mut p, &Point::of(10.0));
    assert_eq!(p, Point::of(10.0));
    checked_add_assign(&mut p, &Point::of(-25.0));
    assert_eq!(p, Point::of(-15.0));
}
