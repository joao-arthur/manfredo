use super::try_checked_add_assign;
use crate::matrix::d1::point::{point_i8::Point as PointI, point_u8::Point};

const MAX: u8 = u8::MAX;

#[test]
fn test() {
    let mut p = Point::min();
    assert_eq!(try_checked_add_assign(&mut p, &PointI::of(10)), Some(()));
    assert_eq!(p, Point::of(10));
    assert_eq!(try_checked_add_assign(&mut p, &PointI::of(-5)), Some(()));
    assert_eq!(p, Point::of(5));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::of(2);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::of(-2)), Some(()));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::of(MAX - 2);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::of(2)), Some(()));
    assert_eq!(p_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::of(2);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::of(-10)), None);
    assert_eq!(p_min, Point::of(2));

    let mut p_max = Point::of(MAX - 2);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::of(10)), None);
    assert_eq!(p_max, Point::of(MAX - 2));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::of(1);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::min()), None);
    assert_eq!(p_min, Point::of(1));

    let mut p_max = Point::of(MAX - 1);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::max()), None);
    assert_eq!(p_max, Point::of(MAX - 1));
}
