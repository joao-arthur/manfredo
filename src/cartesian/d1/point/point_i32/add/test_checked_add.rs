use super::checked_add;
use crate::cartesian::d1::point::point_i32::Point;

#[test]
fn test() {
    assert_eq!(checked_add(&Point::zero(), &Point::new(10)), Point::new(10));
    assert_eq!(checked_add(&Point::new(10), &Point::new(-5)), Point::new(5));
}
