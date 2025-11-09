use super::checked_add;
use crate::cartesian::d2::point::point_i8::Point;

#[test]
fn test() {
    assert_eq!(checked_add(&Point::zero(), &Point::new(10, 13)), Point::new(10, 13));
    assert_eq!(checked_add(&Point::new(10, 13), &Point::new(-5, -3)), Point::new(5, 10));
}
