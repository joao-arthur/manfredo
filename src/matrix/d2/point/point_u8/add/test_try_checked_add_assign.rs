use super::try_checked_add_assign;
use crate::matrix::{
    d1::point::point_u8::MAX,
    d2::point::{point_i8::Point as PointI, point_u8::Point},
};

#[test]
fn test() {
    let mut p = Point::min();
    assert_eq!(try_checked_add_assign(&mut p, &PointI::new(10, 13)), Some(()));
    assert_eq!(p, Point::new(10, 13));
    assert_eq!(try_checked_add_assign(&mut p, &PointI::new(-5, -3)), Some(()));
    assert_eq!(p, Point::new(5, 10));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::new(2, 5);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::new(-2, -5)), Some(()));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::new(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::new(2, 5)), Some(()));
    assert_eq!(p_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::new(2, 5);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::new(-10, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::new(0, -10)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::new(-10, -10)), None);
    assert_eq!(p_min, Point::new(2, 5));

    let mut p_max = Point::new(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::new(10, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::new(0, 10)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::new(10, 10)), None);
    assert_eq!(p_max, Point::new(MAX - 2, MAX - 5));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::new(1, 1);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::new(i8::MIN, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::new(0, i8::MIN)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::min()), None);
    assert_eq!(p_min, Point::new(1, 1));

    let mut p_max = Point::new(MAX - 1, MAX - 1);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::new(i8::MAX, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::new(0, i8::MAX)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::max()), None);
    assert_eq!(p_max, Point::new(MAX - 1, MAX - 1));
}
