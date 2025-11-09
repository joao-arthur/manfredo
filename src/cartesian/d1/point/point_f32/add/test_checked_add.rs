use super::checked_add;
use crate::cartesian::d1::point::point_f32::Point;

#[test]
fn test() {
    assert_eq!(checked_add(&Point::zero(), &Point::new(10.0)), Point::new(10.0));
    assert_eq!(checked_add(&Point::new(10.0), &Point::new(-5.0)), Point::new(5.0));
}
