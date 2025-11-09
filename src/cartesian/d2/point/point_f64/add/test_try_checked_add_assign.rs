use super::try_checked_add_assign;
use crate::cartesian::{
    d1::point::point_f64::{MAX, MIN},
    d2::point::point_f64::Point,
};

#[test]
fn test() {
    let mut p = Point::zero();
    assert_eq!(try_checked_add_assign(&mut p, &Point::new(10.0, 13.0)), Some(()));
    assert_eq!(p, Point::new(10.0, 13.0));
    assert_eq!(try_checked_add_assign(&mut p, &Point::new(-25.0, -30.0)), Some(()));
    assert_eq!(p, Point::new(-15.0, -17.0));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::new(MIN + 2.0, MIN + 5.0);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::new(-2.0, -5.0)), Some(()));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::new(MAX - 2.0, MAX - 5.0);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::new(2.0, 5.0)), Some(()));
    assert_eq!(p_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::new(MIN + 2.0, MIN + 5.0);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::new(-10.0, 0.0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::new(0.0, -10.0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::new(-10.0, -10.0)), None);
    assert_eq!(p_min, Point::new(MIN + 2.0, MIN + 5.0));

    let mut p_max = Point::new(MAX - 2.0, MAX - 5.0);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::new(10.0, 0.0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::new(0.0, 10.0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::new(10.0, 10.0)), None);
    assert_eq!(p_max, Point::new(MAX - 2.0, MAX - 5.0));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::new(MIN + 1.0, MIN + 1.0);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::new(MIN, 0.0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::new(0.0, MIN)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &Point::min()), None);
    assert_eq!(p_min, Point::new(MIN + 1.0, MIN + 1.0));

    let mut p_max = Point::new(MAX - 1.0, MAX - 1.0);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::new(MAX, 0.0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::new(0.0, MAX)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &Point::max()), None);
    assert_eq!(p_max, Point::new(MAX - 1.0, MAX - 1.0));
}
