use super::saturating_add_assign;
use crate::cartesian::{
    d1::point::point_f32::{MAX, MIN},
    d2::point::point_f32::Point,
};

#[test]
fn test() {
    let mut p = Point::zero();
    saturating_add_assign(&mut p, &Point::new(10.0, 13.0));
    assert_eq!(p, Point::new(10.0, 13.0));
    saturating_add_assign(&mut p, &Point::new(-5.0, -3.0));
    assert_eq!(p, Point::new(5.0, 10.0));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::new(MIN + 2.0, MIN + 5.0);
    saturating_add_assign(&mut p_min, &Point::new(-2.0, -5.0));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::new(MAX - 2.0, MAX - 5.0);
    saturating_add_assign(&mut p_max, &Point::new(2.0, 5.0));
    assert_eq!(p_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::new(MIN + 2.0, MIN + 5.0);
    saturating_add_assign(&mut p_min, &Point::new(-10.0, -10.0));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::new(MAX - 2.0, MAX - 5.0);
    saturating_add_assign(&mut p_max, &Point::new(10.0, 10.0));
    assert_eq!(p_max, Point::max());
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::new(MIN + 1.0, MIN + 1.0);
    saturating_add_assign(&mut p_min, &Point::min());
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::new(MAX - 1.0, MAX - 1.0);
    saturating_add_assign(&mut p_max, &Point::max());
    assert_eq!(p_max, Point::max());
}
