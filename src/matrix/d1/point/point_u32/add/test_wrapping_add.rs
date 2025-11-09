use super::wrapping_add;
use crate::matrix::d1::point::{
    point_i32::Point as PointI,
    point_u32::{MAX, Point},
};

#[test]
fn test() {
    assert_eq!(wrapping_add(&Point::min(), &PointI::new(10)), Point::new(10));
    assert_eq!(wrapping_add(&Point::new(10), &PointI::new(-5)), Point::new(5));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_add(&Point::new(2), &PointI::new(-2)), Point::min());
    assert_eq!(wrapping_add(&Point::new(MAX - 2), &PointI::new(2)), Point::max());
}

#[test]
fn out_of_bounds() {
    assert_eq!(wrapping_add(&Point::new(2), &PointI::new(-10)), Point::new(MAX - 7));
    assert_eq!(wrapping_add(&Point::new(MAX - 2), &PointI::new(10)), Point::new(7));
}

#[test]
fn limits_out_of_bounds() {
    assert_eq!(wrapping_add(&Point::new(1), &PointI::min()), Point::new(2147483649));
    assert_eq!(wrapping_add(&Point::new(MAX - 1), &PointI::max()), Point::new(2147483645));
}
