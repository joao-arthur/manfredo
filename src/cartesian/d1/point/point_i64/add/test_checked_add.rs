use super::checked_add;
use crate::cartesian::d1::point::point_i64::Point;

#[test]
fn test() {
    assert_eq!(checked_add(&Point::of(0), &Point::of(10)), Point::of(10));
    assert_eq!(checked_add(&Point::of(10), &Point::of(-5)), Point::of(5));
}
