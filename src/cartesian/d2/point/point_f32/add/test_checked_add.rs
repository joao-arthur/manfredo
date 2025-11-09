use super::checked_add;
use crate::cartesian::d2::point::point_f32::Point;

#[test]
fn test() {
    assert_eq!(checked_add(&Point::zero(), &Point::new(10.0, 13.0)), Point::new(10.0, 13.0));
    assert_eq!(checked_add(&Point::new(10.0, 13.0), &Point::new(-5.0, -3.0)), Point::new(5.0, 10.0));
}
