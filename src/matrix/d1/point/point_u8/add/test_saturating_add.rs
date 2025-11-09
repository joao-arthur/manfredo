use super::saturating_add;
use crate::matrix::d1::point::{
    point_i8::Point as PointI,
    point_u8::{MAX, Point},
};

#[test]
fn test() {
    assert_eq!(saturating_add(&Point::min(), &PointI::new(10)), Point::new(10));
    assert_eq!(saturating_add(&Point::new(10), &PointI::new(-5)), Point::new(5));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_add(&Point::new(2), &PointI::new(-2)), Point::min());
    assert_eq!(saturating_add(&Point::new(MAX - 2), &PointI::new(2)), Point::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(saturating_add(&Point::new(2), &PointI::new(-10)), Point::min());
    assert_eq!(saturating_add(&Point::new(MAX - 2), &PointI::new(10)), Point::max());
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(saturating_add(&Point::new(1), &PointI::min()), Point::min());
    assert_eq!(saturating_add(&Point::new(MAX - 1), &PointI::max()), Point::max());
}
