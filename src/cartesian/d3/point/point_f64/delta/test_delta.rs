use super::delta;
use crate::cartesian::d1::point::point_f64::{MAX, MIN};
use crate::cartesian::d3::point::point_f64::Point;

#[test]
fn test_delta() {
    assert_eq!(delta(&Point::zero(), &Point::zero()), Point::zero());
    assert_eq!(delta(&Point::of(MIN, MIN, MIN), &Point::of(-1.0, -1.0, -1.0)), Point::of(MAX, MAX, MAX));
    assert_eq!(delta(&Point::zero(), &Point::of(MAX, MAX, MAX)), Point::of(MAX, MAX, MAX));
    assert_eq!(delta(&Point::zero(), &Point::of(MIN, MIN, MIN)), Point::of(MIN, MIN, MIN));
    assert_eq!(delta(&Point::of(MAX, MAX, MAX), &Point::zero()), Point::of(MIN + 1.0, MIN + 1.0, MIN + 1.0));
}
