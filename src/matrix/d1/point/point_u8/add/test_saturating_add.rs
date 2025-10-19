use super::saturating_add;
use crate::matrix::d1::point::{
    point_i8::Point as PointI,
    point_u8::{MAX, Point},
};

#[test]
fn test() {
    assert_eq!(saturating_add(&Point::min(), &PointI::of(10)), Point::of(10));
    assert_eq!(saturating_add(&Point::of(10), &PointI::of(-5)), Point::of(5));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_add(&Point::of(2), &PointI::of(-2)), Point::min());
    assert_eq!(saturating_add(&Point::of(MAX - 2), &PointI::of(2)), Point::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(saturating_add(&Point::of(2), &PointI::of(-10)), Point::min());
    assert_eq!(saturating_add(&Point::of(MAX - 2), &PointI::of(10)), Point::max());
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(saturating_add(&Point::of(1), &PointI::min()), Point::min());
    assert_eq!(saturating_add(&Point::of(MAX - 1), &PointI::max()), Point::max());
}
