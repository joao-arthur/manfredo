use super::checked_add;
use crate::cartesian::d1::point::point_i32::Point;

#[test]
fn test() {
    assert_eq!(checked_add(&Point::zero(), &Point::of(10)), Point::of(10));
    assert_eq!(checked_add(&Point::of(10), &Point::of(-5)), Point::of(5));
}
