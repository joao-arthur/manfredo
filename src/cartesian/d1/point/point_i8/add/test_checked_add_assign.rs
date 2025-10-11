use super::checked_add_assign;
use crate::cartesian::d1::point::point_i8::Point;

#[test]
fn test() {
    let mut p = Point::of(0);
    checked_add_assign(&mut p, &Point::of(10));
    assert_eq!(p, Point::of(10));
    checked_add_assign(&mut p, &Point::of(-25));
    assert_eq!(p, Point::of(-15));
}
