use super::checked_add;
use crate::cartesian::d2::point::point_i8::Point;

#[test]
fn test() {
    assert_eq!(checked_add(&Point::of(0, 0), &Point::of(10, 13)), Point::of(10, 13));
    assert_eq!(checked_add(&Point::of(10, 13), &Point::of(-5, -3)), Point::of(5, 10));
}
