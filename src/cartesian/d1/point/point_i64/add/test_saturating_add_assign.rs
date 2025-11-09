use super::saturating_add_assign;
use crate::cartesian::d1::point::point_i64::{MAX, MIN, Point};

#[test]
fn test() {
    let mut p = Point::zero();
    saturating_add_assign(&mut p, &Point::new(10));
    assert_eq!(p, Point::new(10));
    saturating_add_assign(&mut p, &Point::new(-5));
    assert_eq!(p, Point::new(5));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::new(MIN + 2);
    saturating_add_assign(&mut p_min, &Point::new(-2));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::new(MAX - 2);
    saturating_add_assign(&mut p_max, &Point::new(2));
    assert_eq!(p_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::new(MIN + 2);
    saturating_add_assign(&mut p_min, &Point::new(-10));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::new(MAX - 2);
    saturating_add_assign(&mut p_max, &Point::new(10));
    assert_eq!(p_max, Point::max());
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::new(MIN + 1);
    saturating_add_assign(&mut p_min, &Point::min());
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::new(MAX - 1);
    saturating_add_assign(&mut p_max, &Point::max());
    assert_eq!(p_max, Point::max());
}
