use super::checked_add;
use crate::cartesian::d2::point::point_f32::Point;

#[test]
fn test() {
    assert_eq!(checked_add(&Point::of(0.0, 0.0), &Point::of(10.0, 13.0)), Point::of(10.0, 13.0));
    assert_eq!(checked_add(&Point::of(10.0, 13.0), &Point::of(-5.0, -3.0)), Point::of(5.0, 10.0));
}
