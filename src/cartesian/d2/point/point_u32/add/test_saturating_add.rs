use super::saturating_add;
use crate::cartesian::{
    d1::point::point_u32::MAX,
    d2::point::{point_i32::Point as PointI, point_u32::Point},
};

#[test]
fn test() {
    assert_eq!(saturating_add(&Point::min(), &PointI::new(10, 13)), Point::new(10, 13));
    assert_eq!(saturating_add(&Point::new(10, 10), &PointI::new(-5, -3)), Point::new(5, 7));
}

#[test]
fn to_bounds() {
    assert_eq!(saturating_add(&Point::new(2, 5), &PointI::new(-2, -5)), Point::min());
    assert_eq!(saturating_add(&Point::new(MAX - 2, MAX - 5), &PointI::new(2, 5)), Point::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(saturating_add(&Point::new(2, 5), &PointI::new(-10, -10)), Point::min());
    assert_eq!(saturating_add(&Point::new(MAX - 2, MAX - 5), &PointI::new(10, 10)), Point::max());
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(saturating_add(&Point::new(1, 1), &PointI::min()), Point::min());
    assert_eq!(saturating_add(&Point::new(MAX - 1, MAX - 1), &PointI::max()), Point::max());
}
