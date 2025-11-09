use super::try_checked_add_assign;
use crate::cartesian::{
    d1::point::point_i32::{MAX, MIN},
    d2::point::point_i32::Point,
};

#[test]
fn test() {
    let mut p = Point::zero();
    assert_eq!(try_checked_add_assign(&mut p, &Point::new(10, 13)), Some(()));
    assert_eq!(p, Point::new(10, 13));
    assert_eq!(try_checked_add_assign(&mut p, &Point::new(-25, -30)), Some(()));
    assert_eq!(p, Point::new(-15, -17));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::new(MIN + 2, MIN + 5);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::new(-2, -5)), Some(()));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::new(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::new(2, 5)), Some(()));
    assert_eq!(p_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::new(MIN + 2, MIN + 5);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::new(-10, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::new(0, -10)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::new(-10, -10)), None);
    assert_eq!(p_min, Point::new(MIN + 2, MIN + 5));

    let mut p_max = Point::new(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::new(10, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::new(0, 10)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::new(10, 10)), None);
    assert_eq!(p_max, Point::new(MAX - 2, MAX - 5));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::new(MIN + 1, MIN + 1);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::new(MIN, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::new(0, MIN)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::min()), None);
    assert_eq!(p_min, Point::new(MIN + 1, MIN + 1));

    let mut p_max = Point::new(MAX - 1, MAX - 1);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::new(MAX, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::new(0, MAX)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::max()), None);
    assert_eq!(p_max, Point::new(MAX - 1, MAX - 1));
}
