use super::try_checked_add_assign;
use crate::cartesian::d2::point::point_f64::{MAX, MIN, Point};

#[test]
fn test() {
    let mut p = Point::of(0.0, 0.0);
    assert_eq!(try_checked_add_assign(&mut p, &Point::of(10.0, 13.0)), Some(()));
    assert_eq!(p, Point::of(10.0, 13.0));
    assert_eq!(try_checked_add_assign(&mut p, &Point::of(-25.0, -30.0)), Some(()));
    assert_eq!(p, Point::of(-15.0, -17.0));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::of(MIN + 2.0, MIN + 5.0);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::of(-2.0, -5.0)), Some(()));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::of(MAX - 2.0, MAX - 5.0);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::of(2.0, 5.0)), Some(()));
    assert_eq!(p_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::of(MIN + 2.0, MIN + 5.0);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::of(-10.0, 0.0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::of(0.0, -10.0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::of(-10.0, -10.0)), None);
    assert_eq!(p_min, Point::of(MIN + 2.0, MIN + 5.0));

    let mut p_max = Point::of(MAX - 2.0, MAX - 5.0);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::of(10.0, 0.0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::of(0.0, 10.0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::of(10.0, 10.0)), None);
    assert_eq!(p_max, Point::of(MAX - 2.0, MAX - 5.0));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::of(MIN + 1.0, MIN + 1.0);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::of(MIN, 0.0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::of(0.0, MIN)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::min()), None);
    assert_eq!(p_min, Point::of(MIN + 1.0, MIN + 1.0));

    let mut p_max = Point::of(MAX - 1.0, MAX - 1.0);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::of(MAX, 0.0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::of(0.0, MAX)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::max()), None);
    assert_eq!(p_max, Point::of(MAX - 1.0, MAX - 1.0));
}
