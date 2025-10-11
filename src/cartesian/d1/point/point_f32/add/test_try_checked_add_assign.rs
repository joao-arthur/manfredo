use super::try_checked_add_assign;
use crate::cartesian::d1::point::point_f32::{MAX, MIN, Point};

#[test]
fn test() {
    let mut p = Point::of(0.0);
    assert_eq!(try_checked_add_assign(&mut p, &Point::of(10.0)), Some(()));
    assert_eq!(p, Point::of(10.0));
    assert_eq!(try_checked_add_assign(&mut p, &Point::of(-25.0)), Some(()));
    assert_eq!(p, Point::of(-15.0));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::of(MIN + 2.0);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::of(-2.0)), Some(()));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::of(MAX - 2.0);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::of(2.0)), Some(()));
    assert_eq!(p_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::of(MIN + 2.0);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::of(-10.0)), None);
    assert_eq!(p_min, Point::of(MIN + 2.));

    let mut p_max = Point::of(MAX - 2.0);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::of(10.0)), None);
    assert_eq!(p_max, Point::of(MAX - 2.0));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::of(MIN + 1.0);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::min()), None);
    assert_eq!(p_min, Point::of(MIN + 1.0));

    let mut p_max = Point::of(MAX - 1.0);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::max()), None);
    assert_eq!(p_max, Point::of(MAX - 1.0));
}
