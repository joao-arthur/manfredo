use super::saturating_add_assign;
use crate::cartesian::d1::point::{
    point_i64::Point as PointI,
    point_u64::{MAX, Point},
};

#[test]
fn test() {
    let mut p = Point::min();
    saturating_add_assign(&mut p, &PointI::new(10));
    assert_eq!(p, Point::new(10));
    saturating_add_assign(&mut p, &PointI::new(-5));
    assert_eq!(p, Point::new(5));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::new(2);
    saturating_add_assign(&mut p_min, &PointI::new(-2));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::new(MAX - 2);
    saturating_add_assign(&mut p_max, &PointI::new(2));
    assert_eq!(p_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::new(2);
    saturating_add_assign(&mut p_min, &PointI::new(-10));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::new(MAX - 2);
    saturating_add_assign(&mut p_max, &PointI::new(10));
    assert_eq!(p_max, Point::max());
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::new(1);
    saturating_add_assign(&mut p_min, &PointI::min());
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::new(MAX - 1);
    saturating_add_assign(&mut p_max, &PointI::max());
    assert_eq!(p_max, Point::max());
}
