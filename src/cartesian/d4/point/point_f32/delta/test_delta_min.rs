use super::delta_min;
use crate::cartesian::d4::point::point_f32::Point;

#[test]
fn test_delta_min() {
    assert_eq!(delta_min(&Point::of(0.0, 1.0, 2.0, 3.0), &Point::of(10.0, 10.0, 10.0, 10.0)), 7.0);
    assert_eq!(delta_min(&Point::of(1.0, 2.0, 3.0, 0.0), &Point::of(9.0, 9.0, 9.0, 9.0)), 6.0);
    assert_eq!(delta_min(&Point::of(2.0, 3.0, 0.0, 1.0), &Point::of(8.0, 8.0, 8.0, 8.0)), 5.0);
    assert_eq!(delta_min(&Point::of(3.0, 0.0, 1.0, 2.0), &Point::of(7.0, 7.0, 7.0, 7.0)), 4.0);
}
