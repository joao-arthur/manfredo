use super::checked_add;
use crate::cartesian::d1::point::point_f64::Point;

#[test]
fn test() {
    assert_eq!(checked_add(&Point::of(0.0), &Point::of(10.0)), Point::of(10.0));
    assert_eq!(checked_add(&Point::of(10.0), &Point::of(-5.0)), Point::of(5.0));
}
