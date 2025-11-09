use super::try_checked_add_assign;
use crate::matrix::d1::point::{
    point_i64::Point as PointI,
    point_u64::{MAX, Point},
};

#[test]
fn test() {
    let mut p = Point::min();
    assert_eq!(try_checked_add_assign(&mut p, &PointI::new(10)), Some(()));
    assert_eq!(p, Point::new(10));
    assert_eq!(try_checked_add_assign(&mut p, &PointI::new(-5)), Some(()));
    assert_eq!(p, Point::new(5));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::new(2);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::new(-2)), Some(()));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::new(MAX - 2);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::new(2)), Some(()));
    assert_eq!(p_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::new(2);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::new(-10)), None);
    assert_eq!(p_min, Point::new(2));

    let mut p_max = Point::new(MAX - 2);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::new(10)), None);
    assert_eq!(p_max, Point::new(MAX - 2));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::new(1);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::min()), None);
    assert_eq!(p_min, Point::new(1));

    let mut p_max = Point::new(MAX - 1);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::max()), None);
    assert_eq!(p_max, Point::new(MAX - 1));
}
