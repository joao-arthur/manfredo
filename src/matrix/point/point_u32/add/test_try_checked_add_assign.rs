use super::try_checked_add_assign;
use crate::matrix::point::{point_i32::PointI32 as PointI, point_u32::Point};

const MAX: u32 = u32::MAX;

#[test]
fn test() {
    let mut p = Point::min();
    assert_eq!(try_checked_add_assign(&mut p, &PointI::of(10, 13)), Some(()));
    assert_eq!(p, Point::of(10, 13));
    assert_eq!(try_checked_add_assign(&mut p, &PointI::of(-5, -3)), Some(()));
    assert_eq!(p, Point::of(5, 10));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::of(2, 5);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::of(-2, -5)), Some(()));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::of(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::of(2, 5)), Some(()));
    assert_eq!(p_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::of(2, 5);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::of(-10, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::of(0, -10)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::of(-10, -10)), None);
    assert_eq!(p_min, Point::of(2, 5));

    let mut p_max = Point::of(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::of(10, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::of(0, 10)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::of(10, 10)), None);
    assert_eq!(p_max, Point::of(MAX - 2, MAX - 5));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::of(1, 1);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::of(i32::MIN, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::of(0, i32::MIN)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::min()), None);
    assert_eq!(p_min, Point::of(1, 1));

    let mut p_max = Point::of(MAX - 1, MAX - 1);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::of(i32::MAX, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::of(0, i32::MAX)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::max()), None);
    assert_eq!(p_max, Point::of(MAX - 1, MAX - 1));
}
