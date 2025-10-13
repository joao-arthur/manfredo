use super::saturating_add_assign;
use crate::matrix::d1::point::point_f64::{MAX, MIN, Point};

#[test]
fn test() {
    let mut p = Point::of(0.0);
    saturating_add_assign(&mut p, &Point::of(10.0));
    assert_eq!(p, Point::of(10.0));
    saturating_add_assign(&mut p, &Point::of(-5.0));
    assert_eq!(p, Point::of(5.0));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::of(MIN + 2.0);
    saturating_add_assign(&mut p_min, &Point::of(-2.0));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::of(MAX - 2.0);
    saturating_add_assign(&mut p_max, &Point::of(2.0));
    assert_eq!(p_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::of(MIN + 2.0);
    saturating_add_assign(&mut p_min, &Point::of(-10.0));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::of(MAX - 2.0);
    saturating_add_assign(&mut p_max, &Point::of(10.0));
    assert_eq!(p_max, Point::max());
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::of(MIN + 1.0);
    saturating_add_assign(&mut p_min, &Point::min());
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::of(MAX - 1.0);
    saturating_add_assign(&mut p_max, &Point::max());
    assert_eq!(p_max, Point::max());
}
