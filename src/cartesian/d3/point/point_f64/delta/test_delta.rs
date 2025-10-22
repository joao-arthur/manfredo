use super::delta;
use crate::cartesian::{
    d1::point::point_f64::MIN,
    d3::point::point_f64::Point,
};

#[test]
fn test_delta() {
    assert_eq!(delta(&Point::zero(), &Point::zero()), Point::zero());
    assert_eq!(delta(&Point::min(), &Point::of(-1.0, -1.0, -1.0)), Point::max());
    assert_eq!(delta(&Point::zero(), &Point::max()), Point::max());
    assert_eq!(delta(&Point::zero(), &Point::min()), Point::min());
    assert_eq!(delta(&Point::max(), &Point::zero()), Point::of(MIN + 1.0, MIN + 1.0, MIN + 1.0));
}
