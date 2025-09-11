use super::saturating_add;
use crate::matrix::point::{point_i32::PointI32 as PointI, point_u32::Point};

const MAX: u32 = u32::MAX;

#[test]
fn test() {
    assert_eq!(saturating_add(&Point::min(), &PointI::of(10, 13)), Point::of(10, 13));
    assert_eq!(saturating_add(&Point::of(10, 10), &PointI::of(-5, -3)), Point::of(5, 7));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_add(&Point::of(2, 5), &PointI::of(-2, -5)), Point::min());
    assert_eq!(saturating_add(&Point::of(MAX - 2, MAX - 5), &PointI::of(2, 5)), Point::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(saturating_add(&Point::of(2, 5), &PointI::of(-10, -10)), Point::min());
    assert_eq!(saturating_add(&Point::of(MAX - 2, MAX - 5), &PointI::of(10, 10)), Point::max());
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(saturating_add(&Point::of(1, 1), &PointI::min()), Point::min());
    assert_eq!(saturating_add(&Point::of(MAX - 1, MAX - 1), &PointI::max()), Point::max());
}
