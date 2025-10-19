use super::try_checked_add_assign;
use crate::cartesian::{
    d1::point::point_i64::{MAX, MIN},
    d2::point::point_i64::Point,
};

#[test]
fn test() {
    let mut p = Point::of(0, 0);
    assert_eq!(try_checked_add_assign(&mut p, &Point::of(10, 13)), Some(()));
    assert_eq!(p, Point::of(10, 13));
    assert_eq!(try_checked_add_assign(&mut p, &Point::of(-25, -30)), Some(()));
    assert_eq!(p, Point::of(-15, -17));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::of(MIN + 2, MIN + 5);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::of(-2, -5)), Some(()));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::of(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::of(2, 5)), Some(()));
    assert_eq!(p_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::of(MIN + 2, MIN + 5);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::of(-10, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::of(0, -10)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::of(-10, -10)), None);
    assert_eq!(p_min, Point::of(MIN + 2, MIN + 5));

    let mut p_max = Point::of(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::of(10, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::of(0, 10)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::of(10, 10)), None);
    assert_eq!(p_max, Point::of(MAX - 2, MAX - 5));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::of(MIN + 1, MIN + 1);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::of(MIN, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::of(0, MIN)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::min()), None);
    assert_eq!(p_min, Point::of(MIN + 1, MIN + 1));

    let mut p_max = Point::of(MAX - 1, MAX - 1);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::of(MAX, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::of(0, MAX)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::max()), None);
    assert_eq!(p_max, Point::of(MAX - 1, MAX - 1));
}
